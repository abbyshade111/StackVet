# Outside tools run with every report by default, and --no-tools skips them

**Status:** done, 13 October 2026

The owner's decision, 13 October 2026: "yes, please file it and build after the report", after being told the
outside tools (Semgrep, Bandit, CodeQL, and the rest) added about 20 seconds to a report of the haiku-a-5 trial app,
and that their findings are, in the owner's words, the most important.

What it builds (the record is ADR-111, proposed here, accepted in the pull request that builds it):

- `sv report` and `sv bundle` run the outside tools unless `--no-tools` is given; `--tools` is still accepted and
  changes nothing, so scripts and the setup prompts that pass it keep working.
- The help text, `docs/GETTING-STARTED.md`, the setup prompts, and the report's own words for a run without them
  ("`sv report` does not run other people's tools unless you pass --tools") say the new default and `--no-tools`.
- A tool that is not installed is said in the "Outside tools" section as it is today, and changes the exit status
  only with `--fail-on not-assessed` (ADR-029, unchanged).
- Unchanged on purpose: the MCP server still never runs them (ADR-066: running other people's programs is a person's
  decision at a terminal, and a check there has under a minute), and the GitHub Action, whose image carries none of
  the tools (ADR-081), passes `--no-tools` so its reports do not list every tool as missing.
- The Semgrep rules' license is non-commercial only; the getting-started page says so where it describes the default.
