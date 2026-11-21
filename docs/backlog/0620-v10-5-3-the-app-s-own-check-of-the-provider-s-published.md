# V10.5.3: the app's own check of the provider's published details, which needs a second start of the app (20 October 2026)

**Status:** claimed by claude/0620-issuer-check, 21 November 2026

Split from backlog 0500 at the owner's choice of 20 October 2026, as the one piece of it not built. Severity: low.

**What it asks.** V10.5.3 (level 2): the app rejects an authorization server's metadata whose issuer does not exactly
match the issuer the app expects, so a malicious server cannot pass itself off as another through its metadata.

**What was proposed** (0500): the test provider serves a discovery document whose `issuer` differs from the address
it was fetched from, and the check expects the app to refuse to sign anybody in.

**Why it waits.** ADR-127's context and `crates/sv-check/src/oidc.rs` say the provider's published details must be
wrong from the app's first read of them, and the run starts the app once; an app that read the right document at
start never reads the wrong one. Building it needs a second start of the app with the provider set wrong from the
beginning, which nothing in `sv run` does yet. A decision record is needed when built (ADR-127, Later).

**On hold, 24 October 2026, at the owner's word:** "put 0620 on hold in the hold folder, then take 0631 and 0628 in
either order, then develop a plan for 0625 once those are complete". Asked first because building it gives `sv run` a
second start of the app for every app that signs people in through a provider, which changes what `sv` runs and how
long a run takes (ADR-127, Later, 24 October 2026).

**Taken off hold, 20 November 2026, at the owner's word.** Asked whether `sv run` may give an app that signs people in
through a provider a second start, with the test provider's published details set wrong from the beginning, the owner
answered yes. So the item is open again, to be built as planned above: the second start, the check that the app then
refuses to sign anybody in, and ADR-127's entry saying how the second start works and what it adds to a run's time.
