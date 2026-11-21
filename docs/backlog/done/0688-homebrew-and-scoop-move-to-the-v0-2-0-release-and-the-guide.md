# Homebrew and Scoop move to the v0.2.0 release, and the guide's --HEAD line with them

**Status:** done, 21 November 2026

Filed 21 November 2026, after the owner tagged `v0.2.0`: the step backlog 0120 (on hold) waited on a release after
v0.1.0 for. It is work in the two package repositories, with one line of this one that must move with them.

- **Homebrew** (`abbyshade111/homebrew-stackvet`, its formula under `Formula/`): on the day of the tag the formula is
  head-only, building `main` ("Until StackVet has its first release this is a head-only formula"). Give it a `url` for
  the v0.2.0 source archive with that archive's `sha256`, keep `head` for `--HEAD`, and check `brew install` and
  `brew test` build the tagged code, where `sv --version` prints `sv 0.2.0`.
- **Scoop** (`abbyshade111/scoop-stack-vet`, `bucket/sv.json`): still `"version": "0.1.0"`. Move it to 0.2.0, with the
  release's address and the SHA-256 the release workflow published beside the zip,
  `9bac9530269633d49b0698b68d43f4484d6abd3b93b2eb50548544f9e407ccc9`, and check it against the file before relying
  on it. The bucket's weekly workflow runs `checkver.ps1`, which says when the manifest is behind, but nothing updates
  it on its own.
- **The guide** (`docs/INSTALL.md`): its line that `--HEAD` is needed "until the formula moves to a release newer than
  v0.1.0" changes in the same step as the formula, never before, since until then it is true.
- **Not in this item:** packaging `cv`. The release builds and zips `sv` only (`-p sv-cli`); a `cv` formula and
  manifest, as ADR-157's "one release, two packages" plans, are a step of their own.

**Done, 21 November 2026.** Homebrew: abbyshade111/homebrew-stackvet#2 gave the formula the v0.2.0 source archive
(`sha256 a800905db54ae1e403f29dc49d7f784f99bf49eaf546a94b87596c47c04a60f0`) and kept `head`; its test asserts
`sv 0.2.0` for the release build, and its workflow now builds both the release and `--HEAD` on a Mac and on Linux, all
eight checks passing. Scoop: abbyshade111/scoop-stack-vet#1 moved `bucket/sv.json` to 0.2.0, with the hash above
checked against a fresh download of the zip and the zip's attestation verified (built by `release.yml` at
`refs/tags/v0.2.0`); its install workflow passed on Windows. `docs/INSTALL.md` now gives the plain `brew install`, with
`--HEAD` as the way to build `main`. Still not done here: packaging `cv`, and backlog 0120's tries by hand (Scoop on a
Windows computer, Homebrew on Linux).
