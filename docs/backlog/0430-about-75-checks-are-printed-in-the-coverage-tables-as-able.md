# About 75 checks are printed in the coverage tables as able to mark *checked* where the code only ever credits in part, and no gate holds the list

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: requirements and tests (citations). Severity: medium. Verified against the code by the reviewing session before filing. Two examples confirmed by the reviewing session: V7.2.4 (`sessions.rs:1528`) and V13.4.4 (`probes.rs:683`) are always `.in_part()` and `docs/COVERAGE.md:208` and `:236` print them without the marker. Sibling of the item on the website's "can check" column and the summary's "can be credited" definition.

- Where:
  - `tools/coverage.py:424` (`RUST_IN_PART = {…}`, nine checks), `:593` (`RUST_IN_PART |= {"config.versions-pinned", "sbom"}`),
    `:1259-1271` (COVERAGE.md says "checked in part only, by …" only for checks in `IN_PART`), `:1442-1447`
    (REQUIREMENTS.md's "(only ever in part)" marker, from the same set).
  - `crates/sv-check/src/verified.rs:131-152` (`census`: writes `check_id`, the ids, and the place; not `in_part`)
    and `tools/coverage.py:803-809` (`check_credits` reads those three fields), so `--credits` cannot see the mark.
  - Examples of the code: `crates/sv-check/src/signed_in/sessions.rs:1522-1528` (`SESSION_RENEWAL`, V7.2.4,
    `.in_part()` always), `crates/sv-check/src/probes.rs:674-686` (`TRACE_ENABLED`, V13.4.4, `.in_part()` always),
    `crates/sv-check/src/ai.rs:989-997` (`UNBOUNDED`, C7.1.2, `.in_part()` always).
  - The documents: `docs/COVERAGE.md:208` ("| V7.2.4 | L1 | Signed in: `probe.session-not-renewed` |", no
    "checked in part only"); `:236` (V13.4.4, the same); `docs/REQUIREMENTS.md:119` (V7.2.4 "Can be checked",
    no "(only ever in part)"); `:395` (V13.4.4, the same).
- What is wrong: my census (`review/work/inpart.txt`) finds 77 check ids whose every credit site in non-test code
  is chained with an unconditional `.in_part()`, against the 11 the script marks. So the two generated documents,
  which are the public statement of what `sv` can say, print "Can be checked" without the in-part marker for
  about 75 checks (every password rule probe, the session probes, the log probes, every `probe.ai-*` and
  `probe.mcp-*` credit, `probe.cors-any-origin`, `probe.error-detail-leak`, `probe.trace-enabled`,
  `probe.source-control-exposed`, `probe.no-hsts`, `probe.cookie-without-host-prefix`, the upload probes, and
  more; the full list is in the census). The legend says a check without the marker can mark a requirement
  *checked*; these cannot (`status_of`, `crates/sv-report/src/lib.rs:2713`). The per-app report is right, because
  it reads `Verified::in_part`; only the coverage claim is wrong. The 13 October builds that marked ADR-090's
  groups in part edited `RUST_IN_PART` for 0304 and 0307 only, by hand, and no gate catches the rest: the
  credit log the CI test writes does not carry the mark, and `coverage_doc.rs` only holds the document to the
  script. (`probe.password-change-ends-sessions` and `probe.password-change-notified` are both credits-only and
  always in part; the script has the "(credited only, and only ever in part)" wording for that case at `:1442`
  and never reaches it.)
- What to do: write the in-part flag into the credit log line (`census`), and have `--credits` derive the
  in-part set from the log (a check whose every logged credit is in part belongs in it; one with any whole credit
  does not) and fail when `RUST_IN_PART` disagrees, as it already fails for findings-only; then regenerate the
  three generated files. Must not lose: the conditional checks (`in_part_if`: the 18 of groups A, B, C, D, E, F, H,
  and `probe.plain-http-served`) are rightly printed as able to be whole; the gate must count a check in part
  only when no logged credit of it is whole.
- How verified: read, plus the census script (which parses the Rust; its rule-id resolution covers every site
  but two in `adapters.rs` and `confirm.rs` that take a computed id).
- Overlap: 0304 and 0307 (done) each added one check to `RUST_IN_PART`; 0029 (done) built the credit census
  without the mark; nothing filed on the gate or the ~75-check gap (`grep -ril "RUST_IN_PART\|only ever in part" docs/backlog/`
  finds 0304, 0085, 0381 only).
- Needs a decision record?: yes: ADR-059 governs `tools/coverage.py`'s census and `verified.rs`; a Later line
  saying the census carries the in-part mark and what `--credits` now holds.
