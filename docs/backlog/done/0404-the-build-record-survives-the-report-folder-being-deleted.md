# The build record survives the report folder being deleted, and the report says when it may have been

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found 13 October 2026 in the owner's trial in Antigravity. Before `sv report --run`, the AI tool deleted the
`stackvet-report` folder ("Clear report directory"). The folder held `build-loop.jsonl`, StackVet's record of the
tool's calls to its MCP server during the build, which was there at 14:20. The full report at 18:39 then said "Nothing
shows that sv was used while this app was built: no call from an AI coding tool to sv's MCP server is recorded for
it." The tool had called the `stackvet_` tools dozens of times. A person would read that as "StackVet was not used".

To build: keep the build-loop record where deleting or replacing the report folder does not take it with it. Decide
where, as a decision record, since it is a file StackVet writes into the app's folder or elsewhere. And when there is
no record but other signs that StackVet was used, the report says "no record of the build was found; it may have been
deleted" rather than implying none was kept. Other signs could be answers in `stackvet.toml` that only StackVet's tools
write, `security-notes.md` sections in its form, or an earlier report's seal.

**The owner's answer, 14 October 2026** (asked through the board's waiting line while the owner was away; answered as the session recommended): (1): the record of the AI tool's calls lives in a hidden folder of its own in the app, `.stackvet/build-loop.jsonl`, so deleting or replacing `stackvet-report` leaves it, it still travels with the app folder, and the report's seal still covers it. One more name `sv` writes into somebody's folder: Later entries on ADR-017 and ADR-076, and the report says "no record of the build was found; it may have been deleted" when other signs show StackVet was used.

**Done, 14 October 2026:** the record is kept in `.stackvet/build-loop.jsonl` in the app (`sv_scan::ecosystems::build_loop_record_in`),
left out of every walk of the app. A record in the old place in the report folder is read there and moved whole when the
next line is written. With no record, the report names the signs that `sv` was used (answers marked `by = "ai-tool"`,
`security-notes.md` sections with a `Written by:` line, an earlier sealed report) and says the record may have been
deleted. The records are Later entries on ADR-017 and ADR-076.
