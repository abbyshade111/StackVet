# The roadmap is retired, and the backlog folder alone says what is left and in what order

**Status:** claimed by claude/0633-triage, 23 October 2026

Filed 23 October 2026 at the owner's word: "we can stop using the roadmap now and go back to just relying on the
backlog folder".

The roadmap is the table at the top of `docs/BACKLOG.md` (ADR-061, Later), which `CLAUDE.md` tells a session to take
its next item from. `tools/backlog.py` writes its status column (`roadmap`, `roadmap --write`, and `claim` and `done`
when they change an item named there), `--check` fails while a row is stale, `crates/sv-cli/tests/backlog_roadmap.rs`
holds that, and `tools/docs_page.py` and `docs/ARCHITECTURE.md` mention it.

To do:

- Take the table out of `docs/BACKLOG.md`, with a line saying it was retired that day at the owner's word and is in the
  history, and take out the `roadmap` commands, their check, and their test.
- Say in `CLAUDE.md` and `docs/ARCHITECTURE.md` what a session takes next with no word from the owner. Proposed: the
  lowest-numbered open item, unclaimed, whose text names no item it comes after that is not done yet (the owner's order
  of 23 October 2026 is written on 0648, 0649, 0650, and 0626 that way), checked against `main` first.
- A Later entry on ADR-061 records it, with the owner's word.
