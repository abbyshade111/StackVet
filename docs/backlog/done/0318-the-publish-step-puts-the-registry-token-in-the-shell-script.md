# The publish step puts the registry token in the shell script text instead of the environment

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: tools and CI. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** GitHub masks it in logs, but every other secret in these workflows is passed through `env:` (the Docker Hub steps do it right).

**Where.** `.github/workflows/rust.yml:384`: `echo "${{ secrets.GITHUB_TOKEN }}" | docker login ghcr.io -u "${{ github.actor }}" --password-stdin`.

**Evidence.** The line above.

**How it bites.** The token is part of the script text; a shell error that echoes the line would show it before masking catches up.

**Fix.** `env: TOKEN: ${{ secrets.GITHUB_TOKEN }}` and `printf '%s' "$TOKEN" | docker login …`.

**Record.** ADR-051 governs `rust.yml`: an "unchanged, because" line.
