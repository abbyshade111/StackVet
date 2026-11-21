# ConfigVet part 1: the shared finding and credit types move into a crate of their own, sv-core (ADR-157)

**Status:** done, 20 November 2026

Asked for by the owner on 20 November 2026 ("Start the ConfigVet sv-core build"). The first step of ADR-157 (proposed,
decision 2) and of the build order in the design entry "ConfigVet, a companion for an app's deployment configuration
(20 November 2026)".

- **Why:** ConfigVet's crates need `Finding`, `Verified`, `Tier`, and `Secret`, which live in `sv-check` with its
  93,500 lines and seventeen grammars. A crate that wants the types should not have to build every check.
- **What moves** into `crates/sv-core`, which depends only on `sv-scan`: `Severity`, `Confidence`, `Secret`,
  `Location`, `Finding`, `Outranked`, `Tier`, and `Verified`, with their methods unchanged; the lists those methods
  read (`INFORMATION_ONLY`, `TEST_FILE_INFORMATION`, `WORTH_A_LOOK`, the path-rule names and languages) and the two
  rule ids they name; `quoted`, `found`, and the credit census; and the two helpers `Secret::redact` calls
  (`cut_with_count`, `reads_as_words`).
- **What stays** in `sv-check`: everything that settles a list of findings (`merge_same_place`, `one_per_line`,
  `mark_rust_test_code`, `mark_not_the_app`, `is_svs_own`, `rust_test_lines`).
- **No change in behavior:** `sv-check` re-exports every moved item under its old path, so no other crate's code
  changes, and the whole workspace's tests are the proof. The credit census (`SV_CREDIT_LOG`) still names the
  check's own file, since the functions that record it are `#[track_caller]`.
- **Done when:** the workspace builds, `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, and
  `cargo test --workspace` pass, `python3 tools/coverage.py` writes nothing new, ADR-157 is accepted with an "As
  built" section, and `docs/ARCHITECTURE.md` lists the crate.

**Done, 20 November 2026.** `crates/sv-core` holds the types, their methods, the lists those methods read, the credit
census, and the two helpers, about 700 lines over `serde` and `sv-scan`. `sv-check` re-exports each under its old path,
so no other crate's code changed. Locally: `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D
warnings` pass, and `cargo test --workspace --no-fail-fast` passed 4,599 tests in 349 test binaries with none failing,
run with `SV_CREDIT_LOG` set. `python3 tools/coverage.py --credits` over that log says every check credited as the
lists say, and none of its 617,831 lines names a file in `sv-core`: every credit is still logged against the check that
made it. `python3 tools/coverage.py --check` writes nothing new. The decision-records test caught ADR-138 naming a
constant that had moved; five more records governed whole files whose code had partly moved, which no test could see,
and all six now name `sv-core`'s files, each with a dated Later entry (ADR-157, "As built", part 1).
