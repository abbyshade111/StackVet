#!/usr/bin/env python3
"""Redraws, per period, the thirteen hand-drawn figures that `period_figures.py` does not (backlog 0568, part 4), as the
owner answered on 15 October 2026: each of the paper's 19 figures with a panel per period (version 1 and StackVet, the
owner's two of 21 October 2026, StackVet from the promotion of 26 September) and the combined view where it has one. The figures of the 4 October cut-off stay as they were; each one
redrawn here is a file of its own beside it, `figure-<name>-by-period.html`.

    python3 docs/paper/tables/period_records.py            # writes the tables it reads from git, then the figures
    python3 docs/paper/tables/period_records.py --check    # says which differ from what is committed

Every row is placed in a period by the time the record gives it, as `periods.py` places a moment: in the last period
that had begun by then, a date alone being read as the end of that day. A row its record does not date is counted as
"not dated", never guessed into a period. Where a record ends at the cut-off, a period it does not reach says so, and
shows beside it the nearest measure compiled since, named as a different measure where it is one.

Reads, all under docs/paper/: `tables/periods.csv` and `tables/days.csv` (`periods.py`); `tables/cutoff-records/`
(the cut-off documents' tables as dated rows, compiled by reading them: DATA.md says how); `tables/to-today/faults.csv`
(`ledgers.py`); `study.csv`, `requirements.csv`, the self-assessments' `triage.json`, `trials/make_figure.py`'s list,
and the tables of the trials since (`make_tables.py`). From git, at the build's commit, it writes
`tables/reach_history.csv` (every version of `docs/COVERAGE.md` on main's own line) and `tables/snapshots.csv` (what
`sv` was made of at each period's end). Standard library only; writes under docs/paper/ only.
"""
import argparse
import csv
import importlib.util
import io
import json
import re
import subprocess
import sys
from collections import Counter, defaultdict
from datetime import datetime, timedelta, timezone
from pathlib import Path

HERE = Path(__file__).resolve().parent
PAPER = HERE.parent
ROOT = PAPER.parent.parent
RECORDS = HERE / "cutoff-records"
sys.path.insert(0, str(HERE))
from period_figures import (  # noqa: E402
    COLORS, KEYS, NAMES, day_bars, esc, every_day, grouped_table, page, period_legend, rows, tiles,
)

ET = timezone(timedelta(hours=-4))


def git(*args):
    return subprocess.run(["git", *args], cwd=ROOT, check=True, capture_output=True, text=True).stdout


def as_csv(header, body):
    out = io.StringIO()
    w = csv.writer(out, lineterminator="\n")
    w.writerow(header)
    w.writerows(body)
    return out.getvalue()


# --- Periods. ------------------------------------------------------------------------------------------------------


def load_periods():
    ps = {r["period"]: r for r in rows(HERE / "periods.csv")}
    starts = [(k, datetime.fromisoformat(ps[k]["first_et"]).replace(tzinfo=ET)) for k in KEYS]
    return ps, starts


PERIODS, STARTS = load_periods()


def period_of(moment):
    """The period of "YYYY-MM-DD HH:MM" or "YYYY-MM-DD" (read as the end of that day), as periods.py reads it; empty
    when there is no date."""
    moment = (moment or "").strip()
    if not re.match(r"\d{4}-\d{2}-\d{2}", moment):
        return ""
    if len(moment) >= 16:
        at = datetime.fromisoformat(moment[:16]).replace(tzinfo=ET)
    else:
        at = datetime.fromisoformat(moment[:10]).replace(hour=23, minute=59, tzinfo=ET)
    key = KEYS[0]
    for k, start in STARTS:
        if at >= start:
            key = k
    return key


def until():
    return PERIODS["stackvet"]["last_commit"]


def by_period(items, date_of, weight=lambda r: 1):
    counts = Counter()
    for r in items:
        counts[period_of(date_of(r)) or "undated"] += weight(r)
    return counts


def period_panels(render):
    """One section per period, each from `render(key)`, which returns HTML."""
    return "".join(
        f'<h2 style="border-left:6px solid {COLORS[k]};padding-left:8px">{esc(NAMES[k])} '
        f'<span class="muted" style="font-weight:normal;font-size:13px">{esc(PERIODS[k]["first_et"])} to '
        f'{esc(PERIODS[k]["last_et"])}</span></h2>' + render(k)
        for k in KEYS)


PERIOD_NOT_VERSION = ("A period is a span of time, not a version: <code>sv</code>'s rows from before its "
                      "promotion on 26 September, when it was built in <code>agnostic/</code> beside v1, are in "
                      "version 1's period.")


def none_here(text):
    return f'<p class="muted"><i>{text}</i></p>'


def simple_table(header, body, label):
    head = "".join(f"<th>{esc(h)}</th>" for h in header)
    lines = "".join("<tr>" + "".join(
        f'<td class="n">{esc(c)}</td>' if isinstance(c, (int, float)) else f"<td>{c}</td>" for c in r) + "</tr>"
        for r in body)
    return f'<div class="scroll"><table aria-label="{esc(label)}"><tr>{head}</tr>{lines}</table></div>'


def pct(n, d):
    return f"{100 * n / d:.0f}%" if d else "–"


# --- Tables read from git. -----------------------------------------------------------------------------------------

FRAMEWORKS = [("ASVS", "ASVS 5"), ("AISVS", "AISVS 1"), ("Appendix C", "Appendix C"), ("SbD", "Secure by Design")]


def coverage_numbers(text):
    """The first table of COVERAGE.md that starts "| Framework": per framework, can settle, can be credited (when the
    version has that column), and supporting only."""
    lines = text.splitlines()
    out = {}
    for i, line in enumerate(lines):
        if line.startswith("| Framework"):
            head = [c.strip() for c in line.strip("|").split("|")]
            for row in lines[i + 2:]:
                if not row.startswith("|"):
                    break
                cells = [c.strip() for c in row.strip("|").split("|")]
                for key, mark in FRAMEWORKS:
                    if mark in cells[0]:
                        cell = dict(zip(head, cells))
                        num = lambda name: (re.match(r"\d+", cell.get(name, "")) or [""])[0]
                        out[key] = (num("Requirements"), num("Can settle"), num("Can be credited"),
                                    num("Supporting only"))
            break
    return out


