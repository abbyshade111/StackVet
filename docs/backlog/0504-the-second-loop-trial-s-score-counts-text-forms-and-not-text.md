# The second loop trial's score counts text-forms and not text-form

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found 14 October 2026 by session documentation, answering backlog 0389. `docs/prompts/loop-2/score_loop2.py`'s
`text_forms` counts only `browser.text-forms` (the list), not `browser.text-form` (one form), so `scores.json` shows
`text_forms_listed: 0` for every build, the 28 whose typed-text check was credited among them, and
`docs/prompts/loop-2/results.md` (finding 6) says "no build listed `text-forms`", which is true and reads as though no
build named a form. What to do: count both keys in the script (and its self-test), say in `results.md`'s finding 6 that
the answered builds named one form with `text-form`, and re-score the builds when they are next on hand (they are under
`~/sv-loop2` on the owner's computer, not in the repository).
