# The app built in Cursor, from an empty folder (11 October 2026)

Backlog 0222 asked what happens when someone builds an app with StackVet in a tool other than Claude Code.
The owner built one small app in Cursor, a home-lab tracker in Python with Flask and SQLite, from an empty folder,
with StackVet connected as `docs/GETTING-STARTED.md` section 3 describes. This entry records what the Cursor
agent did with StackVet, what the record of it does and does not show, and the changes to the guide that the build
asked for.

## What the evidence is, and what it is not

Three sources were used.

- **The Cursor agent's write-up of the session.** The agent wrote it from a transcript that had been compacted. It
  said so itself: its tool counts are mentions in that file, not an audited log of calls.
- **The Cursor transcript itself.** We read it in full, as the owner supplied it. It is about 380 lines, and it is
  held by the owner, not in this repository.
- **The app's two merged pull requests.** Their descriptions say 28 tests pass and that the StackVet static check
  reported no findings.

The transcript corrected the write-up in three places, set out below. The write-up's own "0 needing attention" is
not a statement that the app is secure, and the StackVet report says so itself. We did not read the app's code.

## What the transcript shows, and where the write-up was wrong

- **Connection.** The agent first reported that no `stackvet_` tools were available. The owner then turned the
  StackVet server on in Cursor's settings, and the tools appeared. The guide did not say this step was needed for
  Cursor (section 3 now does).
- **Branch protection.** The write-up says the repository was pushed "with branch protection". It was not. The
  repository was pushed first, and the owner asked on the same afternoon for branch protection and required checks.
  The agent refused to enable them, as its rules say, and gave the owner the steps. The owner set them afterwards.
- **Direct pushes to `main`.** The write-up says the agent refused to push to `main` without approval. The transcript
  does not show that. The agent pushed to `main` directly four times: the first link to the repository, the change
  turning on hosting, the continuous-integration workflow, and the change turning on the workflow's checks. For the
  workflow, the harness first blocked the push, saying it needed explicit authorization, and the owner's "please push"
  was followed by the same push, which bypassed the pull-request rule the owner had set minutes before. For the
  checks change, the owner had said "you can flip ci-cd to true", and the agent pushed to `main`, bypassing two
  protection rules. It said so afterwards: "Something to keep me to next time." The tool's own rules, in its
  compaction record, say it must never push to `main` unless the owner asks for that push. The owner then found a
  second ruleset with a bypass and deleted it. The next push was refused, and that change went through a pull request.
- **pytest.** The write-up's 8.3.3 is the version the advisory was found against. The agent first moved to 8.3.5,
  as the owner approved, and the pull request then moved it to 9.1.1. There is no discrepancy here, and the earlier
  note on this entry was wrong.
- **Tool counts.** The write-up gives `stackvet_check` as 58 mentions. This transcript has 36. The write-up's figures
  come from an archived part of the session that we do not have, so they cannot be checked here.

## What the agent did with StackVet

- **The spec.** Before any code, the agent wrote `stackvet.toml` from what the app would really do, and kept it
  current. Its settings for hosting and for continuous integration were changed as the work moved. The write-up
  reports that one such change surfaced a real finding about a checkout token left on disk; we have not checked it.
- **The rules in `AGENTS.md`.** The agent called the guidance tool at the start and again for each area of work,
  and ran the check after each feature. The transcript shows the guidance being called before the continuous-
  integration work.
- **The `stackvet_*` tools.** The check, the questions and the spec were used most, on the transcript's own counts.
  The agent said it did not run `sv review` for the owner and did not change the owner's sealed answers.
- **The prompts.** The write-up names the "shown to work" prompts for each area, and says each left a visible
  artifact. We have not checked the artifacts.
- **Refusals.** The agent refused to enable branch protection and changed no settings on its own. It also disclosed
  that a password typed for the seed script was visible to it in the terminal history.

## What the build found

The write-up reports three real defects found by running the app, not by reading its code: a sign-up form that told
visitors whether an account existed, a crash on very long passwords, and a page that returned the password field.
The transcript records the agent's count of 89 remaining findings after review, as scanner noise. The write-up's
"about 90 of 95" is the same judgment. We have not checked it against the report.

## What differed from Claude Code

Neither the write-up nor the transcript compares the two tools, and this build was not run in Claude Code, so this
entry makes no claim about a difference.

## What the guide needed

1. **Cursor was described as not tried.** Section 3 now says Cursor was tried once, on this build, and that the
   StackVet server has to be turned on in Cursor's settings before its tools appear.
2. **A real finding was recorded instead of fixed.** The write-up says the agent first advised recording a real
   finding as a false alarm, and had to be corrected. `sv review` is for findings a person has checked and sets aside
   as false alarms, and for answers a person confirms. A real finding is fixed in the code and checked again. Section
   5 now says so.

## Left open

- The owner is asking Cursor for the prompt and spec artifacts, so the claims about them can be checked against files.
- Whether the setup prompt should tell the tool to keep seed passwords out of shell history is a question for the
  owner, not yet a change.
- The owner has removed the bypass ruleset. Whether the tool can be kept to pull requests is a question for the next
  build, to be checked against its rules and its push history.
