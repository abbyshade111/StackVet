#!/usr/bin/env python3
"""Extends the paper's four hand-made ledgers past the 4 October cut-off to the day of the build (backlog 0568, part 3),
as the owner answered on 15 October 2026: extend them to today, then redraw the figures from them per period.

    python3 docs/paper/tables/ledgers.py            # writes docs/paper/tables/to-today/ and the period figures
    python3 docs/paper/tables/ledgers.py --check    # says which outputs differ from what is committed

Each ledger to today is the ledger at the cut-off, row for row and unchanged, followed by the rows since:
- faults.csv: the rows since are compiled by sessions reading each record (`since-cutoff/faults.csv`); the faults that
  were open at the cut-off carry their fix since in their own columns (`since-cutoff/open_at_cutoff.csv`), and the
  ledger's own columns say what they were at the cut-off.
- corrections.csv: compiled the same way (`since-cutoff/corrections.csv`).
- coordination.csv: claims and merges resolved by hand are counted from git, as COORDINATION.md counted them;
  pull requests closed unmerged are read from GitHub (`since-cutoff/closed_unmerged.csv`); duplicates, collisions,
  finds in another session's work, and work sent between sessions are compiled by reading (`since-cutoff/coordination.csv`).
- tests_by_day.csv: the `#[test]` lines on main at each day's last commit, as TESTS-AND-FAULTS.md counted them (it
  reproduces the published 1,645 and 1,655 exactly), lines changed from the same paths, and the faults found that day.

Every row has its period (docs/paper/tables/periods.py) and says whether it was in the ledger at the cut-off or added
since. Reads git and the repository's own files, with Python's standard library; writes under docs/paper/ only.
"""
import argparse
import csv
import io
import re
import subprocess
import sys
from collections import Counter, defaultdict
from datetime import datetime, timedelta
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import periods as P  # noqa: E402  (the two periods and their boundaries)

ROOT = P.ROOT
PAPER = ROOT / "docs" / "paper"
SINCE = P.TABLES / "since-cutoff"
OUT = P.TABLES / "to-today"
CUTOFF_SHA = "157ddc3"
LINE_PATHS = ["crates/", "tools/", "examples/", "Dockerfile", "Cargo.toml"]
MAIN_MERGE = re.compile(
    r"merge (branch )?'?(origin/)?main|merge main|bring main|merge remote-tracking branch 'origin/main'", re.I
)


def git(*args):
    return P.git(*args)


def read_csv(path):
    if not path.exists():
        return [], []
    rows = list(csv.reader(io.StringIO(path.read_text(encoding="utf-8"))))
    return rows[0], rows[1:]


def cutoff_at():
    return P.when(git("log", "-1", "--format=%cI", CUTOFF_SHA))


# --- Coordination from git: claims and merges resolved by hand, after the cut-off. ---------------------------------


def claims(until, after):
    rows = []
    for line in git("log", "--first-parent", "--merges", f"--since={after.isoformat()}", "--format=%H%x1f%cI%x1f%s%x1f%b%x1e",
                    until).split("\x1e"):
        if not line.strip():
            continue
        sha, iso, subject, body = (line.strip("\n").split("\x1f") + [""])[:4]
        t = P.when(iso)
        if t <= after:
            continue
        changed = git("diff", "--name-only", f"{sha}^1", sha).split()
        title = body.strip().splitlines()[0] if body.strip() else subject
        branch = subject.split(" from ")[-1]
        if changed and all(p.startswith(("docs/backlog/", "docs/BACKLOG.md", "docs/coordination/")) for p in changed) \
                and re.search(r"claim", f"{branch} {title}", re.I):
            pr = re.search(r"#(\d+)", subject)
            rows.append([t.strftime("%Y-%m-%d"), t.strftime("%H:%M"), "claim", "",
                         f"claim: {title} [backlog-only change]", "one change to main that touched only the backlog",
                         f"{sha[:7]} (#{pr.group(1)})" if pr else sha[:7]])
    return rows


