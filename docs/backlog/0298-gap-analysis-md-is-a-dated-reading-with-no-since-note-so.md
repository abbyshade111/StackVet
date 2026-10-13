# GAP-ANALYSIS.md is a dated reading with no since note, so twelve headline faults read as open when they are built

**Status:** claimed by builder-full, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** THREAT-MODELING has a dated "Since" section for exactly this; GAP-ANALYSIS does not. Built since, each verified in the code: 1.1 (the app's tests are a tier: `Status::AppTested`, ADR-050); 1.3 (git history read: `git.rs:72-77`); 1.7 (`Status::CheckedInPart`, ADR-053); 3.1 (the install step, ADR-052); 3.6 (templates and notebooks, ADR-054); 4.1 (`sv review` confirms the level's two answers); 5.1 (`sv check` reads `stackvet.toml` when present); 5.2 (the OSV download table in GETTING-STARTED); 5.3 ("Keeping StackVet up to date"); 6.2 (the level line, `bluf.rs:204-214`); 7.2 (`SV_REQUIRE_BACKEND=1`); 7.5 (one file per item, ADR-061).

**Where.** `docs/GAP-ANALYSIS.md` (7 October, commit `1fe884b`), its short version at `:24-43` and "Where to start" at `:354-374`.

**Evidence.** The code locations above.

**How it bites.** A reader of the gap analysis's short version is told things the code no longer does.

**Fix.** A short dated "Since" section at the end listing which numbered findings were built and where, as THREAT-MODELING does; the body stays as written.

**Record.** No record governs `GAP-ANALYSIS.md`; say so.
