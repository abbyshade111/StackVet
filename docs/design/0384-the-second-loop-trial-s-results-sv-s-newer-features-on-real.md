# The second loop trial's results: sv's newer features on real apps (13 October 2026)


The second loop trial ran 70 builds on two briefs for $27.43 (`docs/prompts/loop-2/results.md`, ADR-109). It asked
whether `sv`'s newer features work on apps an AI coding tool really built, and mostly they do: the account changes
answered in 124 of 160 tries, the `not-offered` list made V7.5.1 whole once confirmed, the build-loop record matched
every transcript, the AI-commits note counted every build's commits, and `sv compare` and `sv`'s own log did what
their records say.

**What only real apps could show.** Three faults the scripted app never exercised. `sv`'s specification advises
loosening the test copy's sign-in limit, and twelve builds did, so the wrong-password check found "no limit" in apps
whose shipped limit was there (0383, 0386). The same check, given no sign-up, tries a made-up name, which an app that
locks real accounts never locks (0384). And every request template reads its form token from the declared address,
so where the form lives on another page (the account page, or the emailed reset link) the app refuses a token-less
request and `sv` blames the app (0385): sixteen account changes and, in at least one build read by hand, the reset.

**The outside check found no sign of builders satisfying `sv` instead of making apps safer**: Bandit and Semgrep,
which the builders never saw, found about as much with `sv` attached as without.

**Reading by hand changed three conclusions** the counts alone pointed to: the cause of the changes that did not take
(the form token, not a lockout), the build-loop record's disagreement (a measure that started too early), and the nine
wrong-password findings (three causes, one of them real).
