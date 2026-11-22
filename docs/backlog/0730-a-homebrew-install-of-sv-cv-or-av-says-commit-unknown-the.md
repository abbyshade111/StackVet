# A Homebrew install of sv, cv, or av says commit unknown: the formulas give the build no commit

**Status:** open

Found by the owner's first install of `av` by hand, 22 November 2026: `brew install abbyshade111/stackvet/av` built
v0.3.1 and `av --version` printed `av 0.3.1 (commit unknown)`.

**Why.** Each program's build script reads the commit from git, or from `SV_GIT_COMMIT` where there is no `.git`
(`crates/sv-cli/build.rs`, `crates/av-cli/build.rs`). Homebrew builds from the release's source archive, which has
no `.git`, and none of the tap's three formulas sets `SV_GIT_COMMIT`. So `sv` and `cv` installed with Homebrew very
likely say the same; only `av`'s was seen.

**What it costs.** A person reporting a fault cannot say which commit their program is, and a report `sv` writes
names its commit as `unknown`. The version is right, and for a release the version names one commit, so this is a
gap in what is said, not in what was built.

**What to do.** In the tap (`abbyshade111/homebrew-stackvet`, another repository: a session on the owner's Mac),
each formula can give the build the release's commit, which the formula knows when it is written for a tag: set
`SV_GIT_COMMIT` to the tagged commit's hash beside the `url` and `sha256`, and have each formula's test hold that
`--version` names it. A `--HEAD` build has a checkout and already says its commit. Check what the Scoop zips say: the
release workflow builds them in a checkout, so they should already name the commit.

`docs/INSTALL.md` says today that a Homebrew install says `commit unknown`; that sentence goes when this is done.
