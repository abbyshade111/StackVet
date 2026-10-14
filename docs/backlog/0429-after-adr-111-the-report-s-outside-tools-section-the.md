# After ADR-111, the report's "Outside tools" section, the coverage tables, and `sv bundle --help` still say the tools need `--tools`

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), areas: `sv-report`, coverage accuracy, and `sv-cli`. Severity:
medium (a misleading report: it gives the person a command that no longer does what the sentence says). Verified
against the code by the reviewing session before filing (the three strings were grepped; the test holds the stale one).

**What is wrong.** ADR-111 (backlog 0394, 13 October, 08:04) made the outside tools run with every `sv report` and
`sv bundle` unless `--no-tools`, and updated the words in `crates/sv-cli/src/lib.rs:441`, the help for `sv report`,
the README, the guide, and the prompts. Three places were not in its list:

- **The report's "Outside tools" section** (backlog 0379, written nine hours before ADR-111): "None ran this time.
   They run only with `sv report --tools`, at a terminal: {tools}." at `crates/sv-report/src/html.rs:517`,
   `crates/sv-report/src/markdown.rs:362`, and `OUTSIDE_TOOLS_SECTION` at `crates/sv-report/src/lib.rs:1595-1597`, held
   by `crates/sv-report/tests/report.rs:824`. The sentence now appears exactly in the runs where the tools did not
   run: `sv report --no-tools`, every report the MCP server writes (`stackvet_write_report` never runs them,
   ADR-066), and the GitHub Action (which passes `--no-tools`). In each the person is told something false, and
   `--tools` "changes nothing" (ADR-111, decision 1).
- **The coverage tables and the short version's "what did not run" line:** `tools/coverage.py:72` ("the tool
   installed (`--tools`)") and `:1249` ("need `--tools`"), written into `docs/COVERAGE.md:31` and `:340` and
   `docs/REQUIREMENTS.md:34`; and `crates/sv-report/src/lib.rs:1213` (`kinds.push("outside tools such as Semgrep
   (`--tools`)")`), reached whenever `run_tools` is false, so the AI coding tool reads "run with `--tools`" after every
   `stackvet_write_report`, which does nothing at a terminal and nothing in the server.
- **`sv bundle --help`** (`crates/sv-cli/src/main.rs:355-359`) lists `[--no-tools]` and says nothing of the tools
   running by default or of Semgrep's connection to semgrep.dev, where `sv report --help` (`:345`) says both; ADR-111's
   decision 5 wants the connection named where a person sees the rule, and a person who bundles and never reports
   reads nothing about a connection their run makes.

**Fix.** (1) "None ran this time. They run with every `sv report` at a terminal unless `--no-tools` is given; the MCP
server and the GitHub Action never run them.", in all three strings and the test, with a test that greps `sv-report`
for `--tools` so the next change of the default cannot miss it. (2) "the tool installed; runs with `sv report` unless
`--no-tools`" in `coverage.py`, regenerate the three files; "outside tools such as Semgrep (they run with `sv report`
at a terminal; the MCP server never runs them)" in `lib.rs:1213`. (3) The two lines from the report help in the
bundle help. Keep the sentence naming the tools and the table. 0377 (open), written before ADR-111, names `--tools`
too and wants the same words when built.

**Record.** ADR-111 is the decision; its Governs list names `sv-cli` files only. `ADR-111: unchanged, because` the
words are brought to it (or a Later line naming the three places, since `main.rs` is governed).
