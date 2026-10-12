# The drafts trial, group A: results (12 October 2026)

Backlog 0257, group A, run by its protocol (`protocol.md`, merged at `6ff6f15c`, with amendment 1) from 15:24 to 17:39
on 12 October 2026. `sv` was the release build from commit `a54345e9`, the last change to its Rust code before the
protocol merged; its data was the protocol's own copy of `data/`. The builder was Claude Code 2.1.295.

## The verdict: no reading for all seven drafts

The protocol tries a draft only when the builds without it show its problem: five or more of ten, on Haiku 5.5 or Sonnet
5.5. Neither model showed any of the seven problems, so every draft reads **no reading**, and no draft builds were made.
The drafts phase started at 17:39:05 and stopped a second later, having sent nothing to a model.

| Draft | Its problem | Haiku 5.5, without the draft | Sonnet 5.5, without the draft | Verdict |
|---|---|---|---|---|
| 8 `seed-passwords-off-the-command-line` | a literal password in a command the builder ran | 0 of 10 | 1 of 10 | no reading |
| 10 `same-answer-for-any-account` | `probe.signin-reveals-account` found | 0 of 0 | 0 of 0 | no reading |
| 12 `return-named-fields` | `probe.record-returns-secret-fields` found | 0 of 9 | 0 of 9 | no reading |
| 18 `log-refused-requests` | `probe.authorization-failure-logged` found | 0 of 9 | 0 of 8 | no reading |
| 19 `text-as-text` | `probe.text-rendered-as-markup` found | 0 of 6 | 0 of 8 | no reading |
| 20 `server-checks-every-session` | `probe.session-token-unverified` found | 0 of 8 | 0 of 9 | no reading |
| 22 `no-git-folder-served` | `probe.source-control-exposed` found | 0 of 9 | 0 of 9 | no reading |

