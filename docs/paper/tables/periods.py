#!/usr/bin/env python3
"""Splits the paper's data into its two periods (backlog 0568, part 2), as the owner answered on 15 October 2026 and
narrowed on 21 October 2026: the paper is not written yet, so its 4 October cut-off is arbitrary; break the timeline
where `sv` replaced v1, compare version 1 with StackVet as it is today, build every table per period, and combined
tables for everything up to the day of the build.

    python3 docs/paper/tables/periods.py            # writes the tables, the splits, and figure-periods.html
    python3 docs/paper/tables/periods.py --check    # says which of them differ from what is committed

The two periods, each boundary read from git, never typed:
  1. version 1 (SecureVibe): from the first commit of this history (18 September 2026; the history before it was
     destroyed, CORRECTIONS.md) to the tag `v1-final`, the last commit of v1 on main;
  2. StackVet: from the merge that promoted `sv` to the top of the repository (the first commit on main to hold
     "Promote sv to the top of the repository; v1 moves to the v1 branch") to the commit the tables are built at.
     The rename to StackVet (the first commit on main to hold "StackVet step 3: ...", ADR-062, 8 October) is inside
     it; until 21 October 2026 it began a third period, and the owner chose to fold the two together.

A row's period is the one its date (Eastern time) falls in; a day the boundary falls in counts from the boundary's
own time. Reads git and the repository's own files, with Python's standard library; writes under docs/paper/ only.
"""
import argparse
import csv
import io
import re
import subprocess
import sys
from collections import Counter, defaultdict
from datetime import datetime
from pathlib import Path
from zoneinfo import ZoneInfo

ROOT = Path(__file__).resolve().parents[3]
TABLES = ROOT / "docs" / "paper" / "tables"
SPLIT = TABLES / "by-period"
FIGURE = ROOT / "docs" / "paper" / "figure-periods.html"
ET = ZoneInfo("America/New_York")

PROMOTION = "Promote sv to the top of the repository; v1 moves to the v1 branch"
RENAME = "StackVet step 3: "
NAMES = {
    "v1": "version 1",
    "stackvet": "StackVet",
}

# What a test is, per language, counted in the tree at a commit. v1 was TypeScript; sv is Rust.
TEST_MEASURES = [
    ("Rust", r"#\[(tokio::)?test", ["*.rs"]),
    ("TypeScript", r"\b(it|test)\(", ["*.test.ts", "*.test.tsx", "*.spec.ts"]),
    ("Python", r"^\s*def test_", ["*.py"]),
]

# The hand-made tables at the 4 October cut-off (docs/paper/DATA.md), and the column each row's date is in. A table
# with no date column is one dated record: its date is given here, with where DATA.md says it comes from.
HAND_TABLES = {
    "coordination.csv": ("date", None),
    "corrections.csv": ("date", None),
    "faults.csv": ("found_date", None),
    "tests_by_day.csv": ("date", None),
    "review.csv": (None, "2026-10-04"),
    "study.csv": (None, "2026-10-03"),
    "requirements.csv": (None, "2026-09-20"),
    "findings.csv": (None, "2026-09-20"),
}


def git(*args):
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=True, capture_output=True, text=True
    ).stdout


def when(iso):
    return datetime.fromisoformat(iso.strip()).astimezone(ET)


def commit_of(subject_start, until):
    """The one commit, not a merge, whose subject starts with `subject_start`."""
    for line in git("log", "--no-merges", "--format=%H\t%s", until).splitlines():
        sha, subject = line.split("\t", 1)
        if subject.startswith(subject_start):
            return sha
    sys.exit(f"periods.py: no commit on {until} starts with {subject_start!r}")


def first_on_main(sha, main):
    """The first commit on main's own line that holds `sha`: the merge that brought it in. Once a commit is on main,
    every later commit on main holds it, so the first is found by halving."""

    def holds(i):
        return subprocess.run(
            ["git", "merge-base", "--is-ancestor", sha, main[i][0]], cwd=ROOT
        ).returncode == 0

    if not holds(len(main) - 1):
        sys.exit(f"periods.py: {sha} is not on main")
    low, high = 0, len(main) - 1
    while low < high:
        middle = (low + high) // 2
        if holds(middle):
            high = middle
        else:
            low = middle + 1
    return main[low]


