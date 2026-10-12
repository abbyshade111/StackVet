# sv probe reads curl's output with no size cap

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: the fence. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** `sv run` caps every command's output at 32 MB (`sv-run/src/lib.rs`, `OUTPUT_LIMIT`); `sv probe` does not. `--head` keeps only headers, but a hostile site can stream headers for the full `--max-time 15`, and `%{certs}` adds the chain.

**Where.** `crates/sv-check/src/production.rs:1289-1296` (`Command::new(curl_program()).args(...).output()`), the same in `stapled` and `old_tls`.

**Evidence.** `.output()` with no capped reader.

**How it bites.** A site answering with endless header lines fills memory for 15 seconds per request.

**Fix.** Read curl's stdout through a capped reader as `read_capped` does, or run it through `sv-run`'s bounded output; a test with a fake curl that never stops.

**Record.** ADR-027 governs `sv probe`: an "unchanged, because" line (the request count and addresses are untouched).
