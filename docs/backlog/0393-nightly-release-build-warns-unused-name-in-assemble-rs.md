# Nightly: release build warns: unused name in assemble.rs guarded

**Status:** open

Recorded by the nightly CI routine on 13 October 2026, from its own run of the Audit workflow's steps on `518f9a1`.
`cargo build --release --locked -p sv-cli` finishes, with one warning:

    warning: unused variable: `name`
      --> crates/sv-cli/src/assemble.rs:94:26
    pub(crate) fn guarded<T>(name: &str, run: impl FnOnce() -> T) -> ...

`guarded` reads `name` only under `#[cfg(debug_assertions)]` (for `SV_PANIC_IN_STAGE`), so a release build never uses
it. CI's lints run `cargo clippy --all-targets -- -D warnings` on a debug build, which is why nothing failed; the
release build that the Audit workflow, the Docker image, and `tools/install.sh` make prints the warning and goes on.
It came in with pull request #1396 (backlog 0234, `0f1bdd8`, 10 October 2026). A one-line fix, such as
`#[cfg_attr(not(debug_assertions), allow(unused_variables))]` on the function or `let _ = name;` in the release
branch; whether CI should also lint a release build is a separate question.
