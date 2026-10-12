# The no-cache comment in rust.yml is duplicated and gives two different build times

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: build time. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** The reason given for no cache is "pinning another third-party action"; since ADR-106 the workflow already pins GitHub's own `upload-artifact` and `download-artifact`, and `actions/cache` is the same publisher, so the reason still holds only as "one fewer dependency in the job that holds the Docker Hub token".

**Where.** `.github/workflows/rust.yml:148-152`: the same two-line comment appears twice, once saying the workspace builds from scratch "in well under a minute", once "in about two minutes"; the measured figure is 126 s (0270).

**Evidence.** The two comments.

**How it bites.** A reader weighing a cache is given a stale reason and a wrong number. For the owner: a registry and target cache would save an estimated 1 to 2 minutes of wall-clock per pull request (four jobs build the dependencies in parallel, so the wall-clock saving is one build's worth, the runner-minute saving is four); `--locked` reproducibility would be unchanged. The owner's trade-off, not a recommendation.

**Fix.** Keep one copy of the comment with the measured number and the reason as it now stands.

**Record.** ADR-106 governs: an "unchanged, because" line; a cache would be a decision and a Later entry.
