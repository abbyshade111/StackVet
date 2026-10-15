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

**Built in part, 15 October 2026 (honesty).** One way the condition failed is corrected. The gate held back when any
package list in the app could not be read, in any language, though ADR-111 holds it back only for one in the
framework's language: a Flask app with a JavaScript library's `package.json` in `static/` that is not valid JSON kept
its Django findings counting. Now an unread list holds the gate back only in its own language (`Unread` carries it).
Tests: `crates/sv-check/tests/other_framework_unread.rs`, through the scanner on an app of the trial's shape
(commented `requirements.txt`, a package folder of templates, a `.venv` holding Django): with an unread npm list the
Django finding goes apart, and with an unread Python list beside a read one it stays.

Not confirmed: that this is what happened in the trial. Without the app or its report here, the scanner was tried on
that shape and read it correctly when no list was broken; nothing else in the path from the tools to the report drops
the mark. The question is in `docs/coordination/waiting/0579.md`: does the trial's report name a package list `sv`
could not read? If it names none, the cause is elsewhere and this item stays open for it.
