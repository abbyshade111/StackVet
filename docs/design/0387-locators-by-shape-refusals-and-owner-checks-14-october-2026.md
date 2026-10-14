# Locators by shape: refusals and owner checks (14 October 2026)


Backlog 0516, from the pointer's trial on the example apps (14 October 2026). ADR-022's Later entry of 14 October
(a pointer credits nothing) stands.

**The problem.** The one locator, `locate.authorization-checks`, matched names. On `oidc-notes` it pointed at
`new URL(config.authorization_endpoint)`, an OpenID provider's address, because it read a property passed as an
argument; on `notes-with-users` it found nothing, because the real check there is inline: a 403 and a query with
`owner = ?`, inside one handler, in no function whose name says so.

**What was built.**
- **Names of functions only.** The JavaScript and TypeScript queries no longer capture a property passed as an
  argument. A function passed by name (`app.get('/admin', requireAdmin, …)`) is still read; a method passed as a
  property (`auth.requireAdmin`) is not, and is found where it is written instead.
- **Two locators by shape**, in `data/locators.json`, each serving V8.1.1 and V8.3.1:
  - `locate.owner-checks`: a comparison, a string (SQL), an object key (`where: { userId: … }`), or a `filter_by`-like
    argument whose text names an owner or user id beside `=`, `!=`, or `:`.
  - `locate.refusals`: a 403 passed to a call, returned beside a body, or set on `status` or `statusCode`, and
    `Forbidden`, `PermissionDenied`, or a `ForbiddenError` raised or thrown.
  Their `names` is matched against the captured text made plain, as a name is; a file is parsed when a word in it, or
  its whole plain text, matches, since `owner = ?` is never one word.
- **Several locators to one question.** `code_lines::point_questions` kept the first locator's places for a question
  and dropped the rest. It now shows the first place of each locator, then the second, up to `MOST_POINTERS`, joins
  what each finds, and counts the rest. `report.json` and the MCP schema keep their shape.
- **The sentence** (`human::POINTERS_SAY`) says StackVet found the lines by their names or their shape.

**Tests.** `crates/sv-check/src/locate/examples_tests.rs`: the five example apps, each with the places it must find
written beside it, and each shape alone, with the OpenID address, the token grant, and the `authorization` header
found nowhere.

**Broken on purpose, each put back:** the property argument read again (2 red: the OpenID test and the examples);
the file prefilter by words only (2 red); a 403 as a keyword argument not read (2 red); SQL strings not read (2 red);
the `status` field rule dropped (1 red, once a `job.retries = 403` line was added; before that, none).

**Not done.** The first 403 shown in `notes-with-users` is a forged form's refusal (`app.py:133`), not the admin
page's (`:209`): backlog 0518.
