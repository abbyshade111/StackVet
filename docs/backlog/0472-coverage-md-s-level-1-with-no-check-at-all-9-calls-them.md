# COVERAGE.md's "Level 1 with no check at all (9)" calls them every app's baseline, though four apply only to an OAuth authorization server

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage accuracy. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `docs/COVERAGE.md:310-314` ("The baseline every app is assessed against, and where a new check does the most
  good. V2.1.1, V2.2.1, V6.1.1, V8.1.1, V10.4.1, V10.4.2, V10.4.3, V10.4.5, V15.1.1"); `data/knowledge/applicability.json`
  (`V10.4` gated on `authorization-server`: "These rules are for whoever runs one"); `tools/coverage.py` (the paragraph
  is written without reading applicability).
- What is wrong: V10.4.x apply to an app that issues OAuth tokens to other applications, which almost no app `sv` is
  for does. The sentence is read as "nine level-1 gaps every app has"; the honest number for an app that is not an
  authorization server is five (V2.1.1, V2.2.1, V6.1.1, V8.1.1, V15.1.1), and all five are documentation requirements
  (see Part 2), where "a new check" is the wrong instrument.
- What to do: write "for an app that is not an OAuth authorization server, five: ...; the other four, V10.4.x, apply
  only to one", or mark the four. One line in `coverage.py`.
- How verified: read.
- Overlap: none found (`grep -rn "Level 1 with no check" docs/backlog/`).
- Needs a decision record?: no.

### Found sound
- `tools/coverage.py --check` passes on the checkout; the three generated files are current, and
  `crates/sv-check/tests/coverage_doc.rs` holds them so.
- The Summary, level, and chapter tables add up (174 / 122 ASVS; 46 / 27 AISVS; 3 / 2 Appendix C; SBD 0 / 0 with 12
  supporting). The AISVS paragraph's "20 of these 49" is 19 AISVS + 1 Appendix C finding-only, consistent with 46 − 27
  and 3 − 2.
- Finding-only checks are kept out of "Can be credited" and out of `reach.json` (`reach_json`'s `credits()`), so the
  report's "could have reached" never counts a check that can only fail a requirement. `RUST_FINDINGS_ONLY` and the
  AST rules' `findingsOnly`/`findingsOnlyFor` are read by the same function the summary uses, and CI's `--credits`
  census holds them to what the suite credits.
- Supporting-only (9 ASVS) are `manualOnly` and SBD by construction (`settles()` excludes both), and are shown under
  their own head, not among "can settle".
- `README.md:26-27` states no number; it points at COVERAGE.md and REQUIREMENTS.md. `docs/GETTING-STARTED.md`,
  `docs/GLOSSARY.md`, `docs/THREAT-MODELING.md`, `docs/DASHBOARD.md` (its "10 checked, 230 not verified" is a labeled
  example) state no coverage figure. `docs/paper/figure-security.html` quotes "can settle" at a dated commit (163 of
  345, 157ddc3) and its own text says "can settle" includes finding-only checks (`:120`); 0006.5 noted it as left for
  the paper's next update.
- The website text's note (`home-page.md:130`) correctly reads today's COVERAGE.md level counts (58 of 70, 95 of 183).
- The "Settled by ..." sections list a requirement under each kind that reaches it; the sentence above the level
  table says so, and the per-kind counts there match that rule.
- `sv explain` reads `reach.json`'s `checks` half, which carries `finding_only` per check, so an explanation never
  shows a finding-only check as able to credit.

### Not verified
- Whether a Semgrep clean run on a real Go app with no `tls.Config` credits V12.1.2 (F2): read from `clean_run_evidence`
  and the map, not run.
- The paper's `figure-security.html` numbers against its named commit: not checked out.
- Anything the report renders for a requirement with a hand check and a tool credit side by side (F2's visible form)
  needs a built `sv` and an app.
