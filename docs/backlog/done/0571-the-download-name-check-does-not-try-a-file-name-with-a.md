# The download-name check does not try a file name with a quote in it

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the inventory of sweep 3 (backlog 0561, ADR-119), item 8; read from the code, not yet shown by a test.

**What is wrong.** `uploads.rs` `download_name_checks` (V5.4.2) sends a file named `sv-probe;svinjected=1.gif` and
credits a name served back with the `;` kept inside it. An app that wraps the name in quotes without escaping `"` keeps
the `;` inside the quotes and passes, but a name with a `"` in it ends the quoted string and starts a parameter of its
own.

**What to build.** Two requests more: upload `sv-probe";svinjected=1.gif` and fetch it back from `serves-at`; a
`Content-Disposition` with a parameter `svinjected` is the finding. A fake-app case that quotes without escaping.

**Built, 15 October 2026 (honesty):** the second name is sent and fetched back when the first came back whole; broken
by the quote, it is the finding; kept whole, the credit says both. The fake app now escapes a quoted name as a quoted
string must, and a new flaw quotes it without escaping. Left open for the coordination board to close, since 0571 is
on honesty's line there.

**Closed by the board, 15 October 2026:** built by honesty and merged (PR 2015); honesty left the item open for the board to close, as its line's items are.
