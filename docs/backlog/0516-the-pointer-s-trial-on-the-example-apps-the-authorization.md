# The pointer's trial on the example apps: the authorization locator finds nothing where the check is inline and points at an OpenID URL where a name starts with authori

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

The one-chapter trial the owner asked for on 13 October (0487's decision), run by the orchestrating session on
14 October 2026 against `main` at 0ba0c5e7, with `sv questions` on the four example apps. The locator
`locate.authorization-checks` (`data/locators.json`, the only one) serves V8.1.1 and V8.3.1 by matching names
(`^(authori[sz]\w*|requirerole\w*|hasrole\w*|canedit\w*|isadmin|...)$`) on definitions, calls, decorators, and
attribute accesses in Python, JavaScript, and TypeScript.

**What it found:**
- `flask-booking`: no pointer. Right: the app (34 lines) has no authorization code.
- `notes-with-users`: no pointer. Wrong by omission: the app refuses a stranger's note with a 403 and an
  `authorization-refused` event (`app.py:209-210`) and reads notes with `where id = ? and owner = ?` (`:241`), but
  inline, inside `do_GET`, with no function whose name says so. The person is told nothing where the real check is.
- `oidc-notes`: a pointer at `server.js:69`, `const to = new URL(config.authorization_endpoint)`, offered under both
  V8.1.1 and V8.3.1. Wrong: that is the OpenID provider's address, not a check; `authori[sz]\w*` matches the property
  name `authorization_endpoint` (and would match `authorization_code` and the `authorization` header at `:99` and
  `:108`). A person is sent to look at the wrong lines, with the sentence that says they are where to look.
- `tested-notes`: no pointer; not read for whether it should have one.

**What it means.** Name-based location is too weak for authorization on small apps, where the check is a comparison
and a 403 rather than a named function, and too broad where OAuth vocabulary shares the prefix. The pointer's own
sentence ("where to look, not evidence: StackVet found them by their names") is honest, but a wrong pointer under
"In the code" still steers the person, and no pointer at all on the one app that has a real check is the common case.

**What to do**, before any of the proposals that rest on pointers (0488 to 0500) is taken:
1. Match only the names of functions that are defined or called, never a property access, a URL field, or a header
   name; drop OAuth and OpenID vocabulary from the pattern (`authorization_endpoint`, `authorization_code`,
   `authorization` as a header) by shape, not by a longer regex.
2. Add shape-based locators for "where the app refuses someone": a 403 sent, raised, or returned (`abort(403)`,
   `send(403, ...)`, `res.sendStatus(403)`, `res.status(403)`, `raise Forbidden`, `HTTPException(403)`), a
   comparison of a record's owner or user id with the session's, and a query with `owner = ?` or `user_id = ?`. Each
   is where to look, never evidence, as now.
3. Hold the locator to the four example apps as fixtures: `notes-with-users` must point at `app.py:209` and `:241`,
   `oidc-notes` must not point at `server.js:69`, `flask-booking` none, with the expected pointers written beside each
   so a change that loses one goes red.
4. Then try the chapter again and write the result here; the proposals wait on that second reading.
**Record:** ADR-022's Later entry of 14 October (the pointer credits nothing) stands; say so with an unchanged line.
