# An in-part probe credit outranks an outside tool's finding

**Status:** claimed by builder-full, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** ADR-023 (Later, 6 October) lets a `probe.` credit that verified the requirement in the same run set a Semgrep, CodeQL, or Bandit finding aside as only worth a look. `run_check` tests only `v.check_id.starts_with("probe.")` and that the credit names the id; it does not read `v.in_part`. So a credit that by its own words rests on one sample (`probe.cross-site-request-accepted` for V3.5.1 with `in_part_if(refused_at.len() == 1 …)`, `forgery.rs:171`; `probe.security-headers` for V3.4.3 on the health path alone, `probes.rs:595`) removes a tool's finding about a different route or file.

**Where.** `crates/sv-report/src/lib.rs:2286-2300` (`mark_outranked`, `run_check`) and `crates/sv-check/src/finding.rs:299-302` (`withholds_credit`).

**Evidence.** `run_check` at the lines above, with no `in_part` test.

**How it bites.** `status_of` reports the requirement as checked in part, or checked if another probe credit is whole, with the tool's finding not counting.

**Fix.** In `run_check`, skip credits with `in_part` (or require every applicable id to have a whole credit); a test with an in-part probe credit and a tool finding on the same id asserting the finding stands.

**Record.** ADR-023 governs: a Later line (an in-part credit outranks nothing).
