# config.secrets-file-committed calls an .npmrc critical by its name alone, though most hold only settings

**Status:** done, 20 October 2026

Filed from `sv`'s self-report of 16 October 2026 (`sv report .` with the outside tools on, at `main` `28aae3a2`).
Severity: medium (a false alarm at critical severity, high confidence).

**What is wrong.** `secrets_file_committed` decides from a file's name alone (`is_secret_file`, `SECRET_FILES`), and
`.npmrc` is on the list beside `.env` and `id_rsa`. An `.npmrc` usually holds only npm's settings; it holds a
credential only when it carries an `_authToken`, `_auth`, or `_password` line. The finding the check writes says, at
critical severity and high confidence, "A file that holds credentials was committed", and its fix says to change
"every credential that was in the file", whether or not there was one.

**Where it showed.** The self-report's one critical finding outside test code: `.npmrc`, in the history from v1. That
file, read at `v1:.npmrc`, holds five settings (`ignore-scripts`, `min-release-age`, and three that turn notices off)
and no credential. The finding says 6 other files were committed too; this filing did not read them, and they want
the same look before this item is closed.

**Where.** `crates/sv-check/src/config.rs`: `SECRET_FILES` and `is_secret_file` (near line 136), used by
`secrets_file_committed` (near line 218) for the tracked files and, through `crate::git::ever_added`, for the
history; the finding itself is `in_history_finding` (near line 412).

**What to do.** For the files whose job is not always to hold a credential (`.npmrc`, `.pypirc`, `.netrc` are the
likely ones), read the file, from the working tree or from the commit that held it, and report it only when it has a
credential line, with the line named and the value cut short as `Secret::redact` does. A file that cannot be read
from the history stays a finding, said as one whose content was not read. Files whose only job is a credential
(`.env`, `id_rsa`, `master.key`) keep going by name.

**Before building, check:** the credit side. V13.3.1 is credited in part from this check (`config.rs`, the
`in_part_if` near line 71, backlog 0331); reading contents must not make the credit claim more than it read.
**Break it and watch:** a test with an `.npmrc` that holds an `_authToken` line built from pieces at run time must
still fail after the change, and one that holds only `ignore-scripts=true` must not.

**Checked against `main`, 20 October 2026** (the backlog clean-up): still nothing built (`SECRET_FILES` and the name-only `is_secret_file` in `crates/sv-check/src/config.rs`). The six other files, read by their keys only: two `.npmrc` that hold settings alone, and four of v1's test fixture `.env` files, three with secret-named keys, which stay findings under the fix.

**Built, 20 October 2026.** `.npmrc`, `.pypirc`, and `.netrc` (`CREDENTIAL_WHEN_A_LINE_SAYS`, `crates/sv-check/src/config.rs`)
are read before they are reported: every version ever committed, on any branch (`git::every_version`: `git log --raw`
under `ever_added`'s overrides, then `git cat-file blob`, which runs no driver the repository names, ADR-032). One is
a finding only when a version has a credential line (`credential_line`: npm's `_authToken`, `_auth`, `_password`, a
`.pypirc`'s `password`, a `.netrc`'s `password`; a value that only names an environment variable, `${NPM_TOKEN}`, is
not one), and the finding names the line and the setting, with the value cut short by `Secret::redact`. A version git
could not read stays a finding, at medium confidence, saying so. Every other secrets file still goes by its name. The
credit is unchanged in kind: a pass is still *checked in part*, now over files `sv` read. On `sv`'s own repository the
critical finding is no longer `v1`'s `.npmrc` but one of v1's test-fixture `.env` files, which stays a finding.
Held by `crates/sv-check/src/config_npmrc_tests.rs` (four tests; broken on purpose two ways: by name alone, and no
line found, each failing them).
