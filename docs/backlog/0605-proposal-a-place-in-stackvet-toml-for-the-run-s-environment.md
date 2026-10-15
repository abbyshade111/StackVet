# Proposal: a place in stackvet.toml for the run's environment file and settings files, so loosened settings there are judged like the start command's

**Status:** claimed by honesty, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Split from backlog 0564 on 15 October 2026 (honesty built the start-command half; this is the part left). A proposal
with a decision for the owner, since it adds manifest keys and changes what sv reads.

**What was seen.** 0564 judges a setting the start command loosens for sv's run (cookies, CSRF, HSTS and HTTPS, CSP,
two-factor codes, rate limits, authentication) and takes out the credits it bears on. A setting loosened in an
environment file the start command sources (`--env-file`, a `.env`) or in a settings file the app reads is not seen,
because `[stack.run]` has no place to name either, and only `start` gives the copy its settings.

**The choice.** (a) Add `env-file` (a path) and `settings` (a list of files) under `[stack.run]`; sv reads them for the
same loosening words, judging as 0564 does, and the spec's comments say so. (b) Read only what the start command
itself names (`--env-file X`, `source .env`), the one case that needs no new key, and say in the report that other
settings files are not read. (c) Leave it: the report says settings files are not read. The orchestrating session
recommends (b) first, since it needs no decision about the manifest, and (a) later if a real app's loosening hides
in a file the start command does not name.

**Record.** ADR-114 Later for (b); a Later entry on the manifest's record as well for (a).

**The owner's answer, 15 October 2026:** (b): read only what the start command itself names (an --env-file, a sourced .env), with that file's format read first, and say in the report that other settings files are not read; no new manifest key. (a) later only if a real app's loosening hides in a file the start command does not name. ADR-114 Later.

**Built, 15 October 2026 (honesty):** option (b). The environment files the start command itself reads
(`--env-file`, a sourced `.env`) are read from the app's folder, for their format first, and each setting there that
loosens the copy takes out the credits it bears on, as 0564 does for the start command. The run note says which
settings files were read and that no other is. ADR-114, Later entry of 15 October 2026.
