# The fence over IPv6, after review: no IPv6 at all on the fenced network, a test on the option list, and ADR-019 said exactly

**Status:** claimed by builder-none, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the separate security review of 0300 (pull request #1725, commit 2ccd4b6f), which the owner asked for on 13 October 2026 and filed at the owner's word the same day. The review's verdict: sound, with gaps. It found no path where an error, a timeout, a missing tool, or an empty inspect result turns into a pass. It read the code and did not run Docker or cargo; points marked [inference] depend on how Docker behaves.

**What is wrong.**
1. Medium. The bridge's IPv6 link-local address (fe80::) is never knocked on: the knock's targets come only from IPAM config, which never lists link-local addresses (`crates/sv-run/src/docker.rs`, about lines 2909-2935 and 1941 at 2ccd4b6f). [inference] On a network with IPv6 on, the app could find the bridge with `ping6 ff02::1%eth0` and reach `[fe80::...%eth0]:22` while the check passes.
2. Medium. Nothing goes red when the new IPv6 option is removed from NO_GATEWAY (about lines 112-119): both Docker tests in `crates/sv-run/tests/fence_ipv6.rs` build their own networks with hand-written options and never call the runner's creation loop.
3. Medium. On Docker 27, which has the IPv6-by-default setting but not gateway modes, every run with that setting on is refused, and the person is not told how to fix it (the fallback, about lines 629-645).
4. Low. Where CI cannot make an IPv6 network, the Docker tests prove little: test 1 returns with only a print in its weak branch, which passes even when the check returns Ok; test 2 is judged only by the check's own verdict; the direct knock uses an unpinned `busybox:1.36`, not the pinned probe image.
5. Low. The verdict (gateway_verdict) never checks that the number of reports equals the number of targets, so cut-off output could read a missing knock as Closed (true before #1725 too).
6. Low. The change from `ip -4 -o addr` to `ip -o addr` has no unit test (putting `-4` back fails closed, not open).
7. The ADR-019 Later entry overstates in two places: "holds the check to that" (only where the IPv6 gateway answers) and "now covers both address families" (not link-local).

**Fix.** Findings 1 and 3 share one fix: `sv` needs no IPv6, so ask for none on the fenced network (`--ipv6=false`, [inference] whether it overrides the daemon's default depends on the API version), and refuse the run when the knocking container holds any inet6 address but loopback, saying in the refusal how to turn the daemon's IPv6 default off. Then: a unit test that NO_GATEWAY's first entry holds both isolated options, or move network creation into a function the Docker test calls (2); in the weak branch, assert the output names the IPv6 address, and use PROBE_IMAGE for the direct knock (4); count reports against targets in the verdict (5); a unit test for the address listing (6). Break each new guard and watch its test go red.

**Record.** ADR-019 governs the fence: correct the 13 October Later entry, saying what is and is not checked (link-local, the option list, which branch CI took) until each is closed.

