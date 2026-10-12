# config.secrets-file-committed is a full credit where its sibling check is in part

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** V13.3.1: "...Secrets must not be included in application source code or included in build artifacts..." The `.gitignore` check is marked in part "because V13.3.1 speaks of every secret in the source"; the "no file named like a secrets file is tracked" check is one sample of the same sentence and is not. V13.3.1 is `manualOnly`, so neither becomes *checked*; the asymmetry is still wrong.

**Where.** `crates/sv-check/src/config.rs:237` and `:50-57`.

**Evidence.** The comment at `:52-53` against the credit at `:237`.

**How it bites.** Two checks of the same sentence rated differently.

**Fix.** In-part both.

**Record.** ADR-053: an "unchanged, because" line.
