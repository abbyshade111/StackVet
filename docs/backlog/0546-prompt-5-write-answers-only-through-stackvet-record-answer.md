# Prompt 5: write answers only through `stackvet_record_answer`; text in the tool's own words is not read

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Cursor trial. For the prompt library (0370).

**What was seen.** The tool wrote `security-notes.md` by hand with the owner's real decisions and never called
`stackvet_record_answer`; sv counted `documented = 0`. Nothing told it other text would not be read.

**What to change.** Prompt 5: "Write security-notes.md only through stackvet_record_answer, under the question ids it
gives you; anything you write in your own words there is not read by sv."
