# The burst limit may be per session, not per user, and is never tried in a fresh session

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the inventory of sweep 3 (backlog 0561, ADR-119); more than two requests, so filed rather than built there.

**What is wrong.** `burst.rs` `burst_one` (V2.4.1) sends its burst in one session. A limit kept per session, not per
user, is credited, though a fresh sign-in starts the count again.

**What to build.** After the limit, sign B in again in a fresh session and send one more create (about three
requests); taken, the limit is per session. Mind the sign-in budget.

**Done, 15 October 2026** (documentation). Where the burst would credit a limit, the second user now signs in again
in a session of its own and sends one more create inside the same minute (`burst.rs`, `once_more`). Refused as a limit
refuses, the credit stands and says so; taken, the limit counts sessions, not the user, and nothing is credited; not
signed in, or answered otherwise, nothing is credited either. One sign-in, spent only where a credit is at stake: the
busiest scripted run stays within the 62 the spec states. The new session takes the old one's place. ADR-119, Later.
Tests: `burst/fresh_session_tests.rs` (5); with the fresh try switched off, the three that guard it fail.