def reach_history(stop):
    body = []
    log = git("log", stop, "--first-parent", "--reverse", "--format=%H\t%cI\t%s", "--",
              "docs/COVERAGE.md", "agnostic/docs/COVERAGE.md")
    for line in log.splitlines():
        sha, iso, subject = line.split("\t", 2)
        at = datetime.fromisoformat(iso).astimezone(ET).strftime("%Y-%m-%d %H:%M")
        text = ""
        for path in ("docs/COVERAGE.md", "agnostic/docs/COVERAGE.md"):
            try:
                text = git("show", f"{sha}:{path}")
                break
            except subprocess.CalledProcessError:
                continue
        nums = coverage_numbers(text)
        if "ASVS" not in nums:
            continue
        pr = (re.match(r"Merge pull request #(\d+)", subject) or [None, ""])[1]
        row = [at, period_of(at), sha[:9], f"#{pr}" if pr else ""]
        for key, _ in FRAMEWORKS:
            total, settle, credit, supporting = nums.get(key, ("", "", "", ""))
            row += [total, settle, credit, supporting]
        body.append(row)
    header = ["time_et", "period", "commit", "pull_request"]
    for key, _ in FRAMEWORKS:
        k = key.lower().replace(" ", "_")
        header += [f"{k}_requirements", f"{k}_can_settle", f"{k}_can_be_credited", f"{k}_supporting_only"]
    return as_csv(header, body)


def files_at(sha, prefix):
    return [p[len(prefix):] for p in git("ls-tree", "-r", "--name-only", sha).splitlines() if p.startswith(prefix)]


def json_at(sha, path):
    try:
        return json.loads(git("show", f"{sha}:{path}"))
    except subprocess.CalledProcessError:
        return None


def lines_at(sha, paths):
    total = 0
    for p in paths:
        total += git("show", f"{sha}:{p}").count("\n")
    return total


def snapshots():
    """What `sv` was made of at each period's last commit. In version 1 it was `agnostic/`, beside v1's own program."""
    header = ["period", "commit", "sv_folder", "crates", "rust_files", "rust_lines", "data_files", "outside_tools",
              "ast_rules", "v1_program_files", "v1_program_lines"]
    body = []
    for k in KEYS:
        sha = PERIODS[k]["last_commit"]
        prefix = "agnostic/" if "agnostic/Cargo.toml" in git("ls-tree", "-r", "--name-only", sha, "agnostic/") else ""
        tree = files_at(sha, prefix)
        rust = [prefix + p for p in tree if p.startswith("crates/") and p.endswith(".rs")]
        crates = sorted({p.split("/")[1] for p in tree if p.startswith("crates/") and p.endswith("/Cargo.toml")})
        data = [p for p in tree if p.startswith("data/") and p.endswith(".json")]
        adapters = json_at(sha, f"{prefix}data/adapters.json") or {}
        ast = json_at(sha, f"{prefix}data/ast-rules.json") or {}
        v1 = [p for p in files_at(sha, "") if re.match(r"(server|web|shared|templates)/.*\.(ts|tsx)$", p)]
        body.append([k, sha[:9], prefix.rstrip("/") or "(the top)", len(crates), len(rust), lines_at(sha, rust),
                     len(data), len(adapters.get("adapters", [])), len(ast.get("rules", [])), len(v1),
                     lines_at(sha, v1) if v1 else 0])
    return as_csv(header, body)


# --- The figures. --------------------------------------------------------------------------------------------------


def timeline(days):
    ds = every_day([d["date"] for d in days])
    # A boundary day has a row for each period it touches: each is its own segment of that day's bar.
    commits, prs = defaultdict(list), defaultdict(list)
    for d in days:
        day, k, main = d["date"], d["period"], int(d["commits_on_main"])
        commits[day].append((main, COLORS[k], f"{day}, {NAMES[k]}: {main} on main"))
        prs[day].append((int(d["pull_requests_merged"]), COLORS[k], f"{day}, {NAMES[k]}: {d['pull_requests_merged']}"))
    for d in days:
        off = int(d["commits"]) - int(d["commits_on_main"])
        commits[d["date"]].append((off, "var(--grey)", f"{d['date']}, {NAMES[d['period']]}: {off} on branches"))

    def panel(k):
        p = PERIODS[k]
        busiest = max((d for d in days if d["period"] == k), key=lambda d: int(d["commits"]))
        return tiles([
            (p["days"], "days with work (a boundary day counts in both periods it touches)"),
            (f'{int(p["commits"]):,}', f'commits; {int(p["commits_on_main"]):,} of them on main\'s own line'),
            (f'{int(p["pull_requests_merged"]):,}', "pull requests merged into main"),
            (busiest["commits"], f'commits on {busiest["date"]}, the busiest day'),
            (f'{int(p["tests_rust_at_end"]):,} · {int(p["tests_typescript_at_end"]):,}',
             "Rust · TypeScript tests in the tree at the end"),
            (f'{p["decision_records_added"]} · {p["backlog_items_done"]}', "decision records added · backlog items done"),
        ])

    a = PERIODS["all"]
    return page(
        "Timeline by Period", "How the project unfolded, by period, to today",
        "The figure of the cut-off (<code>figure-timeline.html</code>) ran from 16 September to 4 October, with twelve "
        "milestones picked by hand. Here every number is read from git to the build, and split at the three "
        "boundaries. The milestones since the cut-off are not chosen here: which events are milestones is the "
        "owner's call.",
        tiles([(a["days"], "days"), (f'{int(a["commits"]):,}', "commits"),
               (f'{int(a["pull_requests_merged"]):,}', "pull requests merged"),
               (a["decision_records_added"], "decision records added")])
        + period_panels(panel)
        + "<h2>Commits each day</h2>" + period_legend()
        + '<p class="note">Colored: on main\'s own line; grey: on branches, merged later or not at all.</p>'
        + day_bars(ds, commits, "Commits each day")
        + "<h2>Pull requests merged each day</h2>" + day_bars(ds, prs, "Pull requests merged each day"),
        "Sources: <code>tables/periods.csv</code> and <code>tables/days.csv</code>, made from git by "
        "<code>tables/periods.py</code>. The history in git begins on 18 September; the days with no commit in main's "
        f"history ({', '.join(d[5:] for d in ds if d not in commits)}) have their place on the axis, empty.")


