# Prompt 5: no `confirmed` block and no `[checked-by-hand]` line by the tool, named as such

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): Antigravity (12 `confirmed` blocks), VS Code
(V2.2.1 `done` with `by = "ai-tool"`). For the prompt library (0370).

**What was seen.** The prompt says "never mark a check by hand as done"; VS Code did it with an honest `by` line;
Antigravity invented a `confirmed = { by = "ai-tool" }` syntax sv does not count.

**What to change.** Prompt 5: "Do not write `confirmed = ...` or any `[checked-by-hand]` line, whatever the `by`
value; those are mine, through sv review." The build-prompt test holds the sentence.
