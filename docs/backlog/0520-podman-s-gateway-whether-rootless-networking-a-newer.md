# Podman's gateway: whether rootless networking, a newer netavark, or a rule sv adds keeps a container from the computer; and the Mac half

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Split from backlog 0508 (done 14 October 2026, the owner's word): the one question that decides whether Podman can
be a second backend for the fence (ADR-019). On the Podman measured in 0508, none of the options `sv` uses
(`--internal`, the isolated-gateway options of 0410 and 0414) keeps a container from reaching the computer through
its network's gateway, and that is the guarantee the fence rests on; a Podman backend would refuse every run.

**To measure, on Linux first:** rootless Podman with pasta and with slirp4netns networking; a current netavark;
and, if neither closes it, a rule `sv` adds itself on the network it creates, verified by asking Podman as the Docker
fence is verified by asking Docker. Each is a knock from inside a container at the gateway, as `gateway_verdict`
does, with the answer written here. **Then the Mac half:** Podman on a Mac runs in a virtual machine
(`podman machine`), whose gateway and network differ from Linux's, so the same knock there. Adoption is decided on
these answers, with a record of its own if it is yes. **Held** until someone with Linux and Podman, or the owner on a
Mac, can run it; the small fixes that do not wait are the sibling item.
