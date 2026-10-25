# The browser checks sign in through the provider, so storage is seen cleared after that sign-out too

**Status:** done, 25 October 2026

Filed 21 October 2026 from backlog 0232, part F, at the owner's word of the same day. Severity: low (V14.3.1 is
checked after the password sign-in only).

**What is missing.** The real-browser checks (`crates/sv-check/src/browser.rs`) sign in through `[stack.run.users]`
alone: storage cleared at sign-out (`probe.storage-kept-after-sign-out`, V14.3.1) is seen for that sign-in and no
other. The owner chose on 20 October 2026 that the sign-out and storage checks run after every way of signing in; the
sign-out check does for a sign-in through another service since 0232 part B (`[stack.run.oidc] logout`), but the
browser cannot sign in through the test provider at all.

**To do:** the browser driven through `[stack.run.oidc] start`, the test provider's sign-in, and back, as `oidc.rs`
drives it without a browser; then the storage check after `logout` there, its credit whole only when every way the
browser signed in was signed out and storage was cleared after each. An API key and a magic link cannot be driven by
a run, and stay *checked in part* for good (the owner's choice of 20 October 2026).

**Plan, 25 October 2026, with the claim.** Checked against `main` that day: `browser.rs` still signs in through
`[stack.run.users]` alone. The browser is driven through `[stack.run.oidc] start`, the test provider's sign-in, and
back to the app, as `oidc.rs` drives it without one; then signed out through `[stack.run.oidc] logout`, and its
storage read as after the password sign-out. A value kept is `probe.storage-kept-after-sign-out` for this way in too;
cleared is credited in part, apart from the password sign-out's credit, as ADR-053's Later entry of the owner's
choice of 20 October 2026 says both stay ("whole would need every way in signed out, and an API key and a magic link
cannot be"). Recorded as a dated Later entry on ADR-053 in the pull request that builds it. Without `logout`, or with
no browser in the run, it is said not assessed, as now.

**Built 25 October 2026** (ADR-053, Later). `crates/sv-check/src/oidc/storage.rs` signs in once more through the test
provider when the run has a browser, hands the browser that session, and reads its storage before and after the
sign-out control leading to `[stack.run.oidc] logout`, through the check the password sign-in uses, now
shared by both ways in. Credited in part and apart, as ADR-053 says. The test provider gained a `refuse`
mode, set for the browser's job, after a real browser showed the provider signing the person straight back in.
Tests: three in `oidc/storage/tests.rs`, two in `script/oidc_browser_tests.rs` (the browser asked for only when the run
has one), one in `browser/tests/storage_wording_tests.rs` (the password sign-in's words kept), and the provider's
contract test for `refuse`. Broken on purpose, one at a time: the step not called, the browser not asked for, the way
in left unnamed, an empty session, the wrong sign-out path, the provider not set to refuse, and the password wording's
colon each turned a test red, the last only after its test was added. Tried end to end in Docker with a real browser on
two copies of `examples/oidc-notes`: one credited in part, one a finding.

