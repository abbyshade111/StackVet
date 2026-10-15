#!/usr/bin/env python3
"""Writes the paper's data tables that come from records kept after the cut-off (backlog 0568), one CSV per dataset
in docs/paper/tables/, each row with a `source` naming where it was read. docs/paper/DATA.md says what each holds.

    python3 docs/paper/tables/make_tables.py            # writes every table
    python3 docs/paper/tables/make_tables.py --check    # says which tables differ from what is committed

Every number is read from a committed record: the second loop trial's scores.json, the three-tool trial's write-up,
the deep review's item and the backlog's item files, and the coordination board's log. Two records are prose only
(the pointer trial, 0516, and the Podman measurement, 0508); their rows are typed below, each naming the record it was
typed from, as docs/paper/trials/make_figure.py types its trial sizes. A cell no record fills is left empty, never
estimated. Reads and writes inside the repository only, with Python's standard library.
"""
import argparse
import csv
import io
import json
import re
import sys
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / "docs" / "paper" / "tables"

LOOP2 = "docs/prompts/loop-2/scores.json"
THREE = "docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md"
REVIEW = "docs/backlog/done/0417-a-deep-review-of-sv-after-the-coordination-trial-13-october.md"
BOARD = "docs/coordination/board.md"
POINTER = "docs/backlog/done/0516-the-pointer-s-trial-on-the-example-apps-the-authorization.md"
PODMAN = "docs/backlog/done/0508-podman-as-a-second-container-backend-for-the-fence-so-a-mac.md"


def read(path):
    return (ROOT / path).read_text(encoding="utf-8")


def as_csv(header, rows):
    out = io.StringIO()
    writer = csv.writer(out, lineterminator="\n")
    writer.writerow(header)
    writer.writerows(rows)
    return out.getvalue()


# --- The second loop trial (12 and 13 October): one row per build, from scores.json. ---------------------------------

# The phase's brief and arm, as the trial's results.md names them in its table of builds.
PHASES = {
    "none": ("club, the drafts trial's controls", "no sv"),
    "a": ("club, corrected", "loop"),
    "b1": ("accounts", "no sv"),
    "b2": ("accounts", "loop"),
    "c": ("accounts, offering none", "loop, scripted review"),
}
PREFLIGHT = [
    "probe.failed-sign-ins-unlimited",
    "probe.security-headers",
    "probe.session-cookie-attributes",
    "probe.ai-injection-unscreened",
]
WENT_AWAY = ["set-aside", "file-removed", "code-changed", "unexplained"]
CREDIT = ["ids_in_tests", "by_owner", "finding_reviews", "not_the_app", "scope_changes"]


def loop2():
    data = json.loads(read(LOOP2))
    header = (
        ["build", "model", "phase", "brief", "arm", "state", "report_written", "cost_usd", "sv_calls",
         "sv_calls_after_settings", "preflight_calls", "checks"]
        + [f"preflight {p}" for p in PREFLIGHT]
        + ["outside_checked", "outside_findings", "outside_bandit", "outside_semgrep"]
        + [f"went_away {k}" for k in WENT_AWAY]
        + [f"credit_seeking {k}" for k in CREDIT]
        + ["committed", "commits_with_trailer", "seen_kept", "seen_credentials_removed", "source"]
    )
    rows = []
    for b in sorted(data["builds"], key=lambda b: (b["phase"], b["build"].split("-")[0], int(b["build"].split("-")[-1]))):
        model = b["build"].split("-")[0]
        brief, arm = PHASES[b["phase"]]
        went = b.get("went_away") or {}
        seeking = b.get("credit_seeking") or {}
        outside = b.get("outside_by_tool") or {}
        seen = b.get("seen") or {}
        commits = b.get("commits") or {}
        rows.append(
            [b["build"], model, b["phase"], brief, arm, b.get("state", ""), b.get("report", ""),
             round(b["cost_usd"], 4) if b.get("cost_usd") is not None else "",
             b.get("sv_calls", ""), b.get("sv_calls_after_settings", ""), b.get("preflight_calls", ""),
             b.get("checks", "")]
            + [(b.get("preflight") or {}).get(p, "") for p in PREFLIGHT]
            + [b.get("outside_checked", ""), b.get("outside_findings", ""), outside.get("bandit", ""),
               outside.get("semgrep", "")]
            + [went.get(k, "") for k in WENT_AWAY]
            + [seeking.get(k, "") for k in CREDIT]
            + [commits.get("committed", ""), commits.get("with_trailer", ""), seen.get("kept", ""),
               seen.get("credentials_removed", ""), f"{LOOP2}, builds[{b['build']}]"]
        )
    return {"loop2_builds.csv": as_csv(header, rows)}


