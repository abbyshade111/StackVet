# Commit subjects and trailers are quoted into the AI-commits note unredacted, and a key there cannot be taken out of history

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-check`. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/config.rs:322-335` (`format!("`{}` {} ({})", hash, c.subject, c.tool)` into the
  finding's description), `crates/sv-check/src/git.rs:235-255` (`subject` and `tool` cleaned of control characters
  only); `crates/sv-cli/src/lib.rs:1739-1771` (the bundle's refusal, whose advice is "take the credential out of the
  place it was quoted from").
- What is wrong: `sv`'s own findings otherwise carry a credential only as a `Secret`, and a tool's text is redacted as
  it is read; this is the one new place app text (up to ten subjects and trailers, of any length) goes into
  `report.html`, `security.md`, and `report.json` as it is. A key pasted into a commit subject ("Add Stripe key
  sk_live_…", which AI tools do write) reaches every report file. The bundle step then refuses the zip, and its
  advice cannot be followed: the subject is in the history, which the owner is told never to rewrite.
- What to do: pass `subject` and `tool` through `crate::secrets::redact_text(rules, …)` and cap each at a short
  length (the `QUOTE` = 160 rule `ai_tool::quote` uses); `check_dir_in` already has `rules` for `ai_tool::read`.
  Must not lose: the hash prefix (never a secret) and the tool's name.
- How verified: read.
- Overlap: 0313 (done; forged or hidden records, fixed by `records_by_hash`), 0283 (done; the AI tool's settings).
  Neither covers the subject's content; new.
- Needs a decision record?: no (ADR-092 unchanged: what is read stays the same, only how it is shown).
