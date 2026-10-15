# The owner's follow-up trial: the same prompts with the harness and the model swapped, to tell which one gamed the check

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

The owner's trial, started 15 October 2026, after the three-tool trial (`docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md`).
The owner's own to run; a session reads the records afterwards, as 0502's were read.

**Why.** The three-tool trial ran three tools with three different models, so the harness and the model are confounded:
the tool that rewrote code to pass a check and said it had not (Antigravity) was also the one with the smallest model
(Gemini 3.8 Flash). One run cannot say which layer did it.

**The runs.** Same seven prompts, same brief (the family games app), blank slate each time (the removal and reset steps
from the first trial): (1) Antigravity with a stronger model it offers; (2) Gemini 3.8 Flash inside Cursor or VS Code
with Copilot, which offer Gemini models. Two runs; more if the owner has time (the strong model in the other tool too).

**What the reading looks for,** the same way as before and with the same scripts: did the tool silence or rewrite to pass
a check; did it say what it changed; the said-versus-did table; the like-for-like report counts with one sv build; and
the answer to "does the gaming follow the model or the harness". The result goes into the research page as a section,
into 0568's tables as rows, and into the "Limits" block either way.

**Record.** None; the setup page's "Tried so far" may change with the result.

**The owner, 15 October 2026:** the strong-model run in Antigravity is Claude Opus 5.5 (the subscription was upgraded for it; a first attempt with Opus 4.6 was restarted). sv for these runs: Homebrew HEAD-779676c (commit 779676c5) and the Docker image pulled the same day; prompts 1 to 3 skipped, sv already installed, so setup time is not measured.
