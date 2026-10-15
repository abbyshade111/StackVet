# `ast.sql-built-by-hand` flags `executescript(f())` where `f` returns a constant schema, a high finding under V1.2.4

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the VS Code app. Severity: medium (the app's only
high finding, false). Verified by the orchestrating session.

**What is wrong.** `app.py:188` is `connection.executescript(_database_schema())`; `_database_schema` (`app.py:94-131`)
returns one triple-quoted string with no parameter and no value from outside. The rule's test at
`crates/sv-check/src/ast/tests.rs:1906` covers `executescript(SCHEMA)` with a constant name (not flagged); a call that
returns a constant is flagged.

**Fix.** When the argument is a call to a function in the same file whose only return is a string literal, treat it as
the literal; otherwise report "a query built by a function sv did not follow" as not verified, not as a finding.
Fake-app case: the VS Code shape, and watch the finding go while `"..." + name` stays.

**Record.** None (a rule's accuracy; its bar is unchanged).

**Done, 15 October 2026:** a call to a Python function at the top of the same module whose one `return` gives
fixed text is judged as that text (`fixed_functions` in `crates/sv-check/src/ast/fixed.rs`): the function is defined
there once, its name is bound nowhere else in the file, it has no `yield`, and its own body (not a function or lambda
inside it) has exactly one `return`, whose value is fixed text by the rules every other value is judged by. The VS Code
app's `connection.executescript(_database_schema())` is no longer reported. Still reported, each in
`crates/sv-check/src/ast/fixed_functions_tests.rs`: a return that joins in what the function is given, a second return,
an f-string with a value in it, a `yield`, the function defined twice, its name bound again, and another object's
method of the same name. Breaking it three ways (two returns allowed, the lookup taken out, a rebinding ignored) turned
the tests red each time. Python only: the other languages get no fixed functions, which keeps their rules as they were.

**The item's second half is its own item, backlog 0570:** reporting a query given by any other function as not
verified instead of as a finding changes what counts as evidence for every app, so it waits on the owner's choice
there.
