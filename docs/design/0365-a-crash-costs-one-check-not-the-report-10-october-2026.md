# A crash costs one check, not the report (10 October 2026)


**Decided by the owner, 10 October 2026** (observability review, part 3, F, backlog 0234): a check that crashes costs
that check, not the report.

What was built: the outside tools and the app's own checks run under a guard. A panic in one is caught, recorded where
it happened, and written into the report as a gap with the reason `crashed` (a new one-word reason for a program
reading `report.json`), saying what the check was and where the panic was. The rest of the report is written. The
panic record lives in the library (`crates/sv-cli/src/crash.rs`) so the guard and the binary's hook share one copy.

What was left: the static scan and the other stages are not guarded. A panic there still ends the run as `sv`'s own
failure, with exit 3. That is a deliberate step, not a finding: those stages feed every other, so a partial result
from them would be a worse report than none.

The app's own checks, when they crash, are reported as "not asked" with the gap beside them, because `RunStatus` has
no variant for a run that started and then failed; a new variant would be a format change, and the owner did not ask
for one.