def boundaries(until):
    """Main's own line, oldest first, and the two periods as (key, first commit, its time, last commit, its time)."""
    main = []
    for line in git("log", "--first-parent", "--format=%H\t%cI\t%s", until).splitlines():
        sha, iso, subject = line.split("\t", 2)
        main.append((sha, when(iso), subject))
    main.reverse()
    v1_end = git("rev-parse", "v1-final^{commit}").strip()
    v1_end_at = when(git("log", "-1", "--format=%cI", v1_end))
    promotion = first_on_main(commit_of(PROMOTION, until), main)
    return main, [
        ("v1", main[0][0], main[0][1], v1_end, v1_end_at),
        ("stackvet", promotion[0], promotion[1], main[-1][0], main[-1][1]),
    ]


def period_of(moment, periods):
    """The period a time (or an ISO date, read as the end of that day) is in."""
    if isinstance(moment, str):
        moment = datetime.fromisoformat(moment).replace(hour=23, minute=59, tzinfo=ET)
    key = periods[0][0]
    for p in periods:
        if moment >= p[2]:
            key = p[0]
    return key


def tests_at(sha):
    counts = {}
    for language, pattern, globs in TEST_MEASURES:
        out = subprocess.run(
            ["git", "grep", "-c", "-E", pattern, sha, "--", *globs],
            cwd=ROOT, capture_output=True, text=True,
        ).stdout
        counts[language] = sum(int(line.rsplit(":", 1)[1]) for line in out.splitlines() if line)
    return counts


def as_csv(header, rows):
    out = io.StringIO()
    writer = csv.writer(out, lineterminator="\n")
    writer.writerow(header)
    writer.writerows(rows)
    return out.getvalue()


def short(sha):
    return sha[:9]


def stamp(t):
    return t.strftime("%Y-%m-%d %H:%M")


# --- The combined tables, read from git to the commit they are built at. ------------------------------------------


def days_table(main, periods, until):
    """One row per day and period on main (the day of a boundary has a row for each side): commits, pull requests
    merged, and the tests and records in the tree at the end of that part of the day."""
    all_commits = Counter()
    for iso in git("log", "--format=%cI", until).splitlines():
        t = when(iso)
        all_commits[(t.date().isoformat(), period_of(t, periods))] += 1
    by_day = defaultdict(list)
    for sha, t, subject in main:
        by_day[(t.date().isoformat(), period_of(t, periods))].append((sha, t, subject))
    done = Counter()
    for _, _, _, status_date, _ in backlog_rows(until):
        if status_date:
            done[status_date] += 1
    order = {key: i for i, key in enumerate(NAMES)}
    rows = []
    for day, key in sorted(by_day, key=lambda k: (k[0], order[k[1]])):
        line = by_day[(day, key)]
        last = line[-1][0]
        tests = tests_at(last)
        adrs = sum(
            1 for p in git("ls-tree", "--name-only", last, "docs/adr/").split()
            if re.search(r"ADR-\d+\.md$", p)
        )
        merged = sum(1 for _, _, s in line if re.match(r"Merge pull request #\d+", s))
        # A backlog item's status line gives its day, not its time: on a boundary's day it is counted on the later side.
        later_side = key == max((k for d, k in by_day if d == day), key=order.get)
        rows.append([
            day, key, all_commits[(day, key)], len(line), merged,
            tests["Rust"], tests["TypeScript"], tests["Python"], adrs, done.get(day, 0) if later_side else 0,
            short(last), f"git log of main to {short(until)}; the tree at {short(last)}",
        ])
    header = [
        "date", "period", "commits", "commits_on_main", "pull_requests_merged", "tests_rust", "tests_typescript",
        "tests_python", "decision_records_in_tree", "backlog_items_done", "last_commit_on_main", "source",
    ]
    return header, rows


