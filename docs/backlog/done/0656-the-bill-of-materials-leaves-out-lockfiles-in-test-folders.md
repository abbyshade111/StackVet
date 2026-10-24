# The bill of materials leaves out lockfiles in test folders and folders named as not the app

**Status:** done, 23 October 2026

From the owner's answer of 23 October 2026 to backlog 0647's choices ("A: read the source", and all three test-code fixes); the design entry is `docs/design/0394-the-path-rule-s-false-alarms-and-how-test-code-is-read-23.md`.

StackVet's report on itself said its bill of materials was not complete (`sbom.incomplete`) because it read sample
lockfiles under `crates/sv-scan/tests/fixtures` as what StackVet ships. To do: leave out lockfiles and manifests in test
code (0654's reading) and in folders named as not the app (`[repository] not-the-app`), and say so in what the bill of
materials says it read, as it already says what it could not read. A lockfile left out never makes the list complete
on its own account. Tests, both ways. ADR-140, proposed.

**Order,** at the owner's word of 23 October 2026: after 0655.

**Done, 23 October 2026** (ADR-140, accepted). What the app ships is an inventory of its own (`build_app_in`), with
the lockfiles and manifests in test code and in folders named as not the app left out and named; it decides V15.1.2's
finding or credit and is what `sbom.cdx.json` and `sv sbom` publish. The known-vulnerability checks still read every
lockfile, so a sample's vulnerable package is still found and listed apart: the first build left test code out of the
one list every caller reads, and `sv check`'s tests caught it. Test: `crates/sv-check/src/sbom/left_out_tests.rs`. The
last of backlog 0647's builds.
