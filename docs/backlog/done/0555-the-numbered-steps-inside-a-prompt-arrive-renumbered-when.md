# The numbered steps inside a prompt arrive renumbered when copied from the page

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): Cursor pasted prompt 6's steps as "8." to "13.",
VS Code as "3." to "8.", Cursor's prompt 5 as "6.", "7.". For the prompt library (0370) and `docs/prompts/setup.md`.

**What was seen.** The page's lists continue the numbering of the surrounding text when rendered, so a copied prompt
carries numbers its own prose does not refer to.

**What to change.** Number each prompt's list from 1 inside its own code block, or use plain lines ("First", "Then"),
so a copy keeps the numbers the prose refers to; the test that holds the page to the guide checks it.

**Done, 15 October 2026:** each step in the setup page's seven prompts and the guide's build prompt now starts with the
words "Step N." on a paragraph of its own, in place of a Markdown list item: 38 steps on the page and the build
prompt's 6 in the guide. Words are kept by a copy as they are written, where a list pasted into a chat is numbered by
the chat. Of the item's two choices, this is the plain-lines one, kept with numbers, since prompt 3 refers to "step 5";
putting each prompt in a code block would show the reader raw text. `crates/sv-cli/tests/setup_prompt_steps.rs` holds
every prompt to it: no list item, steps from 1 in order, and every "step N" a prompt names one it has. With the page as
it was, with the guide as it was, with a step out of order, and with "step 15" in prompt 3, it went red each time.
