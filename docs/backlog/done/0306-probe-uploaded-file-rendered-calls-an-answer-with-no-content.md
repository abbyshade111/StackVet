# probe.uploaded-file-rendered calls an answer with no Content-Type safe

**Status:** done, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** `content_type = fetched.header("content-type").unwrap_or_default()`; `not_html = !content_type.contains("text/html") && …`; `is_safe = attachment || sandboxed || not_html`. An uploaded `.html` served back with no Content-Type header at all is credited V3.2.1 as "something other than a page", although a browser sniffs a type-less response and renders the HTML.

**Where.** `crates/sv-check/src/signed_in/uploads.rs:1155-1163`.

**Evidence.** `unwrap_or_default()` making the missing header an empty string that `not_html` accepts.

**How it bites.** V3.2.1 ("prevent browsers from rendering content ... in an incorrect context, e.g. when a user-uploaded file is requested directly") credited for the exact case it names.

**Fix.** Treat a missing or empty Content-Type as served as a page (or not assessed), and count `X-Content-Type-Options: nosniff` only together with a non-HTML type; a fake-app case with no Content-Type.

**Record.** No record governs `uploads.rs` by name; say so.
