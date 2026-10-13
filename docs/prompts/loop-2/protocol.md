# The second loop trial: protocol, written before any build

Backlog 0257 (the loop trial) and 0006 part 21 (an outside tool as an independent check), under ADR-109. The owner's
choices of 12 October 2026, after the drafts trial's group A gave no reading for any draft
(`docs/prompts/drafts-trial/results.md`): "both briefs, $50 budget, scripted review is fine if labeled", and that the
outside-tool check and the observability features be used. A change made after any result has been seen goes in
"Amendments" at the end, dated and with its reason.

## What it asks

Whether `sv`'s features that have only been run against the scripted app work on apps an AI coding tool really built:
whether each runs, whether it gives an answer or "not assessed" (and why, read from the evidence `sv` kept), and
whether an answer is a false alarm. This trial measures `sv`, not prompts and not the builders; where it compares two
arms, it says so and why.

## The builds

| Phase | Brief | Arm | Haiku 5.5 | Sonnet 5.5 |
|---|---|---|---|---|
| A | `club-brief.md` | `loop`: `sv` attached through its MCP server | 10 | 10 |
| B1 | `accounts-brief.md` | `none`: no `sv` while building | 10 | 10 |
| B2 | `accounts-brief.md` | `loop` | 10 | 10 |
| C | `accounts-none-brief.md` | `loop`, then the scripted review | 5 | 5 |

Seventy builds. **The club brief's `none` arm is not built again:** the drafts trial's twenty control builds of the same
day are it (`~/sv-drafts`), checked again here with the outside tools. They differ from phase A's in the brief's two
corrections (below) and nothing else, which the results say beside every comparison.

