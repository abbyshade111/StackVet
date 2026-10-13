# mail() waits a fixed 10 s per flow when the app sends no mail

**Status:** claimed by builder-none, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** An app with a mail sink configured but a flow that sends nothing waits the full 10 s each time.

**Where.** `crates/sv-run/src/docker.rs:1209-1223`: polls `/api/v1/messages?limit=500` every second (one exec per poll) until enough messages or `MAIL_WAIT_SECONDS = 10`; callers with `at_least = before + 1` at `reset.rs:69`, `:236`, `codes.rs:628`, `passwords.rs:949`, `activation.rs:81`, `mod.rs:2439`.

**Evidence.** The loop and its callers.

**How it bites.** Estimate: up to about 60 s per run.

**Fix.** Nothing for correctness (mail is asynchronous); shorten the wait when the app's own answer already said no mail was sent (a 4xx), and fetch the listing once per suite. Build only if the request timings in a real report show it.

**Record.** No record governs this by name; say so.