def step_chart(points, label, height=240):
    """A step line of ASVS can-settle over time, each step colored by its period."""
    width, left, bottom = 940, 44, 30
    start = datetime.fromisoformat(points[0][0])
    end = datetime.fromisoformat(points[-1][0])
    span = (end - start).total_seconds() or 1
    top = max(v for _, v, _, _ in points) * 1.1
    x = lambda t: left + (width - left - 12) * (datetime.fromisoformat(t) - start).total_seconds() / span
    y = lambda v: height - bottom - (height - bottom - 12) * v / top
    out = []
    for (t, v, k, title), nxt in zip(points, points[1:] + [None]):
        x2 = x(nxt[0]) if nxt else x(t) + 2
        out.append(f'<line x1="{x(t):.1f}" y1="{y(v):.1f}" x2="{x2:.1f}" y2="{y(v):.1f}" stroke="{COLORS[k]}" '
                   f'stroke-width="2.5"><title>{esc(title)}</title></line>')
        if nxt:
            out.append(f'<line x1="{x2:.1f}" y1="{y(v):.1f}" x2="{x2:.1f}" y2="{y(nxt[1]):.1f}" '
                       f'stroke="{COLORS[nxt[2]]}" stroke-width="1"/>')
    for d in every_day([p[0] for p in points]):
        if d[8:10] in ("01", "05", "10", "15", "20", "26", "30") and d > points[0][0][:10]:
            xd = x(d + " 00:00")
            out.append(f'<text x="{xd:.1f}" y="{height - bottom + 14}" text-anchor="middle">{esc(d[5:])}</text>')
    for v in range(0, int(top), 50):
        out.append(f'<text x="{left - 6}" y="{y(v) + 4:.1f}" text-anchor="end">{v}</text>')
    return (f'<div class="scroll"><svg viewBox="0 0 {width} {height}" role="img" aria-label="{esc(label)}">'
            + "".join(out) + "</svg></div>")


def security(history, arms):
    pts = [(r["time_et"], int(r["asvs_can_settle"]), r["period"] or "v1",
            f'{r["time_et"]}, {r["commit"]} {r["pull_request"]}: {r["asvs_can_settle"]} of {r["asvs_requirements"]}')
           for r in history if r["asvs_can_settle"]]
    last = {}
    for r in history:
        last[r["period"]] = r

    def panel(k):
        r = last.get(k)
        ours = [h for h in history if h["period"] == k]
        if not r:
            return none_here("COVERAGE.md did not exist in this period.")
        first = ours[0]
        table = simple_table(
            ["framework", "requirements", "can settle", "can be credited", "supporting only"],
            [[name, int(r[f"{key}_requirements"] or 0),
              f'{r[f"{key}_can_settle"]} ({pct(int(r[f"{key}_can_settle"] or 0), int(r[f"{key}_requirements"] or 0))})',
              r[f"{key}_can_be_credited"] or "not counted then", r[f"{key}_supporting_only"]]
             for (name, _), key in zip(FRAMEWORKS, ["asvs", "aisvs", "appendix_c", "sbd"])],
            f"Reach at the end of {NAMES[k]}")
        extra = ""
        if k == "v1":
            a = Counter()
            for row in arms:
                if row["standard"].upper().startswith("ASVS"):
                    if row["status"] in NOT_APPLIED:
                        continue
                    a[(row["arm"], "all")] += 1
                    if row["status"] == "pass":
                        a[(row["arm"], "verified")] += 1
            extra = ("<p>v1's own record, from the comparison of 20 September (<code>requirements.csv</code>): "
                     + "; ".join(f"arm {arm}: {a[(arm, 'verified')]} of {a[(arm, 'all')]} verified"
                                 for arm in sorted({x for x, _ in a})) + ". v1 verified only apps it had built "
                     "itself; <code>sv</code>, in <code>agnostic/</code>, was being built beside it.</p>")
        return (extra + f'<p>{len(ours)} versions of COVERAGE.md on main in this period: ASVS requirements some check '
                f'can settle went from {first["asvs_can_settle"]} to {r["asvs_can_settle"]}. At its last commit '
                f'(<code>{r["commit"]}</code>, {esc(r["time_et"])}):</p>' + table)

    first, end = history[0], history[-1]
    return page(
        "Security Reach by Period", "How far sv's checks reach, by period, to today",
        "The figure of the cut-off (<code>figure-security.html</code>) drew the ASVS requirements some check can "
        "settle over 86 versions of <code>docs/COVERAGE.md</code>, to 4 October. Here every version on main's own "
        f"line to the build, {len(history)} of them, read from git. \"Can be credited\" (a clean run can mark it "
        "checked) is a column COVERAGE.md added later; before it, the cell says so.",
        tiles([(f'{first["asvs_can_settle"]} → {end["asvs_can_settle"]}',
                f'ASVS requirements some check can settle, {first["time_et"][:10]} to {end["time_et"][:10]}'),
               (f'{end["asvs_can_settle"]} of {end["asvs_requirements"]}',
                f'at the build ({pct(int(end["asvs_can_settle"]), int(end["asvs_requirements"]))}), '
                f'{end["asvs_can_be_credited"] or "–"} of them can be credited')])
        + "<h2>ASVS requirements some check can settle, each version of COVERAGE.md</h2>" + period_legend()
        + step_chart(pts, "ASVS requirements some check can settle")
        + period_panels(panel),
        "Sources: <code>tables/reach_history.csv</code>, written by this script from git (every version of "
        "<code>docs/COVERAGE.md</code>, or <code>agnostic/docs/COVERAGE.md</code> before the promotion, on main's own "
        "line, the first table of each), and <code>requirements.csv</code>. A requirement that can be settled is one "
        "some check can mark checked or needs attention; it says nothing of whether any app passed it.")


