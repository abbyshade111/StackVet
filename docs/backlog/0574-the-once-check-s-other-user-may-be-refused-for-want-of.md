# The once check's other user may be refused for want of permission, not for losing the race

**Status:** claimed by documentation, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the inventory of sweep 3 (backlog 0561, ADR-119); more than two requests, so filed rather than built there.

**What is wrong.** `once.rs` `once_check` (V2.3.4) reads the other user's copies, refused, as the race lost. They may
be refused because the other user may not do the action at all.

**What to build.** A solo try by the other user, after a reset, before the race (more than two requests); refused
alone, the race says nothing.

**The owner's answer, 15 October 2026 (asked through the board's waiting line):** (a). Add an optional reset request under `once` (for example, cancel the booking), sent as the holder: with it, the other user tries alone first, the reset puts the thing back, then the race runs; without it, the credit stays in part and says the other user was not shown able to take it. A Later entry on the record that governs the once check and the manifest key.
