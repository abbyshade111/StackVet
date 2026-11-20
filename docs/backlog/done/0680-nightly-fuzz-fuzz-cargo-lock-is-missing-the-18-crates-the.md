# Nightly: Fuzz: fuzz/Cargo.lock is missing the 18 crates the workspace added with sv report's fetch

**Status:** done, 20 November 2026

Recorded by the nightly CI routine on 29 October 2026. It is not a failed run: no Fuzz or Audit run happened on `main`
in the 26 hours before this routine (both are weekly, on Mondays), so the routine ran both itself, on `main` at
`9567a1f`, as the workflows do.

- **What was found:** building the fuzz targets (`cargo +nightly fuzz run`, as `.github/workflows/fuzz.yml` does)
  rewrote `fuzz/Cargo.lock`, adding 18 packages it did not list: `bitflags`, `bumpalo`, `cfg_aliases`, `crc32fast`,
  `flate2`, `js-sys`, `nix`, `once_cell`, `rustversion`, `typed-path`, `wasm-bindgen` and its three helper crates,
  `windows-link`, `windows-sys`, `zip`, and `zlib-rs` (161 lines added, none removed). So the fuzz build is again made
  from whatever resolves that day, not from a committed lock, which is what backlog 0360 (done, 13 October 2026) fixed
  before.
- **Which pull request brought it:** `zip`, `flate2`, and `nix` first came into the workspace's `Cargo.lock` with
  commit `218de1a`, merged in pull request #2362 ("0670 part 1: sv report fetches a public GitHub or GitLab project
  from its address", 25 October 2026). `fuzz/Cargo.lock` was last changed by pull request #2336, on 24 October.
- **Was it there before?** No earlier nightly item names it; 0360 is the same fault from an earlier cause.
- **What to do:** regenerate `fuzz/Cargo.lock` in `fuzz/` and commit it. The test 0360 suggested, comparing the
  shared crates between the two lockfiles, would catch the next drift in `cargo test` rather than by chance.
- **The fuzzing itself:** all four targets ran their five minutes with no crash and nothing kept in
  `fuzz/artifacts/`: `mcp_line` 39,020 runs, `manifest` 685,129, `lockfiles` 1,786,285, and `tool_reports` 61,626.
  The audit, run the same way (`sv audit` on sv's own manifests and lockfile, against OSV's 3,004 Rust records
  downloaded that morning), compared all 153 packages and matched none (exit 0).

This routine records; it fixes nothing.

**Done, 20 November 2026,** in the dependency-upkeep pull request: `fuzz/Cargo.lock` regenerated from the workspace's
lock as `crates/sv-cli/tests/fuzz_lock.rs` asks (copied, then `cargo update -w` in `fuzz/`). All 18 crates named above
are listed, no shared crate differs from the workspace's version, and `cargo metadata --locked` in `fuzz/` resolves
without rewriting it. The fuzz targets were not run here, since they need nightly Rust; the weekly Fuzz workflow will.
The lockfile test suggested above already exists (backlog 0360's `fuzz_lock.rs`) and caught this drift on four
Dependabot pull requests; it does not catch a crate missing from the fuzz lock, only one at a different version.
