# A sealed report folder is proven sv's wherever it sits and whenever it was written, so a copied or old report passes as current

**Status:** done, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: secrets. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** The seal checks nothing about which app or which run. The review seal was bound to the app folder for exactly this reason (ADR-026, item 11); the report seal was not.

**Where.** `crates/sv-cli/src/report_seal.rs:44-51` (`fields`: `report-folder`, file names, and SHA-256s only) and `:153-166` (`proven`); `crates/sv-cli/src/mcp/resources.rs:22` and `:110` offer any such folder below the root as "sealed when it was written and unchanged since".

**Evidence.** `fn fields(digests) { let mut fields = vec!["report-folder".to_owned()]; for name in SEALED { fields.push(name); fields.push(digest) } }`

**How it bites.** The AI coding tool keeps a copy of a clean report `sv` sealed on this computer (an earlier run of this app, or another app's) and puts it back into `stackvet-report/` after a run it did not like, or copies it from a sibling project; `resources/list` presents it as `sv`'s current report, and `refuse_older` (`report_lock.rs:479`) never runs because `sv` is not writing.

**Fix.** Add the app's id (`seal::App::of(folder.parent())`) and `run_record.started_unix_ms` to the sealed fields; in `proven`, recompute the app id from where the folder is and refuse with "sealed for another folder"; let the resource description say when it was written; tests for a copied folder and an older one.

**Record.** ADR-034 governs the report seal: a Later line, since what the seal proves changes.
