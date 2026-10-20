# V7.6.1's check by hand, proposed in backlog 0500 and never written (20 October 2026)

**Status:** open

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
