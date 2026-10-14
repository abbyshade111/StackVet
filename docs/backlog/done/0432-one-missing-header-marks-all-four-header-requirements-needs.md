# One missing header marks all four header requirements *needs attention*, and one missing cookie attribute marks both cookie requirements

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: requirements and tests (citations). Severity: medium. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/probes.rs:487-495` (`SECURITY_HEADERS`, `requirement_ids: &["V3.4.3", "V3.4.4", "V3.4.5", "V3.4.6"]`),
  `:1314-1340` (`security_headers`: one `finding_on(…, &SECURITY_HEADERS, …)` whatever `missing_headers` listed);
  `:1352-1363` and `:1365-1404` (`COOKIE_ATTRIBUTES`, V3.3.2 and V3.3.4, one finding for a cookie lacking
  HttpOnly or SameSite or both); the same two shapes signed in: `crates/sv-check/src/signed_in/sessions.rs:1067`
  (`PRIVATE_PAGE_HEADERS`) and `:1503` (`SESSION_COOKIE`).
- What is wrong: V3.4.3 is the Content-Security-Policy's minimum, V3.4.4 is `X-Content-Type-Options`, V3.4.5 the
  referrer policy, V3.4.6 `frame-ancestors`. An app whose policy meets its minimum and whose only lapse is a
  missing `Referrer-Policy` gets one finding naming all four, and `status_of` (`crates/sv-report/src/lib.rs:2710`)
  makes V3.4.3, V3.4.4, and V3.4.6 *needs attention* for a header each of them has. Likewise a session cookie
  with `HttpOnly` and no `SameSite` marks V3.3.4 ("the cookie must have the 'HttpOnly' attribute set") as
  failing. The credit side is per rule too, but a credit is only given when nothing is missing, so it is right;
  it is the finding that names more than it found. The 12 October review's F-rule (d), "the same requirements
  on pass and on fail", is met by construction here, which is why it did not catch this: it is (e), several
  clauses, on the finding side.
- What to do: build the finding's `requirement_ids` from what was missing (CSP minimum → V3.4.3,
  `X-Content-Type-Options` → V3.4.4, `Referrer-Policy` → V3.4.5, `frame-ancestors` → V3.4.6; `HttpOnly` → V3.3.4,
  `SameSite` → V3.3.2), keeping one rule id and one finding per page; or split into one finding per header. Must
  not lose: the rule's `requirement_ids` as the set a clean answer credits, the evidence ids, and the page named
  in the text. A test: a page missing only `Referrer-Policy` yields a finding whose ids are `["V3.4.5"]`
  (`crates/sv-check/src/probes/tests.rs` asserts ids on other findings at `:1027`, `:1496`, `:1686`; none on these).
- How verified: read.
- Overlap: 0264 (done) is V3.4.6 and `X-Frame-Options`, the judgment of one header; 0147 (done) mentions
  `Referrer-Policy` in another sense. Nothing on a finding naming requirements it did not find failing.
- Needs a decision record?: yes: ADR-047 governs `probes.rs`'s list of missing headers; a Later line, since it
  changes what a report concludes for three requirements.
