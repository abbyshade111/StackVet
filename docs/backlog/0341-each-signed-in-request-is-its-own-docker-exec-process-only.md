# Each signed-in request is its own docker exec process; only the anonymous probes are batched

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** The file's own header (`docker.rs:9-11`) says the sidecar was introduced because a container per request cost about half a second and a signed-in run made twenty-odd requests; the suites have grown since: 130 static `.send(` call sites across `signed_in/*.rs`, `oidc.rs`, and `ai.rs`, several inside loops, so a full `--run` is a few hundred execs.

**Where.** `crates/sv-run/src/docker.rs:3071-3090` (`fn probe`): every `send` runs `inside_fence_with_input(via, &["sh","-c",&script], &raw)`, one `docker exec -i <sidecar> sh -c …` per HTTP request. `Http::send_in_turn` (one exec for a whole list, `crates/sv-check/src/signed_in/mod.rs:117-122`, `docker.rs:3095-3117`) has one caller, `ask_anonymously_within` (`signed_in/mod.rs:1544`).

**Evidence.** The `probe` function and the one `send_in_turn` caller. `request_timings` in any real `report.json` gives the exact count and the per-request wall time, which includes the exec.

**How it bites.** Estimate: a `docker exec` round trip is 100 to 300 ms on Docker Desktop or Colima before the app is even asked, so 300 requests spend 30 to 90 s in process start-up alone, likely a large share of the running-app stage for a fast app. Check against a real report's `request_timings` first.

**Fix.** Keep one long-lived `docker exec -i <sidecar> sh` open for the run and feed it each request's script and bytes over its stdin, reading answers back between markers (the `at_once_mark` scheme at `docker.rs:2680` already frames answers); or, cheaper, have `Patient`/`Recording` collect consecutive independent `send`s and flush them through `send_in_turn`. Rate-limit waits and the `now()`/`mail()` interleavings stay. The counting fake at `docker.rs:3832-3860` holds it: assert a signed-in suite against the fake app makes fewer execs than requests.

**Record.** ADR-019 governs the fence and `docker.rs`: a Later line (requests still go in from inside the fence; fewer processes carry them).
