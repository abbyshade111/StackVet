# Every sign-in pathway the code has, listed for the owner, and one the manifest does not name reported

**Status:** claimed by documentation, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V6 Authentication (V6.1.1, V6.1.2, V6.1.3, V6.2.10, V6.3.3, V6.3.4, V6.4.4, V6.5.2)**

- **V6.1.1** (L1), documentation of rate limiting and anti-automation against password guessing. Notes question
  exists. Route 4: the pointer is twofold: the rate-limit library and its configuration lines (express-rate-limit,
  django-ratelimit, flask-limiter, rack-attack, `limiter` in Go) found by a locator, and what the signed-in run already
  saw (`probe.failed-sign-ins-unlimited`'s result and the number of attempts). Shown as "StackVet saw the app push back
  after 6 wrong passwords, and found this rate-limit setting at `app/limits.py:12`; your notes should say what these
  are and how they work together." The partial-checks entry proposes filling the notes with what `sv` saw; keep it as
  a shown observation, not a filled answer. Tier: *documented by the owner*.
- **V6.1.2** (L2), a documented list of context-specific words banned from passwords. Notes question exists;
  `probe.context-word-password-accepted` (V6.2.11) already tries the app's name. Route 4: the pointer is the password
  validator (zxcvbn, Django's password validators list in settings, Devise's validatable, the function the sign-up
  handler calls on the password), with "this is where a new password is judged; does it refuse the app's name and
  words like it?" Tier: *documented by the owner*.
- **V6.1.3** (L2) and **V6.3.4** (L2), every sign-in pathway documented, and no undocumented pathway. Notes question
  (V6.1.3) and hand check (V6.3.4) exist. This is the strongest case for route 4 in the list: the pathways are
  mechanical to find. A locator for sign-in routes (paths or handlers matching `login`, `signin`, `auth`, `oauth`,
  `callback`, `token`, `api-key`, `magic`, `sso`; `passport.use(new ...Strategy)`; Django `AUTHENTICATION_BACKENDS`;
  Devise modules; `[Authorize]` schemes) lists them: "StackVet found three ways in: a password form at `/login`
  (`routes/auth.js:14`), Google at `/auth/google` (`:41`), an API key read in `middleware/key.js:7`. Are these all, and
  are they all held to the same rules?" Tier: *documented by the owner* for V6.1.3, *checked by hand* for V6.3.4. Route 1
  on top, finding-only: if the owner lists the pathways in the manifest (`[stack.auth] pathways = [...]`), a sign-in
  route the locator finds that the list does not name is *needs attention* citing V6.3.4 ("an undocumented pathway").
  Absence of such a route credits nothing (a pathway can be a middleware the locator does not know). The partial-checks
  entries propose the list; the manifest comparison is new.
- **V6.2.10** (L2), "must not require periodic credential rotation". No question today. Route 1, finding-only:
  `ast.password-expiry-enforced`, for an expiry setting or a comparison of a `password_changed_at` with a number of
  days (devise-security's `expire_password_after`, Spring's `isCredentialsNonExpired` returning a date comparison,
  ASP.NET Identity's custom validators, Keycloak's `passwordPolicy` with `forceExpiredPasswordChange` in an exported
  realm file, `PASSWORD_EXPIRY_DAYS` and the like). Absence credits nothing. Route 5 settles the rest cheaply: a hand
  check "Has anyone been made to change a password only because time passed?", *checked by hand*. Same as
  partial-checks for the rule; the hand check is new.
- **V6.3.3** (L2), MFA or a combination of factors, or a documented rationale. Hand check exists. Route 4: the pointer
  is the second-factor library (speakeasy, otplib, pyotp, django-otp, devise-two-factor, rotp, `@simplewebauthn`,
  `webauthn`) and the sign-in flow file; plus the run's observation that the test accounts reached private pages with a
  password alone (true of every signed-in run; it says nothing about other accounts, so it is an observation, not a
  finding, as the partial-checks entry also says). Shown as: "StackVet found no second-factor library, and signed in
  with a password alone. If that is the design, ASVS asks for the reasoning written down." Tier: *checked by hand*, or
  *documented by the owner* for the rationale.
- **V6.4.4** (L2), identity proofing when a factor is lost. A support process; no code to point at. Manual with the
  reason, as the existing hand check says.
- **V6.5.2** (L2), lookup secrets (backup codes) hashed with a password-hashing algorithm. Hand check exists. Route 4:
  a locator for the backup-code column (`backup_codes`, `recovery_codes`, `otp_backup` in migrations and models) and
  whether a hashing call (`bcrypt`, `argon2`, `make_password`, `Digest::SHA256` is not enough) touches it in the same
  file; shown with "these codes are stored here; the line that saves them is this one; is it hashed first?" This is one
  a non-programmer cannot judge from the lines, but the AI coding tool can when the lines are in front of it (0397's
  first part); the owner records the answer. Finding-only when the column is plain text and the saving line writes the
  generated value unchanged: `ast.backup-codes-stored-plain`, reachable with tree-sitter when generation and save are
  in one function. Tier: `sv`'s own (finding-only) and *attested by the owner*.
