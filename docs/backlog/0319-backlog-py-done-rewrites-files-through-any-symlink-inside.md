# backlog.py done rewrites files through any symlink inside the repository

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: tools and CI. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** No symlink is tracked today, so nothing is reachable now; a future committed link to a file outside the repository would be read and, if it mentions the moved item's path, rewritten in place.

**Where.** `tools/backlog.py:290-299`: `for path in root.rglob("*")` … `path.is_file()` (which follows links) … `write_text(path, …)`.

**Evidence.** The loop at the lines above.

**How it bites.** A committed link makes `done` write outside the repository.

**Fix.** Skip `path.is_symlink()` in that loop; a self-test case with a link.

**Record.** ADR-061 governs `tools/backlog.py`: an "unchanged, because" line.
