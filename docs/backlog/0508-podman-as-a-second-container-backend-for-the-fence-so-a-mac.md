# Podman as a second container backend for the fence, so a Mac or Linux user without Docker can run the running-app checks

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Filed 14 October 2026 from the owner's question whether a non-Docker route on Mac and Linux is worth looking into.
Without Docker, `sv` already runs everything but the running-app checks: the native program (Homebrew, or the Windows
zip) reads the code, runs the outside tools, and writes the report. What needs a container runtime is the fence
(ADR-019): a network with no gateway, every container read-only with every capability dropped, requests sent from a
sidecar inside. Running the app bare on the host cannot give those guarantees, so a no-container mode would not be
honest and is not proposed. What is worth a look is Podman, which runs rootless, needs no daemon, is a drop-in for most
of the Docker command line `sv` uses (`network create --internal`, `run --read-only --cap-drop ALL`, `exec`, `inspect`,
labels for cleanup), and is what some Linux distributions ship instead of Docker. The look: run the fence tests with
`podman` as the backend (`sv_run::detect` finds the program; the IPv6 and plain-network rules of 0410 and 0414 ask
Docker-specific questions that may answer differently), list what differs, and decide whether a second backend is one
item or several. Nothing changes what the fence guarantees; a record only if Podman is adopted. **Held until the owner
asks for it or a Mac or Linux user without Docker does.**

**The owner's word, 14 October 2026:** take the look now. The hold is lifted; the item goes out with the next lane. Its first step is the measurement (the fence tests with `podman` as the backend, on a Mac with Podman installed, and what differs), written into this item before any code; whether a second backend is adopted is a decision of its own.

**The owner's answer, 14 October 2026** (asked by session honesty through the board's waiting line): (b). The session installs Podman in its own cloud container and measures there; the Linux half is the one that matters most, since Linux users without Docker are the likely Podman users. What differs goes into this item before any code, with a note that the Mac half is still to be measured (the owner's Mac, when there is time).

**The owner's answer, 14 October 2026** (asked through the board's waiting line): (b). Install Podman in the
session's own Linux container and measure there, since the Linux half matters most; write what differs here before
any code, and say that the Mac half is still to be measured.

**The Linux measurement, 14 October 2026** (session honesty). Podman 4.9.3 installed from Ubuntu 24.04's packages
in the cloud session's container, run as root (not rootless), with netavark networking, overlay storage, and
cgroup v1. `sv` runs the program `docker` and nothing else, so the measurement put a `docker` on the PATH that runs
`podman`, and ran `cargo test -p sv-run` with `SV_REQUIRE_BACKEND=1`, so a silent skip could not pass. No code of
`sv`'s was changed. Docker's own daemon is not running in this container, so on Docker every fence test here takes
the no-backend path; the comparison is with what the tests assert, which CI checks on Docker.

What differs, in the order the run met it:

1. **The backend check.** `sv` asks `docker info --format {{.OSType}}`; Podman's `info` has no `OSType` (it is
   `.Host.OS`), so `sv` reads "no container backend" and every fence test takes the honest-absence path. To measure
   past it, the shim translated this one question.
2. **The gateway (0410, 0414): the fence cannot be closed on Podman as `sv` builds it, and `sv` refuses.** Podman
   rejects both of Docker's ways of leaving the gateway out (`-o com.docker.network.bridge.gateway_mode_ipv4=isolated`
   and `-o com.docker.network.bridge.inhibit_ipv4=true`: "unsupported bridge network option"), so `sv` falls back to
   a plain `--internal` network. That network has a gateway address on this computer, and a container on it reached
   a server listening on this computer through it (checked by hand: `wget` from an Alpine container on a Podman
   `--internal` network to the gateway's address got the page). Its gateway is also read differently: Docker's
   `network inspect` field `.IPAM.Config` is `.Subnets` in Podman, so `sv` first failed reading it. With that one
   question translated too, `verify_gateway_closed` knocked, found the gateway reachable, and refused to start the
   app ("a container on the fenced network reached its gateway ... so it was not started"). `sv` fails safe here;
   no app ran with the way to this computer open. Every fence, install, leftovers, limits, stays-up, and interrupt
   test that starts an app fails for this reason alone.
3. **A sidecar that has gone.** `sv` recognizes it by Docker's words, "No such container" or "is not running";
   Podman says "no such container" in lower case, so `sv` does not say the sidecar went
   (`requests_through_one_session_reach_a_real_server_with_one_exec`).
4. **Short image names.** Podman resolves a short name only through its alias list or a search registry, and this
   install has neither for `chromedp/headless-shell`, so the browser does not start ("short-name ... did not resolve
   to an alias"); `node` and `busybox` worked only because Podman ships aliases for them. A name written in full
   (`docker.io/chromedp/headless-shell`) works on both. The four browser tests in `docker.rs` fail for this.
5. **The IPv6 rules (0410, 0414)** take their fallbacks: Podman refuses the isolated-gateway options, so the
   runner falls back to IPv4 alone and `--internal` alone. The tests pass by those paths and exercise nothing about
   IPv6 on Podman.

What works as on Docker: `network create --internal`, `run --read-only --cap-drop ALL --security-opt
no-new-privileges --pids-limit --memory`, labels and the label filters `sv` cleans up by, `exec -i`, `rm -f`, and
`network rm`; the 172 unit tests in `sv-run` that need no running app pass.

Not Podman's doing, and the same on Docker in this container: the outbound check's control cannot reach `1.1.1.1:53`
from an ordinary network (a container here reaches no outside address), and the install test's `pip` fails on this
session's proxy certificate.

**So a second backend is several items, and one is a question first.** Items 1, 3, and 4 are small (the backend
check by Podman's field, the gone-container words in any case, image names written in full, which helps Docker too).
Item 2 is the one that decides adoption: Podman as installed here gives no way, through the options `sv` uses, to
keep a container from reaching this computer through its network's gateway, and the fence's guarantee rests on
that. Whether rootless Podman (pasta or slirp4netns networking), a newer netavark, or a rule `sv` adds itself closes
it is the next thing to measure. Until then a Podman backend would refuse every run, honestly.

**The Mac half is still to be measured:** Podman on a Mac runs in a virtual machine (`podman machine`), whose gateway
and network differ from Linux's; none of the above was tried there.
