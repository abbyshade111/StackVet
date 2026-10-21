#!/usr/bin/env python3
"""Redraws the paper's hand-drawn figures per period from the ledgers extended to today (backlog 0568, part 3), as the
owner answered on 15 October 2026. The figures drawn by hand at the 4 October cut-off stay as they were, the record at
the cut-off; each one redrawn here is a file of its own beside it, `figure-<name>-by-period.html`.

    python3 docs/paper/tables/period_figures.py            # writes the figures
    python3 docs/paper/tables/period_figures.py --check    # says which differ from what is committed

Reads docs/paper/tables/to-today/ (ledgers.py), periods.csv, days.csv, and decision_records.csv (periods.py); writes
under docs/paper/ only, with Python's standard library. Every number on a page is counted here from those tables.
"""
import argparse
import csv
import html
import io
import sys
from collections import Counter, defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
PAPER = HERE.parent
TODAY = HERE / "to-today"
KEYS = ["v1", "stackvet"]
NAMES = {"v1": "version 1", "stackvet": "StackVet", "all": "both"}
COLORS = {"v1": "var(--v1)", "stackvet": "var(--sv)"}


def rows(path):
    return list(csv.DictReader(io.StringIO(path.read_text(encoding="utf-8"))))


def esc(text):
    return html.escape(str(text))


def page(title, heading, lead, body, sources):
    return f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{esc(title)}</title>
