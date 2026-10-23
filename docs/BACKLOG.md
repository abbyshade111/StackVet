# StackVet — what is still to do

> Written under `agnostic/` and moved to the repository root on 26 September 2026 when `sv` became the top of the
> repository; paths written `agnostic/…` in older items are now at the root (`agnostic/data/…` is `data/…`).
> v1's own backlog is on the `v1` branch.

**Where the items are.** Until 8 October 2026 every item was an entry in this file, which had grown to 8,600 lines
with no "Done" section. Each is now a file of its own under `docs/backlog/`, named by a number that keeps the order
the items stood in here and by its title, unchanged, so a reference of the form `BACKLOG, "title"` in the code or the
documents still finds it by a search of that folder. Its third line is its status: `open`, `claimed by <session>,
<date>`, `done, <date>`, or `partly done: <what remains>`. The record of why is `docs/adr/ADR-061.md`. This file holds
the rules and the roadmap, and no item and no list of items: a list every item adds a line to would bring back the
conflicts the layout is for, and a test fails when one appears here (`crates/sv-cli/tests/backlog_board.rs`).

**The rules.** Claim an item before starting it, in a commit of its own, and merge that pull request before building:
`python3 tools/backlog.py claim <number> --by <session>` sets its status line, and refuses an item another session
holds or that is done. A message to another session is not a claim. When the work lands, `python3 tools/backlog.py
done <number>` marks it, or `done <number> --remains "..."` when part of it stays open; a note of what was built goes
in the item's text, as before, and the design entry holds the detail. A new item is `python3 tools/backlog.py new
"Its title"`, which writes an open file numbered one past the highest; two pull requests open at once may take the
same number, which harms nothing. An item's numbered parts each have a status line of their own (`**Part status:**`, in the same four
forms), set with `python3 tools/backlog.py claim 0226.3 --by <session>` and `done 0226.3`, and read by the board from
that line alone; a note of what was built still goes in the part's text. The item's status says `partly done` with
what remains, and `--check` holds the two to each other (backlog 0228). A done item's file is in `docs/backlog/done/`, so the folder itself lists only live work:
`done` moves it there and rewrites any path to it. **A review writes each finding as an item of its own**
(`backlog.py new`), and keeps its write-up in one item that lists them by number, so two sessions working on two
findings never write to one file; `--check` warns of a live item past 300 lines as one to split. `python3 tools/backlog.py list --open`
prints every open item, `summary` the counts, and `--check` holds the layout. A branch written before the split
conflicts here once it merges `main`; the steps to mend it are at the top of the script.

## Roadmap

The order of work, written 8 October 2026 by session securevibe-review at the owner's asking, and changed since only by
the owner's word, each change dated below. This section is the order, not its record: **a claim and a done note go on
the item itself, never here**. A session with no word from the owner takes the first row below that is not done,
claims it, and lands it before taking the next. Check an item against `main` before claiming it: some were overtaken by later work and want a
done note rather than a build.

The status column is read from each item's own status line by `python3 tools/backlog.py roadmap --write`, and by
`claim` and `done` when they change an item named here; `python3 tools/backlog.py --check` fails while a row is stale,
so this table cannot say an item is open when its file says done. The order, the phase, and the dates are written by
hand. `python3 tools/backlog.py show <number>` prints an item; `list --open` prints every item with anything open.

<!-- roadmap:start -->
| Order | Phase | Item | Status | Asked for |
|---|---|---|---|---|
| 0 | Phase 0: the record of direct pushes | 0258 Find pushes to main that skipped the pull-request rule | done, 11 October 2026 | 11 October 2026, ahead of everything else |
| 1.1 | Phase 1: close what is known to be broken | 0188 From the review of 8 October 2026: the medium and low findings | done, 8 October 2026 | 8 October 2026 |
| 1.2 | Phase 1 | 0050 Found by the documentation review (6 October 2026), in `sv` itself | done, 8 October 2026 | 8 October 2026 |
| 1.3 | Phase 1 | 0065 A review of `sv` on 27 September 2026 | done, 28 September 2026 | 8 October 2026 |
| 1.3 | Phase 1 | 0029 The running-app checks, reviewed on 3 October 2026 | done, 12 October 2026 | 8 October 2026 |
| 1.3 | Phase 1 | 0021 Two blind spots found testing the prompt library | done, 10 October 2026 | 8 October 2026 |
| 1.3 | Phase 1 | 0027 Three false alarms on code that does the safe thing | done, 10 October 2026 | 8 October 2026 |
| 1.3 | Phase 1 | 0061 Two limits cato-pipeline hit | done, 8 October 2026 | 8 October 2026 |
| 1.3 | Phase 1 | 0058 Improving the MCP server | done, 8 October 2026 | 8 October 2026 |
| 2 | Phase 2: the shape | 0187 From the architecture assessment of 8 October 2026: the four costs | done, 9 October 2026 | 8 October 2026 |
| 3 | Phase 3: the records | 0092 Records owed, from the first weekly review | done, 4 October 2026 | 8 October 2026 |
| 3 | Phase 3 | 0093 Records owed, from the second weekly review | done, 6 October 2026 | 8 October 2026 |
| 3 | Phase 3 | 0089 Records that disagree with what was built, or are missing | done, 30 September 2026 | 8 October 2026 |
| 3 | Phase 3 | 0091 A weekly review of the decision records | done, 10 October 2026 | 8 October 2026 |
| 4.1 | Phase 4: the product | 0038 The loop trials cannot compare security with the arms that have no `sv` | done, 8 October 2026 | 8 October 2026 |
| 4.1 | Phase 4 | 0036 The loop: `sv` as the MCP server an AI tool uses while it builds | done, 6 October 2026 | 8 October 2026 |
| 4.2 | Phase 4 | 0032 Test the prompt library where the prompts have something to fix | done, 9 October 2026 | 8 October 2026 |
| 4.2 | Phase 4 | 0031 A prompt library | done, 9 October 2026 | 8 October 2026 |
| 4.3 | Phase 4 | 0033 Design-time prompts from the Secure by Design checklist | done, 4 October 2026 | 8 October 2026 |
| 4.3 | Phase 4 | 0034 Design-time help before any code | done, 11 October 2026 | 8 October 2026 |
| 4.4 | Phase 4 | 0006 From the gap analysis of 7 October 2026 | done, 13 October 2026 | 8 October 2026 |
| 4.5 | Phase 4 | 0005 A dashboard view for `sv` | done, 10 October 2026 | 8 October 2026 |
| 4.5 | Phase 4 | 0002 A private page for the owner | done, 9 October 2026 | 8 October 2026 |
| 4.6 | Phase 4 | 0191 From the end-of-day write-up of 8 October 2026: ideas for `sv` itself | done, 9 October 2026 | 8 October 2026 |
| 4.7 | Phase 4 | 0012 Research OWASP's Agentic Skills Top 10 | done, 10 October 2026 | 8 October 2026 |
| 5 | Phase 5: when nothing above is open | 0157 Corroborators for the remaining claims | done, as its markers read on 8 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0147 What a new tool, service, or process would reach | done, 12 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0174 More adapters | done, 9 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0133 A checklist for what only a person can check | done, as its markers read on 8 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0115 Let the owner confirm what the AI coding tool said | done, 9 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0102 False alarms, part 2 | done, 9 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0103 False alarms, part 3 | done, as its markers read on 8 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0120 Packaging `sv` for somebody who is not technical | partly done (what remains is on the item) | 8 October 2026 |
| 5 | Phase 5 | 0117 Promote `sv` to the top of the repository | done, 9 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0143 What the remaining Level 1 and 2 requirements need | done, 9 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0145 More questions for the running app | done, 6 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0063 Eight places where the paper's earlier files disagree with the record | done, 9 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0062 Two more analyses for the paper | done, 9 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0064 Two analyses for the paper, and a stale count | done, 9 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0083 Evaluate Opengrep against semgrep | done, 3 October 2026 | 8 October 2026 |
| 5 | Phase 5 | 0184 Research: could the Kaspa blockchain | done, 9 October 2026 | 8 October 2026; closed at the owner's word on 9 October |
| 6 | After the roadmap | 0222 Test a build with Cursor | done, 11 October 2026 | the owner, 9 October 2026 |
| 6 | After the roadmap | 0223 Test the weekly decision-record review from end to end | done, 12 October 2026 | the owner, 9 October 2026 |
| 6 | After the roadmap | 0224 Mock-ups of the website | done, 12 October 2026 | the owner, 9 October 2026 |
| 7 | The owner's order of 9 October, which comes first | 0006 The record of the build loop (finding 22(d), ADR-076) | done, 13 October 2026 | the owner, 9 October 2026 |
| 7 | The owner's order of 9 October | 0226 A deep review of sv for observability, and its findings | done, 11 October 2026 | the owner, 9 October 2026 |
| 7 | The owner's order of 9 October | 0227 An observability dashboard | done, 11 October 2026 | the owner, 9 October 2026 |
| 8 | Added 12 October, and moved up the same day | 0273 Try a coordinating session for the parallel builds | done, 13 October 2026 | the owner, 12 October 2026 |
| 8 | Added 12 October | 0257 The build-loop trial the coordinating session follows | partly done (what remains is on the item) | the owner, 12 October 2026 |
| 9 | Process, any time | 0192 Process: shorter CI, a merge queue decision, and a nightly routine | done, 12 October 2026 | 8 October 2026 |
| 9 | Process, any time | 0193 One file per backlog item, with a status line | done, 8 October 2026 | the owner's decision, 8 October 2026 |
| 10 | The owner's order of 20 October, after 0489's V2.2.1 | 0624 No check splits into the owner answers and no check yet | done, 20 October 2026 | the owner, 20 October 2026 |
| 10 | The owner's order of 20 October | 0507 Level 3 as an opt-in, when the owner asks for it | done, 21 October 2026 | the owner, 20 October 2026 |
| 11 | The owner's order of 21 October | 0577 The owner's follow-up trial: the same prompts with the harness and the model swapped | done, 21 October 2026 | the owner, 21 October 2026 |
| 11 | The owner's order of 21 October | 0630 What the AI coding tool did that is worth a look | done, 22 October 2026 | the owner, 21 October 2026 |
<!-- roadmap:end -->

Rows that name the same order number are one sub-item of the old roadmap, taken in the order listed. The phases' own
words, with which part of each item they meant, are in the history of this file before 14 October 2026.

**Changes to the order**, each in the owner's words as the roadmap carried them:

- 8 October 2026: phases 1 to 5, written after the day's review and architecture assessment.
- 9 October 2026: after the roadmap, in this order: the Cursor test (0222), the weekly review's end-to-end test
  (0223), and the website mock-ups (0224), none taken while a phase above has an open item. And "the owner's order of
  9 October 2026, which comes first": the record of the build loop (0006, finding 22(d), ADR-076), then the
  observability review (0226) and its findings A, C, and D (ADR-082, ADR-083, ADR-084), then its dashboard (0227), then
  the website mock-ups (0224); the Cursor test waits for the owner.
- 11 October 2026: phase 0, the record of direct pushes (0258), ahead of everything else.
- 12 October 2026: a coordinating session for the parallel builds (0273), set up once the other sessions are on the
  build-loop trial (0257) and the website work (0224), and moved up the same day; the deep review's findings that
  follow it wait on the coordination board until the loop trials are done.
- 20 October 2026: after 0489's V2.2.1 part, the split of *No check* (0624), then level 3 as an opt-in (0507), taken
  then rather than held until every proposal from 0488 to 0500 is built; 0489's remaining parts follow.
- 21 October 2026: the paper's tables in two periods, version 1 against StackVet, with 0577's trial rows and its Limits
  block, then the report's section on what the AI coding tool did that is worth a look (0630).

The status of each item as a read-only review found it on 11 October 2026 is a design entry of its own,
"The roadmap's status checked against the code (11 October 2026)", under `docs/design/`.