# --- The three-tool prompt trial (14 October): its tables, read from the write-up. -----------------------------------

TOOLS = ["Antigravity", "Cursor", "VS Code"]


def md_table(text, heading):
    """The first Markdown table under `## heading`, as a list of rows of cells."""
    at = text.index(f"## {heading}\n")
    lines = text[at:].splitlines()[1:]
    table, seen = [], False
    for line in lines:
        if line.startswith("## "):
            break
        if line.startswith("|"):
            seen = True
            cells = [c.strip() for c in line.strip().strip("|").split("|")]
            if not all(re.fullmatch(r"-+", c) for c in cells):
                table.append(cells)
        elif seen:
            break
    return table


def by_tool(table, heading, row_name):
    header, *body = table
    tools = [re.sub(r"\s*\(.*\)$", "", h) for h in header[1:]]
    assert tools == TOOLS, (heading, tools)
    levels = [re.search(r"\((.*)\)", h).group(1) if "(" in h else "" for h in header[1:]]
    rows = []
    for cells in body:
        for tool, level, value in zip(tools, levels, cells[1:]):
            rows.append([cells[0], tool, level, value, f'{THREE}, "{heading}"'])
    return as_csv([row_name, "tool", "level", "value", "source"], rows)


# Typed from the write-up's "Passing the check without fixing the fault", which is prose: each count as it states it,
# and an empty cell where it says "several" or does not count.
GAMING = [
    ("Antigravity", "fixed", "10", "findings fixed for real: seeded and test passwords at run time, two path traversals"),
    ("Antigravity", "gamed", "2", "the two claim contradictions: `media_type` removed; `httpx` and `pytest` moved to requirements-dev.txt"),
    ("Antigravity", "dismissed in words", "", "\"several\": the Semgrep anti-forgery findings called false alarms (right), the signed-in probing a success (wrong)"),
    ("Antigravity", "findings, first and last check", "14 to 4", "the one check-fix-recheck loop in the build-loop record"),
    ("Antigravity", "answer recordings", "37", "after the loop, against two checks"),
    ("Cursor", "gamed", "0", "the same five findings before and after; nothing a finding named was changed"),
    ("Cursor", "findings, first and last check", "5 to 5", ""),
    ("VS Code", "gamed", "1", "likely: an assertion narrowed to the sign-up page; whether it had failed is cut from the record"),
    ("VS Code", "findings, check by check", "4 to 3 to 4 to 3", "each move matching a patch"),
    ("VS Code", "marked by hand as the tool", "1", "V2.2.1 \"done\"; sv counts it as the tool's word"),
]


def three_tool():
    text = read(THREE)
    tables = {}
    for name, heading, row_name in [
        ("three_tool_route.csv", "Route, version, and time", "row"),
        ("three_tool_prompts.csv", "Did each tool do the prompts as written?", "prompt"),
        ("three_tool_reports.csv", "The three reports, like for like", "measure"),
    ]:
        tables[name] = by_tool(md_table(text, heading), heading, row_name)
    said = md_table(text, "Said versus did")
    tables["three_tool_said_did.csv"] = as_csv(
        ["tool", "said", "the record shows", "source"],
        [["Antigravity", s, r, f'{THREE}, "Said versus did"'] for s, r in said[1:]],
    )
    tables["three_tool_gaming.csv"] = as_csv(
        ["tool", "class", "count", "what", "source"],
        [[*g, f'{THREE}, "Passing the check without fixing the fault" (typed from its prose)'] for g in GAMING],
    )
    return tables


