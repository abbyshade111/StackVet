# Leave Bandit's assert and hardcoded-string rules out of test files, with the reason shown (proposal for the owner's approval)

**Status:** open

From a real report (an app built in Cursor with `sv`, 11 October 2026): 89 findings, 79 of them in one test file. 62
are Bandit's "use of assert" (B101) and 14 are its "hardcoded password string" (B105), all in the test file. Asserts
are how tests are written, so the first is noise for test code; the second is probably test fixtures, but that has to
be checked, not assumed.

**Proposal, for the owner to approve before anything is built** (this is the decision record's proposal; it becomes a
record, `Status: proposed`, when the change is built):

- A file is test code when its path has a `tests` folder, or its name starts with `test_` or ends `_test.py`, `_test.go`,
  or `.test.js`.
- In test code, Bandit's B101 (assert) and B105, B106, and B107 (hardcoded password or credential strings) are not
  reported as findings. The report says so in one line per rule, with the count, and names the folder.
- The secrets scan (`sv`'s own, which looks for real key shapes) still runs on test files, and a real-looking key in a
  test file is still a finding. That is the guard against a real credential being waved through as a fixture.
- Everything else Bandit reports in test code, and everything it reports in app code, is unchanged.

Alternatives considered: leave it as it is (the report stays noisy, and a person has to read 79 lines to find the
few that matter); downgrade these to "info" (still counted, still in the findings list, so the list stays long);
exclude test folders altogether (would also hide a real key in a test file, which the proposal keeps visible).

What changes for the owner: the finding count drops, and the report's counts change; it changes what counts as evidence,
so it needs the owner's yes, and a decision record in the pull request that builds it. Proposed test: a fixture test
file holding an `assert` and a fixture password is quiet; the same code in app code still fires; a real-looking key in a
test file still fires (break the rule, and check each of these fails).
