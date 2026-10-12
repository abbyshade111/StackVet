# Group A: the header checks ask every route stackvet.toml names (11 October 2026)


Backlog 0232, part 1 (group A of the one-sample inventory), and ADR-096. Built by session backlog-org-0232.

**The problem.** The header, cookie, and content-type checks read one to three anonymous answers (the health path,
the root, and a missing page) and were credited *checked in part* for it (ADR-053, Later). Their requirements name
more: V3.4.4 and V3.4.6 all and every HTTP response, V4.1.1 every response with a body, V3.4.8 every response that
renders a document, and V3.3.2 and V3.3.4 each cookie.

**What was built.** `probes::named_routes` collects the paths stackvet.toml names under `[stack.run.users]`, once each,
leaving out placeholders, the health path, and the root; `probes::route_requests` asks each with an anonymous GET, id
`route:<path>`, which `sv-cli`'s `anonymous_requests` adds to the run. The page checks read those answers whatever their
status: `pages`, `html_documents`, the content-type set, and the new `policy_answers` (the CSP's reporting, now read on
every policy, not the health path's alone). `probes::verified_with_routes` gives the five credits whole only when there
were named routes, every one answered, and every answer passed; `probes::verified` still gives them in part.

**What it changes in a report.** An app whose sign-in page, or a redirect from a private page, lacks a header has a
finding naming that route. An app whose named routes all answer correctly is *checked* on V3.4.3 to V3.4.8, V3.3.2,
V3.3.4, and V4.1.1, not in part. The error-page check stays in part.

**Tests.** `crates/sv-check/src/probes/routes_tests.rs` (six) and `crates/sv-cli/src/named_routes_tests.rs` (two).
Every existing probe test passes unchanged.

**Broken on purpose, each put back:**

- Route answers dropped from the judged pages: 1 test red (a redirect without a header is found).
- The credit whole without every route: 2 red (no routes, and one unanswered).
- The run not asking the routes: 1 red. The evidence read without them: 1 red.

**Not run here.** The Docker-backed tests: the build folder is on an external drive Colima does not share. The full
workspace suite was run with no container backend; its only failures were three this computer causes (no `py7zr`, a
commit made during the run, and a space in the build folder's path), none in the code changed.

**Found on the way.** `missing_headers` accepts `X-Frame-Options` toward V3.4.6, which the requirement says may not be
relied upon: backlog 0264.
