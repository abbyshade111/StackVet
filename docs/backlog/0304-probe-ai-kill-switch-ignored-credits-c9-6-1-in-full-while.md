# probe.ai-kill-switch-ignored credits C9.6.1 in full while its own scope says half was not shown

**Status:** claimed by builder-full, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** C9.6.1: "Verify that a manual kill-switch mechanism exists to immediately halt AI model inference and outputs." The credit's scope ends "that the switch takes effect without a restart was not shown" (the "immediately" half), yet there is no `.in_part()` and the check is not in `RUST_IN_PART`, so C9.6.1 reads *checked*.

**Where.** `crates/sv-check/src/ai.rs:3213-3222`; `RUST_IN_PART` in `tools/coverage.py:424`.

**Evidence.** The scope text and the missing `.in_part()` at the lines above.

**How it bites.** A requirement reported checked whose own evidence sentence says it was not.

**Fix.** `.in_part()`, add it to `RUST_IN_PART`, regenerate `docs/COVERAGE.md`; the credit-log check (`coverage.py --credits`) should then hold it.

**Record.** ADR-053 governs in-part credit: an "unchanged, because" line.
