# One tree-sitter query walk per rule per file: up to 31 walks of each Python file

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** Each file is parsed once (`:727`) and each query compiled once (`LazyQuery`, `:253-268`), both good; but 35 rules mean up to 31 full tree walks per Python file (per `data/ast-rules.json`: python 31, javascript and typescript 29, ruby 25).

**Where.** `crates/sv-check/src/ast.rs:739-777`: `for compiled in &rules.compiled { … QueryCursor::new() … cursor.matches(query, tree.root_node(), …)`.

**Evidence.** The loop as quoted.

**How it bites.** Estimate: 1 to 3 times the parse time per file.

**Fix.** Concatenate a language's patterns into one `Query` (tree-sitter reports `pattern_index` per match) and dispatch to the rule by index; the per-rule `function`/`module` regex filters stay. `ast/tests.rs` (3,400 lines of witnesses) holds every rule's behavior, and the coverage gate the citations.

**Record.** ADR-054 governs the AST rules: an "unchanged, because" line (the same rules, one walk).

**Measured, 13 October 2026, and what was built.** Release builds on a 4-core Linux container. Over Python 3.12's
standard library (574 files, 10 MB), parsing took 1.1 s and `read_file` 17 to 18 s, of which the 31 query walks were
12.7 s, about 0.45 s each: each walk cost about what the parse did, so the estimate of 1 to 3 times the parse time was
low by ten. The same 31 queries as one query walked the same files in 0.86 s.

Built as the item proposed: a language's queries are compiled as one (`AstRules::together`, a pattern's rule found by
where its text starts), each file is walked once, the matches are sorted by rule, and each rule judges its own in the
order they came, so the filters after the walk are untouched. If the queries ever fail to compile as one, each rule's is
walked on its own, as before, and `compile_all` refuses data that does that. Compared over three corpora with nothing
differing in findings, lines, confidence, wording, the parameter destinations or the broken queries:

- Python, the standard library (574 files, 10 MB): `read_file` 19.0 s a walk per rule, 8.6 s one walk, 251 findings.
- JavaScript, `/opt/node-tools/node_modules` (2,790 files, 30 MB): 348 s, 288 s, 473 findings.
- TypeScript, the same folder (136 files): 0.79 s, 0.24 s, 5 findings.

Where the rest of JavaScript's 288 s goes was not looked into; one walk saved 60 s of it, and the rest may be worth an
item of its own.
