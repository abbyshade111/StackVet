# The reading of answers past the sign-in and code limits is one algorithm written twice, and the copies have drifted

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: medium.
Verified against the code by the reviewing session before filing.

**What is wrong.** `brute_force_check` (V6.3.1, `signed_in/signin.rs:339-470`) and the emailed-code guessing check
(V6.6.3, `signed_in/codes.rs:334-425`) carry, line for line: `let attempts = allowed + 2;`, the
`matches!(first_status, 0 | 423 | 429)` early return, `answers[allowed as usize]`, the `real` closure, the `alike`
test, the "one more let through" finding, `times.split_at` and `slowing`, and the `Unclear`/`Slowed`/`how` wording:
about ninety lines of judgment, twice. `git log` shows the cost: the 6 October fix ("a crash is never read as a
status that changed"), the "one more let through" reading, and 0280 were each applied to both files by hand. The
copies already differ: `signin.rs:429` counts `last.0 >= 400 && first_status < 400` as refused, `codes.rs:386` only
`423 | 429`; `codes.rs:421` makes the `Unclear` outcome depend on `right_still_works`, `signin.rs:463` does not; and
the cap guard differs in a way that matters (the next item, filed on its own so it is not lost if this one waits).

**Fix.** One function in `signed_in/` (say `past_the_limit(answers, allowed, pages) -> Reading`) returning an enum
(`AlreadyRefusing`, `NotReal { last, next }`, `OneMoreLetThrough`, `Unclear(why)`, `Pushed(How)`, `NotPushed`), with
each caller composing its own requirement id, rule, and sentences; one table-driven test that feeds the same answer
lists through both callers and holds their readings equal. Must not lose: the two findings' different wording; the
`forwarded_check` only `signin.rs` runs on a refusal; the fake-app tests of 0280, which must stay red when the
shared function is broken.

**Record.** None for the refactor. If the two definitions of "refused" are made one, that changes what counts as
pushing back for one of the checks: a Later line on the record governing the signed-in suites (`python3
tools/adr_check.py` names it).
