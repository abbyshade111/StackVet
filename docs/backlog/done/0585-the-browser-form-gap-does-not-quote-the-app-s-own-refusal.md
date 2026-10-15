# The browser-form gap does not quote the app's own refusal ("The form was out of date"), so a person cannot tell a token problem from a bug

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Opus run's reading (backlog 0577), 15 October 2026. Severity: low (presentation; the same family as 0527).

**Fix.** When the browser check's form post is refused, the gap line quotes the page's visible message, as 0527 does
for a failed sign-in.

**Record.** None.

**Done, 15 October 2026** (documentation). When the browser's form post is answered 400 or more, the driver now also
gives back the words the page showed (`browser-driver.mjs`, `after.said`, cut to 2,000 characters), and the gap line
quotes them, cut to 300, with no character that moves or hides text on a terminal (`browser.rs`, `page_said`): "the
text typed into /notes/new was refused (403), and the page said: “The form was out of date…”". Without words, the line
says only the status, as before. Tests: `browser/tests/refused_said_tests.rs` (2); with the quote taken out, the one
that needs it fails. Not tried in a real browser here: Docker is installed in this session but has no engine running.
