# Prompt 5: no `confirmed` block and no `[checked-by-hand]` line by the tool, named as such

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): Antigravity (12 `confirmed` blocks), VS Code
(V2.2.1 `done` with `by = "ai-tool"`). For the prompt library (0370).

**What was seen.** The prompt says "never mark a check by hand as done"; VS Code did it with an honest `by` line;
Antigravity invented a `confirmed = { by = "ai-tool" }` syntax sv does not count.

**What to change.** Prompt 5: "Do not write `confirmed = ...` or any `[checked-by-hand]` line, whatever the `by`
value; those are mine, through sv review." The build-prompt test holds the sentence.

**Done, 14 October 2026, as the owner chose (B):** the item's "no `[checked-by-hand]` line, whatever the `by` value"
would have left a hand check no way into the report, since `sv review` only confirms entries already written and the
guide's step 5 and `docs/REVIEW.md` have the tool record what the person saw, as a proposal. So rule 6 of the build
prompt now says: "Never mark a check by hand as done yourself, and never write `confirmed` or `by = "owner"`: only I
can, with `sv review`. Write a `[checked-by-hand]` entry only for a check I made and described to you, with
`by = "ai-tool"` and my words in `how`." `build_prompt_rules.rs` holds both sentences; with "never write `confirmed`"
taken out of the setup page, it and the word-for-word test went red. Two sentences in the guide's section 6 were
shortened, saying the same, to keep it within 4,200 words.
