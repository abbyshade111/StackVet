# git log --all --diff-filter=A --name-only walks every commit's diff for the secrets-file check

**Status:** claimed by builder-full, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** Git computes the diff of every commit on every branch; the other five git calls per run are cheap.

**Where.** `crates/sv-check/src/git.rs:68-100` (`ever_added`), called from `config.rs:215` whenever no secrets file is tracked now (the common case).

**Evidence.** The command as built.

**How it bites.** Estimate: seconds on a repository with thousands of commits.

**Fix.** A pathspec naming only what `is_secret_file` looks for (`-- '*.env' '*.env.*' '*/id_rsa' …`), so git diffs only matching paths; the `config.rs` tests at `:973-1146` already build repositories with a file committed then removed.

**Record.** ADR-092 governs what is read of git history: an "unchanged, because" line.
