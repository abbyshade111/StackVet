# Bandit's B104 is worth a look, not a thing to fix, when stackvet.toml runs the app in a container

**Status:** done, 23 October 2026

From backlog 0648's survey, at the owner's answer of 23 October 2026 ("Container: worth a look (Recommended)"); the
design entry is `docs/design/0395-other-rules-whose-false-alarms-come-in-large-numbers-across.md`.

Bandit's B104 reports an app listening on every address (`0.0.0.0`), and `sv preflight` tells an app to do exactly
that (ADR-035), since inside the container `sv` starts it in, that is the only way to be reached. Four of the twelve
apps surveyed, two of them `sv`'s own examples, had the line among their things to fix.

To do: when stackvet.toml runs the app in a container (`[stack.run]`), list a B104 finding apart as worth a look, said
beside it (in a container this is how the app is reached; outside one it is on the local network), and still counted.
With no `[stack.run]` it stays a thing to fix, and so does one another tool also reported. Built as other outside rules
are set apart (backlog 0521), with a test each way. A Later entry on ADR-111.

**Done, 23 October 2026** (ADR-111, Later). In an app stackvet.toml runs in a container, Bandit's B104 is set apart as
worth a look, said beside it in the reports and in SARIF, and still counted; with no `[stack.run]`, or when another tool
found the same line, it stays a thing to fix. Tests: `crates/sv-check/src/adapters/listens_in_container_tests.rs` and
`crates/sv-report/src/listens_in_container_tests.rs`.
