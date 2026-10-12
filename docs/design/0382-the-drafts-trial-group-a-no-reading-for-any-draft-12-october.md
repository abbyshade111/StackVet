# The drafts trial, group A: no reading for any draft (12 October 2026)


Backlog 0257, group A: seven of the draft prompts in `docs/prompts/drafts.md` (8, 10, 12, 18, 19, 20, and 22), tried by
the prompt library's rule on the loop trials' club brief, with ten builds without any draft on each of Haiku 5.5 and
Sonnet 5.5 first (`docs/prompts/drafts-trial/protocol.md`). The full account is `docs/prompts/drafts-trial/results.md`.

**What happened.** Neither model showed any draft's problem in its builds without the draft (at most 1 of 10, draft 8 on
Sonnet), so every draft reads "no reading" and none of the seventy draft builds was made. The twenty control builds
cost $5.99 of the $25 estimate. The running checks these drafts are for were answered and credited in 66 of the 72
places they could be.

**What it says.** The 5.5 models, given this brief, already avoid what these drafts warn about. A draft is worth a trial
only where the builds without it go wrong, so the controls come first, and they are cheap: here they spent a quarter of
the estimate and saved the rest. The builds' findings were in the app's AI feature (input in other languages, hidden
content, unbounded output), which no draft yet addresses.

**Two faults in the trial's own setup, for the next one.** The brief (`docs/prompts/trial-3/plain-brief.md`) still names
the manifest `securevibe.toml`, the name before the rename, while `sv init`'s output in the same request writes
`stackvet.toml`; one build wrote both and `sv` refused it. And the club app has no password reset, so draft 10's check
(V6.3.8) could not be read in any build.
