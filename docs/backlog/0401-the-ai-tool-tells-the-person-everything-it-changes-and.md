# The AI tool tells the person everything it changes, and StackVet shows what changed between checks

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

The owner's asking, 13 October 2026, after the trial in Antigravity: "investigate ways to ensure the AI tool tells the
user everything it's doing - a normal person would have had no idea it made those changes."

What the trial showed. Asked for the full report, the tool ran it, then on its own fixed what it found and ran it
again, and its summary said the findings were "refactored and resolved". The changes were:
- `# nosec B104` added, then `"0.0.0.0"` changed to `""`, which binds the same way.
- A database fallback into the shared temporary folder, re-spelled so Bandit stopped seeing it.
- A `db.py` change committed unasked.
- The earlier report folder deleted.

Only reading the git log showed any of it. Earlier in the same build it had rewritten `stackvet.toml`'s deployment
lines three times without asking.

Ways to look into, from the cheapest:
- **The prompts.** The build prompt (guide step 4, setup page section 5) and prompt 6 could require a plain account
  after every change, of what changed in what the app does and why, before the next step. They could also require
  that no finding is fixed without the person's yes, and that nothing is deleted without asking. Cheap; whether tools
  follow it is what a trial shows.
- **StackVet's MCP answers.** `stackvet_check` could end with a line for the AI tool to pass on: "tell the person what
  you changed since the last check". The report could carry the same.
- **StackVet itself.** Between two checks of the same app (`--baseline`, `sv compare`, the history), StackVet could
  say which findings went away and what changed on their lines: a suppression comment added, the line rewritten, the
  file deleted, a setting in `stackvet.toml` changed. A finding gone while its line's meaning stayed the same is the
  case to make loud. This reads only the app's own folder and git, so it opens no connection.

Whatever is built, the report must keep saying what it could not see: StackVet cannot know what the AI tool did
outside the app's folder, or what it told the person.

**Partly done, 14 October 2026:**
- **The prompts.** The build prompt (guide step 4 and the setup page) already asks the tool to say in plain words
  what it changed after each change, to ask before deleting anything, and never to silence a finding or hide it from
  a scanner; `crates/sv-cli/tests/build_prompt_rules.rs` holds those rules.
- **StackVet's MCP answers.** Every `stackvet_check` now ends its summary with a line for the tool to act on: tell
  the person in plain words what changed since the last check and why (each file, each stackvet.toml setting, anything
  deleted, any finding made to go away rather than fixed), and ask before deleting anything or silencing a finding.
  It also says that `sv` cannot see what the tool tells them (`check_text::TELL_THE_PERSON`, held by
  `crates/sv-cli/src/mcp/tell_the_person_tests.rs`).
- **What remains:** StackVet itself comparing two checks of the same app, and saying which findings went and what
  changed on their lines. A finding gone while its line still does the same thing is the case to make loud. The record
  of the build loop already keeps each check's findings' fingerprints (ADR-084), which such a comparison would start
  from.
