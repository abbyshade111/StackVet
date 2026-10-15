# Test-only passwords under `tests/` are high findings for the secrets check while Bandit's hits on the same lines are test information

**Status:** claimed by documentation, 14 October 2026

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

**The owner's answer, 15 October 2026 (asked through the board's waiting line):** (b): only values that read as words or passphrases (like violet-canoe-marble-ledger) become test information; a random-looking value in a test file still counts as a finding, as the ADR-091 ruling of 11 October says. A Later entry on ADR-091 with this line.
