# The secret rules are loaded by six separate paths, each with its own story when they cannot be read

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-cli/src/lib.rs:78` (`Loaded.secret_rules`), `:234` (`cannot_run_said`, loaded per failure
  message, `.ok()`), `:1341` (the bundle), `crates/sv-cli/src/main.rs:1334`, `crates/sv-cli/src/review.rs:364`,
  `crates/sv-check/src/review.rs:116-119` (a `LazyLock`, masking to an empty string when the load fails),
  `crates/sv-run/src/docker.rs:2488` (loaded, and its ~30 regexes compiled, per failing seed line, saying "left out,
  because sv's own rules ... could not be read"). In tests, twelve more loaders: `fn secret_rules()` in nine files
  (`crates/sv-check/tests/{stand_in,suppressed,adapters,clean_coverage,unread_files,gosec_fence,codeql}.rs`,
  `crates/sv-check/src/adapters/{stand_in_tests,fence_tests}.rs`) and `fn rules()` in `ai_tool.rs:543` and
  `suite.rs:931`.
- What is wrong: three different wordings for one failure (a long sentence in `said_without_credentials`, a short one
  in `docker.rs`, silence in `review.rs::masked_in`), and 0335's "every regex compiled once" is not true of the
  `docker.rs:2488` path. A rule file that fails to load is said once per place rather than once.
- What to do: `SecretRules::shared() -> Result<&'static SecretRules>` (a `OnceLock` in `sv-check` over
  `sv_frameworks::data::file`), one failure sentence, and `Loaded` holding a reference to it; the test helpers use it
  too. Must not lose: `Loaded` failing the run when the rules cannot be read, and the `SV_DATA_DIR` override (a
  `OnceLock` must be filled after the environment is read, which it is, since nothing reads the rules before `main`).
- How verified: read.
- Overlap: 0351 (files read many times; different), 0335 found-sound paragraph (contradicted at one site).
- Needs a decision record?: no (ADR-036 governs `data.rs`; the read still goes through `data::file`).
