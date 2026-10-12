# Can sv run read the app container's CPU, memory, disk and egress from the host?

**Status:** done, 11 October 2026

From 0029, part 9 (C9.1.1). The AI probe records CPU, memory, disk and egress quotas as unchecked, because the app is judged from outside. `sv run` starts the app's container from the host, so the host may be able to read its resource use. Research first: find out whether the container backend reports these numbers for a running container and what it costs to read them. Build nothing until the answer is in this item.

## The answer, 11 October 2026 (research only; nothing built)

Tried on this Mac with Docker (the same backend `sv run` uses), on a throwaway busybox container with a limit of
half a CPU and 64 MiB, then removed:

- **Yes, for a running container.** `docker stats --no-stream` reports CPU as a percentage, memory as use against its
  limit, and network and disk traffic since the container started. It took about a second to answer.
- **The limits the container was started with** are in `docker inspect`: CPU as nano-CPUs (500000000 for half a CPU)
  and memory in bytes (67108864 for 64 MiB). A container with no limit shows zero.
- **Disk:** `docker container inspect --size` reports the bytes the container has written to its own layer, and the
  size of its base image. No disk quota was reported: the storage-option field was not present for this container, so a
  disk limit set outside `sv` would not show.
- **Egress is cumulative**, since the container started. A rate needs two readings and the time between them.

What it can and cannot say:

- It is the container's own numbers as the engine counts them, not the app's view from inside. It says nothing about a
  limit the app's own platform sets, on a host that is not this one.
- A run that finishes takes its numbers with it. The readings must be taken while the app runs, which `sv run` already
  does while it waits on the app.
- Cost is one short command per reading, a few per run. Nothing is sent anywhere; the numbers stay on the owner's
  computer.

For the owner to decide: whether `sv run` should take these readings during a run and write them to the report beside
the probe's unchecked quotas. That changes what a report says, so it needs a decision record in the pull request that
builds it, as the rules ask. Until then, the quotas stay unchecked.

## Built, 11 October 2026

The owner approved it the same day. Built in three steps: reading the engine's words (#1464), taking one reading while
the app runs (#1466), and saying it in the report and `sv run`'s printout (ADR-094). The report gives the container's
CPU, memory, network, and disk use and its limits, or says Docker did not say, and C9.1.1's note on the tools' quotas
says a reading was taken. No requirement's status changes: the readings are the whole container's, from outside, not
limits the app puts on its own tools, so the quotas stay not assessed.
