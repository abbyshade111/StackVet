# A crashed answer to the wrong-issuer sign-in reads as a refusal (V10.5.3)

**Status:** open

Found by the ninth weekly review of the decision records (0742, 23 November 2026), reading ADR-021 and ADR-127 against
`main`. Read from the code; not run.

**What happens.** The check built in #2457 (backlog 0620) starts a second copy of the app while the test sign-in
provider's published details name another issuer, and makes one sign-in (`crates/sv-check/src/oidc/metadata.rs`). If
the private page then stays shut, the copy is taken to have refused, and V10.5.3 is credited in part once a control
copy, started with the details right, signs in. "Stayed shut" is judged by whether the private page opened
(`finish` in `crates/sv-check/src/oidc.rs`), so an app that answers the sign-in's return with a server error (a 500)
reads the same as one that refused.

**Why it matters.** ADR-021 says "A pass or a finding that rests on a request that crashed becomes not assessed", and
its 8 October entry says the same holds for the sign-in provider suite. That withholding works from a table
(`RESTS_ON_A_REFUSAL` in `crates/sv-check/src/signed_in/mod.rs`) that holds no rule of the provider suite, this one
included. An app that crashes on a wrong issuer has, in a sense, not signed anyone in; but a crash is not a refusal,
and the rule everywhere else is that a credit never rests on one. ADR-127's entry of 21 November names the control
start as the guard, and says nothing of a crash. The review noted the gap on ADR-021 as a dated entry.

**To do.** Decide which it is, and make the code and the two records say the same: either the return request's
answer is read, and a server error there makes V10.5.3 not assessed with the reason, or ADR-021 and ADR-127 say that
for this check the control start stands in for the crash rule, and why that is enough. A test with a callback that
answers 500 under the wrong details, failing today if the first is chosen.
