# Line number from a byte offset is written nine times, in two conventions

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

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

**What was built (14 October 2026, session documentation).** One `sv_scan::text::line_of(text, at)`, 1-based and
documented, in `crates/sv-scan/src/text.rs`, with its test in `text_tests.rs`. The nine copies call it:
`grants.rs`, `hosted_rules.rs` (its `line_at` deleted), `client_tech.rs`, `launch.rs` (both), `sv-scan`'s `jvm.rs` (its
`line_at` deleted), and the two 0-based users, which now write `line_of(..) - 1` with a comment saying why: `ast/html.rs`
(a fragment's offset in its page, the closure renamed `lines_before`) and `ast/templates.rs`'s `joined` (the newlines
before a piece). `templates.rs:365` compares two lines only with each other, so it takes `line_of` as it is. The site
the item named at `files.rs:927` is in that file's tests, comparing the reader against a count of its own, and stays.
Breaking the rule (`line_of` made 0-based) turned 34 tests red across every caller, the page and template readers
among them. Not done: the cost of a scan from the start per call (`launch.rs` calls it per match) is unchanged; no
caller has been seen to be slow, and a line index would be a change of its own. `templates.rs:156` keeps its own
indexed 0-based lookup, which the item did not name.
