# The old-name notice says where the new folder is, once, and does not read as an error

**Status:** open

Found by the feedback on the Cursor build (11 October 2026). After the SecureVibe-to-StackVet rename (backlog 0218), `sv`
prints a note on every invocation when a config folder still has the old name: "`/Users/.../.config/securevibe` was read
under its old name ... rename it to `.../stackvet` when you choose." The owner and a new user both read it as an error.

What to do: say it once, not on every invocation, and say where the new folder is and that nothing is broken. The notice itself is described in the rename decision (ADR-062);
check whether this change alters that record before building it.