def conflicts(until, after):
    rows = []
    for line in git("log", "--merges", f"--since={after.isoformat()}", "--format=%H%x1f%cI%x1f%s", until).splitlines():
        sha, iso, subject = line.split("\x1f", 2)
        t = P.when(iso)
        if t <= after or not MAIN_MERGE.search(subject):
            continue
        diff = git("show", "--cc", "--format=", sha)
        files, lines, current = [], 0, None
        for d in diff.splitlines():
            if d.startswith("diff --cc "):
                current = d[len("diff --cc "):]
            elif d.startswith(("++", "+ ", " +", "--", "- ", " -")) and not d.startswith(("+++", "---")):
                lines += 1
                if current and current not in files:
                    files.append(current)
        if lines:
            rows.append([t.strftime("%Y-%m-%d"), t.strftime("%H:%M"), "conflict", "",
                         f"{subject} — hand-resolved in: {', '.join(files)}",
                         f"{lines} lines of combined diff that match neither parent", sha[:8]])
    return rows


def closed_unmerged():
    header, body = read_csv(SINCE / "closed_unmerged.csv")
    rows = []
    for number, created, closed, title in body:
        t = datetime.fromisoformat(created.replace("Z", "+00:00")).astimezone(P.ET)
        rows.append([t.strftime("%Y-%m-%d"), t.strftime("%H:%M"), "unmerged-pr", "",
                     f'#{number} "{title}": closed unmerged', "a pull request closed without merging",
                     f"GitHub #{number}, closed {closed}"])
    return rows


# --- Tests and lines by day, after the cut-off. --------------------------------------------------------------------


def test_lines(sha):
    out = subprocess.run(["git", "grep", "-c", "-F", "#[test]", sha, "--", "*.rs"], cwd=ROOT, capture_output=True,
                         text=True).stdout
    return sum(int(line.rsplit(":", 1)[1]) for line in out.splitlines() if line)


def changed_lines(a, b):
    """Lines changed between two commits, and how many of them were moved rather than changed: git marks a block that
    left one place and arrived in another, such as a test module taken out of its file into a file of its own
    (`--color-moved`). Moved lines are not counted as changed, as the published rows left large moves out by hand.
    Of the lines changed, those in test files are counted apart too."""
    out = subprocess.run(
        ["git", "-c", "color.diff.old=red", "-c", "color.diff.new=green", "-c", "color.diff.oldMoved=magenta",
         "-c", "color.diff.newMoved=cyan", "diff", "-M", "--color=always", "--color-moved=plain",
         "--color-moved-ws=allow-indentation-change", a, b, "--", *LINE_PATHS],
        cwd=ROOT, capture_output=True, text=True,
    ).stdout
    changed = moved = tests = 0
    path = ""
    for line in out.splitlines():
        plain = re.sub(r"\x1b\[[0-9;]*m", "", line)
        if plain.startswith("+++ b/"):
            path = plain[6:]
            continue
        if line.startswith(("\x1b[31m-", "\x1b[32m+")):
            changed += 1
            if "/tests/" in path or path.endswith(("_tests.rs", "/tests.rs")):
                tests += 1
        elif line.startswith(("\x1b[35m-", "\x1b[36m+")):
            moved += 1
    return changed, moved, tests


def tests_by_day(main, periods, faults_rows, after):
    by_day = defaultdict(list)
    for sha, t, _ in main:
        if t > after:
            by_day[t.strftime("%Y-%m-%d")].append((sha, t))
    found = defaultdict(Counter)
    for r in faults_rows:
        found[r["found_date"]][r["found_by"]] += 1
        if r["kind"] == "test-gap":
            found[r["found_date"]]["test-gap"] += 1
    rows = []
    previous = git("rev-parse", CUTOFF_SHA).strip()
    for day in sorted(by_day):
        last = by_day[day][-1][0]
        level = test_lines(last)
        before = test_lines(previous)
        lines, moved, test_path = changed_lines(previous, last)
        f = found[day]
        total = sum(v for k, v in f.items() if k != "test-gap")
        rows.append([
            day + (" (after 11:37)" if day == after.strftime("%Y-%m-%d") else ""), "sv", level,
            "#[test] lines at the last commit that day on main's own line", level, level - before, last[:7], last[:7],
            "", total, f["failing_test"], "", total - f["failing_test"], total - f["test-gap"], f["test-gap"], lines,
            test_path, moved,
        ])
        previous = last
    return rows


