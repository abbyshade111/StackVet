# The short version's "only these kinds of run could have reached N requirements" counts a tool of another language as a reach

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage accuracy. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-report/src/lib.rs:1218-1232` (`only_they_reach`: a requirement counts when every kind in
  `reach[id]` is a kind that did not run); `tools/coverage.py:934-957` (`reach_json`: for the `tools` kind, any tool
  with a crediting rule, no language recorded); `data/reach.json` (`"V15.3.5": ["tools"]`, whose only crediting rules are
  `codeql-javascript`'s; likewise V15.3.6, V1.2.2's would be if it were not finding-only).
- What is wrong: `reach.json` says which kinds of run can credit a requirement, not for which languages. A Python app
  reported with `--no-tools`, or by the MCP server, is told that "outside tools" could have reached V15.3.5 and V15.3.6,
  which only CodeQL for JavaScript can; a Go app is told the same for the 28 ASVS requirements only an outside tool
  credits, whatever language the tool reads. The number is a count of what a kind of run would add, so the error is a
  few too many, never a status; `clean_run_evidence` itself is language-aware, so nothing is credited wrongly.
- What to do: have `reach_json` write, for `tools`, the languages of the rules that credit (`adapter.language`, or
  the rule's `languages` for Semgrep), and have `not_run_this_time` take the app's languages and count a `tools` reach
  only when one matches. Do not lose: `sv explain`'s reading of `reach.json`'s `checks` half (`crates/sv-cli/src/explain.rs:351`).
- How verified: read.
- Overlap: `grep -rn "only_they_reach\|not_run_this_time" docs/backlog/` finds nothing. 0147 and 0006.11 are about
  what the tools reach, not this count. None found.
- Needs a decision record?: no (a count, not a status).
