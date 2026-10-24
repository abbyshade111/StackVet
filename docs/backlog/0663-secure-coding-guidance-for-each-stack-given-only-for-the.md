# Secure-coding guidance for each stack, given only for the stack sv detects

**Status:** claimed by claude/0663-stack-guidance, 24 October 2026

Found 24 October 2026, reading Wiz's open-source rules files
(https://github.com/wiz-sec-public/secure-rules-files, commit `04101f88`, 23 December 2025). They hold, for nine
languages and frameworks (Flask, Django, Node.js, React, Spring, ASP.NET Core, C, Terraform, CloudFormation), a short
list of the weaknesses most common in that stack and one rule for each, naming the framework's own safe function.

`sv`'s coding rules (`data/coding-rules.json`, 27 rules from AISVS Appendix C) say how the coding tool behaves: keep
keys out of the chat, treat fetched text as data, never merge its own work. None says how to write the code for a
given stack. `sv` already knows the stack (`data/tech-signatures.json`, `data/adapters.json`) and the feature about
to be built (`data/feature-briefs.json`), so it could hand the tool, through `sv brief` and the `AGENTS.md` section,
a few lines for the stack it found and no other: for Flask, Jinja's escaping and never `|safe` or `Markup` on a
value from the request, `werkzeug.utils.safe_join` for paths, `yaml.safe_load`, never `pickle` on outside data;
for Spring, a parser with external entities turned off; for Django, a new session on sign-in.

To do:

- Choose the stacks from those `sv` detects, not Wiz's list (Rust, Go, and Ruby are in `sv`, C is not). Wiz's
  choice of weaknesses for each stack is a checklist of what to consider, not a source: its copies disagree (backlog
  0664).
- Tie each line to the `sv` rule that catches the fault it guards against, so the guidance and the check name the
  same thing; a line with no such rule says so.
- Give a line only when the stack is detected, and none when nothing is.
- **What it is worth:** nothing, as evidence, like every coding rule (`crates/sv-check/src/coding_rules.rs`, "What it
  is worth"). No requirement changes status because the guidance was given.
- Changes `data/coding-rules.json` or a file beside it, so ADR-017 gets a dated Later entry, and a new data file gets
  its line in `data/README.md`.

**License: use the ideas, never the words.** Wiz's repository is under CC BY-NC-ND 4.0 (attribution,
non-commercial, no derivatives). "No derivatives" means none of its text may be adapted and shipped, and
"non-commercial" clashes with the CC BY-SA 4.0 that `data/coding-rules.json` is shared under. Nothing from it is
copied, reworded, or kept as a test file in this repository. What a rule says (that Flask's `|safe` filter must not be
given untrusted data, say) is a fact, not Wiz's expression, and every line is written in this project's own words,
credited the way `data/prompts.json` credits the Cloud Security Alliance's guide: an `inspired_by` naming the source,
with none of its text. A requirement is cited only after its text is read.

**Claimed 24 October 2026, with ADR-143 proposed** (the plan: `data/stack-guidance.json`, frameworks recognized by
the packages an app lists, each line tied to an existing check and carrying its citations).
