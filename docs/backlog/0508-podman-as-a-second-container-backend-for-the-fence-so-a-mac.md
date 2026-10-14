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
