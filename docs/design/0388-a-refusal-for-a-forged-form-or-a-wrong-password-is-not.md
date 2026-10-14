# A refusal for a forged form or a wrong password is not pointed at (14 October 2026)


Backlog 0518, from the second reading of 0516. ADR-022's Later entry of 14 October (a pointer credits nothing) stands.

**The problem.** `locate.refusals` (0516) finds every 403 an app sends, and a question shows the first few. In
`notes-with-users` the first is `app.py:133`, the 403 for a form whose anti-forgery token is wrong, so V8.1.1 and
V8.3.1 showed a line about forged requests, while the admin page's refusal (`:209`, `:210`), the real check of what a
person may do, was found and not shown. Nine of the app's eleven 403s are of that kind: a forged form, or a wrong
password at sign-in or before a change.

**What was built.** A locator may say `unless`: a place is left out when the condition of the nearest `if` (or
`elif`) above it, made plain, matches. `locate.refusals` says `csrf`, `xsrf`, `forged`, `forgery`, `token`,
`password`, `passwd`, `passphrase`, `comparedigest`, and `checkpw`. Only the nearest `if` counts, so a 403 for the
wrong owner inside an `if` about a token stays, and an `elif` is an `if` of its own. The refusal is left out by the
shape of what leads to it, not by its file order or by reading what it does; it stays where to look, never evidence.
The other path the item named, ordering places by nearness to the question, was not built: leaving out what is about
another check was enough for the example, and is the narrower change.

**On `notes-with-users`:** `locate.refusals` now finds `:209` and `:210` alone, and V8.1.1 and V8.3.1 show `:241`,
`:209`, and `:316`, with two more found. The other four example apps still get no pointer.

**Tests.** `crates/sv-check/src/locate/examples_tests.rs`: the example fixture, now `:209` and `:210` alone; and a
test of `unless` in Python and JavaScript. `crates/sv-cli/tests/example_pointers.rs`: the binary's V8.3.1 on
`notes-with-users` shows `:209` or `:210` and never `:133`.

**Broken on purpose, each put back:** no `unless` (2 red, and the end-to-end test); the outermost `if` read instead
of the nearest (2 red); `elif` not read (none at first, since the test's `elif` sat under an `if` about a token; 1 red
once it sat under one about the person); the password words dropped (1 red).
