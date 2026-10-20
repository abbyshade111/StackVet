# The other V6 pointers 0491's section proposes: rate-limit settings, the password validator, password expiry, a second-factor library, and backup codes

**Status:** done, 20 October 2026

**Was queued on the coordination board** (backlog 0273) until the board was removed on 20 October 2026 (ADR-061, Later); it is now taken the normal way, by a claim.

Filed by documentation finishing backlog 0491, 15 October 2026. Severity: low (coverage, not a fault).

0491's section of the deep review (backlog 0417) proposes more than its title names; 0491 built the sign-in pathways
(V6.1.3, V6.3.4). The rest, each read again in 0491's own text before it is built:

- **V6.1.1:** the rate-limit library and its setting lines (express-rate-limit, django-ratelimit, flask-limiter,
  rack-attack), shown beside the notes question with what the signed-in run saw.
- **V6.1.2:** the password validator (zxcvbn, Django's validators list, Devise's validatable, the function sign-up
  calls on a password), beside the notes question.
- **V6.2.10:** `ast.password-expiry-enforced`, finding-only, and a hand check.
- **V6.3.3:** the second-factor library (speakeasy, otplib, pyotp, django-otp, devise-two-factor, rotp, webauthn), or
  its absence, beside the hand check.
- **V6.5.2:** the backup-code column and the line that saves it, and `ast.backup-codes-stored-plain`, finding-only.

**Record.** ADR-022 Later for the pointers; a finding-only rule's record where it is built.

**Checked against `main`, 20 October 2026** (the backlog clean-up): the status line said V6.1.1 was built; its pointers into the code are, but showing the signed-in run's result for wrong passwords beside the question (0491) is not (`MEASURED_PARTS` in `crates/sv-check/src/human.rs` covers V14.2.4 and V15.2.2 only).

**Built, 20 October 2026,** the two pieces left. **V6.1.1:** `MEASURED_PARTS` shows the signed-in run's wrong-password
check (V6.3.1) beside the question, with this report's verdict (ADR-022, Later); `crates/sv-cli/tests/measured_parts_sign_in.rs`.
**V6.5.2:** `ast.backup-codes-stored-plain`, finding-only, for a code field on an object or record given its value
straight from a random generator with no hashing in the same line (ADR-018, Later); a plain variable is not read, since
that is where the safe way makes the codes before hashing them; `crates/sv-check/src/ast/backup_codes_tests.rs`. V6.5.2
moves from no check to can only be found failing in `docs/COVERAGE.md` (ASVS can settle 189 to 190).