def adr_rows(until):
    rows = []
    for path in sorted(git("ls-tree", "--name-only", until, "docs/adr/").split()):
        m = re.search(r"ADR-(\d+)\.md$", path)
        if not m:
            continue
        text = git("show", f"{until}:{path}")
        title = text.splitlines()[0].lstrip("# ").strip()
        status = re.search(r"^\*\*Status:\*\*\s*([^,.\n]*)", text, re.M)
        # The first commit that touched the file: a record can arrive by a rename (a number taken twice and changed) or
        # in a merge, which `--diff-filter=A` alone misses.
        added = git("log", "-m", "--format=%cI", until, "--", path).split()
        rows.append((m.group(1), title, status.group(1).strip() if status else "",
                     when(added[-1]) if added else None, len(re.findall(r"^## Later", text, re.M)), path))
    return rows


def adrs_table(periods, until):
    rows = [
        [number, title, status, stamp(added) if added else "", period_of(added, periods) if added else "",
         later, path]
        for number, title, status, added, later, path in adr_rows(until)
    ]
    return ["adr", "title", "status", "added_et", "period_added", "later_entries", "source"], rows


MONTHS = {m: i for i, m in enumerate(
    "January February March April May June July August September October November December".split(), 1)}


def backlog_rows(until):
    rows = []
    for path in sorted(git("ls-tree", "-r", "--name-only", until, "docs/backlog/").split()):
        m = re.search(r"(?:^|/)(\d{4})-[^/]*\.md$", path)
        if not m:
            continue
        text = git("show", f"{until}:{path}")
        title = text.splitlines()[0].lstrip("# ").strip()
        status = re.search(r"^\*\*Status:\*\*\s*(.*)$", text, re.M)
        status = status.group(1).strip() if status else ""
        day = re.search(r"done,\s*(\d{1,2}) (\w+) (\d{4})", status)
        status_date = (
            f"{day.group(3)}-{MONTHS[day.group(2)]:02d}-{int(day.group(1)):02d}"
            if day and day.group(2) in MONTHS else ""
        )
        rows.append((m.group(1), title, status, status_date, path))
    return rows


def backlog_table(periods, until):
    rows = [
        [number, title, status.split(",")[0].split(" ")[0], status_date,
         period_of(status_date, periods) if status_date else "", path]
        for number, title, status, status_date, path in backlog_rows(until)
    ]
    return ["item", "title", "status", "done_date", "period_done", "source"], rows


def periods_table(main, periods, days_rows, adrs, until):
    rows = []
    for key, first, first_at, last, last_at in periods + [("all", main[0][0], main[0][1], main[-1][0], main[-1][1])]:
        mine = [r for r in days_rows if key == "all" or r[1] == key]
        how = {
            "v1": "from the first commit of this history to the tag v1-final",
            "stackvet": f"from the merge that holds \"{PROMOTION}\" to the build; the rename to StackVet "
            f"(\"{RENAME.strip()} ...\", ADR-062) is inside it",
            "all": "every period",
        }[key]
        end = mine[-1] if mine else None
        rows.append([
            key, NAMES.get(key, "both"), short(first), stamp(first_at), short(last), stamp(last_at),
            len({r[0] for r in mine}), sum(r[2] for r in mine), sum(r[3] for r in mine), sum(r[4] for r in mine),
            end[5] if end else "", end[6] if end else "",
            sum(1 for a in adrs if a[4] == key) if key != "all" else len(adrs),
            sum(r[9] for r in mine), how,
        ])
    header = [
        "period", "name", "first_commit", "first_et", "last_commit", "last_et", "days", "commits", "commits_on_main",
        "pull_requests_merged", "tests_rust_at_end", "tests_typescript_at_end", "decision_records_added",
        "backlog_items_done", "boundary",
    ]
    return header, rows


# --- The period of every other table's rows. -------------------------------------------------------------------------


def with_period(path, date_column, fixed, periods):
    """A table's rows with a `period` column added, from its date column or its one fixed date."""
    rows = list(csv.reader(io.StringIO(path.read_text(encoding="utf-8"))))
    header, body = rows[0], rows[1:]
    if "period" in header:
        return header, body
    at = header.index(date_column) if date_column else None
    out = []
    for row in body:
        day = row[at][:10] if at is not None and len(row) > at else fixed
        out.append(row + [period_of(day, periods) if re.match(r"\d{4}-\d{2}-\d{2}", day or "") else ""])
    return header + ["period"], out


