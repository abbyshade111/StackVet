# The spec's advice to loosen the test copy's sign-in limit defeats the wrong-password check

**Status:** done, 14 October 2026

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

**Waiting on the owner, 14 October 2026 (session documentation).** The advice is in `crates/sv-manifest/src/spec.rs`
(the `[stack.run.users]` notes: "let the copy `sv` runs allow that many ... and keep the real limit everywhere else").
The question, with three ways to settle it and a suggestion (A and C together), is in
`docs/coordination/waiting/0383.md`. Nothing is built until the owner answers; the decision record is written then.

**The owner's answer, 14 October 2026** (asked through the board's waiting line while the owner was away; answered as the session recommended): A and C together. A: the settings advice no longer asks the test copy to allow sixty sign-ins; it asks that the copy keep stopping wrong passwords at the number declared in `failed-sign-ins`, and raise only a limit that counts every sign-in, if the app has one. C: the wrong-password check says when the start command sets a limit-like value (backlog 0386), so the finding says it may be about a loosened copy. Both change what `sv` tells the AI tool: a decision record of its own, proposed in this item's build.
