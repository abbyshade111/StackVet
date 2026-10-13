# Every file of the app is read from disk and decoded five to six times across the static stage

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** Each read is a `fs::read`, a zero-byte scan, and two UTF-8 validations (`holds_no_written_text` at `files.rs:233` and `decode` at `:110`).

**Where.** `Listing` holds no contents (`crates/sv-scan/src/files.rs:286-305`); `Entry::read_text` (`:254`) reads and decodes on every call. Readers of every code file in one `sv check`: `sv_scan::scan_listing_app` (`crates/sv-scan/src/lib.rs:275`, then `.to_lowercase()` of the whole file at `:296`), `secrets::scan_listing` (`secrets.rs:1070`), `launch::mcp_servers` (`launch.rs:744-748`), `cert_checks::check` (`cert_checks.rs:157-167`), `public_keys::check` (`public_keys.rs:33-37`), `ast::scan_listing` (`ast.rs:1801`); and `ast::note_callers` re-reads and re-parses every Python file (`ast.rs:1495-1507`) whenever any parameter-destination finding exists.

**Evidence.** The six call sites.

**How it bites.** Estimate: on a 50 MB app, about 300 MB read and 600 MB validated, 1 to 2 s, more on a cold cache. Time, not memory: each pass drops the text.

**Fix.** Give `Entry` a memoized text (`OnceLock<Result<Arc<str>, Unread>>`) living only for the static stage, bounded by the 2 MB per-file cap plus a total cap (say 256 MB, past which it reads as now); or fuse the three whole-tree readers and the secrets scan into one read-per-file loop. For `note_callers`, keep the parsed Python trees from `scan_listing`. A `#[cfg(test)]` read counter in `Entry::read_text` and a test that the static stage reads each file of a small fixture once; disable the cache and watch it fail.

**Record.** ADR-031 governs what the walk leaves out, not how often a file is read: an "unchanged, because" line.

**Measured, 13 October 2026, and what was built.** Release builds on a 4-core Linux container, over Python 3.12's
standard library as an app (571 code files, 10 MB):

- Reading and decoding every code file once (`Entry::read_text`): 63 ms from a cold disk, then 5 to 7 ms. Six reads of
  a 50 MB app from the page cache come to about 0.2 s, not 1 to 2 s, against a static stage whose AST read alone takes
  about 8.6 s for these 10 MB (after 0353). The memoized text was not built: it would hold up to 256 MB to save a few
  per cent at most.
- `note_callers` re-read and re-parsed every Python file whenever one parameter-destination finding existed: 1.5 to
  1.7 s here, whatever the function. It now parses only the files whose text holds one of the pending functions'
  names, which is exactly the files `calls_of` can find anything in, since it reads only identifiers spelled that way:
  6 ms for a name few files hold, 33 ms for `urljoin`, which many here do. The parsed trees from `scan_listing` were
  not kept, as the item proposed: that would hold every Python file's tree for the whole scan, to save what the text
  check now saves.
