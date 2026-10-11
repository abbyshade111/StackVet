# The old-name notice says where the new folder is, once, and does not read as an error

**Status:** open

Found by the feedback on the Cursor build (11 October 2026). After the SecureVibe-to-StackVet rename (backlog 0218), `sv`
prints a note on every invocation when a config folder still has the old name: "`/Users/.../.config/securevibe` was read
under its old name ... rename it to `.../stackvet` when you choose." The owner and a new user both read it as an error.

What to do, as the owner asked: the only person who has run `sv` under the old name is the owner, so the rename
needs one mention, in the README, and no note on every run. The README already explains the old names at its section
on the old names (around the line about 11 October's rename, which says the old files are still read and the report
says once what to rename). Remove the per-run note, and keep the one README mention. Whether the report's one-time line
stays is the owner's call, since the README is the one place the owner asked for. The notice is described in the rename
decision (ADR-062); check whether removing it alters that record before building it.
