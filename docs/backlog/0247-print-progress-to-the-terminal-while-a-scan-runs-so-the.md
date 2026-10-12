# Print progress to the terminal while a scan runs, so the person can see what is happening

**Status:** claimed by securevibe-trial-prep, 12 October 2026

Asked for by the owner on 11 October 2026: while `sv` runs, the terminal is quiet until the report is written, so
the person cannot tell whether it is working, stuck, or waiting on something. Look into printing progress as each
stage starts and finishes (the checks, the app's start, the probes, the report), in plain words, and say how long
the slow stages take. Look first at what the stages already name (`STAGES`, which the MCP server reports) and what
the terminal already prints, so the progress is added to them rather than in a second place. Settle, with the
owner, whether the progress goes to the terminal only or also to the report, and whether it is quiet with `--quiet`.
