# A threat's "n of m requirements checked" counts a requirement checked in part as checked

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-run`, `sv-report`, `sv-scan`. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-report/src/threats.rs:338-356` (`ThreatLine::shown`): `format!("checked in part: {} of {of} requirement{} checked", self.checked.len(), ...)`; and `:436`: `Some(Status::Checked | Status::CheckedInPart) => line.checked.push(id.clone())`.
- What is wrong: `line.checked` holds the requirements whose status is *checked* **or** *checked in part* (line 436,
  by design: a threat is only ever checked in part). `shown()` then says "checked in part: 2 of 5 requirements
  checked" when both of the two were themselves only checked in part (one sample each, ADR-053). Backlog 0333's
  purpose was that a reader can tell "one in five looked at" from "all five checked in part"; this sentence still
  blurs the second case, in the other direction: a requirement with an in-part credit reads as "checked". The test at
  `crates/sv-report/tests/threats.rs:197-210` uses only a fully `Checked` requirement, so it does not see this.
- What to do: Keep the two lists apart in `ThreatLine` (a `checked_in_part: Vec<String>`), or count them separately
  in `shown()`: "checked in part: 1 of 5 requirements checked, 1 checked in part". Add a case to the 0333 test with a
  `Status::CheckedInPart` requirement and make it go red first. Nothing about the threat's status changes, and
  `evidence_words` must keep listing both.
- How verified: read.
- Overlap: 0333 (done) asked for the count; this is the one case its wording gets wrong. Nothing else in the backlog.
- Needs a decision record?: no (words only; the status and the rule "a threat is only ever checked in part" are
  ADR-053's and unchanged).
