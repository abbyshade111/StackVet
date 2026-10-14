# Without an `owned` record the forged-form and other-user checks do not run, and preflight does not say which checks a missing key costs

**Status:** open

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
