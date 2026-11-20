# tree-sitter-dart 0.3 makes the Dart sql-built-by-hand rule flag its safe form (Dependabot #2401)

**Status:** open

Recorded on 20 November 2026 while settling the open Dependabot pull requests. Dependabot's pull request #2401
(`tree-sitter-dart` 0.2 to 0.3, opened 6 November 2026) fails `suite (check)`, and the failure is not staleness:

- **What failed:** `ast::tests::the_newer_rules_find_the_unsafe_form_and_leave_the_safe_one`
  (`crates/sv-check/src/ast/tests.rs:3464`). Under the new grammar, `ast.sql-built-by-hand` in Dart reports
  `Future f() => db.rawQuery('select * ' + 'from notes');`, the form the test holds up as safe (two literals joined,
  with no outside text in them). So the Dart query for the rule now matches what it used to leave alone, most likely
  because 0.3 names or nests string literals or `+` expressions differently.
- **Also owed by the pull request:** `ADR-020` and `ADR-077` lines, since it changes `Cargo.toml`, and a regenerated
  `fuzz/Cargo.lock` (the dependency-upkeep pull request of the same day leaves `tree-sitter-dart` at 0.2).
- **What to do:** read 0.3's `node-types.json` against the Dart queries in `data/ast-rules.json` (every Dart rule, not
  only this one: a test failing for one rule where several changed would mean the coverage is accidental), fix the
  queries so each rule's witnesses pass under 0.3, then take the bump with the fuzz lock regenerated. Until then,
  #2401 stays open and unmerged.
