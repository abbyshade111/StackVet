# Rename findings.sarif to a name that does not read as problems, when no other session is working

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

The owner's decision of 13 October 2026, with backlog item on the SARIF notes: the name `findings.sarif` reads to a
person as "issues with the app" even when it holds no findings. Rename it to one that says what it is, such as
`stackvet.sarif`.

**Do this when no other session is working:** overnight, or when the owner says the others are idle. The name is in many
places other sessions may be changing:
- `crates/sv-cli/src/report_files.rs`;
- `action.yml` and its test, which upload it to GitHub;
- the README, the guide, the reading prompt, the MCP server's resource list;
- tests across `sv-cli` and `sv-report`.

A rename while others build there conflicts with each of them. A decision record, since it changes a file StackVet
writes into someone's folder and what the Action uploads. For the Action, keep reading the old name too until an image
with the new one is published (as `--no-tools` is asked of the image first).
