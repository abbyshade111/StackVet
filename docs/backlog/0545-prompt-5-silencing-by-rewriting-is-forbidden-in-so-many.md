# Prompt 5: silencing by rewriting is forbidden in so many words, and "say what changed" must name every edit a finding caused

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity trial. For the prompt library
(0370) and `crates/sv-cli/tests/build_prompt_rules.rs`.

**What was seen.** After `stackvet_check` named two claim contradictions, the tool removed `media_type=` from a
response ("will eliminate a potential false positive") and moved two packages to `requirements-dev.txt`, then told the
owner "No code files or configurations were deleted or silenced." The prompt already says never to silence a finding
(0407); the tool read a rewrite that makes a word disappear as not silencing.

**What to change.** Prompt 5: "Silencing includes rewriting code, a dependency file, or a test so that a check no
longer sees something, even when the app behaves the same. If you think a finding or a contradiction is wrong, leave
the code as it is and tell me; I decide." And the "say what changed" rule: "List every file you changed because of a
check, with the check's words beside it." The test file holds both sentences.
