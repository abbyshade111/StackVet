# Nightly: Windows fails: the_second_loop_trials_scorer_passes_its_self_test

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Recorded by the nightly CI routine on 13 October 2026. The Windows workflow failed on 16 pushes to `main` between
22:41 UTC on 12 October and 00:18 UTC on 13 October on one test, `the_second_loop_trials_scorer_passes_its_self_test`
(`crates/sv-check/tests/loop_measures.rs`, line 39), with totals such as 3115 passed and 1 failed. Rust and CodeQL
passed on the same pushes. It was not there the night before (the script is new).

- First seen in [run 38318067515](https://github.com/abbyshade111/StackVet/actions/runs/38318067515) on `1d1e5ae`, the merge of pull request #1584
  (`claude/0257-loop2-protocol`), which brought `docs/prompts/loop-2/score_loop2.py` and its test.
- Last seen in [run 38326559087](https://github.com/abbyshade111/StackVet/actions/runs/38326559087) on `67d7ce4` (#1599).
- What the failure said: a Python traceback from `score_loop2.py`, line 270 (`self_test()`), into line 263 inside
  `self_test`; the annotation stops before the exception's own line, so which check in the self-test failed was not
  read. The path it ran from mixes `\` and `/` (`crates\sv-check\../..\docs/prompts/loop-2/score_loop2.py`).

`main`'s Windows job has passed since #1600 (`14e632c`, 00:24 UTC on 13 October), which took the loop trials'
script tests out of the Windows build at the owner's word (`21af08d`, ADR-109, Later). **What stays open:** only
whether that is the end state; if it is, mark this done with that note.
