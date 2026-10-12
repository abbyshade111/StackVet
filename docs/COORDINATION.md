# The coordination board: how one session hands out the work

Written 12 October 2026 for the trial in backlog 0273 (the record is `docs/adr/ADR-061.md`, Later, 12 October 2026).
The owner's three principles for it: keep it simple, make everything visible, and document and test it thoroughly.

## Why

Many sessions build `sv` at once, and until now none was in charge: each coordinated with the others only through
`main`. Between 6 and 12 October 2026, 602 pull requests merged, 187 of them only to record a claim, and `main` was
merged back into working branches 429 times; nearly every rule in `CLAUDE.md` about claims, numbers, and merging was
written after two sessions collided. This trial asks whether one session that hands out the work, and nothing else,
makes those collisions stop, and what it costs.

## What it is

- **One role, written here, held by one session at a time.** The orchestrator reads the roadmap and the open items,
  gives each building session one item, and writes down what it did and why. It builds nothing, reviews nothing it
  handed out, and decides nothing that is the owner's.
- **One file, the board,** `docs/coordination/board.md`, written only by the orchestrator through
  `python3 tools/coordination.py`. It has three parts a person can read as they are: the sessions (each with its arm
  and what it holds), the queue (backlog numbers in the order they go out, each ready or on hold with the reason), and
  the log (one dated line per hand-out and why). The orchestrator's memory is this file and nothing else, so a session
  that was not running, or whose memory was summarized, reads the same state as every other, and another session can
  take the role over by reading it.
- **The backlog stays the record.** `assign` makes the claim with `tools/backlog.py` on the builder's behalf, in the
  same commit as the board line, so the item says who holds it on `main` as always; the builder's `done` closes it.
  The board only orders the record, and `python3 tools/coordination.py check` fails when the two disagree (a test
  runs it on every pull request).
- **Not a program that decides.** No software agent, no new service, nothing that runs on its own: a session with a
  written role and a small tool, so there is nothing to run, pay for, or explain beyond this page.

## How a day goes

1. The orchestrator opens the board once (`open --by <session>`), names the building sessions and their arms
   (`session add <name> --arm full|none`), and queues the items in the order they should go out (`queue add <number>`,
   with `--hold "why"` for one that waits).
2. For each free session: `assign <number> --to <session> --why "..."`. The reason is written down every time, in at
   least ten characters. The claim is made and the board updated in one go, and both go to `main` in one small pull
   request, so the hand-out is visible to every session within minutes.
