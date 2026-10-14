# `secrets.credential-assignment` names V13.2.3 (no default credentials) on every hard-coded key, after filtering out the values a default credential has

**Status:** claimed by honesty, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: requirements and tests (citations). Severity: medium. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/secrets.rs:58` (`pub const ASSIGNMENT_REQUIREMENTS: &[&str] = &["V13.3.1", "V13.2.3", "SBD-AC-05"];`),
  `:876-899` (the filters: `if !any_text_is_a_value && reads_as_text_or_a_name(value) { continue; }` …
  `if shannon_entropy(value) < 3.5 { continue; }`), `:962` (`requirement_ids: ASSIGNMENT_REQUIREMENTS…`).
- What is wrong: V13.2.3 says "Verify that if a credential has to be used for service authentication, the
  credential being used by the consumer is not a default credential (e.g., root/root or admin/admin)." The rule
  reports a value under a credential's name only when it does not read as a word and has entropy of 3.5 bits per
  character or more, which `admin`, `password`, `root`, and every stock default fail. So the finding is raised
  for `api_key = "sk-…"` (not a default credential) and never for `password = "admin"` (one). Every such finding
  marks V13.2.3 *needs attention* (a finding beats every credit), for a requirement the check did not find
  failing. The credit side is already right: `secrets.scan` credits only the ids of `data/secret-rules.json`
  (V13.3.1, SBD-AC-05; `secrets.rs:1277-1280`), and the coverage script lists the rule as findings-only.
- What to do: drop V13.2.3 from `ASSIGNMENT_REQUIREMENTS` and from the `RUST_CHECKS` entry in
  `tools/coverage.py:107`, and regenerate; or keep it only on a finding whose value matches a short list of stock
  defaults (the opposite of today's filter). Must not lose: V13.3.1 and SBD-AC-05 on the finding, and the
  redaction. No test names V13.2.3 (`grep -rn V13.2.3 crates/sv-check` finds the constant alone), so the change
  wants one: a hard-coded high-entropy key cites V13.3.1 and SBD-AC-05 and not V13.2.3.
- How verified: read.
- Overlap: 0029 (done) listed the rule as never crediting V13.2.3; it did not read the finding side. 0305 (done)
  removed V11.1.1 from a secrets rule for the same kind of reason; different id.
- Needs a decision record?: yes: ADR-040 governs `secrets.rs`; a Later line, since it changes what a report
  concludes about V13.2.3.
