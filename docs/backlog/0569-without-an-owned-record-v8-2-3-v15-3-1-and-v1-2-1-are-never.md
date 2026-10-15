# Without an owned record, V8.2.3, V15.3.1, and V1.2.1 are never listed as not assessed

**Status:** open

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
