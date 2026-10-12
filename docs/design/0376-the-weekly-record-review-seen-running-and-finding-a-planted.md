# The weekly record review, seen running and finding a planted gap (12 October 2026)



Backlog 0223 asked for the scheduled weekly review of the decision records to be watched running from start to finish,
and to be seen finding a gap planted for it. It was: the routine's first firing after the plant, Monday 12 October 2026,
8:45 Eastern.

**The plant.** On 9 October (#1261, session securevibe-e9) ADR-064 was changed to say the test tool holds its answer for
25 seconds. The stand-in has always held 40 (`HANG_SECONDS` in `crates/sv-run/assets/model-provider.mjs`, `HANG_HOLD` in
`sv_check::ai`). Its commit was titled to say it was a plant. The session read the record's own history with `git log`
for that line to see how late it came, and the plant's commit title is how the helper confirmed the 25 was not original.

**What the routine did.**
- It claimed the week first, in a pull request of its own that merged before any write (backlog 0266). 0091, where the
  prompt says to claim, was already marked done on 10 October, so the claim went in a new item instead; the prompt should
  say "a new item for the week" rather than "under that entry".
- It read every record, ADR-015 to ADR-098 (ADR-099 merged during the run), with six read-only helper agents, one per
  group of records. **The helper reading ADR-053 to ADR-066 found the plant** in its first pass over ADR-064, citing the
  code line and the plant's commit. Two other helpers had no part in it. So the plant was found by reading the code
  beside the record, not by luck of order, and by one group, not several; a second planted gap in another group was not
  tried.
- Besides the plant it found 24 genuine differences (counts that moved, credits that became "in part", tests that
  moved, an index that said "proposed"), two decisions with no record that it wrote (ADR-100, ADR-101), and it counted
  the week's "unchanged, because" lines (419 pull requests, 1,642 lines).

**What fell short.**
- The clone was shallow (274 commits), which would have shown about 76 merges for the week instead of 778 and made the
  lag figures impossible; the prompt should say to deepen it (`git fetch --shallow-since=<date> origin main`).
- The "unchanged, because" count needs the pull request descriptions, which a git clone does not hold; only a session
  with the GitHub tools can do it, and the figure is a search, not a count.
- A helper reading 15 records at a time reports a summary, not the line it read; the claims acted on were checked against
  the code by the session (ADR-064's, ADR-020's `unsafe` count). The rest rest on the helpers' reading.
- `cargo test --workspace` stops at the first failing crate; the prompt should say `--no-fail-fast`. One test needs
  `py7zr` (ADR-089) and failed in the session, which has no such package; CI installs it.

**Taking the plant out.** ADR-064 now carries a dated "Later" entry saying the figure is 40 and why the 25 was there; the
text above it is not rewritten, as the rule for records says. The plant is out.
