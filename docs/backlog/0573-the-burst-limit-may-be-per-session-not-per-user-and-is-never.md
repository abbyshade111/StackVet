# The burst limit may be per session, not per user, and is never tried in a fresh session

**Status:** open

From the inventory of sweep 3 (backlog 0561, ADR-119); more than two requests, so filed rather than built there.

**What is wrong.** `burst.rs` `burst_one` (V2.4.1) sends its burst in one session. A limit kept per session, not per
user, is credited, though a fresh sign-in starts the count again.

**What to build.** After the limit, sign B in again in a fresh session and send one more create (about three
requests); taken, the limit is per session. Mind the sign-in budget.
