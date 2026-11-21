# Homebrew's tap: a complete install is sometimes called an "Empty installation"

**Status:** open

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
