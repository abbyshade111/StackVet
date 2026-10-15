# Semgrep's Django-only CSRF rule marks V3.5.1 "needs attention" on Flask and FastAPI forms that carry a csrf_token

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): the Antigravity, Cursor, and VS Code apps, each run again
with one build of sv (commit 672710f2, `sv report --run --keep-tool-output`). Severity: high (a wrong "needs attention"
on two of three apps). Verified against the templates by the orchestrating session before filing.

**What is wrong.** Semgrep's rule `python.django.security.django-no-csrf-token.django-no-csrf-token` looks for Django's
`{% csrf_token %}` tag inside a `<form method="post">`. The Antigravity app (FastAPI) and the VS Code app (Flask) render
their forms with Jinja and put the token in a hidden input: Antigravity `templates/login.html:9`
`<input type="hidden" name="csrf_token" value="{{ csrf_token }}">`, VS Code `templates/login.html:9` the same with
Flask-WTF's `csrf_token()`. Neither app names Django in `requirements.txt`. The rule fired 7 times on Antigravity and
6 on VS Code, and `data/adapters.json:7586` maps it to V3.5.1 with `targets: ["*.html"]` and no condition on the
framework, so V3.5.1 reads "needs attention" on both apps while the VS Code app's own test for a form posted without a
token (`tests/test_app.py:365-367`) passed under sv. The Cursor app's templates were not flagged.

**Where.** `data/adapters.json:7586` (the mapping); the mapping fields in use are `what`, `requirements`, `languages`,
`targets`, `findings_against`, and `skips`, none of which names a framework; `crates/sv-check/src/adapters.rs` reads
them.

**Fix.** A `frameworks` field on a mapping (or, read from the rule id's second segment, `python.django`), and when the
framework is not among the app's packages, list the hit under "names a requirement this app is not being assessed
against" (`out_of_scope`) with the reason "a Django rule, and this app does not use Django", crediting nothing. The
same for every `python.django.*`, `python.flask.*`, `javascript.express.*`, and the other framework packs. Write the
fake-app case first: a Jinja form with a `csrf_token` input in an app without Django, and watch V3.5.1 stay out of
"needs attention".

**Record.** ADR-111 governs what the outside tools' findings count as: a Later entry.

**Done 15 October 2026, by honesty.** A Semgrep mapping can name the web framework its rule is written for
(`framework`: its name, the package lists that would name it, and its packages), written by
`tools/semgrep_rule_map.py` from its `FRAMEWORKS` table onto 148 rules: Django (45, its template rules included),
Flask, Pyramid, FastAPI, Express, NestJS, Rails, and Laravel. A template-only rule in the Flask, Express, Rails, or
Laravel pack keeps counting: it is the template engine's (Jinja, EJS, ERB, Blade), which other frameworks share. When the
outside tools have run, a finding from such a rule is listed apart (`Outranked::OtherFramework`) when the app's
package lists in that framework's language were read, none could not be, and none names the framework; it is shown
in full with why ("From a rule written for Django, and this app's package lists do not name Django") and decides
nothing in the tables. With no package list in that language, or one unread, the finding counts as before.

The item asked for the hit under "names a requirement this app is not being assessed against". That list holds
findings about requirements the app is not held to; V3.5.1 is one it is held to, so the finding is listed apart the
way findings about requirements it is not held to already are (ADR-023, Later), with its own reason.

**Not done here:** Spring, Play, ASP.NET, Symfony, and Gorilla packs are not tagged, since how their packages are
named in the lists `sv_scan` reads (Maven coordinates, NuGet, Composer vendor names, Go module paths) was not checked
against an app. Library packs (`python.jwt`, `javascript.jsonwebtoken`, and the like) are not frameworks and are left.

