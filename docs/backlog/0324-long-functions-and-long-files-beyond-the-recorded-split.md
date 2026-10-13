# Long functions and long files beyond the recorded split: seven files over 3,000 lines and 68 non-test functions over 150

**Status:** open

**Was queued on the coordination board** (`docs/coordination/board.md`, backlog 0273), which closed on 13 October 2026 when the trial ended; it is taken the normal way since, from the roadmap, with any hold below still standing. From the deep review of 12 October 2026 (backlog 0277), area: code quality. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** The `ast.rs` and `sbom.rs` split (0187) is done; these are the next along natural seams: `docker.rs` holds the fence, the helpers (mail, OIDC, model, browser), the install step, and the probe script runner; `signed_in/mod.rs` holds the HTTP/cookie/page helpers and the suite runner.

**Where.** Files: `crates/sv-run/src/docker.rs` 5,145; `crates/sv-check/src/signed_in/mod.rs` 4,792; `crates/sv-check/src/ai.rs` 3,924; `crates/sv-cli/src/main.rs` 3,576; `crates/sv-check/src/signed_in/sessions.rs` 3,275; `crates/sv-report/src/lib.rs` 3,239; `crates/sv-manifest/src/lib.rs` 3,010. Functions: `ai.rs:788 run` (712 lines), `signed_in/mod.rs:1741 run_checks` (675), `sv-report/src/html.rs:218 page` (660), `markdown.rs:171 compliance` (483), `production.rs:721 run` (450), `main.rs:1354 cmd_check` (449), `adapters.rs:1155 run_one_recorded` (379).

**Evidence.** Line counts above.

**How it bites.** Maintainability; each is the file a new session reads whole to change one thing.

**Fix.** Split `docker.rs` and `signed_in/mod.rs` first, along the seams named, in two or three short pull requests each, behavior unchanged; the ten `#[allow(clippy::too_many_arguments)]` (`sv-cli/src/review.rs` ×6, `signed_in/{account_changes,owner_field,uploads}.rs`, `sv-cli/src/{seen,static_scan}.rs`) mark where a parameter struct is owed.

**Record.** ADR-019 and ADR-052 govern `docker.rs`: a Later line naming the new files' names, since their Governs lists change.
