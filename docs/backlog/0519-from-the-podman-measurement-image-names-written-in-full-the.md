# From the Podman measurement: image names written in full, the backend check by Podman's own field, and the container-gone wording

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Split from backlog 0508 (done 14 October 2026, the owner's word) as the part worth building whether or not Podman is
ever a backend. Three small fixes the measurement found, each with a test:

- **Image names written in full.** `sv` names helper images by short name (`chromedp/headless-shell`, `node`,
  `busybox`); Podman resolves a short name only through its alias list or a search registry, so the browser helper did
  not start there. Write every image name in full (`docker.io/chromedp/headless-shell` and the rest) wherever `sv`
  pulls or runs one, which is what Docker does silently anyway; the pinned digests stay as they are (ADR-019).
- **The backend check reads Podman's own field.** `sv_run::detect` asks `docker info` for the fields Docker gives;
  Podman answers the same command with a different shape, and `sv` should say "Podman, which sv does not run the app
  under yet" rather than a reading that happens to pass or fail.
- **The container-gone wording.** Where a container that has already gone is reported, the words are Docker's
  (`No such container`); say it in `sv`'s own words whatever the backend said.

Nothing changes what the fence guarantees or what a report concludes. `ADR-019: unchanged, because` the images and
their digests stay what they are; say so in the pull request.
