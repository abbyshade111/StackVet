# The download-name check does not try a file name with a quote in it

**Status:** open

From the inventory of sweep 3 (backlog 0561, ADR-119), item 8; read from the code, not yet shown by a test.

**What is wrong.** `uploads.rs` `download_name_checks` (V5.4.2) sends a file named `sv-probe;svinjected=1.gif` and
credits a name served back with the `;` kept inside it. An app that wraps the name in quotes without escaping `"` keeps
the `;` inside the quotes and passes, but a name with a `"` in it ends the quoted string and starts a parameter of its
own.

**What to build.** Two requests more: upload `sv-probe";svinjected=1.gif` and fetch it back from `serves-at`; a
`Content-Disposition` with a parameter `svinjected` is the finding. A fake-app case that quotes without escaping.
