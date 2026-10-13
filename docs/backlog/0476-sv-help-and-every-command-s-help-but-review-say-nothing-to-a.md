# `sv --help` and every command's `--help` but `review` say nothing to a person who is not a programmer

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: usability. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-cli/src/main.rs:46-61` (`more_help`: only `"review"` has a paragraph, which ends by naming
  the guide); 470-484 (`print_help` ends with EXIT STATUS); 222-402 (`COMMANDS`, whose lines name `--fail-on`,
  SARIF, CycloneDX, OSV).
- What is wrong: a person who installed `sv` for the full report (guide section 6) and types `sv` sees a
  programmer's reference with no line saying where their path is. 0254 added the paragraph for `review` only.
- What to do: one closing line in `print_help`: ``New to this? docs/GETTING-STARTED.md walks through setup, the
  build, and the report; `sv doctor` says whether everything is ready.`` and `more_help` paragraphs for `report`
  (what the eight files are and which to open), `doctor`, and `connect`, each pointing at the guide section that
  has the whole flow, as `review`'s does.
- How verified: read.
- Overlap: 0254 (done: `review` only), 0413 (the reports page; `report --help` can point at it).
- Needs a decision record?: no.
