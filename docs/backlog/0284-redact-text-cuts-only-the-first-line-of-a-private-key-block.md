# redact_text cuts only the first line of a private-key block, so the key's body is kept

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: secrets. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** The base64 body after the header is left as written wherever `redact_text` is the only guard: `seen.json` bodies, log tails and tool output (`crates/sv-cli/src/seen.rs`), the failing-test tail (`suite.rs:92`), adapter messages (`adapters.rs:2424`, `:2444`), container lines (`sv-run/src/docker.rs:2380`). No test passes a key block through `redact_text`; the only block tests (`secrets/tests.rs:1654-1660`) are scan tests.

**Where.** `data/secret-rules.json` rule `secrets.private-key-block` (matches the `-----BEGIN … PRIVATE KEY-----` header alone, by design for the scan); `crates/sv-check/src/secrets.rs:965-971` (`redact_text_in` reuses the same single-line matches for redaction).

**Evidence.** `for (_, re) in &rules.rules { for m in re.find_iter(text) { … spans.push((m.start(), m.end())); } }` with the rule's pattern ending at the header's closing `-----`.

**How it bites.** The app prints its configuration at startup, PEM included; `sv report --run` keeps the log tail in `seen.json` with the header shown as redacted and the key body intact. `sv bundle` re-scans with the same rules and sees nothing, so it zips it.

**Fix.** In `redact_text_in`, when the block rule matches, extend the span to the matching `-----END …-----` or the end of the text; add the test ADR-082 asks for with a header-plus-body built from pieces at run time.

**Record.** ADR-082 governs `seen.json`'s redaction: a Later line.
