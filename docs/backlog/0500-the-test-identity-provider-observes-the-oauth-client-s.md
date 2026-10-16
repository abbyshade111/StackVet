# The test identity provider observes the OAuth client's checks (audience, issuer, back-channel logout), and the database account name is read

**Status:** partly done: built: the static part (#2113) and the test provider's checks for V10.3.1, V10.5.5, and V7.6.2 (ADR-127). Left: the token checks for V9.2.2 and V9.2.4; the derived saml condition for V6.8.3; V10.5.3, which needs a second start of the app; V10.3.3 is not cited (its text is about access tokens); V10.3.2 and V10.3.4 have no question

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: The seventeen that apply once one more thing is true**

- With `uploads`: **V5.1.1**, documentation of permitted types, extensions, and sizes per upload feature. Notes
  question exists. Route 4: the accept lists and size limits the code has (`accept=` on the input, `ALLOWED_EXTENSIONS`,
  multer `fileFilter` and `limits`, `MAX_CONTENT_LENGTH`), per upload route, beside the question; the running checks
  for V5.2.1 and V5.2.2 already try them. *Documented by the owner*.
- With `jwt`: **V9.2.2** (the token's type and purpose checked: an ID token not accepted as an access token) and
  **V9.2.4** (an audience restriction when one key signs for several audiences). Route 2, through the existing
  app-token probes (`probe.app-token-*`, V9.1.x, V9.2.1): for V9.2.2, present the app's own ID token where its access
  token goes, when the app issues both (it rarely does; then not applicable with the reason); for V9.2.4, read
  the `aud` claim of the token the app hands out: absent is *needs attention* only when the manifest says one key
  serves several audiences, which nobody will say, so honestly: the `aud` claim shown with the V9.2.3 result, and the
  question "does this key sign tokens for anything else?", *attested*.
- With `oauth` (sign-in through another provider): the test identity provider already in the fence (V10.2.2,
  V10.5.1, V10.5.4, V6.8.2) sees every request the app makes to it, so it can observe, with no new infrastructure:
  **V10.3.1** (the resource server accepts only tokens meant for it: present an access token whose `aud` is another
  service; accepted is a finding, refused *checked*); **V10.3.3** (the user identified from `sub`, not a reassignable
  claim: `probe.oidc-user-keyed-on-email` already does the email half for V10.5.2; cite V10.3.3 beside it after reading
  that its text is about access tokens, not ID tokens, and say so in the scope); **V10.5.3** (metadata issuer
  mismatch: the test provider serves a discovery document whose `issuer` differs from the address it was fetched from;
  the app continuing is a finding); **V10.5.5** (back-channel logout: send a logout token typed wrongly or missing
  `events`; a session ended by it is a finding; only when the manifest says the app supports back-channel logout, else
  not applicable). **V10.3.2** and **V10.3.4** (decisions from `scope`/`authorization_details`, and `acr`/`amr`/`auth_time`)
  and **V6.8.4** (the same, from the IdP's answer) need a feature that requires a stronger step; route 4: the lines
  where the app reads the ID token's claims, with "does it look at anything but the email and name?", *attested*.
  **V6.8.1** (two providers, one identifier): finding-only from the code when the account lookup is by email across
  providers, which `ast.account-found-by-provider-email` already is; cite V6.8.1 beside V10.5.2 after reading its text
  (it asks for `iss` + `sub` registration), as a second citation of the same rule. **V6.8.3** (SAML assertions used once):
  applies only with SAML; a derived `saml` condition from the library (`passport-saml`, `python3-saml`, `ruby-saml`,
  `Sustainsys.Saml2`), not applicable otherwise. **V7.1.3**, **V7.6.1**, **V7.6.2** (federated session lifetimes
  documented and as documented; consent before a session): V7.6.2 can be observed: the provider answers a sign-in the
  app did not start (the "sign-in from another session" probe for V10.1.2 is the same shape); V7.1.3 is documentation
  (notes); V7.6.1 is a hand check with the reason.
- With `external-apis`: **V13.2.1** and **V13.2.2** (backend components authenticated to each other, with least
  privilege). Design questions exist. Route 4: the connection strings and clients the code builds (database URL,
  Redis, a queue, an internal API base URL) with whether each carries credentials and which account name, shown beside
  the question: "the app connects to its database as `postgres`; is that the owner account?" A superuser or `root`
  account name in a connection string is a finding-only candidate for V13.2.2 (`config.database-as-superuser`), which
  the secret rules already read past when they look for the password. *Attested by the owner*; `sv`'s own for the
  finding.

**The owner's choice, 14 October 2026:** build it, after the three probe sweeps (0559 to 0561) land, so the provider's checks are written to ADR-119's rules; a record of its own, since it changes what sv runs inside the fence.
