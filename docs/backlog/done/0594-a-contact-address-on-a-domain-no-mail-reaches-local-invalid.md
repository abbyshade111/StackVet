# A contact address on a domain no mail reaches (`.local`, `.invalid`, `example.com`) clears the placeholder-contact check

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: low.

**What happened.** The "no way to report a security problem" finding was cleared by writing `abigail@familygames.local`
into SECURITY.md, an address on a name that does not resolve. The check looks for placeholders by word, not by whether
the address could receive mail.

**Fix.** Treat `.local`, `.invalid`, `.test`, `.example`, `example.*`, and `localhost` as placeholders too, with the
reason in the finding; never try to send anything (the fence).

**Record.** None.

**Done, 15 October 2026** (documentation). `security_contact.rs` (`reserved`) now also counts a name on `.local`
("found only on the network it is on (RFC 6762), so no mail from outside reaches it") and a name beginning `example.`
under any ending ("one made for examples, almost certainly a stand-in"). `.test`, `.invalid`, `.example`,
`example.com`/`.org`/`.net`, and `localhost` were already counted. Nothing is sent: the check reads the file only.
Tests: `security_contact_unreachable_tests.rs` (2), with names that only look like one (`localgames.dev`,
`counterexample.org`) left alone; with the two new names switched off, the first fails.
