# The other V6 pointers 0491's section proposes: rate-limit settings, the password validator, password expiry, a second-factor library, and backup codes

**Status:** partly done: built: V6.1.1, V6.1.2, V6.3.3, and V6.5.2 pointers, the V6.2.10 hand check, ast.password-expiry-enforced, and V6.3.3 saying when no second-factor library is among the packages (16 October 2026). Left: a finding for backup codes stored without hashing (V6.5.2), which the code alone does not show soundly: it waits on a way to tell a stored code from one only shown

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

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
