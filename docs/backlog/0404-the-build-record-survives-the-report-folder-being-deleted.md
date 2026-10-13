# The build record survives the report folder being deleted, and the report says when it may have been

**Status:** open

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
