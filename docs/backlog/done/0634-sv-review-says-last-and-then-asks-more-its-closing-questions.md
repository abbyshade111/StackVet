# sv review says "Last" and then asks more: its closing questions should say how many are left

**Status:** done, 22 October 2026

Found by the owner on 22 October 2026, running `sv review` at the end of a new Cursor build for the walk-through
(backlog 0373): the review said "Last, the answers that set the app's level.", then asked "One more: the account
changes your app does not offer.", then "One more: how soon a known vulnerability in your app's packages is fixed."
A person reading "Last" takes it that they are nearly done, and then is not.

It is worse than the owner saw: `ask_last` in `crates/sv-cli/src/review.rs` asks up to four closing questions, each
only when it waits, and each was written at a different time with its own opening words:

1. `confirm_scope`: "Last, the answers that set the app's level."
2. `confirm_not_offered`: "One more: the account changes your app does not offer."
3. `confirm_policy`: "One more: how soon a known vulnerability in your app's packages is fixed."
4. `confirm_capabilities`: "Last: what stackvet.toml says your app does and does not do."

Then the checklist before going live follows (`ticks::tick_checklist`). So a review can say "Last" twice, and say it
before three more questions and a checklist.

To do: `ask_last` knows before it asks which of the four wait, so it can say so once ("Four questions left, then the
checklist before going live.") and open each with where it stands ("1 of 4: the answers that set the app's level."),
with no "Last" or "One more" in any of them. A test for each count, from one question waiting to all four, that no
opening says "Last" unless it is the last thing asked; the tests in `crates/sv-cli/src/review/scope_tests.rs` that look
for "Last, the answers" change with it. The prompt that walks a person through `sv review` (`docs/prompts/setup.md`,
prompt 7) may quote these words too: check it.

**Claimed, 22 October 2026,** at the owner's word ("Please continue with 0638 and the other items filed today and outside
the roadmap"), with 0638, each to be built in a pull request of its own.

**Done, 22 October 2026.** `ask_last` counts the closing questions waiting before it asks the first, says "N more
questions." once when there is more than one, and each opens with where it stands ("Question 2 of 4: the account
changes your app does not offer."), or "One more question: …" when it is the only one; none says "Last" or "One
more:". Tests: `crates/sv-cli/src/review/closing_tests.rs` (all four waiting, numbered in order, and the words for one
alone); `scope_tests.rs` looks for the level's question by its subject.
