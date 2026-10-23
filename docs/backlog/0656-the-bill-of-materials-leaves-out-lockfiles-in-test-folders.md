# The bill of materials leaves out lockfiles in test folders and folders named as not the app

**Status:** claimed by claude/0647, 23 October 2026

From the owner's answer of 23 October 2026 to backlog 0647's choices ("A: read the source", and all three test-code fixes); the design entry is `docs/design/0394-the-path-rule-s-false-alarms-and-how-test-code-is-read-23.md`.

StackVet's report on itself said its bill of materials was not complete (`sbom.incomplete`) because it read sample
lockfiles under `crates/sv-scan/tests/fixtures` as what StackVet ships. To do: leave out lockfiles and manifests in test
code (0654's reading) and in folders named as not the app (`[repository] not-the-app`), and say so in what the bill of
materials says it read, as it already says what it could not read. A lockfile left out never makes the list complete
on its own account. Tests, both ways. ADR-140, proposed.

**Order,** at the owner's word of 23 October 2026: after 0655.
