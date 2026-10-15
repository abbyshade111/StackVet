# Without an `owned` record the forged-form and other-user checks do not run, and preflight does not say which checks a missing key costs

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Cursor and VS Code apps. Severity: low
(usability, and the prompts).

**What is wrong.** Two of the three tools left `owned` out of `[stack.run.users]`; the reports' gaps say "stackvet.toml
lists no `owned` record" for V3.5.1, V3.5.2, and V8.2.2, so on those two apps sv's own forged-form check never ran and
the only word on CSRF was a false Django finding. `sv preflight` passed the manifests.

**Fix.** Preflight lists, for each `[stack.run.users]` key the manifest lacks, the checks that will not run and the
requirements they speak to, in one table; and the setup prompt that asks for the manifest says in plain words what
`owned` is for ("a record one person makes that another must not see: it lets sv try to read it as somebody else and
to submit a forged form").

**Record.** None for preflight; the prompt library is 0370's.

**Done, 15 October 2026:** `sv preflight` and `stackvet_preflight` end with a table, "What leaving a key out of
`[stack.run.users]` costs": for each optional key the manifest leaves out, the checks that will not run and the
requirements the report will list as not assessed (`leftOut` in the structured result). The rows live beside the
checks (`crates/sv-check/src/signed_in/unset_keys.rs`), and `unset_keys_tests.rs` runs the signed-in suite against the
fake app with and without each key, so a row names only what leaving its key out makes not assessed. It is kept out of
the count of what to look at, since a key left out is not something `sv run` needs. The build prompt (the setup page's
prompt 5 and the guide's step 4) now says "Fill in `owned` there too when the app has a record one person makes that
another must not see: it lets StackVet try to read it as somebody else and to submit a forged form." ADR-035 has a
Later entry.

Found while building it, not fixed here: without `owned`, the record-fields, stored-markup, and owner-field checks
return without a not-assessed line of their own, so a run with no `owned` never lists V8.2.3, V15.3.1, or V1.2.1 as
not assessed (checked against the fake app on 15 October 2026). Filed as backlog 0569.
