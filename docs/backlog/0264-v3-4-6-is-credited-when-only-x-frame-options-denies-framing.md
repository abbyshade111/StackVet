# V3.4.6 is credited when only X-Frame-Options denies framing, which the requirement says may not be relied on

**Status:** claimed by backlog-org-0264, 12 October 2026

Found on 11 October 2026, reading V3.4.6's own text while claiming backlog 0232, part 1.

`probe.security-headers` cites V3.4.6, and its `missing_headers` (`crates/sv-check/src/probes.rs`) counts framing as
denied when either `X-Frame-Options` says DENY or SAMEORIGIN, or the Content-Security-Policy has `frame-ancestors`.
V3.4.6 asks for the `frame-ancestors` directive of the Content-Security-Policy on every HTTP response, and says of
`X-Frame-Options` that it "is obsolete and may not be relied upon". So an app that relies on `X-Frame-Options` alone is
credited V3.4.6 (or at least not found against it) where the requirement says it does not meet it.

**The fix to weigh.** Count only `frame-ancestors` toward V3.4.6. `X-Frame-Options` alone is then short of V3.4.6,
said in the finding's words as obsolete rather than absent, since browsers still honor it and the app is not left
open. It changes what counts as evidence, so it needs a decision record (a Later entry on the record that governs
`probes.rs`, or one of its own), and a test that an app sending only `X-Frame-Options: DENY` is not credited V3.4.6.
Check `tools/coverage.py` and the report's wording for V3.4.6 alongside.

**Claimed 12 October 2026 by session backlog-org-0264.** Proposed: count only the Content-Security-Policy's
`frame-ancestors` toward V3.4.6; `X-Frame-Options` alone is short of it, said as obsolete rather than missing. Recorded
as a Later entry on ADR-047 (which governs the list of missing headers) in the pull request that builds it. The example
apps' policies gain `frame-ancestors 'none'` where they rely on `X-Frame-Options`, so their reports stay as they are.
