# "Not a git repository" is said of a repository with no commits yet

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: low (wording).

**Fix.** Tell the two apart: "a git repository with no commits yet" gets its own line, and the AI-process section says
what that means for the commit checks.

**Record.** None.

**Done, 15 October 2026** (documentation). The AI-commits check (`config.rs`, `ai_commits_off_merge`, ADR-092) tells a
git repository with no commits yet from a folder that is not one: the repository is found, git reads its files, and
there is no commit checked out. It then says "This folder is a git repository with no commits yet, so there is no
history to read: which commits an AI coding tool made outside a merge can be said once there are some", in the
AI-process section where it was "not a git repository, or git could not read its history". Git reads nothing the
repository names (`git::head_commit` and `git::ls_files`, ADR-032). Tests: `config_no_commits_tests.rs` (3); with the
new case switched off, the empty repository's test fails.
