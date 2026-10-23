# sv review says a report folder "holds no marker of sv's" when no report has been written yet

**Status:** claimed by claude/0636, 22 October 2026

Found in the owner's Cursor build of 22 October 2026 (backlog 0373). At 6:40 PM the owner ran `sv review` before any
`sv report` had written a report: the app's `stackvet-report` folder was created at 6:43:35 PM, by the full report.
The review ended: "The checklist before going live was not offered: no report sv can show it wrote was read in
./stackvet-report (the folder holds no marker of sv's)." That reads as if a folder were there and suspect, when there
was no report at all.

Where: `crates/sv-cli/src/review/ticks.rs` (`tick_checklist`) passes on the reason from
`crates/sv-cli/src/report_seal.rs` (`holds_only_svs`, `check_keeping_report`), which says "the folder holds no marker
of sv's" whether the folder is missing, empty, or holds another's files.

To do: when the folder does not exist, say so plainly: "No report has been written for this app yet, so there is no
checklist before going live to tick. Write the report with `sv report`, then run `sv review` again." Keep the present
words for a folder that is there without sv's marker. A test for each. Check what else passes on
`holds_only_svs`'s reason (`sv explain --report`, `sv compare`, the MCP server's report resources) for the same.

**Claimed, 22 October 2026,** at the owner's word ("Please continue with 0638 and the other items filed today and outside
the roadmap"), with 0638, each to be built in a pull request of its own.