def records_by_period(items, columns, date_key="date", weight=lambda r: 1):
    """A grouped table of `items` by the categories `columns(r)` gives, per period, with an "undated" note."""
    counts = defaultdict(Counter)
    undated = Counter()
    for r in items:
        k = period_of(r[date_key])
        for c in columns(r):
            if k:
                counts[c][k] += weight(r)
            else:
                undated[c] += weight(r)
    return counts, undated


def undated_note(undated, what):
    if not undated:
        return ""
    return (f'<p class="note">Not dated by their record, and so in no period: '
            + ", ".join(f"{esc(c)} {n}" for c, n in sorted(undated.items())) + f" ({sum(undated.values())} {what}).</p>")


SINCE = "since the cut-off"


def faults_since(faults, kinds):
    since = [f for f in faults if f.get("in_the_ledger") == SINCE and f.get("kind") in kinds]
    return Counter(f["period"] for f in since)


def top10(rows_, faults):
    w = lambda r: int(r.get("count") or 1)
    own = [r for r in rows_ if r["section"] == "own_code"]
    fail = [r for r in rows_ if r["section"] == "fail_open"]
    cats = sorted({r["category"] for r in own}, key=lambda c: (c == "none", c))
    counts, undated = records_by_period(own, lambda r: [r["category"]], weight=w)
    fcounts, fundated = records_by_period(fail, lambda r: [r["category"] or "A10"], weight=w)
    since_sec = faults_since(faults, {"security"})
    since_fail = faults_since(faults, {"fail-open"})
    return page(
        "OWASP Top 10 by Period", "The project's security faults against the OWASP Top 10:2025, by period",
        "The figure of the cut-off (<code>figure-top10.html</code>) placed each weakness in StackVet's own code, and "
        "each verdict that failed open, through a CWE on the category's 2025 list, to 4 October. Here those same rows, "
        "each placed in the period its record dates it to. Nothing found since the cut-off has been placed through a "
        "CWE: the faults ledger since has a kind for each fault, not a CWE, so the StackVet period after 4 October is "
        "a count of the ledger's <code>security</code> and <code>fail-open</code> faults, without "
        "a category. Placing them is a reading of each fault, not done here. " + PERIOD_NOT_VERSION,
        "<h2>1. Weaknesses in the project's own code, to the cut-off</h2>" + period_legend()
        + grouped_table(cats, counts, "Own-code weaknesses by category and period") + undated_note(undated, "weaknesses")
        + "<h2>2. Verdicts that failed open (A10), to the cut-off</h2>"
        + grouped_table(sorted(fcounts), fcounts, "Fail-open verdicts by period") + undated_note(fundated, "verdicts")
        + "<h2>3. Since the cut-off, from the faults ledger, without a category</h2>"
        + simple_table(["", *[NAMES[k] for k in KEYS]],
                       [["security faults found", *[since_sec[k] for k in KEYS]],
                        ["fail-open verdicts found", *[since_fail[k] for k in KEYS]]],
                       "Security and fail-open faults since the cut-off")
        + '<p class="note">Version 1 has none since: it ended before the cut-off.</p>',
        "Sources: <code>tables/cutoff-records/top10.csv</code> (the rows of <code>TOP10.md</code>, sections 1 and 2, "
        "each dated: DATA.md says how) and <code>tables/to-today/faults.csv</code> (<code>ledgers.py</code>). What "
        "each version flagged checking itself, and what it found in other apps, are in "
        "<code>figure-self-assessment-by-period.html</code> and <code>figure-study-by-period.html</code>.")


def agentic(rows_, faults):
    kinds = sorted({r["kind"] for r in rows_})
    incidents = [r for r in rows_ if r["kind"] == "incident"]
    risks = sorted({r["risk"] for r in incidents})
    counts, undated = records_by_period(incidents, lambda r: [r["risk"]])
    allk, allu = records_by_period(rows_, lambda r: [r["kind"]])
    lenses = defaultdict(Counter)
    for r in rows_:
        lenses[f'{r["section"]}. {r["lens"]}'][r["kind"]] += 1
    return page(
        "Agentic Top 10 by Period", "The project against the OWASP Top 10 for Agentic Applications, by period",
        "The figure of the cut-off (<code>figure-agentic.html</code>) marked incidents, defenses, open items, and "
        "<code>sv</code>'s checks against the ten agentic risks, from all three lenses, to 4 October. Here the "
        "incidents are placed in the period their record dates them to. Defenses and checks are standing files more "
        "often than events; one whose record names a pull request or commit is dated by it, and the rest are counted "
        "as not dated. No incident since the cut-off has been placed against the agentic list: the faults ledger "
        "since says what kind each fault is, not which agentic risk, so the StackVet period has no panel of its own "
        "here. " + PERIOD_NOT_VERSION,
        "<h2>1. Incidents, by risk and period</h2>" + period_legend()
        + grouped_table(risks, counts, "Agentic incidents by risk and period") + undated_note(undated, "incidents")
        + "<h2>2. Every row, by kind and period</h2>"
        + grouped_table(kinds, allk, "Agentic rows by kind and period") + undated_note(allu, "rows")
        + "<h2>3. The rows by lens, as the cut-off record has them</h2>"
        + simple_table(["lens", *kinds], [[esc(l), *[lenses[l][k] for k in kinds]] for l in sorted(lenses)],
                       "Agentic rows by lens"),
        "Source: <code>tables/cutoff-records/agentic.csv</code>, the rows of <code>AGENTIC.md</code> sections 1 to 5, "
        "each dated by its own text or by the pull request or commit it names (DATA.md says how).")


