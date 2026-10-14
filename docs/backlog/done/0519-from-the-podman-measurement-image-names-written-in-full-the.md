# From the Podman measurement: image names written in full, the backend check by Podman's own field, and the container-gone wording

**Status:** done, 14 October 2026

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

**Built 14 October 2026, by honesty.** Each fix was measured against Podman 4.9.3 in the cloud session's container,
through a `docker` on the PATH that runs `podman`, as in 0508.
- **Image names in full.** `crates/sv-run/images/Dockerfile` names the four helpers with their registry first
  (`docker.io/library/busybox`, `docker.io/axllent/mailpit`, `docker.io/library/node`, `docker.io/chromedp/headless-shell`),
  each with the same tag and fingerprint, so CI fetches and `sv` runs the same images as before. The images an app's
  own `stackvet.toml` names are the owner's and are left as written.
- **The backend check.** Podman's answer to `docker info --format {{.OSType}}` fails on its own type
  (`*define.Info`); `sv` now says "the `docker` command here is Podman, which `sv` does not run the app under yet;
  the running-app checks need Docker" instead of passing Podman's template error on. `sv doctor` says so with
  Podman installed.
- **A gone container.** Recognized in any case (Podman writes "no such container"), and told in `sv`'s own words:
  "it had stopped, or been removed, before the request reached it".

Tests: `every_helper_image_names_its_registry` (with the names checked in `the_file_lists_the_helper_images…`),
and `crates/sv-run/src/docker/podman_words_tests.rs` with Podman's own answers. Broken on purpose, each put back: the
browser named short (2 red), the original case-sensitive check put back (1 red, the Podman test), Podman not named
(1 red).
