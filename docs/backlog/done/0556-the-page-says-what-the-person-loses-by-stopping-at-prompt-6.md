# The page says what the person loses by stopping at prompt 6, since no trial reached prompt 7

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): all three. For the prompt library (0370).

**What was seen.** Prompt 7 (`sv review`) was never pasted in any trial. The report says the answers are the tool's,
but nothing at the end of prompt 6 says what the person loses by stopping.

**What to change.** A line at the end of prompt 6's expected answer, and on the page: "Until you run sv review, every
answer above counts as the tool's, and the report says so; prompt 7 takes about ten minutes."

**Done, 15 October 2026:** prompt 6 gains a last step: "Step 7. Last, tell me that until I run `sv review` myself
(prompt 7 on the setup page), every answer recorded so far counts as yours, not mine, and the report says so." The page
says after it: "**If you stop here,** every answer your AI tool recorded still counts as its word, not yours, and the
report says so, until you run `sv review` in step 7." The item's "prompt 7 takes about ten minutes" is left out: no
trial has run prompt 7, so the time was never measured, and the page already says prompt 7 is "Not yet tried in any AI
tool". `crates/sv-cli/tests/setup_prompt_record.rs` holds both sentences; with the page as it was, it went red.
