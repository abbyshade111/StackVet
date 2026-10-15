# Ticking the checklist before going live in sv review: a hand check for a requirement sv accepts one for, otherwise a sealed note

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Filed by wording, 15 October 2026, building backlog 0589: its second half, the owner's answer of that day on ticks, in
a pull request of its own because it touches the manifest, the seals, and `sv review` (ADR-121, decision 4).

**What to build.**
- `report.json` keeps the checklist's lines (`sv_report::checklist::lines`), each with a stable key, so `sv review`
  can offer them without reading the Markdown.
- `sv review` offers the lines not yet ticked, after the list it goes through now. Each is ticked only when the person
  confirms it there, never by editing `before-going-live.md`, which is written again with each report.
- A tick on a line naming a requirement that `data/human-checks.json` accepts a hand check for is recorded as one:
  `[checked-by-hand]` with `result = "done"`, `by = "owner"`, the day, the line as `how`, and the seal, as
  `record_own` seals one now. A tick widens nothing: a line naming only requirements not in that file (V12.2.1,
  V3.4.1, V12.1.1 and the rest of `live::LIVE_SITE` but V12.1.2 and V12.2.2) is recorded as a note.
- Any other tick is a sealed note in a new `stackvet.toml` table (its key, the day, `by = "owner"`, the seal): a new
  manifest field, a `*_fields` function in `sv-check/src/seal.rs`, the starter manifest's text, and reading it where
  the report is assembled. The next checklist shows the line as done, with when, and it credits nothing.
- Tests: a tick on each kind of line; an unsealed or edited note is not shown as done; the checklist still says it
  credits nothing.

**Record.** ADR-121, decision 4, already says what a tick records; a Later entry there when it is built.
