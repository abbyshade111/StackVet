# Host paths go to docker -v in the short form, so an app folder whose path holds a colon cannot be mounted and the run does not say why

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: the fence. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** A macOS folder such as `Notes: 2026/app` makes Docker refuse the mount (it fails, it does not mount elsewhere), and the run reports a backend failure that does not name the cause.

**Where.** `crates/sv-run/src/docker.rs:649` (`format!("{}:/app:ro", plan.app_dir.display())`), `crates/sv-run/src/install.rs:141` (`"{}:/in/{name}:ro"`).

**Evidence.** The two `format!` calls.

**How it bites.** An owner with a colon in a folder name sees "could not run" with no reason.

**Fix.** `--mount type=bind,src=...,dst=/app,readonly`, or say in `CannotRun` that the folder's path holds a character Docker's `-v` cannot carry; a test with such a path.

**Record.** ADR-019 governs `docker.rs`, ADR-052 `install.rs`: "unchanged, because" lines.
