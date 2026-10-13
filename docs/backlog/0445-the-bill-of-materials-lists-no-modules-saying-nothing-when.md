# The bill of materials lists no modules, saying nothing, when the stdlib data file cannot be read

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-check`. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/sbom/runs_on.rs:181-194` (`python_stdlib` returns an empty set on a missing or
  malformed `python-stdlib.json`), `:80-88` (no module is then ever listed), `cyclonedx.rs` (the `securevibe:modules`
  property then reads as "the modules ... the app's code imports: " with none).
- What is wrong: a damaged install shows a Python app importing nothing of its own library, which reads as a fact
  about the app. The other readers under `sv_frameworks::data` fail loudly. `runs_on_tests.rs:51` would catch a
  missing file in CI, not at a person's run.
- What to do: carry a "not read: `python-stdlib.json` could not be read" into `Language` (a `modules_from` beside
  `version_from`) and write it into the property, or fail the way the other data readers do.
- How verified: read.
- Overlap: 0402 (done; built this). New.
- Needs a decision record?: no.
