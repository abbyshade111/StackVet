# Five base64 implementations in two crates, with two decoders that disagree on the alphabet

**Status:** claimed by documentation, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: low. Verified against the code by the reviewing session before filing.

- Where: encoders `crates/sv-check/src/ssh_format.rs:367` (`pub(crate) fn base64(bytes, pad: bool)`, standard
  alphabet), `crates/sv-check/src/browser.rs:572` (`pub(crate) fn base64(bytes, url_safe: bool)`, never padded),
  `crates/sv-run/src/docker.rs:2737` (`fn base64(input)`, standard, padded); decoders `crates/sv-check/src/ssh_format.rs:387`
  (`unbase64`, standard, strict), `crates/sv-check/src/signed_in/tokens.rs:108` (`unbase64`, web alphabet only, pads
  trimmed), `crates/sv-check/src/public_keys.rs:163` (`base64url`, accepts `-`/`+` as 62 and `_`/`/` as 63, stops at
  `=`).
- What is wrong: the same 20 lines three times for encoding and three strictnesses for decoding. The two web
  decoders differ: a JWT part written in the standard alphabet is read by `public_keys.rs` and refused by
  `tokens.rs`, so the same token can be "decoded" by one check and "not a token" to another. Each copy carries its
  own test (`ssh_format.rs:550`, `browser.rs:2064`, `docker.rs:5095`).
- What to do: keep `ssh_format.rs`'s pair (strict, tested one way only) extended with an alphabet flag, make it
  `pub` under a small `sv_check::encoding` module, and point `browser.rs`, `browser_storage.rs:253`, `tokens.rs:104`,
  `tokens.rs:108`, `public_keys.rs:163`, `fake_app.rs:777`, and `docker.rs:2737` (sv-run already depends on
  sv-check) at it. Decide and write down which alphabet a JWT part may use (RFC 7515 says base64url only; then
  `public_keys.rs` is the one to tighten). Keep every existing test, moved to the one module.
- How verified: read.
- Overlap: none found ("base64" in the backlog is about other things: 0284, 0287, 0147).
- Needs a decision record?: no.
