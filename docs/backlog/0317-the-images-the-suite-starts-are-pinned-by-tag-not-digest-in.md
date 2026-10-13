# The images the suite starts are pinned by tag, not digest, in the workflow

**Status:** claimed by builder-full, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: tools and CI. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** A tag can be moved; the Dockerfile's own bases are pinned by digest (`Dockerfile:25`, `:36`) and the file's comment says why. These are the images `sv-run` starts, so the fix belongs with ADR-052's images.

**Where.** `.github/workflows/rust.yml:97-98`: `busybox:1.36 axllent/mailpit:v1.31 node:22-alpine python:3.12-slim chromedp/headless-shell:151.0.7922.109`.

**Evidence.** The tag-only names at the lines above.

**How it bites.** A moved tag changes what the suite, and `sv run`, start, with no record of it.

**Fix.** Pin by `@sha256:` in one place the workflow and the code both read, and let Dependabot move them.

**Record.** ADR-052 governs the images `sv run` starts: a Later line if the code's references change; ADR-051 for `rust.yml`.
