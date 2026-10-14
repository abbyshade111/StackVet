# `ast.sql-built-by-hand` flags `executescript(f())` where `f` returns a constant schema, a high finding under V1.2.4

**Status:** open

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
