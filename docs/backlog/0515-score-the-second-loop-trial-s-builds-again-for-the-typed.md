# Score the second loop trial's builds again for the typed-text forms, on the owner's computer

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Filed 14 October 2026 by honesty, from backlog 0504. 0504 fixed `docs/prompts/loop-2/score_loop2.py`, which counted
no typed-text form in any build: it now reads `[stack.run.users.browser]` and counts `text-forms` or the one
`text-form`. The builds are not in the repository, so `scores.json` was not scored again, and finding 6 in
`docs/prompts/loop-2/results.md` says its `text_forms_listed` is not a measurement until they are.

What to do, on the owner's computer, where the builds are (`~/sv-loop2`): run `sh docs/prompts/loop-2/run_loop2.sh
score`, copy the new scores into `docs/prompts/loop-2/scores.json`, and write finding 6 from the new count: how many
builds named a form, with which key, and whether every build where the check was answered named one. That also checks
0389's reading, not yet checked build by build, that the unanswered builds named no form.
