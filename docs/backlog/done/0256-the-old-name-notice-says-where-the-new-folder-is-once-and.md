# The old-name notice says where the new folder is, once, and does not read as an error

**Status:** done, 12 October 2026

Found by the feedback on the Cursor build (11 October 2026). After the SecureVibe-to-StackVet rename (backlog 0218), `sv`
prints a note on every invocation when a config folder still has the old name: "`/Users/.../.config/securevibe` was read
under its old name ... rename it to `.../stackvet` when you choose." The owner and a new user both read it as an error.

What to do, as the owner asked: the only person who has run `sv` under the old name is the owner, so the old name
needs one mention and no note on every run. The README already has that mention, in its paragraph on the old names
(around line 527), which says the old files are still read and that the report says once what to rename. Remove the
per-run note. The owner decided, 11 October 2026, to keep the report's one-time line; the README and the report both say the
same thing once, and nothing else mentions it. The notice is described in the rename decision (ADR-062); check whether removing the per-run note alters
that record before building it.

**Done 12 October 2026** (ADR-062, "Later, 12 October 2026"): the per-run note in `sv review` is removed; the old key
folder is still read, and nothing moves a key. The README's paragraph and the report's one-time line stay.
`crates/sv-cli/tests/review_old_folder.rs` now asserts the old folder in use is read and not said, and that the review
really ran at a terminal, so its silence means something.
