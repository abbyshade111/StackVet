# resolve's comment overstates its safety: an owner's no with no corroboration excludes requirements

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** The owner's `false` with corroboration `None` (`sv` could not look) sets the condition false, and `bucket` files every requirement gated on it as not applicable, with `source: claim` and `ClaimState::Unverifiable` shown. That is the designed behavior (the owner's word, labeled), but the comment and the "safety argument" paragraph are not true of it.

**Where.** `crates/sv-manifest/src/lib.rs:1955-1960` ("Both directions add requirements; neither removes one ... never a requirement they needed and were told they did not") against `:1981` `(Some(false), _) => Some(false)`; `bucket` at `crates/sv-frameworks/src/applicability.rs:254-269`.

**Evidence.** The match arm against the comment.

**How it bites.** A reader of the code trusts a safety argument the code does not make.

**Fix.** No code change; fix the comment, or say in the report line that the exclusion rests on an unverifiable claim.

**Record.** No record governs `sv-manifest/src/lib.rs` by name; say so.
