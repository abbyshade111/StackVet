# Every sv sign-up was refused by the app's own username rule, and the gap quotes nothing: nine requirements stopped

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the clean Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: medium (the same family as 0527: the
one line a person needs).

**What happened.** The app limits usernames to 3 to 30 characters; sv's stand-in names are 38-character addresses, so
every sign-up answered with "Username must be between 3 and 30 characters", and nine requirements stopped with a gap
that does not say why.

**Fix.** Quote the page's visible refusal in the gap (as 0527 does for sign-in); and let the stand-in names fit a stated
rule: when the manifest's sign-up form names a `username` field, sv makes a short name (a `{user}` under 20
characters) and keeps the address for the email field. Fake-app case: a sign-up that refuses long names.

**Record.** None.

**Built, 15 October 2026 (honesty):** when the sign-up form carries `{user}` in a `username` field and in another field
(the address), every `username` field in the run is sent a short name made from the address (`sv` and twelve hex
digits, fourteen letters and digits), and the address stays where it is. A `username` that is the form's only
`{user}` is how the app knows the account, and keeps the address. The log checks find a line by the short name as by
the address. When the first test user cannot open a private page, the gap now also says what the sign-up answered and
quotes its page (the field names only, never a value, and nothing when the page repeats the password). Fake-app case:
`usernames_up_to_30`. Tests in `crates/sv-check/src/signed_in/short_names_tests.rs`.

**Closed by the board, 15 October 2026:** built by honesty and merged (PR 2087); left open for the board to close.
