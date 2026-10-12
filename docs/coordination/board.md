# The coordination board

Written only by the orchestrator, with `python3 tools/coordination.py`; the role is `docs/COORDINATION.md` (backlog
0273; ADR-061, Later). A building session reads this file and takes the item named beside its name, and nothing else:
a claim is still the record (`tools/backlog.py`), and this board only says who takes what, in which order, and why.

## Sessions

- builder-full — arm: full — holds: nothing
- builder-none — arm: none — holds: nothing

## Queue

1. 0263 Error-handler signals in the app's code, the static half of V16.5.4 — ready

## Log

- 2026-10-12 13:41 — opened by stackvet-orchestrator
- 2026-10-12 13:41 — session builder-full added, arm full
- 2026-10-12 13:41 — session builder-none added, arm none
- 2026-10-12 13:41 — queued 0263
