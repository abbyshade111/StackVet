# Without an owned record, V8.2.3, V15.3.1, and V1.2.1 are never listed as not assessed

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found 15 October 2026 while building backlog 0533. Severity: medium (honest evidence: a check that did not run is
not said).

**What is wrong.** When `[stack.run.users]` has no `owned`, `owned_checks` (crates/sv-check/src/signed_in/admin.rs,
the `let Some(owned) = &users.owned else` near line 682) records one not-assessed line, for V8.2.2, V3.5.1, and
V3.5.2, and returns. The checks it would have gone on to run, `record_fields_check` (V15.3.1, V8.2.3) and
`stored_markup_check` (V1.2.1), get no line of their own, so the report never lists those requirements as not
assessed for this reason. Checked against the fake app: with `users_full()` less `owned`, the run's not-assessed lines
name none of V8.2.3, V15.3.1, or V1.2.1 (V15.3.3 is named, for another reason).

**Fix.** Name those requirements in the `owned` gap, or give each skipped check its own line, after reading each
requirement's text to confirm the check speaks to it; then add their ids to the `owned` row of
`crates/sv-check/src/signed_in/unset_keys.rs`, whose test will then hold them.

**Done, 15 October 2026** (documentation session). With no `owned` under `[stack.run.users]`, the run now records a
second not-assessed line, for V15.3.1, V8.2.3, and V1.2.1: the record-fields check and the stored-markup check run
only on the record `owned` makes (their only callers in `crates/sv-check/src/signed_in/admin.rs`). Each requirement's
text was read first: V15.3.1 and V8.2.3 are the fields a record hands back, V1.2.1 output encoding. The three ids are
added to the `owned` row of `crates/sv-check/src/signed_in/unset_keys.rs`, so `sv preflight` names them too, and its
test holds them to the run; with the line's ids changed, that test names V15.3.1 and goes red.
