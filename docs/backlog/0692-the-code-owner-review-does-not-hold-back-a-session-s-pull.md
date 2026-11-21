# The code-owner review does not hold back a session's pull request, which is opened as the owner

**Status:** open

Filed 21 November 2026, at the owner's word, from what the owner found reviewing #2446: GitHub offered no way to
approve it. Sessions open pull requests through `gh` signed in as the owner (`abbyshade111`), so the owner is the
author, and GitHub never lets an author approve their own pull request.

**What the record says.** ADR-051's Later entry of 14 October 2026 (backlog 0290): a change to `.github/workflows/`,
`tools/ci_scope.py`, `tools/adr_check.py`, `tools/record_numbers.py`, or `action.yml`, "by a session or by the owner's
own tooling, waits for the owner's review before auto-merge takes it" (the "Main - ADR Check" ruleset's code-owner
review, with `.github/CODEOWNERS` naming the owner).

**What was seen.** Pull requests opened as the owner that changed those files merged with no review at all:
#2352 (`.github/workflows/long-files.yml`) and #2354 (`.github/workflows/rust.yml`). Only Dependabot's waited for the
owner's approval (#2408, #2396, #2258). So a pull request whose author is the code owner seems to count as reviewed by
the code owner, and the guard 0290 built holds back other people's pull requests, not a session's. Read from those
merges, not yet from GitHub's documentation: the first step is to confirm the rule there.

- **The choices, for the owner:**
  - Sessions open pull requests as a separate identity (a second account, or a GitHub App's token), so the owner is a
    reviewer of their work and the code-owner review holds. That changes how sessions sign in, adds a credential to
    look after, and has the identity need write access; ADR-092 reads `Assisted-by:` lines, which would not change.
  - Or the record is corrected to say what the rule holds back (pull requests from anyone but the owner's account),
    and the guard for a session's change to its own judge is left to something else, or to nothing.
- **The witness, whichever is chosen:** a pull request from a session that changes a file under
  `.github/workflows/` and is seen to wait for the owner's review (or, with the second choice, ADR-051 saying it does
  not).
- `.github/rulesets.json` (backlog 0690) records the rule as it is; a change to it goes through that file.
