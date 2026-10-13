# Thirty-odd separate regex passes per file in the secrets scan

**Status:** claimed by builder-none, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** All compiled once (good), but about 30 full passes per file, and most files match none of them.

**Where.** `crates/sv-check/src/secrets.rs:401-403`: `for (rule, re) in &rules.rules { for m in re.find_iter(text) {` over 21 rules from `data/secret-rules.json`, then `url_password_findings`, then `assignment_findings` with `QUOTED_SHAPES` (`:592`), `UNQUOTED`, `SHELL_UNQUOTED`, `DOCKERFILE_UNQUOTED` (`:645-669`); `line_of(text, m.start())` at `:1042` counts newlines from the file's start for every match.

**Evidence.** The loop as quoted.

**How it bites.** Estimate: 30 × the app's size of regex scanning, 0.5 to 2 s on a 50 MB app.

**Fix.** A `RegexSet` of the 21 vendor patterns as a pre-filter (one pass says which rules can match; `find_iter` only for those), the same for the assignment shapes; `line_of` incremental. The secret fixtures (`tests/redact_printed.rs`, `tests/false_alarm_corpus.rs`, the `secrets.rs` tests) hold what is found.

**Record.** No record governs `secrets.rs` by name; say so.
