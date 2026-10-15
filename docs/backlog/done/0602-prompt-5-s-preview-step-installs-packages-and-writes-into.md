# Prompt 5's preview step installs packages and writes into the app's real database without a yes, and never opens the sign-in page

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the clean Cursor + Flash run of 15 October 2026 (backlog 0577). For the prompt library (0370). Severity: medium.

**What happened.** The tool ran `pip install` without asking, its tests wrote into the app's real database (the
"User A Game" in the owner's screenshot), and its preview never opened `/login`, which is why a sign-in page that
sends an empty body over a real server was reported as working. Its own tests used an in-process transport and never
saw the broken response either.

**What to change.** Prompt 5: "Ask before installing anything. Tests use a database of their own, never the app's.
Before you say a page works, open it the way a person would, over the running server, and tell me what you saw."
Prompt 6's preview line says the same.

**Record.** None.

**Done, 15 October 2026.** Setup prompt 5's step 4, and the guide's step 4 word for word, now say: "Ask me before you
delete anything, and before you install anything. Your tests use a database of their own, never the app's. Before you
tell me a page works, open it the way a person would, over the running server, and tell me what you saw." The prompt's
stamp was written again. Held by `crates/sv-cli/tests/build_prompt_preview.rs`; with the database sentence taken out,
it went red. Not done: "prompt 6's preview line", since prompt 6 has no such line (it opens the report, not the app).
Not tried in an AI coding tool.