<style>
:root {{ --ink: #1d1d1f; --muted: #5f6368; --rule: #d9d9de; --page: #fbfbfa; --tile: #f2f1ee;
  --v1: #2a6fcf; --v2: #e8692f; --sv: #2f9e6e; --red: #d64545; --violet: #7a5bc4; --grey: #a3a19c; }}
@media (prefers-color-scheme: dark) {{ :root:not([data-theme="light"]) {{ --ink: #ececf1; --muted: #a8adb4;
  --rule: #3a3b40; --page: #17181b; --tile: #222327; --grey: #6e6c68; }} }}
:root[data-theme="dark"] {{ --ink: #ececf1; --muted: #a8adb4; --rule: #3a3b40; --page: #17181b; --tile: #222327;
  --grey: #6e6c68; }}
body {{ background: var(--page); color: var(--ink); font: 15px/1.55 system-ui, sans-serif; margin: 0 auto;
  max-width: 980px; padding: 16px; }}
h1 {{ font-size: 22px; margin: 8px 0; }} h2 {{ font-size: 17px; margin: 28px 0 6px; }}
p {{ margin: 6px 0 12px; }} .muted, .note {{ color: var(--muted); }} .note {{ font-size: 13px; }}
.tiles {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 12px; margin: 14px 0; }}
.tile {{ background: var(--tile); border-radius: 10px; padding: 12px 14px; }}
.tile b {{ display: block; font-size: 26px; line-height: 1.2; }} .tile span {{ font-size: 13px; color: var(--muted); }}
.legend span {{ margin-right: 14px; font-size: 13px; }} .legend i {{ display: inline-block; width: 10px; height: 10px;
  margin-right: 5px; border-radius: 2px; vertical-align: -1px; }}
svg {{ width: 100%; height: auto; display: block; }} svg text {{ fill: var(--muted); font-size: 11px; }}
table {{ border-collapse: collapse; width: 100%; font-size: 13px; margin: 8px 0; }}
td, th {{ border-bottom: 1px solid var(--rule); padding: 4px 6px; text-align: left; }} td.n {{ text-align: right; }}
.scroll {{ overflow-x: auto; }}
</style>
</head>
<body>
<h1>{esc(heading)}</h1>
<p>{lead}</p>
{body}
<p class="note">{sources}</p>
</body>
</html>
"""


def legend(items):
    return '<p class="legend">' + "".join(f'<span><i style="background:{c}"></i>{esc(t)}</span>' for t, c in items) + "</p>"


def period_legend():
    return legend([(NAMES[k], COLORS[k]) for k in KEYS])


def every_day(dates):
    """Every day from the first date to the last, so a day with nothing still has its place on the axis."""
    from datetime import date, timedelta
    ds = sorted(d[:10] for d in dates if d)
    if not ds:
        return []
    first, last = date.fromisoformat(ds[0]), date.fromisoformat(ds[-1])
    return [(first + timedelta(days=i)).isoformat() for i in range((last - first).days + 1)]


def day_bars(days, series, label, height=220):
    """Bars per day: `series` maps a day to [(value, color, title)], stacked from the bottom; a label over each bar."""
    width, left, bottom = 940, 40, 34
    top = max((sum(v for v, _, _ in series.get(d, [])) for d in days), default=0) or 1
    step = (width - left - 10) / max(len(days), 1)
    out = []
    for i, d in enumerate(days):
        x, y = left + i * step, height - bottom
        total = 0
        for value, color, title in series.get(d, []):
            h = (height - bottom - 24) * value / top
            y -= h
            total += value
            if value:
                out.append(f'<rect x="{x + 1:.1f}" y="{y:.1f}" width="{max(step - 2, 1):.1f}" height="{h:.1f}" '
                           f'fill="{color}"><title>{esc(title)}</title></rect>')
        if total:
            out.append(f'<text x="{x + step / 2:.1f}" y="{y - 4:.1f}" text-anchor="middle">{total}</text>')
        if i % 2 == 0:
            out.append(f'<text x="{x + step / 2:.1f}" y="{height - bottom + 14}" text-anchor="middle">{esc(d[5:10])}</text>')
    out.append(f'<line x1="{left}" y1="{height - bottom}" x2="{width - 10}" y2="{height - bottom}" stroke="currentColor" '
               f'stroke-opacity=".25"/>')
    return (f'<div class="scroll"><svg viewBox="0 0 {width} {height}" role="img" aria-label="{esc(label)}">'
            + "".join(out) + "</svg></div>")


def grouped_table(categories, counts, label):
    """A table: one row per category, one column per period and one for both, each cell a count and a bar."""
    top = max((counts[c][k] for c in categories for k in KEYS), default=0) or 1
    head = "".join(f"<th>{esc(NAMES[k])}</th>" for k in KEYS) + "<th>both</th>"
    body = []
    for c in categories:
        cells = []
        for k in KEYS:
            n = counts[c][k]
            w = 90 * n / top
            cells.append(f'<td><svg viewBox="0 0 92 12" style="width:92px;display:inline-block;vertical-align:middle">'
                         f'<rect width="{w:.1f}" height="10" y="1" fill="{COLORS[k]}"/></svg> {n}</td>')
        body.append(f"<tr><td>{esc(c)}</td>{''.join(cells)}<td class=\"n\"><b>{sum(counts[c].values())}</b></td></tr>")
    totals = "".join(f'<td><b>{sum(counts[c][k] for c in categories)}</b></td>' for k in KEYS)
    return (f'<div class="scroll"><table aria-label="{esc(label)}"><tr><th></th>{head}</tr>{"".join(body)}'
            f'<tr><td><b>total</b></td>{totals}<td class="n"><b>{sum(sum(counts[c].values()) for c in categories)}</b></td>'
            f"</tr></table></div>")


def tiles(items):
    return '<div class="tiles">' + "".join(f'<div class="tile"><b>{esc(a)}</b><span>{b}</span></div>' for a, b in items) + "</div>"


# --- The figures. --------------------------------------------------------------------------------------------------


FOUND_BY = [
    ("outside_review", "An outside review of the code"), ("review", "Another session reviewing the work"),
    ("building_else", "Building or designing something else"), ("owner_use", "The owner, or real use"),
    ("real_input", "Running it on real input, and the trials"), ("ci_tool", "An automated tool in CI"),
    ("harness", "v1's evaluation harness"), ("failing_test", "A failing test"),
    ("outside_project", "Another project using sv"), ("guard_breaking", "Breaking a guard on purpose"),
    ("not_recorded", "Not recorded"),
]


def weight(r):
    """How many faults a row stands for: a row at the cut-off can hold several (`count`), a row since holds one."""
    return int(r.get("count") or 1)


def fixed(r):
    """Fixed at the cut-off, or fixed since."""
    return bool(r["fixed_since_date"]) or (bool(r["fixed_date"]) and not r["test_in_fix"].startswith("n/a"))


def tested(r):
    t = r["fixed_since_test_in_fix"] or r["test_in_fix"]
    return t.startswith("yes")


def tests_faults(faults, tests):
    counted = [r for r in faults if r["in_counts"].startswith("yes") or r["in_the_ledger"] == "since the cut-off"]
    by_period = defaultdict(list)
    for r in counted:
        by_period[r["period"]].append(r)
    tile_items = []
    for k in KEYS + ["all"]:
        mine = counted if k == "all" else by_period[k]
        product = [r for r in mine if r["kind"] != "test-gap"]
        caught = sum(weight(r) for r in product if r["found_by"] == "failing_test")
        done = [r for r in mine if fixed(r) and r["found_by"] != "failing_test"]
        with_test = sum(1 for r in done if tested(r))
        share = f"{100 * with_test // len(done)}%" if done else "—"
        tile_items.append((sum(weight(r) for r in mine), f"<b style=\"font-size:15px;display:inline\">{esc(NAMES[k])}</b>: faults found; "
                           f"{caught} of the {sum(weight(r) for r in product)} in the product caught by a failing test; {share} of the "
                           f"fixed faults the tests missed got a test ({with_test} of {len(done)})"))
    # v1's tests are the counts its commits state, as the figure at the cut-off drew them; sv's are its #[test] lines.
    v1 = {r["date"][:10]: (int(r["v1_stated_server_tests"].split()[0]), r["period"]) for r in tests
          if r["version"] == "v1" and r["v1_stated_server_tests"][:1].isdigit()}
    sv = {}
    for r in tests:
        if r["version"] == "sv" and r["tests_level"]:
            sv[r["date"][:10]] = (int(r["tests_level"]), r["period"])
    days = every_day(set(v1) | set(sv))
    v1_series = {d: [(v1[d][0], COLORS[v1[d][1]], f"{d}: {v1[d][0]} tests, as v1's commits state")] for d in v1}
    sv_series = {d: [(sv[d][0], COLORS[sv[d][1]], f"{d}: {sv[d][0]} #[test] lines in sv")] for d in sv}
    found = defaultdict(Counter)
    for r in counted:
        d = r["found_date"]
        found[d]["failing_test" if r["found_by"] == "failing_test" else
                 "outside_review" if r["found_by"] == "outside_review" else "other"] += weight(r)
    fault_days = every_day(set(days) | set(found))
    fault_series = {d: [(found[d]["failing_test"], "var(--red)", f"{d}: found by a failing test"),
                        (found[d]["outside_review"], "var(--violet)", f"{d}: the outside review"),
                        (found[d]["other"], "var(--grey)", f"{d}: found any other way")] for d in fault_days}
    squares = []
    for k in KEYS:
        done = [r for r in by_period[k] if fixed(r) and r["found_by"] != "failing_test" and r["kind"] != "test-gap"]
        cells = "".join(
            f'<rect x="{(i % 40) * 18}" y="{(i // 40) * 18}" width="14" height="14" rx="2" '
            f'fill="{COLORS[k] if tested(r) else "none"}" stroke="{COLORS[k]}"><title>{esc(r["id"])}: '
            f'{esc(r["fault"][:120])}</title></rect>'
            for i, r in enumerate(done))
        rows_ = (len(done) + 39) // 40 or 1
        squares.append(f'<h3 style="font-size:14px;margin:12px 0 4px">{esc(NAMES[k])}: '
                       f'{sum(1 for r in done if tested(r))} of {len(done)} fixes added or changed a test</h3>'
                       f'<div class="scroll"><svg viewBox="0 0 720 {rows_ * 18}" style="max-width:720px">{cells}</svg></div>')
    body = (tiles(tile_items)
            + "<h2>1. Tests at the end of each day</h2>" + period_legend()
            + "<p class=\"muted\">v1, the counts its commits state:</p>"
            + day_bars(days, v1_series, "v1's tests at the end of each day", height=150)
            + "<p class=\"muted\">sv, the <code>#[test]</code> lines on main at the day's last commit:</p>"
            + day_bars(days, sv_series, "sv's tests at the end of each day, colored by period")
            + "<h2>2. Faults found each day</h2>"
            + legend([("found by a failing test", "var(--red)"), ("the outside review", "var(--violet)"),
                      ("found any other way", "var(--grey)")])
            + day_bars(fault_days, fault_series, "Faults found each day")
            + "<h2>3. Did the suite learn from each surprise?</h2>"
            + "<p>Each square is a fault in the product that the tests missed and that has been fixed; filled when its "
              "fix added or changed a test.</p>" + "".join(squares))
    return page(
        "Tests and Faults by Period", "More tests, fewer surprises? By period, to today",
        "The figure of the 4 October cut-off (<code>figure-tests-faults.html</code>), redrawn per period with the ledgers "
        "extended to today. A fault open at the cut-off counts as fixed when its fix since is recorded.", body,
        "Sources: <code>tables/to-today/faults.csv</code> and <code>tables/to-today/tests_by_day.csv</code> (made by "
        "<code>tables/ledgers.py</code>). Rows to 11:37 on 4 October are the ledgers as published; rows since were "
        "compiled on 15 October 2026 by Claude Code sessions reading each backlog item and merge, each row with its "
        "source and how sure the reading was. A period is the one a row's date falls in (<code>DATA.md</code>, \"The "
        "two periods\"), so `sv`'s faults before the cut-over of 26 September are in version 1's period. Tests are the "
        "<code>#[test]</code> lines on main at each day's last commit; v1's are the counts its commits state.")


def how_caught(faults):
    counted = [r for r in faults if r["in_counts"].startswith("yes") or r["in_the_ledger"] == "since the cut-off"]
    counts = defaultdict(Counter)
    for r in counted:
        counts[r["found_by"]][r["period"]] += weight(r)
    labels = dict(FOUND_BY)
    order = [k for k, _ in FOUND_BY if counts[k]]
    table = grouped_table([labels[k] for k in order], {labels[k]: counts[k] for k in order}, "How faults were found")
    kinds = defaultdict(Counter)
    for r in counted:
        kinds[r["kind"]][r["period"]] += weight(r)
    kind_order = sorted(kinds, key=lambda k: -sum(kinds[k].values()))
    return page(
        "How Faults Were Caught by Period", "How the faults were caught, by period, to today",
        "The figure of the 4 October cut-off (<code>figure-how-caught.html</code>), redrawn per period.",
        "<h2>1. How each fault was found</h2>" + period_legend() + table
        + "<h2>2. What kind of fault it was</h2>" + grouped_table(kind_order, kinds, "Kinds of fault"),
        "Source: <code>tables/to-today/faults.csv</code>. How a fault was found is a judgment from its record, made by "
        "hand at the cut-off and by reading each record since; the rows since name their source and how sure the "
        "reading was (<code>confidence</code>).")


def corrections_figure(corrections):
    counts = defaultdict(Counter)
    by_source = defaultdict(Counter)
    per_day = defaultdict(Counter)
    for r in corrections:
        counts[r["direction"]][r["period"]] += 1
        by_source[r["found_by"]][r["period"]] += 1
        per_day[r["date"]][r["direction"]] += 1
    names = {"down": "Down: the claim was stronger than the evidence", "up": "Up: the claim was weaker than the evidence",
             "wording": "Wording: true, but read as something else"}
    days = every_day(per_day)
    series = {d: [(per_day[d]["down"], "var(--red)", f"{d}: down"), (per_day[d]["up"], "var(--v1)", f"{d}: up"),
                  (per_day[d]["wording"], "var(--grey)", f"{d}: wording")] for d in days}
    order = sorted(by_source, key=lambda k: -sum(by_source[k].values()))
    return page(
        "Corrected Claims by Period", "What the checks claimed, and what turned out to be true, by period",
        "The figure of the 4 October cut-off (<code>figure-corrections.html</code>), redrawn per period with the "
        "corrections since.",
        "<h2>1. Which way each claim was corrected</h2>" + period_legend()
        + grouped_table([names[k] for k in ("down", "up", "wording")], {names[k]: counts[k] for k in names},
                        "Direction of each correction")
        + "<h2>2. Corrections each day</h2>"
        + legend([("down", "var(--red)"), ("up", "var(--v1)"), ("wording", "var(--grey)")])
        + day_bars(days, series, "Corrections each day")
        + "<h2>3. How each was found</h2>" + grouped_table(order, by_source, "How each correction was found"),
        "Source: <code>tables/to-today/corrections.csv</code>. A correction since the cut-off that is not yet merged "
        "says so in its <code>after</code>.")


def coordination_figure(coordination):
    counts = defaultdict(Counter)
    per_day = defaultdict(Counter)
    for r in coordination:
        counts[r["kind"]][r["period"]] += 1
        per_day[r["date"]][r["kind"]] += 1
    names = {"claim": "A claim on the backlog", "conflict": "A merge resolved by hand",
             "unmerged-pr": "A pull request closed unmerged", "cross-review-find": "A fault found in another session's work",
             "duplicate": "Work duplicated", "collision": "A collision", "sent-work": "Work sent between sessions"}
    order = [k for k in names if counts[k]]
    days = every_day(per_day)
    series = {d: [(per_day[d]["claim"], "var(--v1)", f"{d}: claims"), (per_day[d]["conflict"], "var(--v2)",
                   f"{d}: merges resolved by hand"),
                  (sum(per_day[d][k] for k in names if k not in ("claim", "conflict")), "var(--grey)",
                   f"{d}: everything else")] for d in days}
    return page(
        "Coordination by Period", "What coordinating several AI sessions cost, and what it bought, by period",
        "The figure of the 4 October cut-off (<code>figure-coordination.html</code>), redrawn per period with the "
        "events since.",
        "<h2>1. Each kind of event</h2>" + period_legend()
        + grouped_table([names[k] for k in order], {names[k]: counts[k] for k in order}, "Coordination events")
        + "<h2>2. Each day</h2>"
        + legend([("claims", "var(--v1)"), ("merges resolved by hand", "var(--v2)"), ("everything else", "var(--grey)")])
        + day_bars(days, series, "Coordination events each day"),
        "Source: <code>tables/to-today/coordination.csv</code>. Since the cut-off, claims and merges resolved by hand are "
        "counted from git as <code>COORDINATION.md</code> counted them (a merge counts when its combined diff has lines "
        "that match neither parent), pull requests closed unmerged are read from GitHub, and the other kinds by reading "
        "each record. From 13 October the coordination board assigned the work; its log is "
        "<code>tables/board_log.csv</code>.")


def adrs_figure(records, days):
    counts = defaultdict(Counter)
    later = Counter()
    for r in records:
        counts["Decision records added"][r["period_added"]] += 1
        later[r["period_added"]] += int(r["later_entries"] or 0)
    counts["Their dated Later entries, by the period the record was added"] = later
    status = defaultdict(Counter)
    for r in records:
        status[r["status"] or "(none)"][r["period_added"]] += 1
    tree = {d["date"]: (int(d["decision_records_in_tree"]), d["period"]) for d in days}
    ds = every_day(tree)
    series = {d: [(tree[d][0], COLORS[tree[d][1]], f"{d}: {tree[d][0]} records in the tree")] for d in ds if d in tree}
    return page(
        "Decision Records by Period", "The decision records, by period",
        "The decision records (<code>docs/adr/</code>) by the period each was added in, and how many were in the tree "
        "each day. v1's own records are on the <code>v1</code> branch; the count in the tree during version 1 is "
        "theirs.",
        "<h2>1. Records added, and their later entries</h2>" + period_legend()
        + grouped_table(list(counts), counts, "Decision records by period")
        + "<h2>2. Their status</h2>" + grouped_table(sorted(status), status, "Status of each record")
        + "<h2>3. Records in the tree at each day's end</h2>" + day_bars(ds, series, "Decision records in the tree"),
        "Sources: <code>tables/decision_records.csv</code> and <code>tables/days.csv</code> (made by "
        "<code>tables/periods.py</code>). Whether each record matched what was built is in <code>ADRS.md</code>, at the "
        "cut-off, and in the weekly reviews since.")


def reviews_figure(review, deep, open_fixed):
    """The two deep reviews that have tables: 4 October (review.csv, at eff3f17) and 13 October (deep_review.csv)."""
    order = ["critical", "high", "medium", "low", "proposals"]
    first = defaultdict(Counter)
    for r in review:
        sev = r["severity"].lower()
        if r["state"] == "fixed":
            first[sev]["fixed by the cut-off"] += 1
        elif f"SV-DR-{r['id']}" in open_fixed or r["id"] in open_fixed:
            first[sev]["fixed since"] += 1
        else:
            first[sev]["still open"] += 1
    second = defaultdict(Counter)
    for r in deep:
        sev = r["severity"].lower()
        second[sev]["done" if r["status"].startswith("done") else "still open"] += 1

    def table(counts, columns, label):
        head = "".join(f"<th>{esc(c)}</th>" for c in columns)
        body = "".join(
            f"<tr><td>{esc(s)}</td>" + "".join(f'<td class="n">{counts[s][c]}</td>' for c in columns)
            + f'<td class="n"><b>{sum(counts[s].values())}</b></td></tr>' for s in order if counts[s])
        total = "".join(f'<td class="n"><b>{sum(counts[s][c] for s in order)}</b></td>' for c in columns)
        return (f'<div class="scroll"><table aria-label="{esc(label)}"><tr><th>severity</th>{head}<th>all</th></tr>'
                f'{body}<tr><td><b>total</b></td>{total}<td class="n"><b>{sum(sum(v.values()) for v in counts.values())}'
                f"</b></td></tr></table></div>")

    return page(
        "Deep Reviews by Period", "The deep reviews, and what became of their findings, to today",
        "The figure of the 4 October cut-off (<code>figure-review.html</code>) showed the first deep review as it stood "
        "at 11:37 that day, with 1 of its 58 findings fixed. Here it is to today, beside the deep review of 13 October.",
        "<h2>1. The deep review of 4 October (StackVet period, before the rename), 58 findings</h2>"
        + table(first, ["fixed by the cut-off", "fixed since", "still open"], "The 4 October review")
        + "<h2>2. The deep review of 13 October (StackVet), 83 items</h2>"
        + table(second, ["done", "still open"], "The 13 October review"),
        "Sources: <code>review.csv</code> (at the cut-off), <code>tables/since-cutoff/open_at_cutoff.csv</code> (each "
        "finding's fix since, with its pull request), and <code>tables/deep_review.csv</code> (made by "
        "<code>tables/make_tables.py</code>). The 13 October review's proposals are suggestions, not defects. The other "
        "reviews since the cut-off are in <code>tables/to-today/faults.csv</code>, found by <code>review</code>.")


def outputs():
    faults = rows(TODAY / "faults.csv")
    files = {
        PAPER / "figure-tests-faults-by-period.html": tests_faults(faults, rows(TODAY / "tests_by_day.csv")),
        PAPER / "figure-how-caught-by-period.html": how_caught(faults),
        PAPER / "figure-corrections-by-period.html": corrections_figure(rows(TODAY / "corrections.csv")),
        PAPER / "figure-coordination-by-period.html": coordination_figure(rows(TODAY / "coordination.csv")),
        PAPER / "figure-adrs-by-period.html": adrs_figure(rows(HERE / "decision_records.csv"), rows(HERE / "days.csv")),
        PAPER / "figure-review-by-period.html": reviews_figure(
            rows(PAPER / "review.csv"), rows(HERE / "deep_review.csv"),
            {r["id"] for r in rows(HERE / "since-cutoff" / "open_at_cutoff.csv") if r["fixed_date"]}),
    }
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    files = outputs()
    if args.check:
        stale = [p for p, t in files.items() if not p.exists() or p.read_text(encoding="utf-8") != t]
        for p in stale:
            print(f"{p.relative_to(PAPER.parent.parent)} differs from what period_figures.py writes")
        sys.exit(1 if stale else 0)
    for path, text in files.items():
        path.write_text(text, encoding="utf-8")
    print(f"wrote {len(files)} figures")


if __name__ == "__main__":
    main()
