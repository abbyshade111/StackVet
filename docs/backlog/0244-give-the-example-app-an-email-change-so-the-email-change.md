# Give the example app an email change, so the email-change probe runs in CI

**Status:** claimed by backlog-org-0244, 11 October 2026

From 0029, part 14. The probe `probe.email-change-without-password` is built and tested against the example app, but the example app has no email change, so it has never run against an app that has one. Add an email-change flow to the example app, with a test that the probe finds it. Until then the probe's only real-app evidence is the owner's own run.