3. `handout <number>` prints the message to give the builder: the item's text and the five-line footer below. A
   session's first hand-out is pasted by the owner, with one more sentence: when your pull request has merged, fetch
   `main`, read the board, and take the item beside your name; if there is none, check again in 30 minutes. After that
   the board does the handing out, and nobody pastes (the owner's choice, 12 October 2026). So the orchestrator assigns
   each session's next item while it is still building the current one, and the item's own file and this page's footer
   are everything the session needs; the board is what counts, a message only points at it.
4. The builder builds, in a branch and a pull request of its own, and runs `python3 tools/backlog.py done <number>`.
   The orchestrator runs `finish <number> --pr <url>`, and the session is free for the next item. The two may land on
   `main` in either order: `check` reads a held item marked done as the normal hand-back, not a disagreement (the
   first two finished items, 0280 and 0278, each turned their own pull request red before it did).
5. A red `main` goes to exactly one session, through the same `assign`, so two never fix it together.
6. At the end: `measure --since <day> --arms` counts what git shows, and the write-up goes in the item.

The commands are listed at the top of `tools/coordination.py`, and `--self-test` exercises every one, their refusals,
the measures on a history made for the test, and the no-context clone.

## The five lines every hand-out ends with

Both arms get the same footer, whatever else they know. It is a small amount of context by design, and the trial's
write-up says so.

> Rules that hold whatever else you know, from the owner:
> - Delete nothing, force-push nothing, and rewrite no history.
> - Spend none of the owner's money, and change no setting of the repository or of this computer.
> - Open a pull request for the work and let its checks decide; never merge a red one.
> - When something is the owner's to decide, stop and ask the owner.
> - When you are done, run `python3 tools/backlog.py done <number>` and say what was built and what was not.

## The two arms

The owner asked to test "how much context matters once there's orchestration, and whether there's an appreciable
cost difference from bloated context files". Two fresh building sessions, both on the same model (Opus 5.5), and both
**cloud sessions** (the owner's word, 12 October 2026, so this computer is not loaded): each runs in Anthropic's
environment on a clone of the repository from GitHub.

| | Full context | No context |
|---|---|---|
| Standing instructions | `CLAUDE.md` (about 3,100 words, read every turn) and the rules, skills, and settings under `.claude`, which a cloud session loads from the clone as a local one does | A two-line `CLAUDE.md` that says only to take its work from the board |
| Where it runs | A cloud session in the ordinary cloud environment | A cloud session in a second cloud environment whose setup script, printed by `python3 tools/coordination.py cloud-setup-script`, writes the stub and tells git to ignore the change (`update-index --skip-worktree`), so no commit can carry it |
| Memory | None of this computer's: auto-memory is machine-local and never reaches a cloud session, so both arms are without it, and the arms differ on `CLAUDE.md` alone | The same |
| Permissions | The same `.claude/settings.json` | The same, so the difference is knowledge, not permission prompts |

Both are fresh because a running session has already read the files into its conversation, and taking them away does
not take away what it knows. The orchestrator alternates matched items between the arms and writes the arm beside each
hand-out; `measure --arms` reads them back.

**Test the no-context environment once before the trial.** The documentation does not say whether a setup script runs
inside the clone, or whether git's index survives the environment's cache; start one session there and ask it to print
`CLAUDE.md` and `git ls-files -v CLAUDE.md` (an `S` in front means git ignores it). If the stub is not there, the trial
does not start until it is. On this computer, `setup-no-context ~/stackvet-trial-noctx` makes the same arm as a local
clone, should the cloud route fail.

**Tested 12 October 2026:** the owner's test session in that environment printed the stub, the `S` flag, and a clean
tree, so the script runs inside the clone and the flag survives to the session's start. The session then read the
committed `CLAUDE.md` with `git show` and compared, and called the substitution possible tampering. Two things follow.
The no-context arm has no *standing* instructions, but the file stays in the repository's history, as everything does,
so the write-up checks each of its transcripts for whether the session went and read the committed `CLAUDE.md` on its
own, and says so; a session that does has shown how much it wants those rules. And the stub now says the replacement is
deliberate, for the trial, so a builder does not stop to investigate it.

**Reaching a cloud session.** The desktop app's session tools do not see cloud sessions, so the orchestrator cannot
message them from here. Either the owner pastes the hand-out, or it is sent from the Terminal with
`claude -p "<the hand-out>" --cloud <session-id>` (once `claude auth login` has been run). The board is the record
either way; the message only points at it. Cloud sessions carry a `Claude-Session:` trailer on their commits, which the
write-up notes beside the `Assisted-by:` count.

What the no-context session still has: the repository (`docs/ARCHITECTURE.md`, the backlog, the records, the tools'
own help text), the checks CI runs, and the hand-outs. So the useful result is line by line: a rule the no-context
session follows anyway is one CI or the repository already carries, and a candidate to cut from `CLAUDE.md`; a rule
only the full-context session follows is one the file earns its place with.

## What is measured

| From git, by `measure` | By hand or from the app |
|---|---|
| Pull requests merged; those that only record a claim (every file under `docs/backlog/`) | Tokens per item, from each cloud session's own usage display and transcript (the desktop app's per-turn reading is for local sessions only) |
| Merges of `main` back into a branch | American English and plain language in what each wrote |
| Numbers taken twice in the tree | Questions to the owner per item |
| Per arm, with `gh` present: commits without the tool's name (ADR-092), merges of `main` in each pull request | Pull requests refused after passing (GitHub only) |

A caveat to carry into the write-up: standing context is cached after the first turn, and a cached read costs about a
tenth of a fresh one, so the money difference between the arms may be small even where the attention difference is
large. The trial shows which.

## What the orchestrator may not do

- Decide anything that is the owner's: it brings the question to the owner, and the owner's answer goes in the item.
- Review or grade the work it handed out. Reviews stay with a separate session, as `sv` itself keeps the author apart
  from the grader.
- Order merges. Auto-merge stays as it is; the owner left that part out (12 October 2026).
- Write anywhere but the board, the claim lines, and this page's records.

## Handing the role over

Another session takes the role by reading this page and the board, and writing `role taken by <session>` in the log
(a plain line; the tool does not need to know). Nothing else moves, because nothing else was ever held in a session's
memory.
