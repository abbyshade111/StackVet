# A record can govern a named part of a large file, not the whole file

**Status:** done, 12 October 2026

Found by the third weekly review (12 October 2026, backlog 0266) and chosen by the owner the same day. Thirteen records have
a decision living in a large shared file (`signed_in/mod.rs`, `signed_in/rules.rs`, `ai.rs`, `assemble.rs`, `main.rs`,
`lib.rs` in `sv-cli`, `sv-scan/src/lib.rs`, `sv-report/src/lib.rs`, `config.rs`): ADR-015, 021, 022, 023, 029, 037, 053, 082,
088, 091 among them. The owner decided on 6 October that these files stay ungoverned, because governing a whole file makes
nearly every pull request owe a line. A change that undoes such a decision is asked about by nothing.

**Plan, the owner's choice of 12 October 2026 (record ADR-103, proposed here, accepted in the pull request that builds it).**
A Governs pattern may name a part of a file: `path::NAME` fires only when the pull request's diff adds or removes a line
that contains NAME, as a whole word. A pull request that edits another part of the file owes nothing. `tools/adr_check.py`
gains the form with its self-test; `crates/sv-cli/tests/decision_records.rs` fails when a named part is no longer in its
file. Then the names go into the records where this week's review found real drift, first: ADR-021 (`RESTS_ON_A_REFUSAL`),
ADR-053, ADR-064 (`HANG_HOLD`), ADR-067 and ADR-088; the rest as each is next touched.

**Broken on purpose, each put back** (per CLAUDE.md): a pattern whose name never appears in the diff (the check must stay
silent), one whose name does (it must fire), a substring of a longer name (it must stay silent), and a name no longer in
its file (the references test must fail).

**Order.** After 0267, which is smaller and shares no file with this beyond `docs/adr/`.

**Claimed 12 October 2026 by session securevibe-weekly-adr-3**, in the claim pull request, with 0267.

**Built 12 October 2026** (ADR-103, accepted). `tools/adr_check.py` reads `path::NAME` and, for such a pattern, the lines the
pull request adds or removes (`git diff -U0`); `crates/sv-cli/tests/decision_records.rs` fails when a name is in no file its
path matches, and ends a record's Governs list the way the Python does. Named in ADR-021 and ADR-053 (`RESTS_ON_A_REFUSAL`),
ADR-064 (`HANG_HOLD`), ADR-067 (`STATIC_SESSION`) and ADR-088 (`KILL_SWITCH`, `kill_switch`). Run for real against this
tree: an edit elsewhere in `ai.rs` does not ask ADR-064; a change to the `HANG_HOLD` line does. Eight breaks put in on
purpose, each caught by the case written for it. **Remains:** the other records whose decisions live in the large shared files
(ADR-015, 022, 023, 029, 037, 082, 091 among them) take a name as each is next touched; the weekly review lists any left.
`ai.rs` was already governed whole by six records (ADR-071, 073, 074, 075, 079, 094), so the "large files stay ungoverned"
decision of 6 October had already been relaxed there.
