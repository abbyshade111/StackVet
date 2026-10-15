# `tools/install.sh` copies `target/release/sv` after a build that `CARGO_TARGET_DIR` sent elsewhere, and reports the stale one installed

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Cursor trial. Severity: low (caught by the
tool only because it compared commits).

**What is wrong.** `tools/install.sh:33` builds with `cargo build --release --locked -p sv-cli` and then copies
`$repository/target/release/sv`. Cursor's sandbox sets `CARGO_TARGET_DIR`, so the fresh build landed elsewhere and the
old binary (10b39332) was copied and reported "Installed." The tool noticed only by comparing `sv --version` with
the pulled commit and re-ran with `env -u CARGO_TARGET_DIR`.

**Fix.** Pass `--target-dir "$repository/target"` (or read `cargo metadata`'s `target_directory`), and after copying,
compare the installed `sv --version` commit with `git rev-parse HEAD` and fail with the two hashes when they differ.

**Record.** None.

**Done, 15 October 2026.** `tools/install.sh` builds with `--target-dir "$repository/target"`, so the build lands
where the copy reads it whatever `CARGO_TARGET_DIR` says. Before anything is put in place, it compares the commit
the built `sv --version` names with the checkout's `git rev-parse HEAD`; when they differ, it names both, says nothing
was installed, and stops. Without git, or with `SV_BINARY` (a program built elsewhere), nothing is compared. Tests:
`crates/sv-cli/tests/install_target_dir.rs` (2), a scratch checkout with a stand-in `cargo`; each half broken on
purpose turned its test red.
