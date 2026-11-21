# The code-owner review does not hold back a session's pull request, which is opened as the owner

**Status:** done, 21 November 2026

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

**The owner's choices, 21 November 2026, asked with a recommendation for each:** a separate identity for sessions,
not a corrected record; and a GitHub App, not a second account. The decision is ADR-158, proposed in this claim and
accepted in the pull request that builds it. The owner creates the App and keeps its key (step 1 below); the build is
the rest.

- **The owner:** create the App with the permissions ADR-158 lists, install it on the three repositories, make its
   private key, and put the key where ADR-158 says. A session never sees the key.
- **The build:** the script in `tools/` that makes an hour's token from the key; CLAUDE.md's "Git is pre-approved"
   saying sessions open pull requests and push with it; ADR-051's 14 October entry corrected by a Later entry; the
   witness of ADR-158; and what cloud sessions use, said.

**Done, 21 November 2026** (ADR-158, accepted, with its "As built"; ADR-051, Later). The owner made the App
(`stackvet-sessions`, App ID 5862353), installed it on the three repositories, and put its key in the keychain;
`python3 -I tools/session_token.py check` read, with the App's own tokens, that each reaches its repository alone with
contents, pull requests, and workflows to write and metadata to read. CLAUDE.md says sessions push and open pull
requests through the script, and stop without a token. **Witnessed:** #2451, pushed and opened through the script,
changes one comment in `.github/workflows/long-files.yml`; its author is `app/stackvet-sessions`, and GitHub asked
`abbyshade111` for a code owner's review at once, which #2446, opened under the owner's name, never was. It is closed
without merging. The pull request that built this was opened by the App too. Not looked at: cloud sessions, which
open pull requests through the cloud's own GitHub connection.
