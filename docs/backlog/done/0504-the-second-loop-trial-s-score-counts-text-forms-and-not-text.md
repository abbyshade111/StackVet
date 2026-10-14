# The second loop trial's score counts text-forms and not text-form

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found 14 October 2026 by session documentation, answering backlog 0389. `docs/prompts/loop-2/score_loop2.py`'s
`text_forms` counts only `browser.text-forms` (the list), not `browser.text-form` (one form), so `scores.json` shows
`text_forms_listed: 0` for every build, the 28 whose typed-text check was credited among them, and
`docs/prompts/loop-2/results.md` (finding 6) says "no build listed `text-forms`", which is true and reads as though no
build named a form. What to do: count both keys in the script (and its self-test), say in `results.md`'s finding 6 that
the answered builds named one form with `text-form`, and re-score the builds when they are next on hand (they are under
`~/sv-loop2` on the owner's computer, not in the repository).

**Built 14 October 2026, by honesty.** The scorer had a second fault besides the one above: it read `browser` beside
`[stack.run.users]`, where `sv` reads nothing, rather than inside it, so it would have shown 0 even for a build that
listed `text-forms`; its self-test was written to the same wrong place and passed. `text_forms` in the script now reads
`[stack.run.users.browser]` and counts the list's length, or 1 for `text-form`. The self-test now covers both keys, and
one `browser` beside `users` that it must not count. Breaking either half of the fix turns the self-test red, and
`crates/sv-check/tests/loop_measures.rs` runs it. `results.md`'s finding 6 now says the count was wrong and why, and
that `scores.json`'s `text_forms_listed` is not a measurement until the builds are scored again.

**What remains** is backlog 0515: scoring the builds again, on the owner's computer, where they are (`~/sv-loop2`),
and writing finding 6 from the new count.