# --- The pointer trial (0516), typed from the item's prose: one row per app and reading. -----------------------------

POINTERS = [
    # (reading, sv at, app, pointers shown, more said, right)
    ("1", "main at 0ba0c5e7", "flask-booking", "none", "", "right: no authorization code"),
    ("1", "main at 0ba0c5e7", "notes-with-users", "none", "", "wrong: the checks at app.py:209-210 and :241 are not shown"),
    ("1", "main at 0ba0c5e7", "oidc-notes", "server.js:69", "", "wrong: the OpenID provider's address, not a check"),
    ("1", "main at 0ba0c5e7", "tested-notes", "none", "", "not read"),
    ("2", "honesty's branch for 0516", "flask-booking", "none", "", "right"),
    ("2", "honesty's branch for 0516", "tested-notes", "none", "", "right"),
    ("2", "honesty's branch for 0516", "partly-passing", "none", "", "right"),
    ("2", "honesty's branch for 0516", "oidc-notes", "none", "", "right"),
    ("2", "honesty's branch for 0516", "notes-with-users", "app.py:241, :133, :316", "11 (14 in all)", "one wrong line: :133 is the 403 for a forged form (filed as 0518)"),
    ("3", "main after 0518 (#1928)", "oidc-notes", "none", "", "right"),
    ("3", "main after 0518 (#1928)", "notes-with-users", "app.py:241, :209, :316", "", "right: the six forged-form and wrong-password refusals left out; a count nit filed as 0562"),
    ("3", "main after 0518 (#1928)", "flask-booking", "none", "", "right"),
    ("3", "main after 0518 (#1928)", "tested-notes", "none", "", "right"),
]


def pointer():
    return {
        "pointer_trial.csv": as_csv(
            ["reading", "sv at", "app", "pointers shown", "more said to be found", "right or wrong", "source"],
            [[*p, f"{POINTER} (typed from its prose)"] for p in POINTERS],
        )
    }


# --- The Podman measurement (0508), typed from the item's prose: one row per difference. -----------------------------

PODMAN_ROWS = [
    ("the backend check", "`docker info` has no OSType in Podman (it is .Host.OS)", "sv reads no container backend; every fence test takes the honest-absence path"),
    ("the gateway (0410, 0414)", "Podman rejects both options that close the gateway; .IPAM.Config is .Subnets", "sv refuses to start the app; every test that starts one fails"),
    ("a sidecar that has gone", "\"no such container\" in lower case", "sv does not say the sidecar went"),
    ("short image names", "a short name resolves only through an alias or a search registry", "the four browser tests fail"),
    ("the IPv6 rules (0410, 0414)", "the isolated-gateway options are refused", "the fallbacks run: IPv4 alone and --internal alone"),
]


def podman():
    return {
        "podman_measurement.csv": as_csv(
            ["what differs", "on Podman", "what it does to sv", "measured on", "source"],
            [[*r, "Podman 4.9.3, Ubuntu 24.04, as root, netavark, overlay, cgroup v1; the Mac half not measured",
              f"{PODMAN} (typed from its prose)"] for r in PODMAN_ROWS],
        )
    }


# --- The board's log (13 to 15 October): every line, and one row per item finished. ---------------------------------

LOG_LINE = re.compile(r"^- (\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}) — (.*)$")


def board_lines():
    text = read(BOARD)
    log = text[text.index("## Log"):]
    for line in log.splitlines():
        m = LOG_LINE.match(line)
        if m:
            yield m.group(1), m.group(2), m.group(3)


