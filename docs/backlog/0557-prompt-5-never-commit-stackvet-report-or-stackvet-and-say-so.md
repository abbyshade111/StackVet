# Prompt 5: never commit stackvet-report/ or .stackvet/, and say so

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): Antigravity committed `stackvet-report/` three
times and added `.stackvet/` to `.gitignore` on its own. For the prompt library (0370).

**What to change.** Prompt 5: "Add stackvet-report/ and .stackvet/ to .gitignore before the first commit; the report
is for me to read, not for the repository." (The report quotes the app's code and the run's answers.)

**The owner's answer, 15 October 2026 (asked through the board's waiting line):** A. Prompt 5 has the tool ignore only stackvet-report/ in git; .stackvet/ stays in git so a fresh checkout or CI still has the build record (ADR-076). The item's title is narrowed to the report folder.
