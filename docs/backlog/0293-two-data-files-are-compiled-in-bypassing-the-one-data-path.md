# Two data files are compiled in, bypassing the one data path, and the guard test cannot see them

**Status:** claimed by builder-full, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: code quality. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** `data/README.md` records both as compiled in, so it is a decision, not an oversight. But `SV_DATA_DIR` cannot override them, and the test matches only the literal text `env!("CARGO_MANIFEST_DIR")`: it would also miss `option_env!("CARGO_MANIFEST_DIR")` and a path written with spaces inside the macro.

**Where.** `crates/sv-check/src/signed_in/passwords.rs:23` (`include_str!("../../../../data/breached-password-evidence.json")`) and `crates/sv-report/src/threats.rs:95` (`atlas-references.json`); the test `nothing_but_the_data_finder_reads_from_the_build_folder_at_run_time` in `crates/sv-cli/tests/moved.rs:106-218`.

**Evidence.** The two `include_str!` lines and the test's literal match.

**How it bites.** A third compiled-in file can arrive unrecorded, and ADR-036's one way of finding data has two exceptions nothing tests.

**Fix.** Have the test also flag `include_str!`/`include_bytes!` of anything under `data/` unless the file is on an allow-list that `data/README.md` must name; or load the two through `sv_frameworks::data` like everything else.

**Record.** ADR-036 governs the data path: a Later line naming the two exceptions, or the build that removes them.
