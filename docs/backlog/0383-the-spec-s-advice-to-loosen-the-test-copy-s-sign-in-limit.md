# The spec's advice to loosen the test copy's sign-in limit defeats the wrong-password check

**Status:** claimed by documentation, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the second loop trial (backlog 0257, `docs/prompts/loop-2/results.md`), 13 October 2026.

`sv init`'s specification tells the AI coding tool that `sv` signs in up to 60 times in one run and to "let the copy
`sv` runs allow that many (for example, through a setting only the test copy is started with) and keep the real limit
everywhere else". Twelve of the 90 builds scored (the trial's 70 and the drafts trial's 20 controls) did so in their `start` command (a sign-in limit raised to 60, 100,
or 1000), with and without `sv` attached while building. The wrong-password check then tries `failed-sign-ins` plus two
wrong passwords against that loosened copy, and reports "Wrong passwords can be tried without limit": seven of the
nine such findings in the trial came from this, each about a limit the shipped app keeps.

What a fix has to settle: the 60 sign-ins `sv` needs are mostly right passwords, so the advice could ask for a limit
that still stops wrong passwords at the declared number while letting right ones through; or `sv` could spread its
sign-ins across its accounts; or the check could say when the start command sets a limit-like setting (0386). Each
changes what `sv` tells the AI tool, so it is a decision with a record of its own.
