# The code sign-ins in the log (15 October 2026)


Backlog 0232, group E, the sign-ins, second batch (ADR-053, "Later, 15 October 2026: the code sign-ins in the log").
The first batch is "The sign-ins in the log, whole when every one is found (15 October 2026)".

**What was built.** `crate::logs::FactorMarkers` holds, for each way of signing in with a code, the window around a
code the app took and the window around one it refused. The emailed-code check brackets its first code
(`email-code-use-1`) and the code it sends across sessions (`email-code-use-crossed`); the two-factor check brackets
the code step of its old code (`totp-1`) and of the current one (`totp-2-0`). `Markers::mark_factor` takes the way off
`untried`. The log reader reads a window for a line that names a sign-in or a code given to sign in, and says it
failed or not to match.

**Why only the code step for two-factor.** Its attempt signs in with the password first, and the app logs that. A
window around the whole attempt would hold the password's "signed in" line, and an app that logs nothing about codes
would be credited with logging the code.

**Why never the bare word `code`.** Access lines carry `status_code` and `error_code`. Read as code words, a JSON
access line inside the window would stand for the event. The test
`an_access_line_with_a_status_code_is_not_a_code_given` is the witness.

**Tests.** `crates/sv-check/src/signed_in/log_codes_tests.rs`: one run with sign-up, a seeded two-factor account, and
the emailed code, in both log styles; whole when everything is found; in part when the codes are not logged (the
fake's `logs_no_codes`), and when the app takes the old code and a code from another session, so no refusal could be
tried. Broken on purpose, each put back: the code words unread; the old code taken still counted as a refusal; the
emailed code never marked; the bare word `code` read. Each turned its own tests red.
