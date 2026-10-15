# An SVG refused by its extension is not tried again named as a GIF

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the inventory of sweep 3 (backlog 0561, ADR-119); two or more requests, filed rather than built there.

**What is wrong.** `uploads.rs` `svg_check` (V1.3.4) credits an SVG refused by its `.svg` name. An app that judges by
the name alone would take the same SVG named `.gif` and may serve it as an image the browser runs.

**What to build.** Upload the same SVG named `.gif`, fetch it back from `serves-at`, and read what it is served as;
served as `image/svg+xml` or sniffed as one, the refusal was only of the name.

**Built, 15 October 2026 (honesty):** when the `.svg` is refused, the same image is sent named `.gif`. Refused too:
the credit says so. Taken and served from `serves-at` as an SVG, XML, or HTML type with its script still in it: a
finding under the same rule, High, or Medium when served as an attachment or with a sandbox policy, and no credit.
Taken and served as another type, or with the script removed: the credit says what it was served as. Taken with no
`serves-at`, not fetched back, or served with no `Content-Type`: not assessed, saying why. Tests in
`crates/sv-check/src/signed_in/uploads/svg_named_gif_tests.rs`.

**Closed by the board, 15 October 2026:** built by honesty and merged (PR 2085); left open for the board to close.
