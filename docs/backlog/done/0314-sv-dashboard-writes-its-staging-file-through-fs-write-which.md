# sv dashboard writes its staging file through fs::write, which follows a link

**Status:** done, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: secrets. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** The staging file `.<name>.sv-<pid>` is written with `std::fs::write`, which follows a link at that name; every other file `sv` writes uses `create_new` (`write_without_following`, `history::private_file`).

**Where.** `crates/sv-cli/src/main.rs:1970-1976`.

**Evidence.** `std::fs::write` at `main.rs:1970-1976` where the rest of the tree uses `write_without_following`.

**How it bites.** The target is refused inside any app folder, so the folder is the owner's choice; a planted link in a shared folder (`/tmp`, a world-writable project parent) with a guessed pid would be written through.

**Fix.** `write_without_following(&parent, name, page.as_bytes())`; a test with a link at the staging name.

**Record.** ADR-017 governs what `sv` writes and never through a link: an "unchanged, because" line (this brings the dashboard under the rule).
