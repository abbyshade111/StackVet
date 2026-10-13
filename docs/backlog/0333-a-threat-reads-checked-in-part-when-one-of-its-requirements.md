# A threat reads checked in part when one of its requirements is checked and the rest are not verified

**Status:** claimed by builder-none, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** With five requirements under a threat, one *checked* and four *not verified*, the threat reads "checked in part"; the introduction says the best a threat can be is checked in part, which is honest, but the label does not distinguish "all five checked in part" from "one in five looked at".

**Where.** `crates/sv-report/src/threats.rs:420-428`; the introduction at `:28`.

**Evidence.** The label logic at the lines above.

**How it bites.** A reader takes "checked in part" for more than one requirement looked at.

**Fix.** Wording: "n of m requirements checked".

**Record.** No record governs `threats.rs` by name; say so.
