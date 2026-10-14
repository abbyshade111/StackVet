# CLAUDE.md says only pwned_passwords.py needs the network, and cvss4_tables.py does not hold its checkout to the pinned commit

**Status:** claimed by documentation, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: tools and CI. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** The guide's sentence is incomplete, and a wrong CVSS checkout silently produces different tables with a different commit written in the header.

**Where.** `CLAUDE.md` (the Commands section names `tools/pwned_passwords.py` alone); `tools/atlas_references.py:47-52` fetches a pinned ATLAS release by itself (its header says so; `data/README.md:33` says what it writes); `tools/cvss4_tables.py` runs JavaScript from a checkout the owner clones and does not check that checkout's commit against the one ADR-033 names.

**Evidence.** The lines above.

**How it bites.** A session runs `atlas_references.py` inside the sandbox and gets a cut-short download; a wrong CVSS clone produces wrong tables unnoticed.

**Fix.** Name `atlas_references.py` beside `pwned_passwords.py` in `CLAUDE.md`; have `cvss4_tables.py` refuse a checkout whose `git rev-parse HEAD` is not the pinned commit unless `--commit` says otherwise.

**Record.** ADR-033 governs the CVSS tables: a Later line or an "unchanged, because" line.