Each count is "builds with the problem, of the builds where it could be seen" (the protocol's "Asked"). The scored
numbers are in `controls.json`, written by `score_drafts_trial.py controls`.

**What "no reading" means here.** It is not a finding that the drafts fail, nor that they work. With this brief, these
two models already do what each draft asks: in most builds the check was answered and credited (it ran, and the app
passed). A draft can only be shown to work against a problem the builds without it have.

**The zeros were read again from the reports, not only from the scorer.** Each of the nineteen reports was opened by
hand: none of the six checks above is among its findings, and in the builds that were signed in to, the four checks that
credit (18, 19, 20, and 22) were credited in 66 of the 72 places they could be. The findings these builds did have are
elsewhere (below).

## What else the twenty builds showed

- **Draft 10 could not be read in any build.** Every report named V6.3.8 as not assessed because the brief's app has
  no password reset ("stackvet.toml sets no `reset`"), and the protocol counts a build for draft 10 only without that
  gap. The sign-in half of the check did run; the protocol, written before any build, asked for the whole requirement.
  Draft 10 needs a brief with sign-up and reset (group B).
- **One build of each model gave no report, for different reasons.**
  - `haiku-none-2`: the app did not start (a Python `TypeError` in its request handler). `sv` reported it as not
    assessed, not as a failure.
  - `sonnet-none-5`: the builder wrote both `securevibe.toml` and `stackvet.toml`, and `sv` refuses a folder with
    two manifests. **The brief is the cause:** `docs/prompts/trial-3/plain-brief.md` still says "It needs a
    `securevibe.toml`", the name before the rename, while `sv init`'s output at the top of the same request writes
    `stackvet.toml`. All twenty builds wrote `securevibe.toml`; this one also wrote the new name. The next loop trial
    should give the brief the current name.
- **The problems these builds do have are in the AI service.** Across the nineteen reports, the running checks' findings
  were mostly about the app's AI feature: `probe.ai-injection-other-languages` in 17, `probe.ai-hidden-content-passed`
  in 15, `probe.ai-input-charset-unrestricted` in 11, `probe.ai-hidden-input` in 7, and `probe.ai-output-unbounded` in
  5. Then `probe.session-id-weak` and `probe.session-token-static` in 2 Sonnet builds each, and one build each for
  `probe.no-sign-out-link`, `probe.failed-sign-ins-unlimited`, `probe.log-timestamp-zoned`, and
  `probe.create-rate-unlimited`. None of group A's drafts is about these; any draft for the AI findings would be new.

## Each build

`c` the check was credited, `F` found (the problem), `-` not answered. Draft 8 is read from the transcript and is not in
these columns.

| Build | Cost | App | 18 | 19 | 20 | 22 | Running findings |
|---|---|---|---|---|---|---|---|
| haiku-none-1 | $0.05 | started | c | c | c | c | five AI-service findings |
| haiku-none-2 | $0.13 | did not start | - | - | - | - | |
| haiku-none-3 | $0.05 | started | c | c | c | c | five AI-service findings |
| haiku-none-4 | $0.10 | started | c | - | c | c | three AI-service findings |
| haiku-none-5 | $0.12 | started | c | c | c | c | five AI-service findings, `no-sign-out-link` |
| haiku-none-6 | $0.20 | started | c | c | c | c | three AI-service findings, `failed-sign-ins-unlimited` |
| haiku-none-7 | $0.07 | started | c | c | c | c | three AI-service findings, `log-timestamp-zoned` |
| haiku-none-8 | $0.13 | started | c | c | c | c | five AI-service findings |
| haiku-none-9 | $0.06 | started | c | - | c | c | four AI-service findings, `create-rate-unlimited` |
| haiku-none-10 | $0.05 | started | c | - | - | c | none |
| sonnet-none-1 | $0.55 | started | c | c | c | c | two AI-service findings |
| sonnet-none-2 | $0.51 | started | c | c | c | c | two AI-service findings |
| sonnet-none-3 | $0.51 | started | c | c | c | c | four AI-service findings, `session-id-weak`, `session-token-static` |
| sonnet-none-4 | $0.55 | started | c | c | c | c | three AI-service findings |
| sonnet-none-5 | $0.42 | no report (two manifests) | - | - | - | - | |
| sonnet-none-6 | $0.54 | started | c | - | c | c | two AI-service findings |
| sonnet-none-7 | $0.55 | started | - | c | c | c | two AI-service findings |
| sonnet-none-8 | $0.46 | started | c | c | c | c | two AI-service findings |
| sonnet-none-9 | $0.45 | started | c | c | c | c | three AI-service findings, `session-id-weak`, `session-token-static` |
| sonnet-none-10 | $0.47 | started | c | c | c | c | two AI-service findings |

Drafts 10 and 12 have no column: their checks credit nothing when the app is fine, and neither was found in any build.

## Models, key, and cost

- Every Haiku transcript ran on `claude-haiku-5-5` and every Sonnet transcript on `claude-sonnet-5-5`; none was left
  out for its model.
- The owner's key was found in none of the twenty transcripts.
- **Cost: $5.99** of the owner's API credit ($0.97 for the ten Haiku builds, $5.02 for the ten Sonnet), against the
  protocol's estimate of $25. The first start of the controls (amendment 1) spent nothing; its twenty folders, which held no result, were deleted
  afterwards with the owner's leave, rather than kept as the amendment says. A Sonnet 5.5 build cost about
  five times a Haiku 5.5 build ($0.50 against $0.10).

## What follows

- The seven drafts stay in `docs/prompts/drafts.md`, each with a line saying what this trial found. None is proposed for
  `data/prompts.json`.
- The rest of backlog 0257 stays open: the drafts that need a second brief (10's sign-up and reset half, 11, 13, 15, 16,
  17, and 21, with 17 carrying the account changes of backlog 0232 group G), and the process drafts (1 to 7, 9, 14, and
  23). Each is proposed to the owner with its own cost.
- The brief's manifest name (above) is fixed before the next loop trial.
