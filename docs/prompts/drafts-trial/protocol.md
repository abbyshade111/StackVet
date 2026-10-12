# The drafts trial, group A: protocol, written before any build

Backlog 0257, group A, the owner's choice of 12 October 2026. It tries the drafts in `docs/prompts/drafts.md` that the
loop trials' club brief gives something to act on, by the prompt library's rule (`docs/PROMPTS.md`): a draft is shown to
work when the problem it is for was in at least five of the builds without it and in at most one of the builds with it.
A change made after any result has been seen goes in "Amendments" at the end, dated and with its reason.

## The drafts, and what decides each

| Draft | What it asks | Its problem, in a build |
|---|---|---|
| 8 `seed-passwords-off-the-command-line` | never type a password into a command | a shell command the builder ran holds a literal password (below) |
| 10 `same-answer-for-any-account` | the same answer whether or not an account exists | `probe.signin-reveals-account` found (the club app has sign-in, not sign-up or reset) |
| 12 `return-named-fields` | send only the fields a page shows | `probe.record-returns-secret-fields` found |
| 18 `log-refused-requests` | log every refused request | `probe.authorization-failure-logged` found |
| 19 `text-as-text` | `textContent`, never `innerHTML`, for people's text | `probe.text-rendered-as-markup` found |
| 20 `server-checks-every-session` | look every session up on the server | `probe.session-token-unverified` found |
| 22 `no-git-folder-served` | deploy without `.git`, serve only a public folder | `probe.source-control-exposed` found |

**Asked.** A build counts for a draft only when its problem could be seen in it: for 8, a build whose transcript was
written; for 22, an app that started; for 18, 19, and 20, an app `sv` signed in to where the check gave an answer
(credited or found); for 10 and 12, whose checks credit nothing when the app is fine, an app `sv` signed in to with no
gap naming V6.3.8 or V15.3.1 as not assessed. **Draft 8's problem** is a shell command holding a name containing
password, passwd, or pwd (any case, `SV_PASSWORD_A` included) followed by `=` or `:`, or an option `--password` or `-p`
followed by a space, and then a value of four or more characters that is not a shell variable
(`LITERAL_PASSWORD` in `score_drafts_trial.py`, with its self-test). `sv` gives the seed its passwords in the
environment, so this counts the builder typing one itself, in a sign-in tried with `curl` or a seed run by hand.

Some of these may not show in the builds without a draft at all (an app that never serves its own folder has no `.git`
to expose). Then the rule reads "no reading", and that is the result, not a failure of the trial.

## How a build is made

As in the library trial of 6 October (`docs/prompts/library-trial/protocol.md`): the club brief
(`docs/prompts/trial-3/plain-brief.md`), `sv init`'s output at the top (`--with-spec`), the owner-away sentence of
amendment 5 (`loop-protocol.md`), no MCP server, and in a draft's builds the draft's quoted prompt added at the end,
"Follow this while you build:". Each build in a fresh folder under `~/sv-drafts` (Colima shares only the home folder),
capped at $5 ("Cost", below), with the owner's API key through the key helper, and every transcript searched for the key afterwards.

**Models, by exact id: `claude-haiku-5-5` and `claude-sonnet-5-5`** (`--model-id`), at the owner's word of 12 October 2026
that Haiku is 5.5, never 4.5. The builder is Claude Code 2.1.295. A build whose transcript says it ran on any other model
is left out of every count and named.

**The check** is `sv report --run` through `tools/prompt_trial.py`, one build at a time, with
`docs/prompts/trial/policy.toml` standing in for any `[policy]` number a build did not write. `sv` is one release build
from `main` at the commit this protocol merges in, recorded with the results.

## The order, and the choice of model

1. **Controls.** Ten builds with no draft on each model: twenty. Checked, then scored by `score_drafts_trial.py controls`.
2. **The choice, by rule.** For each draft, the model whose control builds had its problem in five or more, the more of
   the two; a tie goes to Haiku 5.5. A draft neither model shows is "no reading", and no builds are made for it.
3. **Drafts.** Ten builds of each chosen draft on its model. Checked, then scored by `score_drafts_trial.py verdicts`.

The run is `run_drafts_trial.sh controls`, then `run_drafts_trial.sh drafts`.

## What is said

Each draft's verdict by the library's rule: **shown** (problem in five or more without, at most one with), **not
shown**, or **no reading**, with both counts. The harm rule of the library trial is applied and reported: a draft's
builds behind its control by 3 of 10 in apps started or signed in to, or by a median of 5 running-app checks answered;
and, as that trial found it misleads when apps fail to start for other reasons, the median among apps that started is
reported beside it. A verdict moves a draft into `data/prompts.json` only by the owner's decision, in a pull request of
its own; a draft not shown stays in `drafts.md` with what happened.

## Cost

About eighty builds: twenty controls and up to seventy draft builds, estimated at **$25** of the owner's API credit.
The owner, 12 October 2026, before any build: there is more than $200 of credit, so "while optimizing cost is still
important, builds shouldn't be canceled if they have to go over the estimated cost". So nothing stops a build or the
trial for cost: each build is capped at **$5** by the Claude program, against a build that runs away, not against the
estimate; the first ten builds averaging more than $0.60, and a total that ten more builds could take past $25, are each
said in the run's output, and the trial goes on. The owner is told the total as it goes.

## Amendments

None yet.
