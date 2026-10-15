# The sign-ins in the log, whole when every one is found (15 October 2026)


Backlog 0232, group E, the sign-ins (ADR-053, "Later, 15 October 2026: the sign-ins in the log").

**What was built.** The log check for V16.3.1 reads three sign-ins: one refused for a name with no account
(`log-marker-failed`), one accepted by an account made for nothing else (`log-marker-ok`), and now the
password-guessing check's first wrong password at the account it made (`guess-0`), bracketed by
`/sv-log-before-wrong-…` and `/sv-log-after-wrong-…`. The credit is whole when all three are found and stackvet.toml
names no other way to authenticate; otherwise it is in part and names what was not shown.

**Why the guess, and not a sign-in of its own.** The first try sent a wrong password for the accepted account straight
after it. That took the busiest run's sign-ins to 63, one past the 62 the spec promises the builder
(`a_run_signs_in_no_more_often_than_the_spec_says` caught it). The guessing check's first guess is already that event,
below any limit, at an account that exists, so bracketing it costs two requests for pages nobody has and no sign-in.
With no `failed-sign-ins` under `[policy]`, there is no guess, and the credit says the wrong password could not be
tried.

**Tests.** `crates/sv-check/src/signed_in/log_sign_ins_tests.rs`, against the fake app in both log styles: whole when
everything is found; in part when the app does not log a wrong password for a real account (the fake's flaw
`logs_no_wrong_password`), when a password change is named, and when no guess was made. Broken on purpose, each put
back: the not-found wrong password ignored; the other ways never listed; the first guess not bracketed. Each turned
the tests that guard it red, and only those.
