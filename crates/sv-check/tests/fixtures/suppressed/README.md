# Suppressed findings

Real reports from five tools, each run over the small app beside it, in which some finding has been
marked to be ignored. `crates/sv-check/tests/suppressed.rs` (and, for CodeQL,
`crates/sv-check/tests/suppressed_codeql.rs`) checks that such a run is never credited
as clean, and that what the tool reports as suppressed is shown.

- `bandit-1.9.4-nosec.sarif`: `bandit --recursive --format sarif --quiet blanket.py targeted.py`
  inside `bandit-app/`. No results; `_totals` counts one `# nosec` line and one skipped test.
- `bandit-1.9.4-ignore-nosec.sarif`: the same with `--ini /dev/null --ignore-nosec`, as `sv` runs it.
  Both injections are reported.
- `gosec-2.29.0-tracked.sarif`: `gosec -fmt sarif -quiet -track-suppressions ./...` inside
  `gosec-app/`. Three results marked `inSource`, one not.
- `semgrep-1.178.0-nosemgrep.sarif`: `semgrep scan --config semgrep-rule.yml --sarif --metrics off
  app.py` over `semgrep-app/app.py`, copied out of this folder first: semgrep skips anything under a
  folder called `tests/` by default, so run here it reports nothing at all.
- `brakeman-8.0.6-ignored.sarif`: `brakeman --format sarif --no-progress --quiet --force .` inside
  `brakeman-app/`, whose `config/brakeman.ignore` lists the SQL injection. Both warnings are reported;
  the ignored one is marked `external` and names the ignore file.
- `codeql-2.27.2-js-lgtm.sarif` and `codeql-2.27.2-py-lgtm.sarif`: CodeQL 2.27.2 run as `data/adapters.json`
  runs it (`database create` then `database analyze` with the security-extended suite, `--format=sarif-latest`)
  over `codeql-app-js/` and `codeql-app-py/`, on 15 October 2026 (backlog 0509). Each app has the same command
  injection four times: once unmarked, once with `lgtm[...]` on the line, once with `codeql[...]` on the line
  before, once with `codeql[...]` on the line. All four are reported in each language, none with a
  `suppressions` entry: the command-line tool does not act on these comments (GitHub's code scanning does, after
  the fact), so a finding such a comment would hide still reaches the report.
