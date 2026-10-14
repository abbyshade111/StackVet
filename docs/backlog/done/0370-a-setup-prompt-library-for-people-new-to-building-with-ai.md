# A setup prompt library for people new to building with AI

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Asked for by the owner on 12 October 2026, from the website's words (backlog 0224, `docs/website/home-page.md`,
section 7). The site's "Get started" has two paths, and the first, for someone who has used an AI chat but never
built an app with one, points at **setup prompts**: each is copied into an AI coding tool that can run commands, and
the tool does the setup with the person, checks what the computer already has, says what to install and where to get
it, asks before it installs or changes anything, and stops and says what happened in plain words when a step fails.

Today there is one prompt, `docs/prompts/setup.md`, not yet tried in any AI tool, and it stops on Windows ("StackVet
has not been tried on Windows yet"), which is out of date since Docker and Scoop work there (backlog 0120). To build:
the library, a prompt per step where one prompt is too much at once, Windows taken in; then **the owner tests it**,
and the tools it really worked in are the ones the site names. Until then the site's line naming them stays blank.

**The owner's plan, 14 October 2026:** when this library is finalized, the owner tries it in Antigravity, Cursor, and VS Code (backlog 0502, widened to the three), so the library's "Tried so far" rests on all three before the website names the tools it works in. Finalize first; the trial follows.

**Claim released, 14 October 2026** (the owner's word): session securevibe-trial-prep was archived after the coordination trial with no branch pushed for this item, so the claim was stale. Open again, queued on the board.

**Done, 14 October 2026: finalized for the owner's trial.** The library was already built by the time this was taken
up: seven prompts in `docs/prompts/setup.md`, one per step (look, install, set up, check, build, the full report,
and recording the answers as the owner's), with a Windows version of the setup prompt using Scoop (backlogs 0407,
0474). This reading checked every command, tool name, and page the prompts cite against `main`: the `stackvet_`
tools, `stackvet_check`'s `questions` section, `docs/INSTALL.md`'s section for `--run`, `brew upgrade --fetch-HEAD
sv`, `scoop update sv`, and `sv review`. Three lines said more than the code does, and were corrected: the full report
signs in as test users only when `stackvet.toml` names them (said twice); and `sv review`'s key takes a passphrase
if the person wants one, and the review runs only in a terminal the person types in. A paragraph after "Tried so far" now
names the next step, the owner's trial in Antigravity, Cursor, and VS Code (backlog 0502). The prompt tests
(`setup_prompts.rs`, `setup_prompt_seven.rs`, `connect.rs`) pass.

Not done here: what prompt 6 says of CodeQL waits on backlog 0415 (honesty holds it), and what the Windows prompt says
of the checks that start the app is backlog 0510, next on this line. The site's line naming the tools stays blank
until the trial.
