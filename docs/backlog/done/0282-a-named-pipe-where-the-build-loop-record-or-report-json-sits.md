# A named pipe where the build-loop record or report.json sits hangs the MCP server for good

**Status:** done, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: MCP server. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** The review of 8 October (item 6) added a guard so a pipe under `stackvet.toml`, `security-notes.md`, or the decisions file is refused before it is read (`mcp/confine.rs:37-53`, `READ_BY_NAME`). Two other files the server opens by name have no such guard. `append` refuses a link, checks the size (a pipe has length 0, so that passes), and opens with `create(true).append(true)` plus `O_NOFOLLOW`; opening a named pipe for writing waits until a reader appears. `stackvet_prompts` refuses a link at `report.json` and then `read_to_string`s it with no kind check and no size cap.

**Where.** `crates/sv-cli/src/build_loop.rs:306-330` (`append`), called for every tool call through `crates/sv-cli/src/mcp/mod.rs:714` and `:727` (`let _ = crate::build_loop::record(...)`); and `crates/sv-cli/src/mcp/tools.rs:363-380` (`stackvet_prompts` with `path`) reading through `crates/sv-cli/src/lib.rs:1232` (`prompts_for_report`).

**Evidence.** `append`: `options.create(true).append(true); … options.open(&path)` with no `is_file()` check. `tools.rs:371-376`: only `!meta.file_type().is_symlink()` is checked. By contrast `read_resource`, `refuse_older_than`, `read_holder_file`, `report_marker_in`, and `read_record` all check `is_file()`.

**How it bites.** The AI coding tool, or anything it ran, does `mkfifo app/stackvet-report/build-loop.jsonl`. The next call to any tool, even an unknown name, blocks inside `append` on the server's one request thread; the 50-second check limit does not apply, since the record is written after the check; the error would be swallowed anyway; every later request queues behind it. The server is dead until the owner kills it. The same with `mkfifo app/stackvet-report/report.json` and a `stackvet_prompts` call with `path`.

**Fix.** In `build_loop::append` and in `tools.rs` before `prompts_for_report`, refuse anything whose `symlink_metadata` is not `is_file()`, with the sentence `confine.rs:44-52` already uses; cap `prompts_for_report`'s read at `MAX_RESOURCE_BYTES`; add a case for each name to `crates/sv-cli/src/mcp/fifo_tests.rs`, which covers `stackvet.toml` only today.

**Record.** ADR-076 governs `build_loop.rs` and ADR-066 the server: a Later line on ADR-076 (the record is written only as a plain file) or an "unchanged, because" line.
