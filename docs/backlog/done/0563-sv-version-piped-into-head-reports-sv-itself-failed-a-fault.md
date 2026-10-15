# `sv --version` piped into `head` reports "sv itself failed: a fault in sv" on the closed pipe

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Seen while rebuilding for the pointer's second reading, 14 October 2026. Severity: low.

**What is wrong.** `sv --version | head -1` prints the version line and then "sv itself failed: a fault in sv, not in
your app (failed printing to stdout: Broken pipe (os error 32) ...). Nothing was assessed ... Please report it". A
closed pipe is the reader's choice, not a fault; every Unix tool exits quietly on it.

**Fix.** Treat EPIPE on stdout as a quiet exit (status 0 for `--version` and the printing commands; the report
commands have already written their files), never as sv's own fault. One test with a closed pipe.

**Record.** None.

**Done, 15 October 2026.** A closed pipe ends quietly with status 141, never "sv itself failed", and writes no crash
file: `crash::closed_pipe` reads the panic Rust's `println!` raises on a reader that went away ("failed printing to
stdout: Broken pipe (os error 32)", or Windows' "(os error 232)"), and `main` ends with `exit::CLOSED_PIPE`.

**Not 0, as the item asked.** A check command's own status (1, needs attention; 2, not assessed) is cut short with
its output, and 0 under `set -o pipefail`, as a CI pipeline runs, would read as a pass: something not assessed read
as passing. 141 is what `cat` or `ls` end with when their reader goes away (128 and the broken-pipe signal's 13), the
same convention as Ctrl-C's 130 beside it in `exit.rs`, and no pipeline reads it as success. Without `pipefail` a
shell takes the last command's status, so `sv --version | head -1` still ends 0 there. If the owner wants 0, it is
one constant. `docs/ARCHITECTURE.md` says it beside Ctrl-C's 130. Test: `crates/sv-cli/tests/closed_pipe.rs` (2),
with the reading end closed before `sv` starts, so every run meets the closed pipe; without the new arm in `main` it
went red with status 3.
