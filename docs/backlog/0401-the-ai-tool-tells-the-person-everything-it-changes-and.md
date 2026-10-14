# The AI tool tells the person everything it changes, and StackVet shows what changed between checks

**Status:** open

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
