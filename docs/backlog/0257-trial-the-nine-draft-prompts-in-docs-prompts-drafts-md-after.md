# Trial the nine draft prompts in docs/prompts/drafts.md, after the git-history check (0258)

**Status:** partly done: group A (12 October 2026) and the second loop trial (12 and 13 October) gave no reading for drafts 8, 10, 12, 17, 18, 19, 20, and 22; the process drafts (1 to 7, 9, 14, 23) and those needing another brief (11, 13, 15, 16, 21) each wait on the owner's yes and a budget

The nine drafts in `docs/prompts/drafts.md` (design entry 0368, from the Cursor build) are written but not tested, and
none is served by `sv`. Each draft names what its trial would measure. The trial needs the owner's yes and a budget,
since it spends model money, and the library's rule (`docs/PROMPTS.md`) decides which drafts are promoted.

Run it after the noise work (the three items the other session adds on the high false-positive rate, which the owner
asked for on 11 October 2026), and after the check of the repository's own history (0248 in the git-history item, which is first on the roadmap), so
the trial can measure that check at the same time. Promote a draft to `data/prompts.json` only if its trial meets the
rule; keep the rest in the drafts file with what happened.

**12 October 2026:** the drafts are fourteen now, at the owner's word ("yes, add the five prompts to the drafts"): five more (10 to 14) from faults the Cursor build hit, each checked against its requirement's text. The other session's recommendations, when the owner has them, go in the same file before the trial.

**12 October 2026, later:** twenty-three now: nine more (15 to 23) recommended by the other session at the owner's request, each checked here against its requirement's text in `data/frameworks`, with the merges it suggested listed for the owner in the drafts file.

**12 October 2026, the owner's request: the trial's design must test the account changes of backlog 0232 group G (ADR-105, #1544).** They have run only against the scripted app; the trial is their first real test. Draft 17 (`password-before-account-changes`) measures them, and they run only when the build's `stackvet.toml` says how each change is made, so the design includes:

- At least one trial app that offers a phone number, two-factor sign-in, and a recovery address, each changeable while signed in, with a private page that shows each value.
- That app's `stackvet.toml` naming `change-phone`, `change-two-factor`, and `change-recovery`, each with its `shows` page, beside `change-email`; and `signup`, since every change is tried on an account made for it.
- One app that offers none of them, listing `not-offered`, so the trial sees both what an unconfirmed list does in the report (a gap, V7.5.1 in part) and what it does once the owner confirms it in `sv review` (V7.5.1 whole, saying which part rests on the owner's word).
- Measured with and without draft 17: whether each change refuses a wrong password, read from `probe.phone-change-without-password`, `probe.two-factor-change-without-password`, `probe.recovery-change-without-password`, and `probe.email-change-without-password`; how often a change reads as not assessed because the page writes the value differently or the change did not take, which says whether the `shows` design holds up on real apps; and whether the AI tool writes `not-offered` for a change the app does have.

**Scope of this claim (12 October 2026), the owner's choice the same day ("Group A first"):** the drafts the loop
trials' club brief (`docs/prompts/trial-3/plain-brief.md`) gives something to act on, with no new setup: 8, 18, 19, 20
and 22, and 10 and 12 if their checks fit the club app (to be confirmed in the protocol, before any build). The shape
of the library trial of 6 October (`docs/prompts/library-trial/`): ten builds without any draft on each of Haiku 5.5
(`claude-haiku-5-5`, never Haiku 4.5, at the owner's word the same day) and Sonnet 5.5 (`claude-sonnet-5-5`), shared by every draft, then ten builds for each draft on the model whose control builds show its problem; the
library's rule decides. About 80 builds, a budget of about $20 to $25 of the owner's API credit, approved the same
day. The protocol is written and merged before the first build (`docs/prompts/drafts-trial/protocol.md`). Drafts that
need a second brief (10's sign-up half, 11, 13, 15, 16, 17, 21), and the process drafts (1 to 7, 9, 14, 23), are not in
this claim; each is proposed to the owner with its own cost.

**Scope of this claim (12 October 2026, evening), the owner's choice after group A gave no reading ("both briefs, $50
budget, scripted review is fine if labeled"):** the second loop trial, which tests `sv`'s own waiting features on
real AI-built apps rather than prompts (ADR-109, proposed). Two briefs with `sv` attached through its MCP server: the
club brief corrected (`stackvet.toml`, the text forms and private pages listed, git allowed with signed commits), and
a new accounts brief (sign-up, reset, and the four account changes of 0232 group G, plus a form offering none, with
`not-offered` confirmed in `sv review` by a labeled script). Each build checked a second time with the outside tools
(0006 part 21, claimed with this), `SV_LOG` set, `sv` run twice per build for the history, and the build-loop record
and kept evidence read. Haiku 5.5 and Sonnet 5.5 by exact id; a budget of $50; nothing canceled for cost. The protocol
and its tools are written and merged before any build. Draft 17's own trial waits for the accounts brief's controls.

**Checked against `main`, 20 October 2026** (the backlog clean-up): `docs/prompts/drafts.md` now holds twenty-three drafts, not nine; ADR-109 is accepted (12 October); draft 17's trial ran in the second loop trial, with no reading, and `drafts.md` does not yet say so beside it. No draft is in `data/prompts.json`.
