# A cloud session's pull request that changes a code-owned file is reopened as the sessions' App

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word, from the security review of 21 November (finding A6, its fifth point;
the gap ADR-158 leaves open). A cloud session opens pull requests through the owner's own GitHub connection, so GitHub
counts the owner's review as given and the code-owner review never asks for it. GitHub cannot tell the session from
the owner, so no rule about who wrote a change can; the owner chose, on 21 November 2026, a rule plus a check that
catches a session's mistake, with the gates on publishing, releasing, and deploying (backlog 0704, 0711, and 0712) as
what holds whoever the author is.

- **To build:** CLAUDE.md says a cloud session does not open a pull request that changes a file `.github/CODEOWNERS`
  names: it pushes its branch, and a session on the owner's Mac opens the pull request as the App. The required
  "Decision records" check fails a pull request from a `claude/` branch, opened by the owner's account, that changes
  a code-owned file, naming the files and saying to reopen it as the App. Its script is code-owned, reads CODEOWNERS's
  own lines, and refuses a pattern it cannot match exactly rather than skipping it.
- **What it does not do:** stop a session set on getting around it (another branch name would); say so in the record.
