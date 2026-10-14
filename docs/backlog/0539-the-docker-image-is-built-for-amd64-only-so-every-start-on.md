# The Docker image is built for amd64 only, so every start on an Apple Silicon Mac warns and runs under emulation

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): the Cursor and VS Code trials (Docker's platform
warning in Cursor's MCP log and the VS Code transcript), the Antigravity trial (step 121). Severity: medium (every
Mac user sees a warning the prompts do not explain, and the server runs slower than it need). A decision: what the
release workflow builds.

**What is wrong.** `.github/workflows/rust.yml:429` runs a plain `docker build` on `ubuntu-latest` with no
`--platform`, so `ghcr.io/abbyshade111/stackvet-sv` is linux/amd64 only. Docker Desktop and Colima on Apple Silicon
print "The requested image's platform (linux/amd64) does not match the detected host platform" on every start and run
it through Rosetta or QEMU.

**Fix.** Build with `docker buildx build --platform linux/amd64,linux/arm64` and push one multi-arch manifest (the
Rust build already cross-compiles for the release zip); attest both. Until then, `sv connect --docker` and prompt 3
say the warning is expected and harmless.

**Record.** ADR-108 (the release and the image) or ADR-080: a Later entry.
