# A --tools run is not network-free: Semgrep downloads its packs from semgrep.dev at run time, and no document beside the no-network rule says so

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: the fence. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** It is the tool's connection, not `sv`'s, and the rule is about `sv`; but the report's reader is not told that `sv report --tools` reaches a third party.

**Where.** `data/adapters.json` runs Semgrep with `--config p/security-audit --config p/default`, fetched from its registry on each run; `tools/semgrep_packs.py:15` says so; `docs/SEMGREP-FALSE-ALARMS.md:51` says the opposite for a different run.

**Evidence.** The `--config p/…` arguments.

**How it bites.** An owner who chose `sv` for opening no connection runs `--tools` and Semgrep phones home.

**Fix.** One sentence in ADR-018 or the README beside the no-network rule, and in the report's tools section when Semgrep ran; or pin the packs locally with `semgrep_packs.py` and point the adapter at the file.

**Record.** ADR-018 and ADR-032 govern the adapters: a Later line on whichever carries the sentence.
