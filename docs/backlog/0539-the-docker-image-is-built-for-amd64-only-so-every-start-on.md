# The Docker image is built for amd64 only, so every start on an Apple Silicon Mac warns and runs under emulation

**Status:** claimed by honesty, 14 October 2026

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

**The owner's choice, 14 October 2026:** yes: build the image for linux/amd64 and linux/arm64 with buildx and push one manifest, attesting both; until it lands, the connect output and prompt 3 say the platform warning is expected. ADR-108 (or ADR-080) Later in the building pull request. The workflow file is routed to the owner's review by CODEOWNERS.

**Built, 15 October 2026 (honesty).** Not with one `docker buildx build --platform` under QEMU, which would build Rust
under emulation for hours, but in the same shape: a native job on GitHub's arm64 runner builds and drives the arm64
image, and `publish` pushes both and joins them into one manifest, signing it and each platform's image (ADR-080,
Later). The interim note on the warning is not added: this is the fix it was waiting for. Left open for the board to
close; what the published image holds is seen only after the first push to `main`.
