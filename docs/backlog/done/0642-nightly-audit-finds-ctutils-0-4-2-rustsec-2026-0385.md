# Nightly: Audit finds ctutils 0.4.2: RUSTSEC-2026-0385

**Status:** done, 24 October 2026

Recorded by the nightly CI routine on 23 October 2026. The Audit workflow did not run in the last 26 hours (it runs
on Mondays and when `Cargo.lock` changes), so the routine ran its steps itself, in its cloud session, on `main` at
`3970e3b` (pull request #2235): it built `sv` with `cargo build --release --locked -p sv-cli`, downloaded OSV's
export for Rust crates (2,961 records), copied the workspace's `Cargo.toml`, `Cargo.lock`, and each crate's
`Cargo.toml` to a folder of their own, and ran `sv audit` on it. It compared 92 packages and exited 1, with one
known vulnerability:

    [medium] ctutils 0.4.2 has a known vulnerability: RUSTSEC-2026-0385
       Non-constant-time mask generation and `u128` comparisons

The "medium" is `sv`'s placeholder: the advisory carries no CVSS vector `sv` can read. OSV lists every version before
0.4.4 as affected and 0.4.4 as the fix.

- **How it reaches `sv`:** `ctutils` is not a direct dependency. `cargo tree -i ctutils` shows it under `digest`
  0.11.3, which `sv-check` reaches through `ed25519-dalek` 3.0.0 (via `curve25519-dalek` 5.0.0), `hmac` 0.13.0, and
  `pbkdf2` 0.13.0 (via `bcrypt-pbkdf` 0.11.0).
- **Was it there the night before?** It could not have been: OSV dates the advisory 23 October 2026, and the last
  Audit run on `main` (#2156's merge, 20 October, run
  <https://github.com/abbyshade111/StackVet/actions/runs/39444465732>) passed. No pull request brought it; it is a new
  advisory against a crate already in the lockfile. The next scheduled Audit run (Monday 26 October, 07:00 UTC) will
  go red on it unless it is fixed first.
- **What to do:** update `ctutils` to 0.4.4 or later in `Cargo.lock` (`cargo update -p ctutils`), if `digest`'s
  requirement allows it, and check that the Audit workflow passes; Dependabot may open the same update. Before
  deciding it does not matter, read the advisory: whether `sv` uses the affected comparisons on a secret is the
  question. This routine records; it fixes nothing.

Seen again on 24 October 2026: Audit (the Audit workflow did not run in the last 26 hours, so the routine ran its steps in its cloud session), on `main` at commit `bb87b7b` (pull request #2269): `sv audit` compared 92 packages against 2,969 advisory records and exited 1, the same single finding, `ctutils` 0.4.2, RUSTSEC-2026-0385; `Cargo.lock` still holds 0.4.2.

**Done, 24 October 2026.** `cargo update -p ctutils --precise 0.4.4`: `ctutils` moved from 0.4.2 to 0.4.4, OSV's fix for
RUSTSEC-2026-0385, and nothing else in `Cargo.lock` changed; `fuzz/Cargo.lock` moves the same way, as `fuzz_lock.rs` holds it to the workspace's versions. A move of pinned versions owes the decision records no
line (ADR-128). It builds with `--locked`, and `sv-check`'s tests, the crate that reaches `ctutils` through `digest`,
pass. The pull request's Audit run is the check that the advisory is gone.
