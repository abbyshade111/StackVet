# A contact address on a domain no mail reaches (`.local`, `.invalid`, `example.com`) clears the placeholder-contact check

**Status:** claimed by documentation, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: low.

**What happened.** The "no way to report a security problem" finding was cleared by writing `abigail@familygames.local`
into SECURITY.md, an address on a name that does not resolve. The check looks for placeholders by word, not by whether
the address could receive mail.

**Fix.** Treat `.local`, `.invalid`, `.test`, `.example`, `example.*`, and `localhost` as placeholders too, with the
reason in the finding; never try to send anything (the fence).

**Record.** None.
