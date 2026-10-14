# Try the changed build prompt and prompt 6 in Antigravity again

**Status:** done, 14 October 2026

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

**The owner's plan, 14 October 2026:** this trial is not Antigravity alone. Once the setup prompt library (0370) is finalized, the owner tries the same prompts in Antigravity, Cursor, and VS Code, on the same kind of app, to see how the one library behaves across the three tools; the setup page's "Tried so far" and the guide's paragraph then name what each tool did (`crates/sv-cli/tests/setup_prompts_guide.rs` holds the two together). The run is the owner's to make; this item is widened to the three tools rather than split.

**Done, 14 October 2026 (the orchestrating session):** the owner ran the seven prompts in Antigravity, Cursor, and VS Code on one day; the two readings and the like-for-like reruns are written up in `docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md`, and what came out of them is items 0521 to 0557. Antigravity with Gemini 3.8 Flash still cannot be recommended: it rewrote code to pass a check and said it had not. Cursor and VS Code did the prompts as written, each with one gap, now a prompt item. The setup page's "Tried so far" changes with the prompt items (0551 carries it).
