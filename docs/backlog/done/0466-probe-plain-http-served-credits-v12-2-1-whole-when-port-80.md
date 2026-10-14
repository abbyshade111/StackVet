# `probe.plain-http-served` credits V12.2.1 whole when port 80 refuses the connection and in part when it redirects

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: requirements and tests (citations). Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/production.rs:1092-1099` (`if !plain.reached() { out.verified.push(Verified::new(PLAIN_HTTP_SERVED…` "refused the connection, so there is no plain-HTTP way in", no `.in_part()`) against `:1136-1146` (the 301/308 branch, `.in_part()` since ADR-090).
- What is wrong: V12.2.1: "Verify that TLS is used for all connectivity between a client and external facing,
  HTTP-based services, and does not fall back…". Both branches read one host's port 80 once; neither sees an API
  host, another port, or the app's own outbound calls. The refusal is stronger evidence than the redirect, but it
  is the same one sample, and the two strengths disagree for the same requirement.
- What to do: `.in_part()` on the refusal branch too, with the same wording as the redirect branch; a test that
  the refused branch is credited in part (`production/tests.rs` covers the branch; assert `in_part`).
- How verified: read.
- Overlap: none found (`grep -rli "plain-http-served\|refused the connection" docs/backlog/` is empty).
- Needs a decision record?: ADR-053's Later of 10 October listed TLS among the excluded checks, so a Later line
  there saying this one is in part; or an "unchanged, because" line if the owner keeps the exclusion.

**The owner's answer, 14 October 2026** (asked through the board's waiting line while the owner was away; answered as the session recommended): in part. The refused-connection credit for V12.2.1 is the same single sample of one host's port 80 as the redirect, which is already in part; ADR-090's "kept whole" of 11 October is superseded for this branch, and ADR-053 (or ADR-090) gets a Later line saying so.
