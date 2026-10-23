# StackVet — what is still to do

> Written under `agnostic/` and moved to the repository root on 26 September 2026 when `sv` became the top of the
> repository; paths written `agnostic/…` in older items are now at the root (`agnostic/data/…` is `data/…`).
> v1's own backlog is on the `v1` branch.

**Where the items are.** Until 8 October 2026 every item was an entry in this file, which had grown to 8,600 lines
with no "Done" section. Each is now a file of its own under `docs/backlog/`, named by a number that keeps the order
the items stood in here and by its title, unchanged, so a reference of the form `BACKLOG, "title"` in the code or the
documents still finds it by a search of that folder. Its third line is its status: `open`, `claimed by <session>,
<date>`, `done, <date>`, or `partly done: <what remains>`. The record of why is `docs/adr/ADR-061.md`. This file holds
the rules, and no item and no list of items: a list every item adds a line to would bring back the
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

## What comes next

The roadmap that stood here from 8 October 2026, a table of the order of work, was retired on 23 October 2026 at the
owner's word: "we can stop using the roadmap now and go back to just relying on the backlog folder". The table, and
the owner's changes to its order, are in this file's history (ADR-061, Later). The backlog folder alone now says what
is left and in what order. With no word from the owner, a session takes the lowest-numbered open item that is
unclaimed, unless its text says it comes after an item that is not done yet (a line `**Order,** ... after NNNN.`), and
checks it against `main` first: some items are overtaken by later work and want a done note rather than a build.
