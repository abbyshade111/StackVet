# Prompt 6: a sandboxed shell, a kept copy of sv, and a commit that must match the pulled source

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): Antigravity (needed `BypassSandbox` for brew,
Docker, `sv report`, and `open`), Cursor (found a kept copy behind a broken link and hand-built in the owner's
checkout; the first install copied a stale binary). For the prompt library (0370).

**What to change.** Prompt 6: "If a command fails with 'operation not permitted', tell me and run it outside your
sandbox. If you find an older or kept copy of StackVet, tell me and install with Homebrew unless I say otherwise.
After installing, `sv --version` must name the commit you expect; if not, stop and tell me."

**Done, 15 October 2026:** prompt 6 now says, in its opening, "If a command fails with "operation not permitted", or
your own sandbox keeps it from running, tell me, and run it outside your sandbox only after I say yes." The item had
the tool run it outside the sandbox on its own; the prompt's rule that it asks before running anything holds there
too, so it asks. Step 1 adds "If you find an older or kept copy of StackVet, or a link to one, tell me, and install
with Homebrew (Scoop on Windows) unless I say otherwise." Step 2 adds "After installing or updating, `sv --version`
must name the commit you expected, the latest on StackVet's GitHub page: if it does not, stop and tell me." It names
the commit step 2 already compares with; a copy built from a ZIP prints `commit unknown` (`docs/INSTALL.md`), which
names none, so the tool stops there too. `crates/sv-cli/tests/setup_prompt_record.rs` holds the three sentences; with
the page as it was, it went red.
