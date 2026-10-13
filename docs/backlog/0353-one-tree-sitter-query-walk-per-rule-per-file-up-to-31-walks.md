# One tree-sitter query walk per rule per file: up to 31 walks of each Python file

**Status:** claimed by builder-none, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** Each file is parsed once (`:727`) and each query compiled once (`LazyQuery`, `:253-268`), both good; but 35 rules mean up to 31 full tree walks per Python file (per `data/ast-rules.json`: python 31, javascript and typescript 29, ruby 25).

**Where.** `crates/sv-check/src/ast.rs:739-777`: `for compiled in &rules.compiled { … QueryCursor::new() … cursor.matches(query, tree.root_node(), …)`.

**Evidence.** The loop as quoted.

**How it bites.** Estimate: 1 to 3 times the parse time per file.

**Fix.** Concatenate a language's patterns into one `Query` (tree-sitter reports `pattern_index` per match) and dispatch to the rule by index; the per-rule `function`/`module` regex filters stay. `ast/tests.rs` (3,400 lines of witnesses) holds every rule's behavior, and the coverage gate the citations.

**Record.** ADR-054 governs the AST rules: an "unchanged, because" line (the same rules, one walk).
