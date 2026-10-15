# Prompt 6: "open report.html in my web browser" means the computer's browser, or says both are fine

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): VS Code opened it in its own browser pane;
Antigravity and Cursor used `open`. For the prompt library (0370).

**What to change.** "Open it in the computer's own web browser, not a browser pane inside this tool" (or accept both
and say so). A small thing; it decides whether the person sees the report where they can keep it.

**Done, 15 October 2026:** prompt 6's step 6 now says "open `report.html` in this computer's own web browser for me
(`open` on a Mac, `start` in PowerShell, `xdg-open` on Linux): not in a browser pane inside this tool, which closes with
it, and not in this chat, where it shows as code." The item's first choice, the computer's own browser, was taken over
accepting both, since a pane inside the tool is the case it names as the one where the person cannot keep the report.
`crates/sv-cli/tests/build_prompt_rules.rs`, whose assertion held the old words, now holds the new ones; with the page
as it was, it went red.
