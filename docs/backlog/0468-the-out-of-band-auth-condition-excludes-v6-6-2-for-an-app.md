# The `out-of-band-auth` condition excludes V6.6.2 for an app whose emailed sign-in codes `sv` itself checks against V6.6.2

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: requirements and tests (citations). Severity: low. Verified against the code by the reviewing session before filing.

- Where: `data/applicability-v2.json:372-376` (`"scope": "V6.6.2", "condition": "out-of-band-auth", "source": "claim"`,
  reason "does not use out-of-band sign-in codes sent by phone, SMS or push notification");
  `crates/sv-manifest/src/spec.rs:272` (`# out-of-band-auth = ?  # sign-in codes sent by phone, SMS or push notification`);
  `data/claim-corroborators.json:1137-1160` (corroborated by twilio, vonage, firebase-admin only);
  `crates/sv-manifest/src/lib.rs:1913` (the condition is the claim alone); `crates/sv-check/src/signed_in/codes.rs:78-86`
  (`EMAIL_CODE_UNBOUND` credits V6.6.2 from the `email-code` flow).
- What is wrong: V6.6.2: "Verify that out-of-band authentication requests, codes, or tokens are bound to the
  original authentication request…". `sv` treats an emailed sign-in code as one of those (it credits and finds
  against V6.6.2 and V6.6.3 from `email-code`), but the question that decides whether V6.6.2 applies names phone,
  SMS, and push only. An owner with emailed codes answers `out-of-band-auth = false` truthfully by the spec's
  words, and V6.6.2 is excluded "not applicable" while the check that speaks to it ran. V6.6.3 is unaffected
  (its rule is `auth`, from `data/knowledge/applicability.json`).
- What to do: let an `email-code` entry under `[stack.run.users]` corroborate `out-of-band-auth` (resolve adds,
  never removes), or widen the spec's wording and the reason to "by phone, SMS, push notification, or email".
  Must not lose: the owner's `false` still labeled as an unverifiable claim where nothing corroborates it.
- How verified: read.
- Overlap: none found (`grep -rli out-of-band-auth docs/backlog/` is empty).
- Needs a decision record?: yes, a small one: a condition's meaning changes which requirements apply (a Later
  line on the record that governs `data/applicability-v2.json`, or a new short ADR if none does).
