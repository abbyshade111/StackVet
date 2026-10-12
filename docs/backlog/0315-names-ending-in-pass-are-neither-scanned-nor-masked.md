# Names ending in _PASS are neither scanned nor masked

**Status:** claimed by builder-full, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: secrets. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** The lists hold `password`, `passwd`, `pwd` but not `pass` as a word, so `DB_PASS=…`, `SMTP_PASS: "…"`, `REDIS_PASS` are missed by the assignment rule and left in full by `redact_text` (a `.env`-style line in a log tail, a tool's quoted line).

**Where.** `crates/sv-check/src/secrets.rs:321-350` (`is_secret_name`) and `:356-371` (`is_masked_name`).

**Evidence.** The name lists at the lines above.

**How it bites.** A `.env` with `DB_PASS=` is not found by the scan, and the value shows in a log tail.

**Fix.** Match `pass` as a whole token of the name (split on `_`, `-`, and case), so `bypass` and `passport` stay out; tests for `DB_PASS` found and `BYPASS_URL` not.

**Record.** No record governs `secrets.rs` by name; `data/secret-rules.json` is data; say so.
