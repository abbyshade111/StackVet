# ADR-096's whole credit needs only one named route, and the credit's evidence does not name the route answers it rests on

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: requirements and tests (citations). Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/probes.rs:564-567` (`let every_route = !routes.is_empty() && routes.iter().all(|path| find(…).is_some());`),
  `:589-596` (`.in_part_if(!every_route)` on V3.4.3–V3.4.6), `:523-553` (`name_credits`: evidence for these
  credits is `["home", "missing", "root"]` only).
- What is wrong: V3.4.4 says "all HTTP responses" and V3.4.6 "every HTTP response". With `[stack.run.users]`
  naming one route (say `login`), three anonymous answers make both *checked*, worded "on all 1 route stackvet.toml
  names". ADR-096 records "at least one such route" as the bar, so this is the owner's decision as written; it is
  flagged because one route is the weakest reading of "every". Separately, the credit says it rests on the
  routes' answers but its `evidence` (ADR-082) still lists the three fixed ids, so `seen.json` cannot show a
  reader the route answers behind a whole credit.
- What to do: either require two or more named routes for the whole credit (as groups B, E, and F do) with a
  Later line on ADR-096, or leave it and say so; and add the `route:` answer ids to `name_credits` for the five
  credits.
- How verified: read.
- Overlap: 0232 part 1 and ADR-096 (done) built this; nothing filed on the one-route bar or the evidence ids.
- Needs a decision record?: yes for the bar (ADR-096 Later); the evidence ids are ADR-082's and need an
  "unchanged, because" line or a Later entry.

**The owner's answer, 14 October 2026** (asked through the board's waiting line while the owner was away; answered as the session recommended): two or more named routes for the whole credit, as groups B, E, and F have; with one route the credit is in part. ADR-096 gets a Later line. The second part (the route answers named in each credit's evidence) stands as built.
