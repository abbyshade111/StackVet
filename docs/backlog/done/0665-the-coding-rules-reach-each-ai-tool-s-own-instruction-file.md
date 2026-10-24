# The coding rules reach each AI tool's own instruction file, not only AGENTS.md

**Status:** done, 24 October 2026

Found 24 October 2026, reading Wiz's open-source rules files
(https://github.com/wiz-sec-public/secure-rules-files, commit `04101f88`, 23 December 2025), which are written for
seven tools: Cursor (`.cursor/rules/*.mdc`), Cline (`.clinerules`), Claude Code (`CLAUDE.md`), Windsurf
(`.windsurfrules`), Codex (`AGENTS.md`), Aider (`CONVENTIONS.md`), and GitHub Copilot
(`.github/copilot-instructions.md`).

`sv rules` writes its section into `AGENTS.md` alone (`crates/sv-cli/src/main.rs`, `cmd_rules`). A tool that does not
read `AGENTS.md` never sees the rules, and nothing tells the person. `sv` already reads all seven files, for hidden
characters and its own marks (`crates/sv-check/src/ai_tool.rs`), but writes to one.

To do:

- For each tool, read its current documentation and record whether it reads `AGENTS.md` from the project folder, and
  how (on its own, through an import such as `@AGENTS.md` in `CLAUDE.md`, or only when the person passes it in, as
  Aider's `--read` does). Not checked yet: nothing here says which tools read it today.
- For a tool that does not, the smallest fix: a one-line pointer to `AGENTS.md` in that tool's own file, between
  `sv`'s markers, with the owner's text outside them left alone, never a second copy of the rules (backlog 0664).
  Where no pointer works, `sv rules` says so and names the tool.
- Write only through `write_without_following`, refusing a link, as `AGENTS.md` is written (design 0123).
- **A decision:** this changes what `sv` writes into someone's folder, so a new ADR goes in the same pull request
  (`python3 tools/record_numbers.py next adr` after `git fetch`), written first as `Status: proposed` in the claim.

**License: use the ideas, never the words.** Wiz's repository is under CC BY-NC-ND 4.0 (attribution,
non-commercial, no derivatives). "No derivatives" means none of its text may be adapted and shipped, and
"non-commercial" clashes with the CC BY-SA 4.0 that `data/coding-rules.json` is shared under. Nothing from it is
copied, reworded, or kept as a test file in this repository. What a rule says (that Flask's `|safe` filter must not be
given untrusted data, say) is a fact, not Wiz's expression, and every line is written in this project's own words,
credited the way `data/prompts.json` credits the Cloud Security Alliance's guide: an `inspired_by` naming the source,
with none of its text. A requirement is cited only after its text is read.

**Claimed 24 October 2026, with ADR-144 proposed.** Each tool's documentation was read that day (the table is in
ADR-144). Claude Code reads `AGENTS.md` only when the app has no `CLAUDE.md`, so the plan is a marked `@AGENTS.md`
import in a `CLAUDE.md` that is already there, and a note for each tool that still will not see the rules.

**Built 24 October 2026** (ADR-144, accepted). `sv rules` puts a marked `@AGENTS.md` import (`@../AGENTS.md` from
`.claude/`) into a `CLAUDE.md` the app already has, never creates one, never copies the rules, and never writes
through a link to the file or to `.claude`. It names Claude Code with only a `CLAUDE.local.md`, Codex with an
`AGENTS.override.md`, Aider with an `.aider.conf.yml` that does not read `AGENTS.md`, and Cline when `.clinerules` is
there, and leaves each of those files alone. `crates/sv-check/src/tool_files.rs` plans it;
`crates/sv-cli/tests/rules_tool_files.rs` holds it, and each of its guards was broken on purpose and watched go red.
