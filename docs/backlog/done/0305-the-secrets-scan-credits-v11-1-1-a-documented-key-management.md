# The secrets scan credits V11.1.1 (a documented key-management policy) for finding no PEM block

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** V11.1.1: "Verify that there is a documented policy for management of cryptographic keys and a cryptographic key lifecycle that follows a key management standard such as NIST SP 800-57..." Not finding a `-----BEGIN PRIVATE KEY-----` block is not evidence of a documented policy. V11.1.1 is `manualOnly`, so the report shows it as supporting evidence rather than *checked*; it is still a wrong citation printed beside the requirement.

**Where.** `data/secret-rules.json` (`secrets.private-key-block` requirementIds V11.1.1, V13.3.1, SBD-AC-05); `crates/sv-check/src/secrets.rs:1136-1140` (`clean_scan` credits the union of every rule's ids); `docs/REQUIREMENTS.md:358` shows the rule under V11.1.1.

**Evidence.** The rule's `requirementIds`.

**How it bites.** A citation that does not speak to its requirement, against the rule that a citation is a claim.

**Fix.** Drop V11.1.1 from the rule (a committed private key is V13.3.1's concern), or make the rule finding-only for V11.1.1; regenerate `docs/COVERAGE.md` and `docs/REQUIREMENTS.md`.

**Record.** No record governs `secret-rules.json` by name; say so.
