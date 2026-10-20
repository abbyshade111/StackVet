# V7.6.1's check by hand, proposed in backlog 0500 and never written (20 October 2026)

**Status:** done, 20 October 2026

Split from backlog 0500 at the owner's choice of 20 October 2026: 0500 proposed V7.6.1 as a check by hand, with its
reason, and it was never written. Severity: low.

**What it asks.** V7.6.1 (level 2): session lifetime and termination between the app and the identity provider it
signs people in through behave as documented, asking the person to sign in again when needed, such as when the
longest time between the provider's sign-ins is reached.

**What to do.** A check by hand in `data/human-checks.json`, in the words of someone who is not a programmer: sign in
through the provider, wait past the documented lifetime (or sign out at the provider), and see whether the app asks
again. It names its reason (no run can wait out a real provider's lifetime). Today the requirement is listed only in
`data/security-notes.json`'s `elsewhere` list, and `docs/REQUIREMENTS.md` shows it as "No check". A check by hand
credits nothing until the owner records it with `sv review`; `crates/sv-check/tests/human_checks.rs` refuses an
entry another catalog already asks about, so check the notes catalog first.

**Built, 20 October 2026.** `data/human-checks.json` has V7.6.1, after V7.3.2: sign in through the outside service,
sign out of it in another tab or wait past the time written down, and reload a private page; staying signed in
whatever happens at the service is the wrong answer, and it says a run cannot wait out a real service's time. It is
asked only where V7.6 applies (`oauth`, `data/applicability-v2.json`). Held by
`signing_in_through_another_service_gets_a_check_by_hand_that_says_why` (`crates/sv-check/tests/rules_only_the_owner_knows.rs`),
which fails with the entry taken out.
