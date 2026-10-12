# windows.yml leaves the checkout's token in .git/config while the branch's build scripts and tests run

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: tools and CI. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** The token is `contents: read` on a public repository, so what it can do is small, but it is the pattern `sv` itself flags.

**Where.** `.github/workflows/windows.yml:37`: `uses: actions/checkout@3d3c42e5…` with no `persist-credentials: false`, unlike every other checkout in the six workflows (`rust.yml:51-57` says why, and that `sv check` reports exactly this, AC.12.2).

**Evidence.** The `uses:` line with no `with:` block.

**How it bites.** A build script on a branch reads the token; it can read what is already public.

**Fix.** `with: persist-credentials: false`.

**Record.** ADR-051 governs `windows.yml`: an "unchanged, because" line.
