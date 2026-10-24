# One source for the coding rules, never generated again for each tool, and why

**Status:** claimed by claude/0664-one-source, 24 October 2026

Found 24 October 2026, reading Wiz's open-source rules files
(https://github.com/wiz-sec-public/secure-rules-files, commit `04101f88`, 23 December 2025). Their generator
(`generate_rules.py`) asks a language model, once for each stack and once again for each of seven tools, to "identify
the top 5-7 relevant CWEs" itself. The copies for one stack therefore disagree:

- Flask's `CLAUDE.md` names CWE-22, CWE-306, and CWE-352; its `AGENTS.md` names CWE-16, CWE-287, and CWE-918 instead.
- React's `CLAUDE.md` keeps its rule against hard-coded secrets under no CWE, where React's `AGENTS.md` and Cursor
  file cite CWE-798; it also gives CWE-347 a name that is not MITRE's ("Improper Verification of Cryptographic
  Signature").
- The Terraform and CloudFormation files name risks but no CWE at all.

Two people building the same kind of app with different tools are given different security rules, and neither is
told. `sv` already does the opposite: one data file, `data/coding-rules.json`, rendered the same way every time. That
is right, but the reason is written nowhere, so a later session adding a format (backlog 0665) or guidance for each
stack (backlog 0663) could reach for generation for each tool.

To do:

- Write the reason down as a dated Later entry on ADR-017: the rules, and any guidance for each stack, come from one
  source in `data/`, written by hand and cited after reading, and every format a tool reads is rendered from it,
  never generated again by a model for each tool.
- A test that renders every format `sv` writes and asserts each holds the same rule ids, so a format that drops a
  rule fails (break it on purpose: drop one rule from one rendering and watch the test go red).

**License: use the ideas, never the words.** Wiz's repository is under CC BY-NC-ND 4.0 (attribution,
non-commercial, no derivatives). "No derivatives" means none of its text may be adapted and shipped, and
"non-commercial" clashes with the CC BY-SA 4.0 that `data/coding-rules.json` is shared under. Nothing from it is
copied, reworded, or kept as a test file in this repository. What a rule says (that Flask's `|safe` filter must not be
given untrusted data, say) is a fact, not Wiz's expression, and every line is written in this project's own words,
credited the way `data/prompts.json` credits the Cloud Security Alliance's guide: an `inspired_by` naming the source,
with none of its text. A requirement is cited only after its text is read.

**Claimed 24 October 2026.** The plan: a dated Later entry on ADR-017, and a test in `crates/sv-cli/tests/` that
renders every place the coding rules and the framework lines go and holds each to the same rules.
