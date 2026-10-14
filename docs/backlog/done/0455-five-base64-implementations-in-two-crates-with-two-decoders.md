# Five base64 implementations in two crates, with two decoders that disagree on the alphabet

**Status:** done, 14 October 2026

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

**What was built (14 October 2026, session documentation).** One module, `crates/sv-check/src/encoding.rs`
(`sv_check::encoding`), holds base64 for the whole tree: `base64(bytes, alphabet, pad)` and `unbase64(text, alphabet,
padding)`, built from `ssh_format.rs`'s strict pair with an alphabet flag (`Standard` or `Web`) and a padding flag
(`Required`, as an SSH key or a PEM file is written, or `Optional`, as a web token's part is). Two named calls hold the
token rule: `token_part` and `read_token_part`. The five copies are gone: `ssh_format.rs`, `browser.rs`,
`browser_storage.rs`, `signed_in/tokens.rs`, `signed_in/fake_app.rs`, `public_keys.rs`, and sv-run's `docker.rs` all call
the module. Their tests moved to `crates/sv-check/src/encoding_tests.rs` with their expectations unchanged.

The alphabet a JSON Web Token's part may use, decided and written in the module's header: base64url only (RFC 7515,
section 2), so `public_keys.rs` is the one tightened. A Supabase key whose payload is written with `+` or `/` is no
longer read as a service-role key; `crates/sv-check/src/public_keys_alphabet_tests.rs` holds that, with its setup
asserted (the payload really does differ between the alphabets). Breaking the rule two ways turned the tests red: the
key reader set to take either alphabet failed the new test; the web decoder set to take `+` and `/` failed it and
`encoding_tests.rs`'s own test. The signed-in token checks have no test of their own with a standard-alphabet part;
the module's test covers them, since they read through the same call.

Two smaller changes in what is read: a token part is now read strictly, as an SSH key always was, so a part whose last
group carries bits no byte uses, or whose padding is not a whole group (`YQ=`), is not a token's part. RFC 4648
(section 3.5) has an encoder set those bits to zero, so a token a conforming library wrote reads as before. Not done: three test-only copies kept their own encoders and
decoders (`secrets/tests.rs`, `public_keys.rs`'s `b64url` in its tests, and `crates/sv-run/tests/oidc_provider.rs`),
as an outside judge of what the module writes rather than the module checking itself.
