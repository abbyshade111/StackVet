# The group A credits can be whole with the health path's answer left out by the rate limiter, and then read "the app's answers on ,"

**Status:** claimed by honesty, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-check`. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/probes.rs:580-585` (`names` is built from judged pages that are not routes),
  `:586-598` (credited when `judged` is non-empty, whole when `every_route`), `:1162-1169` (`pages` holds `home`,
  a 2xx `root`, and every route); `crates/sv-cli/src/lib.rs:343-344` and `crates/sv-run/src/lib.rs:692-694`
  (`probes_rate_limited`: an answer the rate limiter gave is left out of `probe_responses`).
- What is wrong: when `home` was left out (answered by the limiter) and `root` was not 2xx, `judged` is the route
  answers alone: `names` is empty, the scope reads "the app's answers on , and on all 2 routes stackvet.toml names,
  as somebody not signed in", and the credit is whole without the health path being judged at all. Narrow trigger;
  I am not sure it occurs outside a run already in trouble.
- What to do: require `home` among `judged` for the credit (as `CONTENT_TYPE` does at `:689-692`), and build the
  sentence so an empty `names` cannot leave a bare comma.
- How verified: read.
- Overlap: 0077 (done; a limiter's 429 left out). New.
- Needs a decision record?: no.