def generated_dates():
    """The tables make_tables.py writes, and how a row's date is read: a column, or the record's own dates."""
    return {
        "loop2_builds.csv": (None, "2026-10-13"),
        "three_tool_route.csv": (None, "2026-10-14"),
        "three_tool_prompts.csv": (None, "2026-10-14"),
        "three_tool_reports.csv": (None, "2026-10-14"),
        "three_tool_said_did.csv": (None, "2026-10-14"),
        "three_tool_gaming.csv": (None, "2026-10-14"),
        "pointer_trial.csv": (None, "2026-10-14"),
        "podman_measurement.csv": (None, "2026-10-14"),
        "deep_review.csv": (None, "2026-10-13"),
        "board_log.csv": ("date", None),
        "board_items.csv": ("finished", None),
    }


# --- The figure: the two periods side by side, and both together. -------------------------------------------------


COLORS = {"v1": "#7a5195", "stackvet": "#2f7ed8"}


def tests_said(rust, typescript):
    parts = [f"{n} {language}" for n, language in ((rust, "Rust"), (typescript, "TypeScript")) if n]
    return ", ".join(parts) or "0"


def figure(days_rows, periods_rows, until):
    width, height, left, bottom = 860, 300, 56, 40
    days = [r[0] for r in days_rows]
    top_commits = max(r[2] for r in days_rows) or 1
    top_tests = max(r[5] + r[6] for r in days_rows) or 1
    step = (width - left - 20) / max(len(days), 1)
    bars, line, labels = [], [], []
    for i, r in enumerate(days_rows):
        x = left + i * step
        h = (height - bottom - 20) * r[2] / top_commits
        bars.append(
            f'<rect x="{x + 1:.1f}" y="{height - bottom - h:.1f}" width="{max(step - 2, 1):.1f}" height="{h:.1f}" '
            f'fill="{COLORS[r[1]]}"><title>{r[0]}: {r[2]} commits, {r[4]} pull requests merged '
            f'({NAMES[r[1]]})</title></rect>'
        )
        y = height - bottom - (height - bottom - 20) * (r[5] + r[6]) / top_tests
        line.append(f"{x + step / 2:.1f},{y:.1f}")
        if i % 4 == 0:
            labels.append(
                f'<text x="{x + step / 2:.1f}" y="{height - bottom + 16}" text-anchor="middle">{r[0][5:]}</text>'
            )
    table = "".join(
        "<tr>" + "".join(f"<td>{c}</td>" for c in (r[1], f"{r[3]} to {r[5]}", r[6], r[7], r[9], tests_said(r[10], r[11]), r[12], r[13]))
        + "</tr>"
        for r in periods_rows
    )
    legend = " ".join(
        f'<span><i style="background:{COLORS[k]}"></i>{NAMES[k]}</span>' for k in ("v1", "stackvet")
    )
    return f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>The two periods</title>