def decisions(rows_, records):
    who = [("owner", "The owner"), ("both", "Both: the owner raised it, the AI shaped it"), ("ai", "An AI session")]
    chosen = [("owner", "chosen by the owner"), ("ai", "chosen by an AI session"), ("split", "split")]

    def panel(k):
        mine = [r for r in rows_ if period_of(r["date"]) == k]
        added = sum(1 for r in records if r["period_added"] == k)
        note = (f"<p>{added} decision records were added in this period (<code>tables/decision_records.csv</code>); "
                "who proposed and who chose each has not been read from them, so they are not in the table.</p>")
        if not mine:
            return none_here("No decision in DECISIONS.md is dated in this period: its record ends at the cut-off.") + note
        body = [[label, *[sum(1 for r in mine if r["proposed_by"] == p and r["chosen_by"] == c) for c, _ in chosen],
                 sum(1 for r in mine if r["proposed_by"] == p)] for p, label in who]
        return (simple_table(["proposed by", *[c for _, c in chosen], "all"], body, f"Decisions in {NAMES[k]}")
                + f"<p>{sum(1 for r in mine if r['chosen_by'] == 'owner')} of {len(mine)} final calls were the owner's.</p>"
                + (note if added else ""))

    undated = [r for r in rows_ if not period_of(r["date"])]
    return page(
        "Who Decided What, by Period", "Who proposed, and who chose, by period",
        "The figure of the cut-off (<code>figure-decisions.html</code>) placed the 36 decisions that shaped the project "
        "to 4 October by who first put each forward and who made the final call. Here each is placed in the period of "
        "its date in <code>DECISIONS.md</code>. The recommendation dots of the cut-off figure come from transcripts "
        "and are not dated one by one, so they are not split.",
        period_panels(panel)
        + (undated_note(Counter(r["proposed_by"] for r in undated), "decisions") if undated else ""),
        "Sources: <code>tables/cutoff-records/decisions.csv</code> (the table of <code>DECISIONS.md</code>, each "
        "decision dated by its When column) and <code>tables/decision_records.csv</code> (<code>periods.py</code>).")


FOUND_BY_USE = {"owner_use": "the owner, or real use", "real_input": "running it on real input, and the trials",
                "outside_project": "another project using sv"}


def found_by_use(faults):
    counts = defaultdict(Counter)
    for f in faults:
        if f.get("in_the_ledger") != SINCE:
            continue
        by = f.get("found_by", "")
        group = (FOUND_BY_USE[by] if by in FOUND_BY_USE else "a failing test" if by == "failing_test"
                 else "anything else")
        counts[group][f["period"]] += 1
    return counts


def usability(rows_, faults):
    groups = list(dict.fromkeys(r["group"] for r in rows_))
    counts, undated = records_by_period(rows_, lambda r: [r["group"]])
    since = found_by_use(faults)
    return page(
        "Usability by Period", "What people hit that the tests did not, by period",
        "The figure of the cut-off (<code>figure-usability.html</code>) listed every problem found by a person using "
        "the software, or watching it used, to 4 October. Here each is placed in its period. Usability problems since "
        "the cut-off have not been compiled as such (the three-tool trial of 14 October found many, in its own "
        "write-up); the nearest measure compiled is the faults ledger's faults found by use, which leaves out "
        "friction that is not a fault.",
        "<h2>1. Problems found by use, to the cut-off, by group and period</h2>" + period_legend()
        + grouped_table(groups, counts, "Usability problems by group and period") + undated_note(undated, "problems")
        + "<h2>2. Since the cut-off: faults by what found them, from the faults ledger</h2>"
        + grouped_table(list(FOUND_BY_USE.values()) + ["a failing test", "anything else"], since,
                        "Faults since the cut-off by what found them"),
        "Sources: <code>tables/cutoff-records/usability.csv</code> (the problems of <code>figure-usability.html</code> "
        "and <code>figure-how-found.html</code>, each dated: DATA.md says how) and "
        "<code>tables/to-today/faults.csv</code>, rows since the cut-off.")


def how_found(rows_, faults, tests):
    v1 = [r for r in rows_ if r["version"] == "v1"]
    who = Counter(r["found_by"] for r in v1)
    since = found_by_use(faults)
    end = {k: PERIODS[k] for k in KEYS}

    def panel(k):
        if k == "v1":
            return (tiles([("0", "found by v1's test suites: 1,078 server and 215 template tests at "
                                 "<code>v1-paper</code>, all passing"),
                           (str(len(v1)), "found by a person trying to use it: "
                            + ", ".join(f"{esc(b)} {n}" for b, n in who.most_common()))])
                    + '<p class="note">v1\'s record of 18 to 20 September, unchanged.</p>')
        # StackVet runs from the promotion of 26 September: its record of problems found by use before the 4 October
        # cut-off is the usability figure's, and the faults ledger counts from the cut-off on, the whole period's.
        return none_here("Before the 4 October cut-off, this period's record of problems found by use is the usability "
                         "figure's (<code>figure-usability-by-period.html</code>). From the cut-off on, the faults "
                         "ledger:") \
            + simple_table(["found by", "faults since the cut-off"], [[g, since[g][k]] for g in since],
                           f"Faults in {NAMES[k]} since the cut-off") \
            + f'<p class="note">{int(end[k]["tests_rust_at_end"]):,} Rust tests in the tree at the end.</p>'

    return page(
        "How Problems Were Found, by Period", "How the problems were found, by period",
        "The figure of the cut-off (<code>figure-how-found.html</code>) is v1's record: thirteen usability failures in "
        "three days, none of them found by the test suites. No period since has a record of the same kind, usability "
        "failures against tests; the nearest measure is the faults ledger's: what found each fault since the cut-off. "
        "It counts faults, of every kind, so it is a different measure and is labeled so.",
        period_panels(panel),
        "Sources: <code>tables/cutoff-records/usability.csv</code> (v1's thirteen, from "
        "<code>figure-how-found.html</code>), <code>tables/to-today/faults.csv</code>, and "
        "<code>tables/periods.csv</code>.")


