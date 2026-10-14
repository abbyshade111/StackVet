# Six small pairs that should be one function each

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: low. Verified against the code by the reviewing session before filing.

- Where and what differs:
  - `same_file`: `crates/sv-cli/src/mcp/resources.rs:348/354` and `crates/sv-cli/src/report_lock.rs:475/481`; the
    non-Unix fallbacks differ (`a.len() == b.len() && b.is_file()` against `a.file_type().is_file()` ignoring `b`),
    so the same "did two looks see one file" question is answered differently on Windows by the two places that ask it.
  - `first_line`: `crates/sv-check/src/adapters.rs:2057` (first line, trimmed, may be empty) and
    `crates/sv-run/src/docker.rs:2504` (first non-empty line or "no detail").
  - The cut "… (N more characters)": `crates/sv-check/src/finding.rs:72`, `crates/sv-cli/src/seen.rs:370`,
    `crates/sv-cli/src/code_lines.rs:90`, and `crates/sv-check/src/ai_tool.rs:71` (the same cut without the count).
  - `provider_args` and `model_args`: `crates/sv-run/src/docker.rs:1506` and `:1539`, identical but for the length
    of the `env` array and which script is passed.
  - `parse_head` and `parse_response`: `crates/sv-check/src/production.rs:1616` and `crates/sv-run/src/docker.rs:3218`,
    both read a status line and `name: value` headers lowercased; only the first handles a proxy's extra status line.
  - `ProbeResponse::header` (`crates/sv-check/src/probes.rs:61`, lowercases the name asked for) and `Answer::header`
    (`crates/sv-check/src/production.rs:100`, exact match; safe today because every caller passes lowercase).
- What to do: one of each, the stricter behavior kept (`report_lock`'s `same_file` on Unix, `resources`' fallback
  with `is_file`; `docker.rs`'s `first_line`; one `cut_with_count(text, n)`; `helper_args(network, name, env:
  &[&str], script)`), each with the tests of both.
- How verified: read.
- Overlap: 0046 (first line of a crash, done; the wording, not the helper). None for the rest.
- Needs a decision record?: no.
