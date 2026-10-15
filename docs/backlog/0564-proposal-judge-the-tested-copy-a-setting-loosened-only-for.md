# Proposal: judge the tested copy: a setting loosened only for sv's run earns no credit for the requirement it serves

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the loop-test session's observability ideas, 14 October 2026, assessed by the orchestrating session and agreed by
the owner the same day: first of three. A proposal, held until the owner says go; after the probe sweeps 0559 to 0561,
since it has the same shape (read the setup before believing the answer). Severity: medium.

**What was seen.** The second loop trial's builds raised the sign-in limit in their start command for sv's run, and the
owner's answer on 0383 (A and C) now has sv say when the copy it ran was loosened. The same happens with other
settings: `Secure` cookies off "for the test", debug on, a rate limit raised, a password rule relaxed through an
environment variable in the start command or a settings file the manifest names. Today the report lists such a
setting but does not judge it.

**What to build.** Read the start command, the environment the manifest sets, and the settings files it names for a
value that loosens a requirement sv then credits (cookie flags, rate and lockout limits, debug and verbose modes,
password rules, CSRF off, TLS verification off). When one is found, the requirement that setting serves gets no credit
from the run, and the report says "the copy sv ran had X set to Y, so what the run saw is not what the app does", with
the line. Keep the listing as it is for settings sv does not credit.

**Record.** A Later entry on the record that governs what the run credits (ADR-096 or the probes' record), since what
counts as evidence changes.

**The owner's go, 14 October 2026:** build it whenever the board has room; in the order 0564, 0565, 0566, after the probe sweeps.
