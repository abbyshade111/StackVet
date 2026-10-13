# The Dockerfile has no dependency layer, and copies examples/ into the build stage

**Status:** claimed by builder-none, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: build time. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** In CI this costs nothing extra, since every runner is fresh and no `--cache-from` is used; locally it costs the whole dependency build (estimate 2 to 5 minutes) on every `docker build` after a one-line change.

**Where.** `Dockerfile:27-34`: `COPY crates` lands before `cargo build`, so any source change invalidates the layer that compiles all 84 external crates and 178 MB of C; `Dockerfile:30` copies `examples/` into the build stage though nothing under `crates/*/src` includes it at compile time.

**Evidence.** The `COPY` order and the `include_str!` sweep (only `data/` and `tests/fixtures/` are compiled in).

**How it bites.** Anyone building the image locally waits for the dependencies every time.

**Fix.** If local image builds matter: a first stage copying only `Cargo.toml`, `Cargo.lock`, and stub `crates/*/Cargo.toml` with empty sources, `cargo build --release --locked -p sv-cli` to compile dependencies, then the real sources and a second build (the hand-rolled form of `cargo chef`, with no new tool); copy `examples/` straight into the runtime stage. Keep `--locked`, the digests, and the `SV_GIT_COMMIT` build argument.

**Record.** ADR-036 and ADR-080 govern the image: "unchanged, because" lines (the same binary from the same sources).
