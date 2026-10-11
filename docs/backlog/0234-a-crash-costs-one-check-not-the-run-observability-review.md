# A crash costs one check, not the run (observability review, part 3, F)

**Status:** partly done: the static scan and the other stages are not guarded; only the outside tools and the app's own checks are

The owner's word, 10 October 2026, asked with the three other open decisions of that day: yes (part 3, F, of backlog 0226).

A check that crashes costs that check, not the run: each stage of `sv report` run so that a panic in one is caught,
recorded as "not assessed: the check crashed", with the place, and the rest of the report written. A new reason for
not assessed, so its record is written with the build. Medium.

**Built 10 October 2026:** the report's two checks that run outside the scan, the outside tools and the app's own checks, are each guarded: a panic in one costs that check, which is recorded as a gap saying it crashed and where, and the rest of the report is written (`crates/sv-cli/src/assemble.rs`, `guarded`). The panic record moved into the library, `crates/sv-cli/src/crash.rs`, so the guard and the binary's hook share one copy. Test: `crates/sv-cli/tests/stage_crash.rs`. The static scan and the other stages are not yet guarded: each is still a whole-run fault.
