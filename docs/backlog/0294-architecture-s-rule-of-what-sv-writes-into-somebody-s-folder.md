# ARCHITECTURE's rule of what sv writes into somebody's folder leaves out stackvet.toml and design-decisions.md, which sv review writes

**Status:** claimed by builder-full, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** "What `sv` writes into somebody's folder is the report folder, `security-notes.md`, and its section of `AGENTS.md`, and nothing else (ADR-017)". `sv review` also writes into `stackvet.toml` (the recorded entries, date, and seal: `README.md:254-255`) and seal lines into `design-decisions.md`, which ADR-017's own Later entry of 5 October records (`docs/adr/ADR-017.md:72-76`). The map contradicts both the README and the record it cites.

**Where.** `docs/ARCHITECTURE.md:131-132`.

**Evidence.** The three texts at the lines above.

**How it bites.** A reader of the map, the first page a new session opens, learns a rule the code does not follow, and a reviewer judging a write by the map calls a right one wrong.

**Fix.** Add "`stackvet.toml` and `design-decisions.md`, where `sv review` writes what the owner recorded and its seal" to the map's sentence.

**Record.** ADR-017 is unchanged; the map is corrected to it: an "unchanged, because" line.
