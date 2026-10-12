# Take the ADR index out of the records' Governs lists

**Status:** done, 12 October 2026

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

**Built 12 October 2026, and the plan changed on the way.** None of the thirteen records governed the index. Ten wrapped
the standing sentence ("A pull request that changes ...") so that the check's split, `". A pull request"`, missed it, and
the footer's `(docs/adr/README.md)` was read as a governed file; two more ended their list with `.) A pull request`. So the
cause was in `tools/adr_check.py`, not in the lists: `governs()` now ends the list at the standing sentence however the
line is wrapped, with `.)` as well as `.`, and the self-test holds both forms (the wrapped one fails with the old split;
checked). After the change no record governs anything under `docs/adr/`, and the thirteen lists are as they were written. No
record needed a "Later" entry, since the thing they governed did not change. The weekly review's remark in 0266, that "the
index is governed by 8 or more records", was the parser's artifact and is corrected here.
