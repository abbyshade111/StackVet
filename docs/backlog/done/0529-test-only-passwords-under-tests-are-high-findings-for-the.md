# Test-only passwords under `tests/` are high findings for the secrets check while Bandit's hits on the same lines are test information

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Cursor app. Severity: low (inconsistent, and
noise a non-programmer will fret over). Verified by the orchestrating session.

**What is wrong.** `tests/conftest.py:9` `PASSWORD = "violet-canoe-marble-ledger"` and `tests/test_auth.py:76` are in
`findings` as high (`secrets.credential-assignment`, SBD-AC-05 "needs attention"), while `bandit.B105` on lines of the
same test files is in `test_information` (low). One check treats `tests/` as the app and another does not.

**Fix.** Apply the test-code marking the outside tools get to the secrets check: a credential-looking value in a test
file is information with the words "in a test file; a test password reused elsewhere is still a risk", not a finding
against V13.3.1 or SBD-AC-05. A value that looks like a real key (a provider's prefix) stays a finding wherever it is.

**Record.** The secrets check's record: a Later entry, since what counts as a finding changes.

**Done, 15 October 2026** (documentation session), as the owner answered, (b). A credential assignment in a test file
whose value is made of words, such as `violet-canoe-marble-ledger`, is test information, listed in the report's own
section and not counted, with the words "a test password reused elsewhere is still a risk" in its description. A
random-looking value in a test file still counts (ADR-091), and so does the same passphrase in the app's own code. The
finding's title, and so its fingerprint, are unchanged. ADR-091 has a dated "Later" entry. Tests:
`crates/sv-check/src/secrets/test_passphrase_tests.rs` (5).
