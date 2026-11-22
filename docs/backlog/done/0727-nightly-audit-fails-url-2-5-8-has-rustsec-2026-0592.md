# Nightly: Audit fails: url 2.5.8 has RUSTSEC-2026-0592

**Status:** done, 22 November 2026

Recorded by the nightly CI routine on 22 November 2026. This routine records; it does not fix.

**What failed.** The Audit workflow (`audit.yml`), run by hand on `main` at commit 9091bb3e, step "Audit sv's own
crates": https://github.com/abbyshade111/StackVet/actions/runs/45020762106. `sv audit` compared 160 packages against
3,214 advisory records from OSV's download and found one known vulnerability:

    [medium] url 2.5.8 has a known vulnerability: RUSTSEC-2026-0592 (GHSA-hrj5-67jp-wqp9)
       Stack overflow in `Url::origin()` on nested `blob:` URLs

The "medium" is `sv`'s placeholder, because the advisory carries no severity score `sv` can read; read the advisory
before deciding how urgent it is. The fuzzing crate's audit, in the same run, found nothing among its 117 packages.

**Where it comes from.** `url` is not one of `sv`'s own direct dependencies: `Cargo.lock` lists it as needed by `boon`
alone. Whether a `url` release outside the affected range exists, and whether `boon` accepts it, was not checked here.

**When it began.** No commit brought it: the Audit run on push at 8c504665 (22 November, 01:29 UTC, run 44971717787)
passed with the same `Cargo.lock` entry, so the advisory is new in OSV's records since then.

**What to do.** Read the advisory, then upgrade `url` in `Cargo.lock` past the affected range if a fixed release is
out (`cargo update -p url`) and run the tests, or, if none is out yet, record why the finding is accepted for now.

**Done, 22 November 2026.** Overtaken the same day, before this item was picked up: the version bump for v0.3.1
(#2537) met the same failure on its own `audit` check and moved `url` to 2.5.9 with `cargo update -p url`
(`form_urlencoded` to 1.2.3 with it). The test that uses `boon` passed (`oscal_export`), and the `audit` check on
that pull request passed on the new lockfile, which is what shows 2.5.9 is past the advisory's range; the advisory's
own range was not read. `url` is in the lockfile only through `boon`, which `sv-cli`'s tests use, so it was in no
program that shipped.
