# The once check's other user may be refused for want of permission, not for losing the race

**Status:** claimed by documentation, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the inventory of sweep 3 (backlog 0561, ADR-119); more than two requests, so filed rather than built there.

**What is wrong.** `once.rs` `once_check` (V2.3.4) reads the other user's copies, refused, as the race lost. They may
be refused because the other user may not do the action at all.

**What to build.** A solo try by the other user, after a reset, before the race (more than two requests); refused
alone, the race says nothing.

**Waiting on the owner, 15 October 2026** (documentation). `once` has no request that puts the one thing back, so a
solo try by the other user before the race would take it and leave the race nothing to take: "after a reset" cannot be
built from stackvet.toml as it stands. The question, with three ways to answer it, is on the board
(`docs/coordination/waiting/0574.md`).
