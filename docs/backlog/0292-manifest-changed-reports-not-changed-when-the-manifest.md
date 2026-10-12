# manifest_changed reports not changed when the manifest cannot be read at the end of the run

**Status:** claimed by builder-none, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: code quality. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** `let now = std::fs::read(sv_manifest::locate(app_dir).ok()??.path).ok()?;` returns `None`, which the caller reads as the file is as it was. A `stackvet.toml` deleted or made unreadable during a run is the one case this guard exists for, and it says nothing.

**Where.** `crates/sv-cli/src/report_lock.rs:537-545`; the caller at `crates/sv-cli/src/report_folder.rs:83`.

**Evidence.** The line above: `.ok()?` on both `locate` and `read`.

**How it bites.** A failure reads as the good outcome, against the honesty rule.

**Fix.** Return the gap when `locate` or `read` fails ("could not be re-read at the end of the run"), distinct from "unchanged"; a test that removes the manifest during the run.

**Record.** ADR-041 governs the report folder and its lock: an "unchanged, because" line (this is the manifest guard beside it).
