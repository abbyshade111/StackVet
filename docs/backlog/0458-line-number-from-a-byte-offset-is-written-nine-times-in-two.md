# Line number from a byte offset is written nine times, in two conventions

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: low. Verified against the code by the reviewing session before filing.

- Where: 1-based: `crates/sv-check/src/grants.rs:238`, `crates/sv-check/src/hosted_rules.rs:104` (`fn line_at`),
  `crates/sv-check/src/client_tech.rs:217`, `crates/sv-check/src/launch.rs:466` and `:725`, `crates/sv-scan/src/jvm.rs:113`
  (`fn line_at`), `crates/sv-scan/src/files.rs:927`; 0-based: `crates/sv-check/src/ast/html.rs:58`,
  `crates/sv-check/src/ast/templates.rs:365` and `:794`.
- What is wrong: `text[..at].matches('\n').count()` with and without `+ 1`. The 0-based ones are added to a base
  line by their callers, which is fine, but the next copy made from the wrong neighbor puts a finding one line off,
  and nothing names the convention. Each closure is also O(n) per call (`launch.rs:466` is called per match).
- What to do: one `line_of(text, at) -> usize` (1-based, documented) in `sv-scan`, and the two 0-based users add
  their base explicitly (`base + line_of(..) - 1`) so the convention is visible where it is used.
- How verified: read.
- Overlap: none found.
- Needs a decision record?: no.
