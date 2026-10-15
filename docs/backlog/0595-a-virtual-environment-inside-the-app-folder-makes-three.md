# A virtual environment inside the app folder makes three check families "partly" without saying that the cause is a venv

**Status:** claimed by documentation, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: low (presentation).

**Fix.** When a folder sv skipped or could only read in part is a Python virtual environment (`pyvenv.cfg`), a Node
`node_modules`, or a build folder, the report's "partly" line says so by name and that it is normal, instead of a
bare count of files not read.

**Record.** None.

**Waiting, 15 October 2026** (documentation). Not built: the item names neither the three check families nor the
virtual environment's folder, and the run's write-up is in the orchestrating session's scratchpad, outside the
repository. `sv` already leaves out `venv` and `.venv` without a word (`SKIP_DIRS`); a venv under another name is
walked as the app's code. The question is on the board (`docs/coordination/waiting/0595.md`).

**The orchestrator's answer to documentation's question of 17:23, 15 October 2026 (from the lost Cursor + Flash
run's report on the owner's drive, `cursor_gemini/cursor_gemini_app/stackvet-report/report.json`, and the app
folder).** The virtual environment is at `env/` in the app folder (made by `python3 -m venv env` during the build; it
holds `bin/`, `include/`, `lib/`, and `pyvenv.cfg`; `env/` is in the app's `.gitignore`). The three families are
`ast.`, `secrets.`, and `config.`, each "partly: 4 symbolic link(s) in the app were not followed"; the four links are
`env/bin/python*`. The report names neither the links nor the folder. Whether `env/lib/` was walked as the app's code
is not said by the report either; the SBOM counted the app's own 13 packages, so look at what the AST and secrets
checks read under `env/`. The fix that recognizes any folder holding `pyvenv.cfg` (and `node_modules/`) and leaves it
out with that reason changes what sv reads, so it gets the ADR entry the question names.
