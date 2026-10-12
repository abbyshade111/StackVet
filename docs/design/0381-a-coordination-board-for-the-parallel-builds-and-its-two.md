# A coordination board for the parallel builds, and its two arms (12 October 2026)

Backlog 0273; the record is ADR-061, Later, 12 October 2026. Written by session stackvet-orchestrator, at the owner's
word ("you can go ahead with the role document, board, and tool now").

**What was asked.** A review that morning found that nearly every rule in `CLAUDE.md` about claims, numbers, and
merging was written after two sessions collided, and counted the cost from git for 6 to 12 October: 602 pull requests
merged, 187 of them only to record a claim, 429 merges of `main` back into branches. The owner asked for a one-day
trial of one session that hands out the work, under three principles: keep it simple, prioritize transparency,
document and test thoroughly. Then, the same day, for two arms: one building session with the standing context as it
is, one with none of it, to test "how much context matters once there's orchestration, and whether there's an
appreciable cost difference from bloated context files".

**The shape chosen, and why.**

- *A role, not a program.* The orchestrator is a session reading `docs/COORDINATION.md`, using the tools that already
  exist (`tools/backlog.py` for claims, `tools/record_numbers.py` for numbers, `gh` for pull requests). A software
  agent of its own would be a fourth thing to run, pay for, and explain, and the simplest thing that can hand out work
  is a session with a written role.
- *Its whole memory is one file in git.* `docs/coordination/board.md` has three parts, each a list of plain lines:
  sessions (name, arm, what each holds), queue (backlog numbers in order, ready or on hold with the reason), log (one
  dated line per hand-out, with the reason, which `assign` refuses without). Only the orchestrator writes it, through
  `tools/coordination.py`, so the shared-file conflicts of 8 October do not come back; everyone reads it. A session
  whose context was summarized, or one taking the role over, reads the same state as every other. The tool reads the
  lines back by pattern and refuses a line it does not know rather than guessing, so the file a person reads is the
  file the tool reads.
- *The backlog stays the record.* `assign` makes the claim with `tools/backlog.py` first, on the builder's behalf, and
  touches the board only when the claim succeeded; the builder's `done` closes the item as always. `check` fails when
  the board and the backlog disagree (a held item whose status line says otherwise, a queued item that is done), and
  `crates/sv-cli/tests/coordination.rs` runs it on every pull request.
- *The measures come from git,* so the trial is judged by what anyone can count: pull requests merged, those that
  only record a claim (every file under `docs/backlog/`), merges of `main` into branches (a merge commit whose
  subject names `main`), and numbers taken twice in the tree. Pull requests refused after passing live on GitHub
  only, and the tool says so instead of guessing. The self-test builds a small history with one of each and checks
  the counts; its first run caught a `\b` after a closing quote that never matches, which would have counted every
  merge of `main` as zero.
- *The two arms.* Both fresh sessions on the same model (Opus 5.5, the owner's word), because a running session has
  already read the files into its conversation and taking them away does not take away what it knows. The no-context
  arm runs in a separate clone made by `setup-no-context`: the memory folder is keyed on the path, so a new path
  starts empty; `CLAUDE.md` there is a two-line stub, and `git update-index --skip-worktree` keeps git from seeing the
  change, so no commit can carry it (the self-test checks `git status` is clean there). Every hand-out, to either
  arm, ends with the same five-line footer (no deleting, no force-push, no spending, no settings; ask the owner; run
  `done`), which is a small amount of context by design and said so. `measure --arms` reads the arms back from the
  log and, when `gh` is present, each finished pull request's commits for the rule no check enforces, the tool's
  name on a commit (ADR-092).
- *Left out, at the owner's word.* Sending pull requests in to merge one at a time (close to the merge queue
  declined the same day); a budget line (the trial workflow puts its estimate to the owner already); the hold on
  the deep review's findings until the loop trials finish, lifted the same evening.

**What is still by hand.** American English and plain language in what each arm wrote; the session's context size per
turn (the app shows it) and tokens per item from the transcript; questions to the owner per item. The write-up names
each.

**What to watch for.** The footer and the hand-out text are context the no-context arm does get; the write-up should
say which of its behaviors could have come from those lines alone. And the trial's own number: 0273 says the role
stays only if the counts fall and the items finished per day do not.
