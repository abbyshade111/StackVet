# `sv --version` piped into `head` reports "sv itself failed: a fault in sv" on the closed pipe

**Status:** claimed by documentation, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Seen while rebuilding for the pointer's second reading, 14 October 2026. Severity: low.

**What is wrong.** `sv --version | head -1` prints the version line and then "sv itself failed: a fault in sv, not in
your app (failed printing to stdout: Broken pipe (os error 32) ...). Nothing was assessed ... Please report it". A
closed pipe is the reader's choice, not a fault; every Unix tool exits quietly on it.

**Fix.** Treat EPIPE on stdout as a quiet exit (status 0 for `--version` and the printing commands; the report
commands have already written their files), never as sv's own fault. One test with a closed pipe.

**Record.** None.
