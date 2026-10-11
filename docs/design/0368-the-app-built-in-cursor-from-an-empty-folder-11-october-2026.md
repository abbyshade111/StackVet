# The app built in Cursor, from an empty folder (11 October 2026)

Backlog 0222 asked what happens when someone builds an app with StackVet in a tool other than Claude Code.
The owner built one small app in Cursor, a home-lab tracker in Python with Flask and SQLite, from an empty folder,
with StackVet connected as `docs/GETTING-STARTED.md` section 3 describes. This entry records what the Cursor
agent did with StackVet, what the record does and does not show, and the two changes to the guide that the build
asked for.

## What the record is, and what it is not

The account below comes from the Cursor agent's own write-up of the session, which it produced from a transcript
that had been compacted once. The agent said so itself: its tool counts are mentions in that file, not an audited
log of calls. We did not read the Cursor transcript, and we did not read the app's code. What we checked directly
was the app's two merged pull requests. Their descriptions say 28 tests pass and that the StackVet static check
reported no findings, and the StackVet report the agent described as "0 needing attention" is not a statement that
the app is secure. The report itself says so.

Two points in the write-up do not agree with each other, and the owner should settle them before the record is
relied on:

- The write-up says the repository was pushed to "with branch protection", while the agent also says it refused to
  turn branch protection on. Who set it is not recorded.
- The write-up gives pytest's starting version as 8.3.3. The pull request that upgraded it says 8.3.5.

## What the agent did with StackVet

- **The spec.** Before any code, the agent wrote `stackvet.toml` from what the app would really do, and kept it
  current. Its settings for hosting on GitHub and for continuous integration were changed as the work moved, and
  each change altered which requirements applied. The agent reported that one such change surfaced a real finding
  about a checkout token left on disk.
- **The rules in `AGENTS.md`.** The agent read the rules at the start and called the guidance tool again for each
  area of work, before dependencies, before continuous integration, and before each feature. It ran the check after
  each feature and read the "not examined" part of each report before the findings.
- **The `stackvet_*` tools.** The agent's own counts, which are mentions in a compacted file, show the check, the
  questions, the spec, the guidance and the pre-run check used most. The agent also reported that it did not run
  `sv review` for the owner and did not touch the owner's sealed answers.
- **The prompts.** The record names the "shown to work" prompts for the spec, Git, secrets, headers, the production
  server, the window isolation, the security contact, passwords and cache rules, and says each left a visible
  artifact. We have not checked the artifacts.
- **Things the owner did.** Reviewing and merging were the owner's. The agent stopped at a push to `main` until the
  owner said to push, and it stated that the owner must decide on settings.

## What the build found

The record reports three real defects found by running the app, not by reading its code: a sign-up form that told
visitors whether an account existed, a crash on very long passwords, and a page that returned the password field.
It also reports that about 90 of the 95 findings from the full tool run were noise for this kind of app, and that
sorting them was most of the work. That count is the agent's judgment. We have not checked it.

## What differed from Claude Code

The record does not compare the two tools, and this build was not run in Claude Code, so this entry makes no
claim about a difference.

## What the guide needed

1. **Cursor was described as not tried.** Section 3 now says Cursor was tried once, on this build, with the same
   settings block.
2. **A real finding was recorded instead of fixed.** The agent first advised recording a real finding as a false
   alarm, and had to be corrected. `sv review` is for findings a person has checked and sets aside as false alarms,
   and for answers a person confirms. A real finding is fixed in the code and checked again. Section 5 now says so.

## Left open

- The owner is asking Cursor for the artifacts the record points to, so the claims above about the prompts and the
  spec can be checked against files.
- The agent reported that seed passwords showed up in terminal history during the build. It disclosed this itself.
  Whether the setup prompt should tell the tool to keep seed passwords out of shell history is a question for the
  owner, not yet a change.
