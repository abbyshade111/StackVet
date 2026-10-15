# `sv doctor` inside a tool's sandboxed shell says there is no container backend when the socket is merely unreachable

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity trial. Severity: low.

**What is wrong.** Antigravity's step 463 (sandboxed shell): doctor said Docker was not reachable as if absent; step
465 (outside the sandbox): fine. A person reads "install Docker" and Docker is installed.

**Fix.** When the socket exists but cannot be opened, say "Docker is installed but this shell cannot reach it
(permission denied on <socket>); run sv doctor outside the tool's sandbox or in your own terminal"; prompt 6 says the
same.

**Record.** None.

**Done, 15 October 2026.** When `docker info` answers "permission denied" (or "operation not permitted", as a macOS
sandbox makes it) on Docker's socket, `sv` reads that Docker is installed and this shell is kept from it
(`unreachable_socket` in `crates/sv-run/src/docker.rs`; `CannotRun::NoBackend` carries the socket as `unreachable`).
It then says "Docker is installed but this shell cannot reach it (permission denied on <socket>)", that what needs the
app running is not assessed, and to run `sv doctor` outside the tool's sandbox or in the person's own terminal, and
run again there. `sv doctor` shows that line as "can't tell", not "not ready", so nobody is sent to install Docker;
the report files the run as stopped rather than not installed. "Cannot connect to the Docker daemon", where nothing
is listening, is still "No container backend". Setup prompt 6, step 4, says the same in the doctor's words, and that
nothing needs installing. Tests: `sv-run` `docker/unreachable_tests.rs` (2), `sv-cli` `doctor/unreachable_tests.rs`
(2), `run_gap_reason_tests.rs` (1), and `tests/setup_prompt_six_docker.rs` (1). Not tried against a real sandboxed
shell here: the Docker messages in the tests are the ones Docker prints, written in by hand.
