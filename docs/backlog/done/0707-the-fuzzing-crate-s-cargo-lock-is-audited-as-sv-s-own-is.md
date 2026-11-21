# The fuzzing crate's Cargo.lock is audited, as sv's own is

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word, from the security review of 21 November (finding A6, its third point).
`.github/workflows/audit.yml` runs `sv audit` over the workspace's `Cargo.toml`, `Cargo.lock`, and each crate's
`Cargo.toml`, the files `sv` is built from. `fuzz/` is a crate of its own outside the workspace (ADR-077), with its own
`fuzz/Cargo.lock`, built and run by `.github/workflows/fuzz.yml` on a nightly toolchain, and nothing audits it.

- **To build:** the audit also copies `fuzz/Cargo.toml` and `fuzz/Cargo.lock` into a folder of their own and runs
  `sv audit` over it with the same downloaded list, reported as a separate result so a finding in the fuzzing crate's
  dependencies is not read as one in `sv`'s. The workflow's `paths` include `fuzz/Cargo.lock` and `fuzz/Cargo.toml`.
  Its comment at the top says what is audited and why the fuzzing crate is now in it.
- **Check:** the step is seen to fail on a lock file holding a crate version with a known advisory (made in a scratch
  folder at run time, never committed), and to pass on the real one.
- **Records:** ADR-077 or the audit's own record gains a Later entry, whichever governs `audit.yml`.

**Done, 21 November 2026:** `.github/workflows/audit.yml` audits `fuzz/Cargo.toml` and `fuzz/Cargo.lock` in a step
and a folder of their own, after `sv`'s own, and a change to either runs it; `crates/sv-cli/tests/audit_fuzz_lock.rs`
holds the workflow to it, and each of four breaks to the step turned it red. Over the real files `sv audit` compared all
117 packages and found nothing; with `time` 0.1.40 added to a scratch copy (never committed) it named RUSTSEC-2020-0071
and exited 1. ADR-077, Later; the step is on its Governs line.
