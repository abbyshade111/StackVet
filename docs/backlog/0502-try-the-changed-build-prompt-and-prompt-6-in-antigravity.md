# Try the changed build prompt and prompt 6 in Antigravity again

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Split from backlog 0407 on 14 October 2026 (session documentation), so 0407 could close: it changed the build prompt
(the guide's step 4 and the setup page's step 5) and prompt 6 for what the owner's trial in Antigravity on 13 October
2026 showed, and `crates/sv-cli/tests/build_prompt_rules.rs` holds each change. What is left is the trial again: the
changed prompts, in Antigravity, with the Gemini 3.8 Flash model, on the same kind of app, to see whether the tool now
asks before deciding where the app runs and who signs in, names the routes, leaves findings unsilenced, says what it
changed, keeps a reading of the code apart from a check, and opens the report in the browser. The run is the owner's
to make (their computer, their tool, their account). Until it is made and does better, the setup page's "Tried so
far" keeps not recommending Antigravity with that model; when it is, that paragraph and the guide's are changed
together (`crates/sv-cli/tests/setup_prompts_guide.rs` holds them to each other).
