# Nightly: Audit fails: url 2.5.8 has RUSTSEC-2026-0592

**Status:** open

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
