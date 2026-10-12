# Report timestamps say which time zone they are in

**Status:** done, 12 October 2026

Found by the feedback on the Cursor build (11 October 2026). Report files carried timestamps that did not match the local
time the person was working in, so a fresh report briefly looked stale. Nothing in the report says which time zone its
times use.

What to decide: whether report times should be written in local time with the zone shown, or in UTC with "UTC" shown in
the same place every time. Either way, the report should label the zone, so a reader can compare it with a clock.

**Claimed 12 October 2026 by session backlog-org-0255.** The choice: UTC, labeled. Report times stay in UTC, so the same
run reads the same wherever it is opened, and the time a person reads is written `2026-10-04 18:55:02 UTC` rather than
`2026-10-04T18:55:02Z`, whose `Z` only a programmer reads as UTC. The `started` field of the report file, for programs,
is unchanged. Nothing a report concludes changes.

**Done 12 October 2026:** `sv_report::utc_for_a_person` writes `2026-10-04T18:55:02Z` as `2026-10-04 18:55:02 UTC`, and
the run's description on every page uses it. The report file's `started`, and SARIF's `startTimeUtc`, are unchanged.
Tests: `crates/sv-report/src/utc_tests.rs` (two) and `crates/sv-cli/tests/dated_pages.rs`, which now looks for the time as
a person reads it. Broken on purpose: the report not using it (1 red); the shape not checked (1 red).
