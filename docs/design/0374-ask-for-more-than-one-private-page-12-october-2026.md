# Ask for more than one private page (12 October 2026)

Backlog 0232, part 8 (group H of the one-sample inventory). No check changes.

**The problem.** Several checks ask each private page stackvet.toml lists and credit in part when only one is listed:
the made-up session (V7.2.1), the logging of a refused request (V16.3.2), caching (V14.3.2), and the sign-out control
(V7.4.4). Groups B and E made the first two whole when two or more pages are listed. Nothing told the owner or the AI
tool that listing one page caps those credits.

**What was built.** `sv preflight` has an item, `pages`, "look at this", when exactly one private page is listed (a page
with a placeholder does not count): it names the page, says which checks ask each listed private page and that they
can then credit only in part, and asks for the app's other private pages. Admin pages are not counted: many apps have
one, and the settings cannot say there are no others. The spec `sv init` writes says, beside `private` and `admin`,
to list every one and why.

**The example.** `examples/notes-with-users` lists its four private pages (`/account`, `/notes`, `/password`,
`/account/email`), and the three form pages now send `Cache-Control: no-store` and a sign-out control as `/account`
does. `crates/sv-cli/tests/host_cookie.rs` expects four pages opened in the browser.

**Tests.** `crates/sv-cli/src/preflight/pages_tests.rs` (two). The app in the preflight tests that gives `sv run`
everything now lists two private pages. **Broken on purpose, each put back:** the item never said (2 red); a page with
a placeholder counted (1 red); the item said with two pages (3 red).
