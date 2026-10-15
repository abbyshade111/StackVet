# The once check's other user may be refused for want of permission, not for losing the race

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the inventory of sweep 3 (backlog 0561, ADR-119); more than two requests, so filed rather than built there.

**What is wrong.** `once.rs` `once_check` (V2.3.4) reads the other user's copies, refused, as the race lost. They may
be refused because the other user may not do the action at all.

**What to build.** A solo try by the other user, after a reset, before the race (more than two requests); refused
alone, the race says nothing.
