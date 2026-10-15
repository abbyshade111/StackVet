# The once check's other user may be refused for want of permission, not for losing the race

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the inventory of sweep 3 (backlog 0561, ADR-119); more than two requests, so filed rather than built there.

**What is wrong.** `once.rs` `once_check` (V2.3.4) reads the other user's copies, refused, as the race lost. They may
be refused because the other user may not do the action at all.

**What to build.** A solo try by the other user, after a reset, before the race (more than two requests); refused
alone, the race says nothing.

**The owner's answer, 15 October 2026 (asked through the board's waiting line):** (a). Add an optional reset request under `once` (for example, cancel the booking), sent as the holder: with it, the other user tries alone first, the reset puts the thing back, then the race runs; without it, the credit stays in part and says the other user was not shown able to take it. A Later entry on the record that governs the once check and the manifest key.

**Waiting on the owner, 15 October 2026** (documentation). `once` has no request that puts the one thing back, so a
solo try by the other user before the race would take it and leave the race nothing to take: "after a reset" cannot be
built from stackvet.toml as it stands. The question, with three ways to answer it, is on the board
(`docs/coordination/waiting/0574.md`).

**Done, 15 October 2026** (documentation), as the owner answered (a). `once` takes an optional `reset`
(`sv-manifest`, `OnceAction::reset`; the spec says what it is for). With it, before the race, each user sends the
action alone and then `reset` (`once.rs`, `each_alone`): refused alone, or the reset not accepted, the race is not
judged and says why; both taken and given back, the credit says either could have taken it. Without it, the credit,
in part as before, says neither user was shown able to take it alone. Four requests more, no sign-ins. ADR-021, Later.
Tests: `once/alone_tests.rs` (4); with the solo tries switched off, the three that need them fail.