# --- The ledgers to today. ---------------------------------------------------------------------------------------


def with_period(header, rows, date_at, periods, added):
    out = []
    for row in rows:
        day = row[date_at][:10] if len(row) > date_at else ""
        key = P.period_of(day, periods) if re.match(r"\d{4}-\d{2}-\d{2}", day) else ""
        out.append(row + [key, added])
    return header + ["period", "in_the_ledger"], out


# Two columns the rows since the cut-off add: the backlog item a row was read from, and how sure its reading was.
READING = ["backlog_item", "confidence"]


def faults_to_today(periods):
    header, at_cutoff = read_csv(PAPER / "faults.csv")
    fixes = {r[0]: r for r in read_csv(SINCE / "open_at_cutoff.csv")[1]}
    fix_header = ["fixed_since_date", "fixed_since_pr", "fixed_since_test_in_fix", "fixed_since_source"]
    h = header + fix_header + READING
    rows = []
    for r in at_cutoff:
        f = fixes.get(r[0])
        rows.append(r + ([f[1], f[2], f[3], f[4]] if f else ["", "", "", ""]) + ["", f[5] if f else ""])
    h2, rows = with_period(h, rows, h.index("found_date"), periods, "at the cut-off")
    since_header, since = read_csv(SINCE / "faults.csv")
    added = [[dict(zip(since_header, r)).get(c, "") for c in h] for r in since]
    _, added = with_period(h, added, h.index("found_date"), periods, "since the cut-off")
    return h2, rows + added


def simple_to_today(name, date_column, periods, extra=(), reading=False):
    header, at_cutoff = read_csv(PAPER / name)
    h = header + (READING if reading else [])
    pad = [""] * (len(READING) if reading else 0)
    h2, rows = with_period(h, [r + pad for r in at_cutoff], h.index(date_column), periods, "at the cut-off")
    since_header, since = read_csv(SINCE / name)
    added = [[dict(zip(since_header, r)).get(c, "") for c in h] for r in since] + [list(r) + pad for r in extra]
    _, added = with_period(h, added, h.index(date_column), periods, "since the cut-off")
    return h2, rows + added


def outputs(until):
    main, periods = P.boundaries(until)
    after = cutoff_at()
    files = {}
    fh, faults = faults_to_today(periods)
    files[OUT / "faults.csv"] = P.as_csv(fh, faults)
    files[OUT / "corrections.csv"] = P.as_csv(*simple_to_today("corrections.csv", "date", periods, reading=True))
    git_rows = sorted(claims(until, after) + conflicts(until, after) + closed_unmerged(), key=lambda r: (r[0], r[1]))
    files[OUT / "coordination.csv"] = P.as_csv(*simple_to_today("coordination.csv", "date", periods, git_rows,
                                                                    reading=True))
    th, tbody = read_csv(PAPER / "tests_by_day.csv")
    since_faults = [dict(zip(fh, r)) for r in faults if r[-1] == "since the cut-off"]
    tests_rows = tests_by_day(main, periods, since_faults, after)
    th = th + ["lines_moved"]
    th2, trows = with_period(th, [r + [""] for r in tbody], 0, periods, "at the cut-off")
    _, tnew = with_period(th, tests_rows, 0, periods, "since the cut-off")
    files[OUT / "tests_by_day.csv"] = P.as_csv(th2, trows + tnew)
    return files, main, periods


def main_():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--until", default=None, help="the commit to build at (default: the one periods.csv names)")
    args = parser.parse_args()
    until = args.until
    if until is None:
        for row in csv.DictReader(io.StringIO((P.TABLES / "periods.csv").read_text(encoding="utf-8"))):
            if row["period"] == "all":
                until = row["last_commit"]
    until = git("rev-parse", until).strip()
    files, _, _ = outputs(until)
    if args.check:
        stale = [p for p, t in files.items() if not p.exists() or p.read_text(encoding="utf-8") != t]
        for p in stale:
            print(f"{p.relative_to(ROOT)} differs from what ledgers.py writes at {until[:9]}")
        sys.exit(1 if stale else 0)
    for path, text in files.items():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    print(f"wrote {len(files)} files at {until[:9]}")


if __name__ == "__main__":
    main_()
