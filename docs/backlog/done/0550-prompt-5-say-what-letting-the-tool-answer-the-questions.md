# Prompt 5: say what letting the tool answer the questions costs, since every trial ended that way

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): all three. For the prompt library (0370).

**What was seen.** "One question at a time" ended with the owner's "for each question, can you record what you think
is the right answer" after six questions (Antigravity), one (VS Code), and a feature request instead of an answer
(Cursor). The answers then counted as the tool's.

**What to change.** Either tell the person in the prompt that letting the tool answer is allowed and what it costs
(the answers count as the tool's until `sv review`, and the report says so), or let the tool answer in one pass and
ask only where it is unsure. The prompt says which; the page's "Tried so far" says what the owner did.

**The owner's choice, 14 October 2026:** keep "one question at a time", and add a sentence saying the person may let the tool answer, and that every such answer counts as the tool's until sv review, which the report says too.
 And when the tool answers a question itself, it must quote the lines of the code that prove the answer (file and line), beside the answer: an answer with no lines is a guess, not a reading. (The owner, 14 October 2026.)

**Done, 15 October 2026, as the owner chose:** rule 6 of the build prompt (the guide's step 4 and the setup page's
step 5, word for word) keeps "ask me the questions one at a time" and now goes on: "I may let you answer some yourself:
each answer you give counts as yours, not mine, until I record it with `sv review`, and the report says so. When you
answer one, or tell me what the code does for one, quote the file and lines that show it beside the answer, and say it
is your reading, not a check: an answer with no lines is a guess." The two sentences fold into the rule's earlier "quote
the lines and say it is your reading", so its words are kept. `build_prompt_rules.rs` holds both; with "an answer with
no lines is a guess" taken out of the setup page, it and the word-for-word test went red. Seven sentences elsewhere in
the guide were shortened, saying the same, to keep it within its 4,200 words (4,199).