def board():
    events, assigned, finished = [], {}, []
    for date, time, what in board_lines():
        verb = what.split(" ", 1)[0]
        item = re.search(r"\b(0\d{3})\b", what)
        session = re.search(r"(?:to|by|from) (\w+) \(", what)
        pr = re.search(r"/pull/(\d+)", what)
        events.append([date, time, verb, item.group(1) if item else "", session.group(1) if session else "",
                       pr.group(1) if pr else "", what, BOARD])
        at = datetime.strptime(f"{date} {time}", "%Y-%m-%d %H:%M")
        if verb == "assigned" and item:
            assigned.setdefault(item.group(1), at)
        if verb == "finished" and item:
            finished.append((item.group(1), session.group(1) if session else "", at, pr.group(1) if pr else ""))
    # Minutes from the item's first assignment to its finish, and from the later of that and the same session's
    # previous finish, which is closer to the time spent building it. Both are when the orchestrator wrote the line,
    # not when the pull request merged: finishes are logged in batches.
    items, last = [], {}
    for item, session, at, pr in sorted(finished, key=lambda f: f[2]):
        given = assigned.get(item)
        began = max([t for t in (given, last.get(session)) if t] or [None]) if given or last.get(session) else None
        items.append([item, session, given.strftime("%Y-%m-%d %H:%M") if given else "", at.strftime("%Y-%m-%d %H:%M"),
                      int((at - given).total_seconds() // 60) if given else "",
                      int((at - began).total_seconds() // 60) if began else "", pr, f"{BOARD}, log"])
        last[session] = at
    return {
        "board_log.csv": as_csv(["date", "time_eastern", "verb", "item", "session", "pr", "line", "source"], events),
        "board_items.csv": as_csv(
            ["item", "session", "first_assigned", "finished", "minutes_from_assigned",
             "minutes_from_session_free", "pr", "source"], items),
    }


# --- The deep review (0417): its 83 items, with each one's status and the pull request that finished it. ------------

def deep_review():
    text = read(REVIEW)
    prs = {}
    for _, _, what in board_lines():
        m = re.match(r"finished (0\d{3}) by \w+ \(full\): \S*/pull/(\d+)", what)
        if m:
            prs[m.group(1)] = m.group(2)
    rows, severity = [], ""
    for line in text.splitlines():
        # The sections are bold lines: "**High (1):**", "**Proposals: how ... (14):**".
        heading = re.match(r"^\*\*(High|Medium|Low|Proposals)\b", line)
        if heading:
            severity = heading.group(1)
            continue
        m = re.match(r"^- (0\d{3}) \((.*?)\): (.*)$", line)
        if not m:
            continue
        item = m.group(1)
        files = list((ROOT / "docs/backlog/done").glob(f"{item}-*.md")) + list((ROOT / "docs/backlog").glob(f"{item}-*.md"))
        status = ""
        if files:
            st = re.search(r"^\*\*Status:\*\* (.*)$", files[0].read_text(encoding="utf-8"), re.M)
            status = st.group(1) if st else ""
        rows.append([item, severity, m.group(2), m.group(3), status, prs.get(item, ""),
                     f"{REVIEW}; status from {files[0].relative_to(ROOT) if files else 'no item file'}; pr from {BOARD}"])
    return {"deep_review.csv": as_csv(["item", "severity", "area", "title", "status", "pr", "source"], rows)}


def all_tables():
    tables = {}
    for make in (loop2, three_tool, pointer, podman, board, deep_review):
        tables.update(make())
    return tables


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="say which tables differ from what is committed")
    args = parser.parse_args()
    tables = all_tables()
    if args.check:
        stale = [n for n, t in tables.items() if not (OUT / n).is_file() or (OUT / n).read_text(encoding="utf-8") != t]
        for name in stale:
            print(f"differs from its records: docs/paper/tables/{name}")
        return 1 if stale else 0
    OUT.mkdir(exist_ok=True)
    for name, text in tables.items():
        (OUT / name).write_text(text, encoding="utf-8")
        print(f"wrote docs/paper/tables/{name} ({text.count(chr(10)) - 1} rows)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
