# tools/: `write_text` is defined in twelve scripts, `slug` in two, and `git` three ways

**Status:** open

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
