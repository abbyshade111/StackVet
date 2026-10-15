# The Django-rule gate from 0521 did not apply on a Flask app: ten django-no-csrf-token findings in the Opus run's report and in today's rerun

**Status:** claimed by honesty, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the owner's follow-up model trial (backlog 0577), Antigravity with Claude Opus 5.5, 15 October 2026. Severity:
high (the fix for 0521 is on main and the wrong finding is still there). Verified by the orchestrating session: the
report the tool made (sv HEAD-779676c, after #1972) and a rerun with sv at f3197a2c both list ten
`semgrep.python.django.security.django-no-csrf-token.django-no-csrf-token` findings under `findings` (not under the
"other framework" listing) on `vibegames/templates/*.html`, a Flask app whose `requirements.txt` names Flask, Werkzeug,
Jinja2, and no django.

**What to look at.** 0521's gate lists such a finding apart only "when the app's package lists in that framework's
language were read, none could not be, and none names the framework". Something in that condition fails for this app:
a package list that could not be read, or the gate running before the lists were, or the `--keep-tool-output` path.
The app copy is in the orchestrating session's scratchpad (`trials/rerun/opus-app`) and on the owner's drive. Make
the fake-app case from this app's shape (a `requirements.txt` with comments, a package folder, a `.venv` beside it),
watch the ten findings stay in `findings` before the fix, and go to the listing apart after.

**Record.** ADR-111: unchanged, because the gate's rule is unchanged; its condition is corrected.

**The orchestrator's answer to honesty's question of 14:40, 15 October 2026 (read from the rerun's report on the
Opus app, sv f3197a2c, and the app's files).** No package list is named as unread: `look_away.not_read` is empty, and
nothing in report.json carries "could not be read". compliance.md's line 15 does say "13 packages, not complete: some
of the app's package lists could not be read, and the report says which", but the only finding behind it is
`sbom.incomplete`, "13 package(s) are listed at the version asked for rather than the version installed" (no lockfile),
so that sentence is wrong in this case too (a second fault, or the same one from the other side: "not complete" from
a missing lockfile is being read as "a list could not be read"). The app's package-list-shaped files outside `.venv`
are exactly two: `requirements.txt` (with comment lines) and `stackvet.toml`; no package.json, Pipfile, pyproject, or
lockfile, and nothing at depth 2 on the drive either. If `*.toml` is taken as a possible package list, `stackvet.toml`
would be the one "that could not be read"; worth checking. The ten Django findings are in `findings`, V3.5.1
"checked in part" from the forged-form probe, the app Flask 3.1.3.
