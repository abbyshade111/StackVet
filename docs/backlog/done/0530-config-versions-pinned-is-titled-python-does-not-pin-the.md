# `config.versions-pinned` is titled "Python does not pin the versions it installs" when every version is pinned

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), all three apps. Severity: low (wording).

**What is wrong.** Each app's `requirements.txt` pins every line with `==`; the finding's text says the problem is the
missing lockfile with hashes. The title contradicts the text, and a person reading only titles is misled.

**Fix.** Retitle to "No lockfile with hashes beside requirements.txt" (and the matching title for each ecosystem), keep
the text.

**Record.** None.

**The owner's answer, 15 October 2026 (asked through the board's waiting line):** A: rename the finding, and also match what was recorded under the old title (accepted risks, reviews, and --baseline), held by a test.

**Done, 15 October 2026.** The pinning finding is now titled by what is missing (`title_for_unpinned` in
`crates/sv-check/src/config.rs`): "No Python lockfile beside `requirements.txt`" when there is no lockfile, each
ecosystem named once with the files it has none beside ("No Python lockfile beside `requirements.txt` or `worker/setup.cfg`,
and no npm lockfile beside `web/package.json`"). A requirements file that does not pin and hash every line keeps a title
of its own ("`requirements-dev.txt` does not pin and hash every package it lists"), and a version range keeps "does not
pin the versions it installs", which is then true; the text under each title is unchanged. The tests are in
`crates/sv-check/src/config_pin_title_tests.rs`.

A config finding's fingerprint is made from its title, so the finding also carries the fingerprint its old title gave it
(`earlier_fingerprints`), which `fill_fingerprints` in `crates/sv-check/src/review.rs` now keeps beside the one it
works out. `apply` matches an entry in `stackvet.toml` or `security-notes.md` on either, for the rule that reported the
finding, and `--baseline` already reads `earlier_fingerprints` (`crates/sv-cli/src/baseline.rs`). The tests, in
`crates/sv-check/src/review_renamed_title_tests.rs`: a sealed accepted risk recorded under "Python does not pin the
versions it installs" still counts for the renamed finding; one recorded under the new title counts; one under another
title does not (the control). Breaking each of the three pieces (the carried fingerprint, keeping it, and matching on
it) turned the first test red, and nothing else.
