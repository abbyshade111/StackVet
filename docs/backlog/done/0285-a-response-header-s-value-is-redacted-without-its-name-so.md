# A response header's value is redacted without its name, so only the nine listed headers lose a random token

**Status:** done, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: secrets. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** `is_masked_name`, which knows `session`, `token`, `auth…`, never sees the header's name; only the nine names in `SESSION_HEADERS` (set-cookie, cookie, authorization, x-api-key, x-auth-token, x-csrf-token, and three more) are cut by name. The existing test (`seen_tests.rs:346-351`) uses a vendor-shaped key under `x-debug-key`, which the pattern rules catch regardless of name, so it does not exercise this.

**Where.** `crates/sv-cli/src/seen.rs:144-154` (`cut(&value)` on the value alone); `SESSION_HEADERS` at `:19-29`.

**Evidence.** `let value = if SESSION_HEADERS.contains(&name.as_str()) { session_value(name, value) } else { value.clone() }; (name.clone(), cut(&value))`

**How it bites.** The app answers with `X-Session-Token`, `X-Access-Token`, `X-Refresh-Token`, `X-Amz-Security-Token`, or `Refresh-Token` holding a random token; it is written to `seen.json` as is, in anonymous and signed-in exchanges alike.

**Fix.** Redact `format!("{name}: {value}")` and keep the tail, or expose `is_masked_name` and cut the whole value when the header's name says session, token, auth, key, or secret; a test with a random (non-vendor) token under `x-session-token`.

**Record.** ADR-082 governs `seen.json`: a Later line.
