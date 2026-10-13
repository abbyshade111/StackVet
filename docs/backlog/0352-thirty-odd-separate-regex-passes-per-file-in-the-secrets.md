# Thirty-odd separate regex passes per file in the secrets scan

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** All compiled once (good), but about 30 full passes per file, and most files match none of them.

**Where.** `crates/sv-check/src/secrets.rs:401-403`: `for (rule, re) in &rules.rules { for m in re.find_iter(text) {` over 21 rules from `data/secret-rules.json`, then `url_password_findings`, then `assignment_findings` with `QUOTED_SHAPES` (`:592`), `UNQUOTED`, `SHELL_UNQUOTED`, `DOCKERFILE_UNQUOTED` (`:645-669`); `line_of(text, m.start())` at `:1042` counts newlines from the file's start for every match.

**Evidence.** The loop as quoted.

**How it bites.** Estimate: 30 × the app's size of regex scanning, 0.5 to 2 s on a 50 MB app.

**Fix.** A `RegexSet` of the 21 vendor patterns as a pre-filter (one pass says which rules can match; `find_iter` only for those), the same for the assignment shapes; `line_of` incremental. The secret fixtures (`tests/redact_printed.rs`, `tests/false_alarm_corpus.rs`, the `secrets.rs` tests) hold what is found.

**Record.** No record governs `secrets.rs` by name; say so.

**Measured, 13 October 2026, and what was built.** Timed in release builds on a 4-core Linux container, over 32 MB of
this repository's own `crates/`, `data/`, `docs/` and `examples/` (1,762 files):

- The 21 vendor patterns, run one by one with `find_iter`: 41 ms in all. As one `RegexSet`, only to say which can
  match: 1.34 s.
- The six quoted assignment shapes, run one by one: 205 to 230 ms. As one `RegexSet`: 7.67 s.
- A whole scan with both sets as pre-filters: 8.6 to 9.1 s, against 7.4 to 7.6 s without them (45 findings either
  way).

Each pattern starts with a literal (`AKIA`, `sk-ant-`, `ghp_`) that the regex engine finds with a fast substring
search, so thirty passes cost little; a set loses that. The pre-filter was not built. What was: `line_of`, which
counted line breaks from the file's start for every finding, gave way to `Lines`, which finds a text's line breaks
once, and only if a finding needs a line. A 1.7 MB file with 20,000 keys in it took 7.8 s to scan before and 0.11 s
after, with the same 20,000 findings.
