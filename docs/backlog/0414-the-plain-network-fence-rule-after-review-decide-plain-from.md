# The plain-network fence rule, after review: decide plain from what Docker built, per address family, and restore the reached-or-silent test

**Status:** claimed by builder-none, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the separate security review of 0301 (pull request #1739, commit 7e0eb5db) that the owner asked for on 13 October 2026. The review's verdict: sound with gaps; the new rule is correct and fails closed where it applies. It read the code and did not run Docker or cargo; points marked [inference] depend on how Docker behaves.

**The owner's word, 13 October 2026, on 0301:** keep the refusal on old Docker (a plain `--internal` network whose knock gets no answer refuses the run).

**What is wrong.**
1. Medium. `plain` is decided by which `network create` call returned 0 (`made_with.is_none()`, `crates/sv-run/src/docker.rs` about lines 629-641 and 672 at 7e0eb5db), not by what Docker built. [inference] A daemon older than 27/28 may accept `com.docker.network.bridge.gateway_mode_ipv4=isolated` as an unknown option and ignore it: `made_with` is set, `plain` is false, the bridge keeps its host address, the knock on port 9 times out behind a firewall that drops all but SSH, the verdict is Closed, and the app reaches SSH. That is 0301's own case, untouched, and the record names the run "with ...isolated".
2. Low-medium. NO_GATEWAY entries 2 (IPv4 isolated only) and 3 (`inhibit_ipv4`) leave any IPv6 gateway on the host, yet are judged with `plain = false`; an ip6tables drop rule makes the IPv6 knock time out and read Closed. Carried over from 0300; see 0410.
3. Medium. `crates/sv-run/tests/fence.rs` (about 416-425): the positive control used to require "reached its gateway", which caught an `nc` that stopped reporting refusals; with `plain = true` silence now also passes it, so a broken refusal detection stays green.
4. Medium-low. Nothing guards the wiring: changing `made_with.is_none()` to `false` turns no test red. Neither Docker test goes through the create loop's fallback, or says which reason it took.
5. Low. When every target is the container's own address (about 3002-3004), the result is Closed even on `plain`.
6. The ADR-019 Later entry (about lines 332-343) says the fallback happens only "when Docker refuses every way" (not so under 1) and cites `tests/fence.rs` as holding the change (it no longer tells reached from silent); it should say plainly that a plain network can never pass. The refusal's message names Docker 28, though Docker 27 with `inhibit_ipv4` also avoids the plain branch.

**Fix.** Decide `plain` per address family from what Docker reports: the inspect output of an isolated network lists no Gateway for that family, a plain one names it (or read the server version and do not trust the `gateway_mode` options below 28); treat a family whose gateway Docker names as plain. Pull that choice into a pure function with its own test. Run the positive control with `plain = false` and assert Reachable, and add the `plain = true` call separately, printing which reason it saw. Have a Docker test go through the create loop's fallback and say which branch it took. Make the own-address-only case Unknown on a plain family. Correct ADR-019. Break each new guard and watch its test go red.

**Record.** ADR-019 governs the fence: correct the 13 October Later entry for 0301 and record the owner's word above.

