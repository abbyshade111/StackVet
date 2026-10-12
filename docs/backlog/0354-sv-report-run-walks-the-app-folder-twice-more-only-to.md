# sv report --run walks the app folder twice more, only to compute the private files

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** Three walks per `--run` (the static stage plus two), the second and third only for `private_files`.

**Where.** `crates/sv-cli/src/lib.rs:871`: `more_questions` calls `Listing::of(&plan.app_dir)`, from `anonymous_requests` (`:279`) and again from `running_app_evidence` (`:307`).

**Evidence.** The two call sites.

**How it bites.** Estimate: 50 to 300 ms per walk of a 10,000-file app.

**Fix.** Compute `(admin_pages, private_files)` once from the static scan's listing and carry it in the `RunPlan`; extend `tests/one_walk.rs` to the `--run` path.

**Record.** ADR-031 governs the walk: an "unchanged, because" line.
