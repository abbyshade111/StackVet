# A commit message can forge or hide records in the note of AI commits outside a merge

**Status:** claimed by builder-full, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: secrets. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** `%B` is the whole body, which may itself contain both separator bytes.

**Where.** `crates/sv-check/src/git.rs:193-221` (`--format=%H%x1f%s%x1f%B%x1e`, split on U+001E and U+001F); `config.rs:278-310` prints the hash prefix, subject, and tool into the finding.

**Evidence.** A body holding `\x1e<fake hash>\x1f<fake subject>\x1fAssisted-by: …` adds a record with a made-up hash and subject; a `\x1e` placed before the real trailer makes the real commit vanish from the list.

**How it bites.** The note ADR-092 describes can be made to name commits that do not exist, or to miss ones that do.

**Fix.** Accept a record only when its hash is 40 hex characters, and read the body with a second `git show -s --format=%B <hash>` per listed commit (or strip 0x1e/0x1f from the body before the split and reject malformed records); a test with a planted body.

**Record.** ADR-092 governs this note: a Later line or an "unchanged, because" line.