<style>
:root {{ --ink: #1d1d1f; --muted: #5f6368; --rule: #d9d9de; --page: #ffffff; }}
@media (prefers-color-scheme: dark) {{ :root {{ --ink: #ececf1; --muted: #a8adb4; --rule: #3a3b40; --page: #17181b; }} }}
body {{ background: var(--page); color: var(--ink); font: 15px/1.5 system-ui, sans-serif; margin: 0 auto; max-width: 900px; padding: 16px; }}
svg {{ width: 100%; height: auto; }} svg text {{ fill: var(--muted); font-size: 11px; }}
table {{ border-collapse: collapse; width: 100%; font-size: 13px; }} td, th {{ border-bottom: 1px solid var(--rule); padding: 4px 6px; text-align: left; }}
.legend span {{ margin-right: 14px; }} .legend i {{ display: inline-block; width: 10px; height: 10px; margin-right: 4px; }}
.note {{ color: var(--muted); font-size: 13px; }}
</style>
</head>
<body>
<h1>The two periods</h1>
<p class="legend">{legend} <span>— the line: tests in the tree (Rust and TypeScript) at each day's end</span></p>
<svg viewBox="0 0 {width} {height}" role="img" aria-label="Commits per day, colored by period, with the tests in the tree as a line">
{''.join(bars)}
<polyline points="{' '.join(line)}" fill="none" stroke="currentColor" stroke-width="1.5"/>
<line x1="{left}" y1="{height - bottom}" x2="{width - 20}" y2="{height - bottom}" stroke="currentColor" stroke-opacity=".3"/>
<text x="{left - 6}" y="24" text-anchor="end">{top_commits}</text>
<text x="{left}" y="12">commits per day</text>
<text x="{left - 6}" y="{height - bottom}" text-anchor="end">0</text>
{''.join(labels)}
</svg>
<table>
<tr><th>period</th><th>dates (Eastern)</th><th>days</th><th>commits</th><th>pull requests merged</th><th>tests at the end</th><th>decision records added</th><th>backlog items done</th></tr>
{table}
</table>
<p class="note">Made by docs/paper/tables/periods.py from git, to commit {short(until)}; each boundary is named in
docs/paper/DATA.md. Commits are every commit reachable from main that day; bars are in the period of the day's last
commit on main. Tests are counted in the tree, not run: Rust's #[test] and #[tokio::test], and TypeScript's it( and
test( in test files. Backlog items done are read from each item's status line, which exists from 9 October; earlier
work is not counted there. Data: docs/paper/tables/periods.csv and days.csv.</p>
</body>
</html>
"""


def outputs(until):
    main, periods = boundaries(until)
    days_header, days_rows = days_table(main, periods, until)
    adrs = adr_rows(until)
    adrs_header, adrs_rows = adrs_table(periods, until)
    adrs_by_period = [(a[0], a[1], a[2], a[3], period_of(a[3], periods) if a[3] else "") for a in adrs]
    periods_header, periods_rows = periods_table(main, periods, days_rows, adrs_by_period, until)
    backlog_header, backlog_rows_ = backlog_table(periods, until)
    combined = {
        "periods.csv": (periods_header, periods_rows),
        "days.csv": (days_header, days_rows),
        "decision_records.csv": (adrs_header, adrs_rows),
        "backlog_items.csv": (backlog_header, backlog_rows_),
    }
    files = {TABLES / name: as_csv(*table) for name, table in combined.items()}
    tagged = dict(combined)
    for name, (column, fixed) in HAND_TABLES.items():
        tagged[name] = with_period(ROOT / "docs" / "paper" / name, column, fixed, periods)
    for name, (column, fixed) in generated_dates().items():
        tagged[name] = with_period(TABLES / name, column, fixed, periods)
    for name, (header, rows) in tagged.items():
        if name == "periods.csv":
            continue
        at = next(i for i, h in enumerate(header) if h.startswith("period"))
        if name in HAND_TABLES or name in generated_dates():
            files[SPLIT / "all" / name] = as_csv(header, rows)
        for key in NAMES:
            mine = [r for r in rows if len(r) > at and r[at] == key]
            if mine:
                files[SPLIT / key / name] = as_csv(header, mine)
    files[FIGURE] = figure(days_rows, periods_rows, until)
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="say which outputs differ from what is committed")
    parser.add_argument("--until", default="HEAD", help="the commit to build at (default: HEAD)")
    args = parser.parse_args()
    until = args.until
    if args.check and (TABLES / "periods.csv").exists():
        # The tables are built at one commit, named in periods.csv; checking against it keeps the check stable.
        for row in csv.DictReader(io.StringIO((TABLES / "periods.csv").read_text(encoding="utf-8"))):
            if row["period"] == "all":
                until = row["last_commit"]
    until = git("rev-parse", until).strip()
    files = outputs(until)
    if args.check:
        stale = [p for p, text in files.items() if not p.exists() or p.read_text(encoding="utf-8") != text]
        for p in stale:
            print(f"{p.relative_to(ROOT)} differs from what periods.py writes at {short(until)}")
        sys.exit(1 if stale else 0)
    for path, text in files.items():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    print(f"wrote {len(files)} files at {short(until)}")


if __name__ == "__main__":
    main()
