# The weak-key check reports in test code and lists the finding apart, instead of skipping test files

**Status:** claimed by claude/0647, 23 October 2026

From the owner's answer of 23 October 2026 to backlog 0647's choices ("A: read the source", and all three test-code fixes); the design entry is `docs/design/0394-the-path-rule-s-false-alarms-and-how-test-code-is-read-23.md`.

`crates/sv-check/src/weak_keys.rs` skips test files, where the code rules report in test code and list the finding
apart (ADR-023). To do: read test files too, mark a finding there as in test code (by 0654's reading), so it is listed
apart and still counts. Test both. A Later entry on ADR-137.

**Order,** at the owner's word of 23 October 2026: after 0654.
