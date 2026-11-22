# From the second trial of AgentVet: a retry is paired across sessions, latest lags a merge, and three smaller things

**Status:** partly done: part 4 is open, for a session on the owner's Mac or the owner (the workflow is a code-owned file)

From the owner's second trial of AgentVet on their own computer, 21 November 2026 (ADR-159, "As built", parts 9 and
10; backlog 0716). All five things tried passed: `av proxy --help`; `sv` in its container wrote a note outside the
app's folder through the `--notes` mount, and `av report` said the build-loop record passes through every place
noted; `av report --proxy-record` read the record; a tool outside `[may] call` was named; and `av checkpoint`, run by
the owner at a terminal, signed two records, which the next report said stand as signed. The trial also found these.

1. **A call made again after a no is paired across sessions, and the counts cannot be read for one session.** The
   pass-through's record has no line that says `av proxy` started, so `av report --proxy-record` matched a call an
   earlier session had turned down with the same call in a later session, 90 minutes apart, and said it was made
   again; and "6 calls" was two sessions' calls with no way to see either alone. Do: `av proxy` writes a line when it
   starts; the reader counts by run, lists a call made again only within one run, and says separately when the same
   turned-down call came back in a later run. A record from before this has no such line and is read as one run,
   said to be.
   **Part status:** done, 22 November 2026
2. **A stray comma and a double space in what a report says of a checkpoint's key.** "which this computer's list of
   trusted keys trusts for this app,  (a key on this computer with a passphrase))". Find where the two pieces are
   joined for `av report`, and check `sv`'s own reports do not say the same.
   **Part status:** done, 22 November 2026
3. **ADR-159 still says two things nothing had shown, and now the trial has.** Part 7 says `av checkpoint` was never
   run at a real terminal, and part 10 that no container had really written a note. Record the trial's results
   there, with what it did not cover (Colima on a Mac only; no Linux run, where the container's user matters).
   **Part status:** done, 22 November 2026
4. **`latest` lags a merge when `main` moves quickly.** The merge's image was published under its commit's tag, and
   `latest` was left to the next commit's run, which had not finished: `docker pull` of the image with no tag gave
   one older than the merge for some minutes. By design (backlog 0539), and it settles when the newer run
   publishes; the cost is that "merged and green" does not yet mean "`latest` has it". Worth a look at whether the
   newer run could be slow or skip the image (a pull request that changes only the backlog), which would leave
   `latest` behind for longer. The workflow is a code-owned file: for a session on the owner's Mac, or the owner.
   **Part status:** claimed by Claude Code session (packaging and ConfigVet), 21 November 2026
5. **`sv connect` prints the image with no tag, even when it was run from a tagged image.** Where the computer's
   `latest` is older than the image that printed the settings, the settings start an older `sv` than the one that
   wrote them; in the trial that would have dropped the `--notes` mount's use without a word. Whether `sv connect`
   should name the image it is running from, take an option for the image, or say beside the settings which image
   they start, is the owner's to choose: a pinned tag does not move when the person updates. **The owner chose, 22 November 2026:**
   keep the image untagged, and say beside the settings which image they start and to pull it first, so it is the
   newest ("go with your recommendation").
   **Part status:** done, 22 November 2026

Not faults, written down so they are not found again: a line of the notes file holds two hashes, the record's head
and the notes file's own chain, as designed; and the app's settings were back to plain Docker before the trial
because the first trial's last step put its dated copy back.
