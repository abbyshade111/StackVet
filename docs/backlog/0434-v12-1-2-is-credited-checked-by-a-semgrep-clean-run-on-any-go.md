# V12.1.2 is credited *checked* by a Semgrep clean run on any Go or Python app, while the hand check beside it says sv does not check it

**Status:** claimed by honesty, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage accuracy. Severity: medium. Verified against the code by the reviewing session before filing.

- Where: `data/adapters.json`, semgrep rules `go.lang.security.audit.crypto.tls.tls-with-insecure-cipher`
  (`languages: ["go"]`, `requirements: ["V12.1.2"]`) and
  `python.lang.security.audit.insecure-transport.ssl.no-set-ciphers.no-set-ciphers` (`["python"]`, `["V12.1.2"]`),
  both under `requirements`, not `findings_against`; `docs/COVERAGE.md:286` ("V12.1.2 | L2 | Outside tools:
  `semgrep`", under "Settled only by an outside tool") and the Summary's "Can be credited"; `data/human-checks.json`,
  entry `V12.1.2`: "`sv` does not check this: it takes dozens of connections to the live site ... Run a scanner made
  for it"; `tools/coverage.py:433` (`MENTIONS` says V12.1.2 "is named in the report's 'Before going live' list ... which
  credits nothing"); `crates/sv-check/src/adapters.rs:1832-1845` (`clean_run_evidence`: a rule counts for a clean
  run when it was loaded and the app has a file in its language); `docs/backlog/done/0147-*.md:250-254` (the owner's
  decision of 6 October: "leave it unchecked ... and have the report say how to check it with a dedicated scanner").
- What is wrong: V12.1.2 asks that "only recommended cipher suites are enabled, with the strongest cipher suites set
  as preferred" on the live site. The two Semgrep rules match a cipher list written into the app's own code
  (`tls.Config{CipherSuites: ...}`, `ssl.SSLContext.set_ciphers(...)`). Most apps built with an AI coding tool have no
  such code: TLS ends at a proxy or a hosting platform. For such a Go or Python app a Semgrep run finds nothing, the
  clean run is credited (`clean_run_evidence` asks only that the rule was loaded and the app has Go or Python files),
  and the requirement reads *checked* beside a hand-check line that says `sv` does not check it. Nothing read the
  cipher suites the site offers, or their order. Until this morning this needed `--tools`; since ADR-111 the tools run
  with every `sv report`, so every Go and Python app with Semgrep installed gets this credit by default. V12.1.1 has
  the same shape (fifteen tool rules credit it from code, while `probe.old-tls-accepted`'s refusal is deliberately
  "said and not credited" because the requirement also asks for the newest version preferred, `tools/coverage.py:347-349`),
  but V12.1.1's rules at least read the setting the requirement names when the app sets it; V12.1.2's do too, and the
  gap is the same: an app that sets nothing is credited for what its proxy does.
- What to do: move both V12.1.2 rules to `findings_against` (a cipher list with a weak suite is a finding; no cipher
  list is nothing), so V12.1.2 becomes "can only be found failing" and the hand check beside it stays true. Consider the
  same for the V12.1.1 rules that read application code rather than server configuration (the nginx `generic` rules
  read the server's own file and are a fairer credit, in part). Alternatively keep the credit and make it *in part*
  with its scope saying "the cipher list written in the app's code, when there is one"; but a credit for an app that
  writes none is not a reading of anything, so `findings_against` is the honest choice. Do not lose: the finding when a
  weak suite is written in code.
- How verified: read (the adapter map, `clean_run_evidence`, the hand-check text, the owner's decision in 0147).
  Not run: I did not run Semgrep on a Go app to watch the credit arrive; the crediting path is the same one every
  Semgrep `requirements` entry takes.
- Overlap: 0147 (done; the owner's decision and the hand-check entry, which did not touch the adapter map), 0085.1
  (done; moved fifteen rules to `findings_against`, not these), 0006.12 (done; Bandit and gosec citations), ADR-018
  (Later, 7 October: template and server-configuration rules count only when handed a file). Nothing in the backlog
  names V12.1.2's Semgrep credit; `grep -rn "V12.1.2" docs/backlog/` finds 0147 and 0148 only.
- Needs a decision record?: yes: it changes what counts as evidence for V12.1.2 (and V12.1.1 if taken), a Later entry
  on ADR-018 (the adapter map is in its Governs list), and the owner's decision of 6 October 2026 in 0147 should be
  cited as the reason.
