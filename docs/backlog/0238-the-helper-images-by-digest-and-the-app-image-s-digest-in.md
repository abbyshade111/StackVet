# The helper images by digest, and the app image's digest in the report (observability review, part 3, K)

**Status:** partly done: the digests are not yet checked by a docker pull on a computer with Docker running; the app image's digest is read from the local Docker and is empty when Docker is not running

The owner's word, 10 October 2026, asked with the three other open decisions of that day: yes (part 3, K, of backlog 0226).

The helper images `sv` runs (`busybox`, `mailpit`, `node`, the headless browser) named by digest, not by tag, and the
app image's digest said in the report, so a run is repeatable and a moved tag cannot change what runs. Changes what
`sv` runs, so its record is written with the build. Small.

**Built 10 October 2026:** the four helper images `sv` runs (busybox, Mailpit, the Node provider, the headless browser) are named by digest as well as tag (`crates/sv-run/src/docker.rs`, `HELPER_IMAGES`). The digests are the registry's own manifest digests for those tags, read from Docker Hub's manifest headers on 10 October 2026: no local Docker was running, so they are not yet checked by a `docker pull` on this computer. The run record now lists the helper images, and the app's own image digest when the app was run, read from the local Docker only (`image_digest`). Tests: `docker_images_tests.rs`; broken on purpose by removing one digest, it failed.
