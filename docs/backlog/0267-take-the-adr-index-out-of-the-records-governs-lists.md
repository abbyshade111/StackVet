# Take the ADR index out of the records' Governs lists

**Status:** claimed by securevibe-weekly-adr-3, 12 October 2026

Found by the third weekly review (12 October 2026, backlog 0266) and chosen by the owner the same day ("yes, please claim
both as backlog items and fix as you recommend"). `docs/adr/README.md` is in the Governs lists of thirteen records (ADR-017,
042, 044, 045, 046, 049, 054, 055, 077, 092, 096, 098, 099), so every new record's index row makes each owe an
"unchanged, because" line, and the lines repeat one sentence: 1,642 lines in 419 pull requests in a week. A reason that is
the same every time carries nothing.

**Plan.** Take `docs/adr/README.md` out of those thirteen Governs lists, with a dated "Later" entry on each saying why. The
index decides nothing, `crates/sv-cli/tests/decision_records.rs` already fails when a record has no index row, and the
weekly review checks the status labels in it. No new record: the change is to the lists, and ADR-103 (backlog 0268)
describes the rule they follow.

**Claimed 12 October 2026 by session securevibe-weekly-adr-3**, in the claim pull request, with 0268.