# v1's costs, typed from OPTIMIZATION.md as the cut-off figure has them (no table holds them; v1's own call log is on
# the v1 branch, workspace/llm-audit.jsonl).
V1_RUNS = [("AI build", 8, 4.28, 44.9), ("Build without AI", 9, 0.00, 4.1), ("Check with AI", 6, 2.38, 12.0),
           ("Check without AI", 28, 0.00, 0.6)]
V1_SPEND = (63.46, 823, "18 to 20 September, 51 successful runs; 97% of the input tokens served from the prompt cache")


def load_trials():
    spec = importlib.util.spec_from_file_location("trials_figure", PAPER / "trials" / "make_figure.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.TRIALS


def written_at(path):
    """When a trial's write-up first reached the repository, on main's history to the build: the time that places the
    trial in a period, since a trial's dates in its list are days and two periods meet on 8 October."""
    found = [p for p in [path] if (ROOT / p).exists()] or [str(q.relative_to(ROOT)) for q in ROOT.glob(path)]
    if not found:
        return ""
    times = git("log", until(), "--diff-filter=A", "--format=%cI", "--", *found).split()
    if not times:
        return ""
    return min(datetime.fromisoformat(x).astimezone(ET) for x in times).strftime("%Y-%m-%d %H:%M")


def later_trials(loop2, pointer, route):
    """The trials since 8 October, from their own tables: (name, date, builds, dollars or None, write-up, question)."""
    drafts = [r for r in loop2 if r["phase"] == "none"]
    built = [r for r in loop2 if r["phase"] != "none"]
    cost = lambda rs: round(sum(float(r["cost_usd"] or 0) for r in rs), 2)
    tools = sorted({r["tool"] for r in route})
    return [
        ("Drafts trial", written_at("docs/prompts/drafts-trial/results.md"), len(drafts), cost(drafts), "docs/prompts/drafts-trial/results.md",
         "Do the seven draft prompts remove their problems? (controls only: no draft reached a reading)"),
        ("Second loop trial", written_at("docs/prompts/loop-2"), len(built), cost(built), "docs/prompts/loop-2/",
         "The loop with StackVet's plan and brief, phases A, B1, B2, and C"),
        ("Three-tool prompt trial", written_at("docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md"), len(tools), None, "docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md",
         f"Do the setup prompts work in {', '.join(tools)}? (run by the owner in each tool; no dollars recorded)"),
        ("Pointer trial", written_at("docs/backlog/**/0516-*.md"), 0, None, "backlog 0516",
         f"Does sv questions point at the right code? ({len(pointer)} readings of example apps, no builds)"),
    ]


def trial_rows(loop2, pointer, route):
    out = [(n, written_at(src), b, c, src, q) for n, _, b, c, src, q in load_trials()]
    return out + later_trials(loop2, pointer, route)


def trials(all_trials):
    def panel(k):
        mine = [t for t in all_trials if period_of(t[1]) == k]
        if not mine:
            return none_here("No prompt or loop trial was run in this period. v1's own evaluation runs are in "
                             "<code>figure-cost-by-period.html</code>.")
        spent = sum(t[3] for t in mine if t[3] is not None)
        return (tiles([(str(len(mine)), "trials"), (f'{sum(t[2] for t in mine):,}', "builds (or apps)"),
                       (f"${spent:,.2f}", "of the owner's API credit, where recorded")])
                + simple_table(["trial", "write-up first committed", "builds", "cost", "question"],
                               [[esc(n), d, b, f"${c:,.2f}" if c is not None else "not recorded", esc(q)]
                                for n, d, b, c, _, q in mine], f"Trials in {NAMES[k]}"))

    return page(
        "Prompt Trials by Period", "The prompt and loop trials, by period",
        "The figure of the cut-off (<code>figure-trials.html</code>, made by <code>trials/make_figure.py</code>) drew "
        "the eleven trials of 5 to 8 October, all in the StackVet period, before the rename. Here they are with the "
        "trials since, each placed by "
        "its date. The pasted-prompt comparisons of the cut-off figure stay there: no trial since has one that reached "
        "a reading.",
        period_panels(panel),
        "Sources: the list in <code>trials/make_figure.py</code> (each trial's write-up), "
        "<code>tables/loop2_builds.csv</code> (the drafts trial's 20 controls are its phase <code>none</code>), "
        "<code>tables/three_tool_route.csv</code>, and <code>tables/pointer_trial.csv</code>.")


def cost(all_trials):
    def panel(k):
        if k == "v1":
            return (simple_table(["run type", "runs", "median cost", "median minutes"],
                                 [[n, r, f"${c:.2f}", m] for n, r, c, m in V1_RUNS], "v1's runs")
                    + f"<p>${V1_SPEND[0]:.2f} across {V1_SPEND[1]} AI calls, {esc(V1_SPEND[2])}. v1 itself called the "
                    "AI, to write and to review the app it built.</p>")
        mine = [t for t in all_trials if period_of(t[1]) == k]
        spent = sum(t[3] for t in mine if t[3] is not None)
        return (tiles([("$0", "AI spend by <code>sv</code> itself: it makes no AI calls"),
                       (f"${spent:,.2f}", f"of the owner's API credit on {sum(t[2] for t in mine if t[3] is not None)} "
                                          "trial builds, where the cost was recorded")])
                + simple_table(["trial", "builds", "cost", "per build"],
                               [[esc(n), b, f"${c:,.2f}" if c is not None else "not recorded",
                                 f"${c / b:,.2f}" if c and b else "–"] for n, _, b, c, _, _ in mine],
                               f"Spend in {NAMES[k]}"))

    return page(
        "Cost by Period", "What the AI cost, by period",
        "The figure of the cut-off (<code>figure-cost.html</code>) is mostly v1's record: what its runs cost, where "
        "the money went, and what made the work faster. From the StackVet period on, <code>sv</code> makes no AI calls of its "
        "own; the money spent is the AI coding tool's, building apps in the trials. The speed-ups of the cut-off "
        "figure have no record since and are not split.",
        period_panels(panel),
        "Sources: <code>OPTIMIZATION.md</code> (v1's runs, typed here as the cut-off figure has them), "
        "<code>trials/make_figure.py</code>, and <code>tables/loop2_builds.csv</code>. The owner's subscriptions to "
        "the AI coding tools of the three-tool trial are not counted.")


def self_assessment():
    v1 = ("v1, 20 September", "2026-09-20", 195, 160, 21, 14, 2)
    runs = [v1]
    for path, label in [("self-assessment-v2/product-only/triage.json", "sv, 27 September, product code"),
                        ("self-assessment-v2/2026-10-04/product-only/triage.json", "sv, 4 October, product code")]:
        d = json.loads((PAPER / path).read_text(encoding="utf-8"))
        c = Counter(f["verdict"] for f in d["findings"])
        runs.append((label, d["triaged_on"], len(d["findings"]), c["false-alarm"], c["accepted"], c["real"],
                     c["real"]))

    def panel(k):
        mine = [r for r in runs if period_of(r[1]) == k]
        if not mine:
            return none_here("sv has not been run on itself, and triaged, in this period.")
        return simple_table(["run", "findings", "false alarms", "accepted, by design", "left open", "real"],
                            [[esc(r[0]), r[2], f"{r[3]} ({pct(r[3], r[2])})", r[4], r[5], r[6]] for r in mine],
                            f"Self-checks in {NAMES[k]}")

    return page(
        "Self-Checks by Period", "Each version checking itself, by period",
        "The figure of the cut-off (<code>figure-self-assessment.html</code>) set three runs side by side: v1 on 20 "
        "September, <code>sv</code> on 27 September and on 4 October. Each is one dated record, placed here in its "
        "period. For v1, \"left open\" and \"real\" are the cut-off figure's; for <code>sv</code>, the open ones are "
        "the real ones.",
        period_panels(panel),
        "Sources: <code>self-assessment-v2/product-only/triage.json</code> and "
        "<code>self-assessment-v2/2026-10-04/product-only/triage.json</code>, counted here; v1's row is typed from the "
        "cut-off figure, whose source is v1's <code>artifacts/self-assessment/</code> at <code>v1-final</code>.")


NOT_APPLIED = ("not-applicable", "out-of-level")
STUDY_STATUSES = ("applicable", "needs_attention", "checked", "not_verified", "documented", "stated", "attested",
                  "by_hand", "ai_process", "not_verified share of applicable")


def three_tool_panel(three, what):
    tools = sorted({r["tool"] for r in three})
    measures = list(dict.fromkeys(r["measure"] for r in three))
    cell = {(r["tool"], r["measure"]): r["value"] for r in three}
    level = {r["tool"]: r["level"] for r in three}
    return (f"<p>{what}</p>" + simple_table(["measure", *[f"{t} ({level[t]})" for t in tools]],
                                          [[esc(m), *[esc(cell.get((t, m), "")) for t in tools]] for m in measures],
                                          "The three-tool trial's reports"))


def study(study_rows, three):
    apps = list(dict.fromkeys(r["app"] for r in study_rows))
    where = [r for r in study_rows if r["run"] == "run-11"]
    measures = [m for m in dict.fromkeys(r["measure"] for r in where if r["lens"] == "sv findings")
                if m.startswith(("findings", "own code:"))]
    cell = {(r["app"], r["measure"]): r["value"] for r in where if r["lens"] == "sv findings"}

    def panel(k):
        if k == "v1":
            return none_here("No study in this period: v1's comparison of 20 September is "
                             "<code>figure-three-arms-by-period.html</code>.")
        # The study of 3 October, before the rename, and the one record since of apps checked by `sv`.
        return ("<p>The comparison study, run 11, 3 October, before the rename: five AI-built apps under one "
                "<code>sv</code>.</p>"
                + simple_table(["", *apps], [[esc(m), *[esc(cell.get((a, m), "")) for a in apps]]
                                             for m in measures], "Where the findings are")
                + three_tool_panel(three, "No re-run of the study. The nearest record since is a different "
                                          "experiment: the three-tool trial of 14 October, three apps built from one "
                                          "prompt in three AI coding tools, each checked by <code>sv</code>."))

    return page(
        "Five Apps Compared, by Period", "Apps checked by sv, by period",
        "The figure of the cut-off (<code>figure-study.html</code>) is one study, run 11 of 3 October, early in the "
        "StackVet period. It stays one study; the StackVet panel shows it, then the one record since of apps checked "
        "by <code>sv</code>, named as the different experiment it is.",
        period_panels(panel),
        "Sources: <code>study.csv</code> (run 11, the lens of where the findings are) and "
        "<code>tables/three_tool_reports.csv</code>.")


def three_arms(req, study_rows, three):
    arms = defaultdict(Counter)
    for r in req:
        if r["standard"].upper().startswith("ASVS") and r["status"] not in NOT_APPLIED:
            arms[f'Arm {r["arm"]}: {r["path"]}'][r["status"]] += 1
    statuses = sorted({s for c in arms.values() for s in c})

    def panel(k):
        if k == "v1":
            return ("<p>The comparison of 20 September: OWASP ASVS 5.0 requirements that applied to each arm, by "
                    "status.</p>" + simple_table(["arm", *statuses, "all"],
                                                 [[esc(a), *[c[s] for s in statuses], sum(c.values())]
                                                  for a, c in arms.items()], "The three arms"))
        # The study's statuses of 3 October, before the rename, then the three-tool trial's reports since.
        req_rows = [r for r in study_rows if r["run"] == "run-11" and r["lens"] == "requirements"
                    and r["measure"] in STUDY_STATUSES]
        apps = list(dict.fromkeys(r["app"] for r in req_rows))
        ms = list(dict.fromkeys(r["measure"] for r in req_rows))
        cell = {(r["app"], r["measure"]): r["value"] for r in req_rows}
        return ("<p>No three-arm comparison. The nearest records: the study's requirement statuses, five apps "
                "under <code>sv</code>'s blank manifest, 3 October, before the rename,</p>"
                + simple_table(["", *apps], [[esc(m), *[esc(cell.get((a, m), "")) for a in apps]] for m in ms],
                               "The study's requirement statuses")
                + three_tool_panel(three, "and the three-tool trial's reports, like for like, 14 October."))

    return page(
        "Three Arms, by Period", "What each check could verify, by period",
        "The figure of the cut-off (<code>figure-three-arms.html</code>) is v1's comparison of 20 September. It is "
        "kept whole in its period; the StackVet panel shows the nearest records of requirements by status, each a "
        "different experiment and named as one.",
        period_panels(panel),
        "Sources: <code>requirements.csv</code>, <code>study.csv</code>, and <code>tables/three_tool_reports.csv</code>.")


def how_sv_works(snaps, history):
    last = {}
    for r in history:
        last[r["period"]] = r
    s = {r["period"]: r for r in snaps}

    def panel(k):
        r = s[k]
        where = "in <code>agnostic/</code>, beside v1's own program" if r["sv_folder"] == "agnostic" else "at the top"
        extra = (f'<p>v1\'s own program at the same commit: {int(r["v1_program_files"]):,} TypeScript files, '
                 f'{int(r["v1_program_lines"]):,} lines (<code>server/</code>, <code>web/</code>, '
                 "<code>shared/</code>, <code>templates/</code>).</p>" if int(r["v1_program_files"]) else "")
        reach = last.get(k)
        return (f'<p>At <code>{r["commit"]}</code>, the period\'s last commit: <code>sv</code> {where}.</p>'
                + tiles([(r["crates"], "crates"), (f'{int(r["rust_lines"]):,}', f'lines of Rust in {r["rust_files"]} files'),
                         (r["data_files"], "data files it reads"), (r["outside_tools"], "outside tools it can run"),
                         (r["ast_rules"], "rules read from the code's syntax tree"),
                         (reach["asvs_can_settle"] if reach else "–", "ASVS requirements some check can settle")])
                + extra)

    return page(
        "How sv Works, by Period", "What sv was made of at the end of each period",
        "The figure of the cut-off (<code>figure-how-sv-works.html</code>) is a drawing of how <code>sv</code> works, "
        "from the code at 4 October: a picture of stages, not a count, and it stays the drawing. A drawing per period "
        "would be drawing it again by hand, the owner's to ask for. What can be counted from the code at each "
        "period's end is counted here; <code>HOW-SV-WORKS.md</code> and <code>SINCE-THE-CUTOFF.md</code> say what "
        "the stages added since are.",
        period_panels(panel),
        "Sources: <code>tables/snapshots.csv</code> and <code>tables/reach_history.csv</code>, written by this script "
        "from git at each period's last commit (<code>tables/periods.csv</code>).")


def outputs():
    stop = until()
    history_text = reach_history(stop)
    snaps_text = snapshots()
    history = list(csv.DictReader(io.StringIO(history_text)))
    snaps = list(csv.DictReader(io.StringIO(snaps_text)))
    faults = rows(HERE / "to-today" / "faults.csv")
    loop2, pointer = rows(HERE / "loop2_builds.csv"), rows(HERE / "pointer_trial.csv")
    route, three = rows(HERE / "three_tool_route.csv"), rows(HERE / "three_tool_reports.csv")
    all_trials = trial_rows(loop2, pointer, route)
    study_rows, req = rows(PAPER / "study.csv"), rows(PAPER / "requirements.csv")
    usab = rows(RECORDS / "usability.csv")
    return {
        HERE / "reach_history.csv": history_text,
        HERE / "snapshots.csv": snaps_text,
        PAPER / "figure-timeline-by-period.html": timeline(rows(HERE / "days.csv")),
        PAPER / "figure-security-by-period.html": security(history, req),
        PAPER / "figure-top10-by-period.html": top10(rows(RECORDS / "top10.csv"), faults),
        PAPER / "figure-agentic-by-period.html": agentic(rows(RECORDS / "agentic.csv"), faults),
        PAPER / "figure-decisions-by-period.html": decisions(rows(RECORDS / "decisions.csv"),
                                                             rows(HERE / "decision_records.csv")),
        PAPER / "figure-usability-by-period.html": usability(usab, faults),
        PAPER / "figure-how-found-by-period.html": how_found(usab, faults, rows(HERE / "tests_by_day.csv")
                                                             if (HERE / "tests_by_day.csv").exists() else []),
        PAPER / "figure-trials-by-period.html": trials(all_trials),
        PAPER / "figure-cost-by-period.html": cost(all_trials),
        PAPER / "figure-self-assessment-by-period.html": self_assessment(),
        PAPER / "figure-study-by-period.html": study(study_rows, three),
        PAPER / "figure-three-arms-by-period.html": three_arms(req, study_rows, three),
        PAPER / "figure-how-sv-works-by-period.html": how_sv_works(snaps, history),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    files = outputs()
    if args.check:
        stale = [p for p, t in files.items() if not p.exists() or p.read_text(encoding="utf-8") != t]
        for p in stale:
            print(f"{p.relative_to(ROOT)} differs from what period_records.py writes")
        sys.exit(1 if stale else 0)
    for path, text in files.items():
        path.write_text(text, encoding="utf-8")
    print(f"wrote {len(files)} files")


if __name__ == "__main__":
    main()
