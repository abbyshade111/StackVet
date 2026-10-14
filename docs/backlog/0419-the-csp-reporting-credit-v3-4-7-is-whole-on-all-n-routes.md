# The CSP-reporting credit (V3.4.7) is whole "on all N routes" while a named route with no policy at all is skipped

**Status:** claimed by honesty, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-check` (checks' honesty).
Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** V3.4.7 reads "Verify that the Content-Security-Policy header field specifies a location to report
violations." `policy_answers` keeps only the answers that carry a `content-security-policy` header, so a named route
that answered with no policy at all drops out of the reading; `every_route` is computed separately and is true when
every named route *answered*, whatever it carried. With routes `/login` (a policy with `report-uri`) and `/account`
(no policy), `csp_reporting` finds nothing wrong, `every_route` is true, and V3.4.7 is credited whole with the words
"and on all 2 routes stackvet.toml names". The missing policy on `/account` is a `probe.security-headers` finding
(V3.4.3 to V3.4.6), so the report says V3.4.3 failed on `/account` and V3.4.7 is *checked* there. A header that is
not there specifies no location.

**Where.** `crates/sv-check/src/probes.rs:2353-2359` (`policy_answers`), `:727-741` (the credit, `.in_part_if(!every_route)`),
`:563-567` (`every_route`).

**Fix.** For this one credit, "every route" must mean every named route answered *with a policy*: in part (or whole
only over the routes that had one, named in the scope) when any named route answered without one. Keep the finding as
it is, and the credit still needing the health path's policy. Write the fake-app case first (one route with a policy,
one without) and watch the whole credit go red.

**Record.** ADR-096 governs the group A credits: a Later entry.