**How a build is made**, as in the drafts trial (`docs/prompts/drafts-trial/protocol.md`): `loop_trial.py` with
`--with-spec`, the owner-away sentence, `--allow-git`, `--model-id claude-haiku-5-5` or `claude-sonnet-5-5` (never Haiku
4.5, at the owner's word of 12 October 2026), `--budget 5`, the owner's API key through the key helper (quoted), each in
a fresh folder under `~/sv-loop2`, and every transcript searched for the key afterwards. The builder is Claude Code
2.1.295. A build whose transcript says it ran on any other model is left out of every count and named. The MCP server is
attached as `stackvet`, and `loop_measures.py` counts its tools under both names (below).

**The briefs.** `club-brief.md` is `docs/prompts/trial-3/plain-brief.md` with three changes: it names `stackvet.toml`
(it said `securevibe.toml`, and a drafts-trial build wrote both and was refused), it names StackVet, and it asks for a
git repository with each commit ending `Assisted-by: Claude Code`. `accounts-brief.md` is a garden-plots site with
sign-up, a password reset by email (through the mail server `sv` gives the app at `SMTP_HOST` and `SMTP_PORT`), and
changes to the email address, phone number, recovery address, and two-factor phone number, each shown on `/account`.
`accounts-none-brief.md` is the same site with only the email change: no phone, no two-factor, no recovery address.
Neither brief tells the builder what to put in `stackvet.toml`: whether the builder declares a feature from `sv`'s
specification is itself measured.

## How each build is checked

`tools/prompt_trial.py`, one build at a time, with `docs/prompts/trial/policy.toml` standing in for any `[policy]`
number a build did not write, and with:

- **`sv report --run`**, as before: the running app, behind the fence.
- **A second check, `sv report --tools --keep-tool-output`** (`PT_TOOLS=1`), without `--run`: Semgrep and Bandit on the
  code (CodeQL is not installed on the trial's Mac, and the report says so). The MCP server never runs an outside tool,
  so the builder never saw this view. Semgrep's rules are licensed for non-commercial use, which this research trial is.
- **For phase C only, the scripted review** (`PT_REVIEW`, `scripted_review.py`) before both checks, when the build's
  settings list `not-offered`: it confirms that list in `sv review` by typing `owner`, makes the signing key without a
  passphrase, and presses Enter at every other question. **Every result that rests on it says "confirmed by a script
  standing in for the owner, not by a person."** Tried on 12 October 2026, before this protocol, on a drafts-trial
  build that had listed `not-offered` itself: it confirmed the list, left the app's level unconfirmed, and `sv` sealed
  the entry with the trial's key.
- **The trial's own settings folders**: `XDG_CONFIG_HOME` and `XDG_DATA_HOME` point into `~/sv-loop2`, so the signing
  key, the history switch (`keep-history`, set), and the history are the trial's, and the owner's are never read or
  touched. **`SV_LOG`** is set to the trial's log for every check.

`sv` is one release build from `main` at the commit this protocol merges in, recorded with the results, with its data
from that commit's copy of `data/`.

## The measures

Each from the reports, the transcripts, and what `sv` kept, by `score_loop2.py` (written and self-tested in the same pull
request as this protocol) and `loop_measures.py`:

1. **The account changes (ADR-105), phases B and C.** For each of `probe.email-change-without-password`,
   `probe.phone-change-without-password`, `probe.recovery-change-without-password`, and
   `probe.two-factor-change-without-password`: declared by the builder or not, then found, credited, or not assessed,
   with the gap's reason. V7.5.1's status. In B, any `not-offered` entry for a change the app has (the builder's
   word against the brief's). In C, whether the builder wrote `not-offered`, and V7.5.1's status after the scripted
   review.
2. **The preflight's "`sv run` will look for this"** (ADR-035, Later), phase A against the drafts trial's controls:
   how many builds called the preflight, and the rates of `probe.failed-sign-ins-unlimited`, `probe.security-headers`,
   `probe.session-cookie-attributes`, and `probe.ai-injection-unscreened` found. A comparison across two days' briefs,
   reported as that.
3. **How each finding went away, and credit-seeking edits** (`loop_dodging.py`), in phases A, B2, and C.
4. **The outside check** (0006 part 21): the outside tools' findings per build, by tool, in each arm, beside `sv`'s own
   static findings. A loop arm cleaner on `sv`'s view and no cleaner on the outside tools' is reported as the sign of a
   builder satisfying `sv` rather than making the app safer.
5. **The AI-commits note** (ADR-092): `config.ai-commits-off-merge` found in builds whose history has commits carrying
   `Assisted-by` or `Co-Authored-By` straight on the default branch, and whether any build committed at all.
6. **The typed-text check on every form** (ADR-097): forms listed under `text-forms`, and `probe.text-rendered-as-markup`
   whole or in part.
7. **More than one private page**: how many private pages each build lists, and the private-page checks' answers.
8. **The specification's sentences**: settings files `sv` could not read, `[capabilities.ai] enabled` in the club
   builds, `admin` and `seed` under `[stack.run.users]`.
9. **V6.3.8 with sign-up and reset**, phase B: whether `probe.signin-reveals-account` was answered, now that the app has
   a reset (the drafts trial could not read it).
10. **The observability features.** The build-loop record's count of `sv` calls against the transcript's (ADR-084):
    they should agree, and a difference is a finding about `sv`. What `sv` kept of the running app (`seen.json`), and
    how many credentials it removed. The history, which has two runs for each build (the `--run` check and the
    `--tools` check): `sv compare` on each build's two reports (written beside them by the run script), read for whether
    it names each requirement that moved and why. (`sv dashboard` is not tried: it reads each app's own
    `stackvet-report` folder, and the trial writes its reports beside the copies instead.) Any crash file `sv` wrote (ADR-100), each a finding about
    `sv`. The `SV_LOG` lines, as a check that the log names stages and never a path.

**False alarms are read by hand**: every finding of measures 1 and 6, and ten of the others chosen by the scorer at
random with a fixed seed, against the evidence `sv` kept; each called right, wrong, or cannot tell, with the reason.

## Draft 17

Phase B1 is draft 17's control (`password-before-account-changes`). If its problem (any of the four
`*-change-without-password` found) shows in five or more of one model's ten B1 builds, ten builds with the draft on that
model would follow, as the library's rule asks. They are **not run by this protocol**: the owner is asked first, with
their cost.

## Cost

Estimated from the drafts trial's costs ($0.10 a Haiku build and $0.50 a Sonnet build without `sv`), doubled for the
`loop` arm's calls to `sv`, and a third more for the accounts brief's larger app: phase A $12, B1 $8, B2 $16, C $8, so
**about $44** of the owner's $50. The owner, 12 October 2026: builds are not canceled when they go over an estimate.
So nothing stops a build or the trial for cost; each build is capped at **$5** against a runaway, and the run prints the
total after each phase.

## The order

1. The outside-tool check of the drafts trial's twenty controls (no model money).
2. Phase A, then B1, B2, and C, each built and then checked.
3. Scored by `score_loop2.py`, the false alarms read by hand, and written up in `results.md`, with the `sv` commit, the
   cost, and every build's transcript searched for the key.

## Amendments

1. **13 October 2026, after phases A, B1, and B2 were checked and before any of phase C's was: phase C's checks run
   apart.** In the run script, the scripted review's setting was passed unquoted, and its path holds the USB drive's
   name, which has a space; `env` split it, and none of phase C's ten builds was checked ("env: USB/...: No such file
   or directory"). The builds were made as the protocol says and are unchanged; their checks were run afterwards by
   `prompt_trial.py` with the same settings, the same `sv`, and the setting passed whole, and the scoring was run again.
   The run script now passes it whole, tried with a path holding a space. Checking spends no model money.
2. **13 October 2026, after phase A's Haiku builds were checked: the build-loop record is held to the calls made once
   the settings file existed.** The record begins only when the app has its settings file (the MCP server writes no
   line before, `crates/sv-cli/src/mcp/mod.rs`), so measure 10's comparison with every call in the transcript
   disagreed in all ten builds by design. It is now held to the calls made after the build first wrote the file
   (`score_loop2.py`, `calls_after_settings`), with a self-test.
3. **13 October 2026, after phases A and B were seen: two more readings, added because of what was seen and said as
   such.** The environment settings a build's `start` command gives the copy `sv` runs (`start_settings`), since
   eleven builds lowered their own sign-in limits or cookie settings there; and requirement ids in tests' names written
   with underscores (`test_V6_3_1_...`), which `loop_dodging.py` counted only when written with dots.
4. **13 October 2026, after the trial: the count of builds corrected.** The protocol said "Ninety builds"; its table,
   and the cost estimate of $44, are seventy (A 20, B1 20, B2 20, C 10). The ninety the scorer reports are those seventy
   and the drafts trial's twenty controls. Nothing was built or checked differently; only the word was wrong, and it was
   repeated in the results pull request's description, two backlog items, and a design entry before it was caught.
