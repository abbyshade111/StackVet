# An SVG refused by its extension is not tried again named as a GIF

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the inventory of sweep 3 (backlog 0561, ADR-119); two or more requests, filed rather than built there.

**What is wrong.** `uploads.rs` `svg_check` (V1.3.4) credits an SVG refused by its `.svg` name. An app that judges by
the name alone would take the same SVG named `.gif` and may serve it as an image the browser runs.

**What to build.** Upload the same SVG named `.gif`, fetch it back from `serves-at`, and read what it is served as;
served as `image/svg+xml` or sniffed as one, the refusal was only of the name.
