# Nothing holds a ConfigVet outside tool to the version its offline flags were measured at

**Status:** open

Found by the ninth weekly review of the decision records (0742, 23 November 2026), reading ADR-157 part 3 against
`main`.

`cv check` starts hadolint, Checkov, and Trivy when they are installed, unless `--no-tools`. It opens no connection
itself, and what keeps those three from connecting is the flags in `data/cv-adapters.json`, each measured on 21
November 2026 at one version (Checkov 3.4.6 with `--skip-download`; Trivy 0.74.0 with `--skip-check-update`,
`--skip-version-check`, and `--disable-telemetry`). Nothing compares the version installed with the one measured, so
a newer Checkov or Trivy that phones home some other way would do so under `cv check` with the report saying nothing.
`sv`'s own outside tools have the same shape, and ADR-111 says so for Semgrep.

**To do.** The report names each tool's version beside the version its flags were measured at, and says plainly when
they differ ("its offline flags were measured at 0.74.0; this is 0.80.1, not measured"). No credit changes. A test in
a sibling file. Two smaller things seen beside it: `crates/cv-cli/Cargo.toml`'s comment "It depends on no crate of
`sv`'s checks, runner, or report" has been false since #2456 (it depends on `sv-check`); and the test that keeps
ConfigVet's live clients out of `sv` (`crates/sv-cli/tests/cv_live_fence.rs`) reads `path = "../..."` lines only, so
a dependency written through the workspace table would pass it unseen (none is written so today).
