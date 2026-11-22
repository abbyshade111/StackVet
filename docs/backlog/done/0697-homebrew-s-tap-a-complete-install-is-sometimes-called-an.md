# Homebrew's tap: a complete install is sometimes called an "Empty installation"

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word, after the second time. Work in `abbyshade111/homebrew-stackvet`'s
`.github/workflows/test.yml`, not this repository.

**What happened, twice:**

- **9 October 2026:** a Linux run of the `install` job stopped with Homebrew's "Empty installation", and the rerun of
  the same commit passed. The log could not be read where it was looked at, so the workflow gained its "What was
  installed, when anything failed" step.
- **21 November 2026:** `install (macos-latest, head)`, on the push run of the tap's PR #5 (run 44921228898).
  `cargo build` finished ("Finished `release` profile ... in 3m 37s"), then Homebrew stopped with
  `abbyshade111/stackvet/sv: Empty installation`. The diagnostic step showed the install complete: in
  `Cellar/sv/HEAD-2dfff8b`, `libexec/sv` (43,930,832 bytes), `libexec/data/`, and `bin/sv` linking to
  `../libexec/sv`, with 93 GB free on the disk. The same job on the same commit's `pull_request` run passed, as did the
  other ten jobs. Only that job was rerun.

So Homebrew's own check called a full folder empty, on both systems, in a `--HEAD` build both times that it is known.
Each time it blocks a merge until someone reads the log and reruns.

**What to do:**

- **Find why, if it can be found.** Homebrew decides "Empty installation" in its own code (search the brew source for
  the message); read what it counts, and whether a `--HEAD` build or a link in `bin` can make it count nothing.
  If a cause in the formula shows, fix it there.
- **Otherwise, one retry that says so.** On exactly that message, uninstall and install once more, and print a line
  that the first attempt was called empty, with the folder's listing, so a run that passes on the retry still shows it
  happened. Any other failure is not retried.
- **Report it to Homebrew** if it reproduces on demand.
- **The witness:** a step that fails with that message once is retried and the job passes, saying so; a step that
  fails with any other message is not retried.

**Done, 21 November 2026** (in `abbyshade111/homebrew-stackvet`, its PR #6): **the cause was found.** For a `--HEAD`
build Homebrew names the version from GitHub's latest commit on `main` when it starts, the build asks GitHub again
and installs under that answer, and the check after the build looks for the first name (`formula_installer.rb`
raises "Empty installation" after `formula.update_head_version`; `GitHubGitDownloadStrategy#last_commit` asks the
GitHub API). In the failed attempt of run 44921228898, Homebrew named the build `HEAD-e7a0a6b` at 15:51:22 UTC,
`2dfff8b` merged into `main` at 15:52:02, the build step started at 15:52:11 and installed into `HEAD-2dfff8b`, and
the check found no `HEAD-e7a0a6b`. The rerun passed because `main` did not move during it. The formula is not at fault.

- **The retry that says so:** `.github/scripts/install-sv.sh` retries a `--HEAD` install once, only when the log says
  "Empty installation", the named folder does not exist, and another HEAD folder holds the install, with a warning and
  the folder's listing. Any other failure, a release build, or a retry that fails again fails the step.
- **The witness:** `.github/scripts/test-install-sv.sh` plays six cases with a stand-in for `brew`; the `install` job
  runs it first. Breaking each of the script's five guards made one of its cases fail (at first three breaks went
  unseen, because the refused cases were refused for two reasons each; each now differs from the race in one way).
  On the pull request's own run the tests passed on all four install jobs, on a Mac and on Linux.
- **Not done:** reporting it to Homebrew, which would post publicly; the owner's to decide.
