# Give the example app an email change, so the email-change probe runs in CI

**Status:** done, 11 October 2026

From 0029, part 14. The probe `probe.email-change-without-password` is built and tested against the example app, but the example app has no email change, so it has never run against an app that has one. Add an email-change flow to the example app, with a test that the probe finds it. Until then the probe's only real-app evidence is the owner's own run.

**Done, 11 October 2026:** `examples/notes-with-users` changes an account's email address at `/account/email`, only
with its password, reading only the new address from the form, and its `stackvet.toml` declares `change-email`.
`crates/sv-cli/tests/email_change_example.rs` runs `sv report --run` on a copy of the example, where V7.5.1 is credited
in part, and on a copy with the password check taken out, where `probe.email-change-without-password` is the finding.
Both were run here by hand under Colima, from copies under the home folder, and came out so: the example refused a
wrong password (403) and took the right one (303); the broken copy took the wrong one (303). The role probe that rides
on `change-email` (`probe.email-change-role-trusted`) ran too: five role fields sent with the change, no finding. The
test itself ran here only on its no-backend path, because Colima does not share the build folder on the external
drive; CI runs it under Docker with `SV_REQUIRE_BACKEND=1`.
