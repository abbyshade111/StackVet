# resources/read answers differently for a folder that does not exist and one outside the root, which confine.rs deliberately avoids

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: MCP server. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** `app_dir` in `confine.rs:19-28` gives one answer for a path that does not exist and one outside the root, so nobody can learn through the server which folders exist elsewhere on the computer (the deep review's improvement 7). `read_resource` canonicalizes the URI's parent first and answers `no such folder`, and only then checks the root and answers `that is outside the folder`.

**Where.** `crates/sv-cli/src/mcp/resources.rs:98-106` (`read_resource`).

**Evidence.** `let folder = path.parent().and_then(|p| p.canonical().ok()).ok_or_else(|| not_found("no such folder"))?; if !folder.starts_with(&self.root) { return Err(not_found("that is outside the folder …")); }`

**How it bites.** `resources/read` with `file:///Users/x/.ssh/report.json` answers one way and with `file:///Users/x/.sshx/report.json` the other: an existence oracle for any folder the owner can stat. Existence only, never contents, and the AI tool on the same machine usually has other ways to look, which is why it is low; but the server's own rule is one answer.

**Fix.** Check the uncanonicalized parent `starts_with(root)` before canonicalizing, or return the same `not_found` text for both cases; a test with a folder outside the root and a folder that does not exist asserting the two answers are identical.

**Record.** ADR-034 governs `resources.rs`: an "unchanged, because" line (the seal is untouched).
