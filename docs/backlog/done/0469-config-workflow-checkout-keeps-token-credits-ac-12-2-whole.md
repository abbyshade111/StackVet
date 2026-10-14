# `config.workflow-checkout-keeps-token` credits AC.12.2 whole for one of its two clauses

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: requirements and tests (citations). Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/workflows.rs:726-733` (`Verified::new(CHECKOUT_TOKEN, &["AC.12.2"], "…every checkout sets persist-credentials: false; credentials kept on the runner some other way are not in any file")`, no `.in_part()`).
- What is wrong: AC.12.2: "Verify that secrets, credentials, and pipeline job tokens are not persisted into
  workspaces that process AI-touched or fork-originated untrusted code. For example, set `persist-credentials:
  false` on checkout …, and scrub CI runners of cached credentials before AI tooling runs." The checkout half is
  read; the scrub half is not, and the scope says so, yet the credit is whole and the requirement reads
  *checked*.
- What to do: `.in_part()`, and add the check to the in-part set (or let F1's gate do it).
- How verified: read.
- Overlap: none found.
- Needs a decision record?: ADR-053: an "unchanged, because" line.
