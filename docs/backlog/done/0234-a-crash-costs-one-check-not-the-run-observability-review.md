# A crash costs one check, not the run (observability review, part 3, F)

**Status:** done, 11 October 2026

The owner's word, 10 October 2026, asked with the three other open decisions of that day: yes (part 3, F, of backlog 0226).

A check that crashes costs that check, not the run: each stage of `sv report` run so that a panic in one is caught,
recorded as "not assessed: the check crashed", with the place, and the rest of the report written. A new reason for
not assessed, so its record is written with the build. Medium.

**Built 10 October 2026:** the report's two checks that run outside the scan, the outside tools and the app's own checks, are each guarded: a panic in one costs that check, which is recorded as a gap saying it crashed and where, and the rest of the report is written (`crates/sv-cli/src/assemble.rs`, `guarded`). The panic record moved into the library, `crates/sv-cli/src/crash.rs`, so the guard and the binary's hook share one copy. Test: `crates/sv-cli/tests/stage_crash.rs`. The static scan and the other stages are not yet guarded: each is still a whole-run fault.

**Built 11 October 2026, the rest:** the list of the app's files is guarded (`StaticScan::read`, `crates/sv-cli/src/static_scan.rs`): if it crashes, no reader runs on an empty list, and each reader's gap and `examined` entry say so. The known-vulnerability check, stage 6 of the report, is guarded (`crates/sv-cli/src/assemble.rs`). The last stage, putting the report together, is left as a whole-run fault on purpose: it is the report itself, and a report that says nothing would be worse than none. Tests: `a_crashed_file_list_stops_every_reader_and_the_report_says_so` and `a_crashed_known_vulnerability_check_costs_itself_and_the_report_says_so` in `crates/sv-cli/tests/stage_crash.rs`. Record: ADR-023, Later, 11 October 2026 (the second entry that day).
