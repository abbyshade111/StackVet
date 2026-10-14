# tools/: `write_text` is defined in twelve scripts, `slug` in two, and `git` three ways

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `def write_text` in twelve of the scripts (identical bodies, `tools/backlog.py:72`, `tools/atlas_references.py:32`,
  `tools/codeql_suites.py:31`, ...); `def slug(title)` identical in `tools/design_entry.py:47` and `tools/backlog.py:112`;
  `def git(...)` in `tools/coordination.py:414` (`check=True`, so a failure is a `CalledProcessError` traceback),
  `tools/merge_main.py:49` (`SystemExit` with stdout and stderr), `tools/record_numbers.py:61` (`SystemExit` with
  stderr only), and an inner one at `tools/adr_check.py:198`; `ROOT`/`HERE` computed in sixteen scripts.
- What is wrong: `tools/coordination.py:45` already does `import backlog`, so the scripts are not strictly
  standalone, and a change to how files are written (0120's UTF-8 and LF rule is what `write_text` holds) is twelve
  edits. `crates/sv-cli/tests/tools_text.rs` holds the encoding, so a bare `write_text` is caught; this is about
  copies, not a fault. No `shell=True` remains (0321 done); every `subprocess.run` is list form; `backlog.py:293` and
  `docs_page.py:425` refuse symlinks before writing (0319 done).
- What to do: `tools/_shared.py` with `write_text`, `slug`, `git`, `ROOT`, imported by the scripts (sys.path is the
  script's folder, so `import _shared` works when run as `python3 tools/x.py` from anywhere); keep the `self_test`s.
  Not verified: whether any script is copied out of the repository and run alone, which an import would break
  (`install.sh` does not).
- How verified: read; `grep` counts.
- Overlap: 0319, 0321, 0322 (done or open; different faults in the same files).
- Needs a decision record?: no.

**What was built (14 October 2026, session documentation).** `tools/_shared.py` holds `write_text` (UTF-8, Unix line
endings, and, when asked, the folder made first) and `slug`, with a self-test. Eleven of the twelve scripts that each had
`write_text`, and the two that had `slug`, import them instead; `coordination.py`, whose copy also made the folder,
uses it with that set. The two `slug`s were written differently: `backlog.py`'s cut the whole name to 60 characters at its last step and
fell back to "item", `design_entry.py`'s cut the first word and fell back to "entry"; both give the same names, so the
shared one is `design_entry.py`'s, and each script keeps its own fallback word in a one-line `_named`. The item assumed `import
_shared` would work from the script's folder; it does not under `python3 -I`, as CI runs the tools, so each script puts
its folder on `sys.path` first, as `coordination.py` already did for `import backlog`. Held by
`crates/sv-cli/tests/tools_shared.rs`, which runs the self-test under `-I` and fails when a script defines either
helper again; a copy put back in `merge_main.py`, and the shared helper set to write CRLF, each turned it red. Every
changed script's self-test passes under `-I`, and those without one (four) start and reach their argument parsing.
Not done, and why: the three `git` helpers stay their own, since they differ in what they return and how they fail,
and callers rely on each (`coordination.py` catches `CalledProcessError`; `merge_main.py` returns the whole result and
can be asked not to fail; `record_numbers.py` exits with git's message); `ROOT` stays one line in each script, since
importing it would cost the `sys.path` line it saves. Not verified: whether anyone runs a script copied out of
`tools/` (nothing in the repository does: `install.sh` and the workflows run them in place).

`pwned_passwords.py` keeps its own `write_text`. It writes the public breached password `sv` tries at sign-up, and
CodeQL's clear-text alerts on that write (79 and 80) were dismissed as false positives in that file on 27 September
2026; through the shared helper CodeQL raised the same write again, as alert 121 at `tools/_shared.py`, on pull request
#1836. Dismissing that one instead is the owner's call, not a session's, and a rewrite only to keep a scanner from
seeing it is ruled out, so the script stays as it was and `tools_shared.rs` names it as the one exception, with why.
