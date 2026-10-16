# Semgrep's insecure-WebSocket rule is a text pattern that fires on any file, comments and Rust included

**Status:** open

Filed from `sv`'s self-report of 16 October 2026 (`sv report .` with the outside tools on, at `main` `28aae3a2`).
Severity: low (false alarms that mark a requirement as needing attention).

**What is wrong.** Semgrep's `javascript.lang.security.detect-insecure-websocket` matches the text `ws://` in any file,
whatever its language, and in comments and strings alike. `data/adapters.json` maps it with `"languages": ["*"]`, and
`sv` keeps every match Semgrep returns and counts it against V12.3.1 (the self-report also shows V4.4.1 on it).

**Where it showed.** 27 findings in `sv`'s own repository: 23 in Rust files, 2 in Python, and 2 in HTML. The one
outside test code is `crates/sv-check/src/ast.rs:223`, a doc comment that explains why `sv`'s own `ws://` rule cannot
show an absence. It is the only finding outside test code behind V4.4.1, which the self-report lists as needing
attention for the first time.

**Where.** `data/adapters.json`, the rule's entry (near line 6116); `crates/sv-check/src/adapters.rs`, the rule's
`languages` field (near line 229), which today decides only what a clean run is evidence for, not which findings are
kept.

**What to do.** Decide between two courses and record which: give the rule the languages it was written for
(`javascript`, `typescript`) and have `sv` set aside a match in a file of another language, saying so, or keep `"*"`
and have `sv` drop matches that sit in a comment of the file's language. `sv`'s own `ast.plaintext-websocket-url` already
covers the address written into code in the languages it reads, so the first course loses little. Either way, check
`docs/research/SEMGREP-FALSE-ALARMS.md`, which does not list this rule yet, and add it.

**Break it and watch:** a fixture with `ws://` in a JavaScript string must still give the finding; one with `ws://`
in a Rust comment must not.
