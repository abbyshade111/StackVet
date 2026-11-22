# cv --version names the commit it was built from, as sv and av do

**Status:** open

Filed 22 November 2026, at the owner's word. Found while building backlog 0730: `sv --version` and `av --version` name
their commit (`crates/sv-cli/build.rs`, `crates/av-cli/build.rs`, which read `SV_GIT_COMMIT` where there is no `.git`),
but `cv --version` prints its version alone (`crates/cv-cli/src/main.rs`), so a person reporting a fault in `cv`
cannot say which commit it is.

- **To do:** a build script for `cv-cli` as `av-cli`'s is, and `cv --version` printing `cv <version> (commit <hash>)`;
  then the tap's `Formula/cv.rb` sets `SV_GIT_COMMIT` and its test holds it, as `sv.rb` and `av.rb` do since 0730 (the
  tap is another repository: a session on the owner's Mac). The release workflow's Windows zip is built in a checkout,
  so it should name the commit already; check.
- **Check:** a test that `cv --version` names a commit, and the tap's install jobs printing it.
