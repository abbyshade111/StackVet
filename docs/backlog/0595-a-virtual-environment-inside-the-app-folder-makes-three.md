# A virtual environment inside the app folder makes three check families "partly" without saying that the cause is a venv

**Status:** claimed by documentation, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: low (presentation).

**Fix.** When a folder sv skipped or could only read in part is a Python virtual environment (`pyvenv.cfg`), a Node
`node_modules`, or a build folder, the report's "partly" line says so by name and that it is normal, instead of a
bare count of files not read.

**Record.** None.
