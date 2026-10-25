# New checks and probes for the ASVS requirements StackVet says nothing about

**Status:** partly done: eight parts, built in the order the plan below gives; the fifth (tokens with too little scope, or for another person with the same email) is next

Filed at the owner's request, 20 October 2026, from DESIGN, "What could be added for the ASVS requirements StackVet
says nothing about (20 October 2026)", which has each requirement's proposal and why. Severity: low (each adds
coverage; none fixes a wrong answer). Each part is a new or extended rule: it needs a test that breaks what it guards,
regenerated coverage, and a decision record where it changes what `sv` runs or what a report concludes. None needs `sv`
to open a new network connection. The level 1 and 2 ones come first; 71 of the 103 requirements are level 3 and wait
for backlog 0507.

1. **Stock default credentials in connection addresses and compose files** (V13.2.3, L2): static, finding only; the
   fix backlog 0431 offered. Small.
   Built 24 October 2026: two findings-only pattern rules in `data/secret-rules.json`,
   `secrets.default-credential-address` (a database or broker address whose user and password are both stock
   defaults, such as postgres:postgres or guest:guest) and `secrets.default-credential-setting` (a database or
   broker image's password setting given one, in a compose or settings file). They cite V13.2.3, and V13.3.1 and
   SBD-AC-05 too, as a password written into the code; each finding says a default kept only for a test copy is
   fine if the real service has its own. Pattern rules can now be findings-only (`findingsOnly`), so a clean
   secrets scan does not claim V13.2.3, and `tools/coverage.py` reads them so: V13.2.3 is "Can only be found
   failing". Tests in `crates/sv-check/src/secrets/default_credentials_tests.rs`, with the defaults built from
   pieces. Broken on purpose: without the findings-only filter, a clean scan offered V13.2.3; with the patterns
   emptied, the found cases went red; and before `coverage.py` knew the flag, the page said "Can be checked".
   **Part status:** done, 24 October 2026
2. **LaTeX started with shell escape** (V1.2.8, L2): static, finding only. Small.
   Built 24 October 2026: `ast.latex-shell-escape`, a code rule that only ever reports. pdflatex, xelatex,
   lualatex, or latexmk started with `-shell-escape`, `--shell-escape`, or `-enable-write18`, through Python's
   `subprocess` and `os.system`, PyLaTeX's `generate_pdf(compiler_args=...)`, Node's `child_process`, or a shell
   script; `-no-shell-escape` is not reported. Every other language says it is not looked for there yet, and
   credits nothing either way. Thirteen witnesses in `crates/sv-check/src/ast/tests.rs`. V1.2.8 is "Can only be found
   failing". Broken on purpose: without the boundary before the dashes, both `-no-shell-escape` cases were
   reported; with the program names emptied, the seven found cases went unfound.
   **Part status:** done, 24 October 2026
3. **TURN relay settings** (V17.1.1, V17.1.2): static, from the coturn file, crediting in part. Small to medium.
   Built 24 October 2026 (ADR-146): `crates/sv-check/src/turn_relay.rs` reads every `turnserver.conf`, and a coturn
   service's `command` flags in a compose file. V17.1.1 is credited in part when the settings refuse every reserved
   range (0.0.0.0/8, 10.0.0.0/8, 100.64.0.0/10, 169.254.0.0/16, 172.16.0.0/12, 192.168.0.0/16, fc00::/7, fe80::/10),
   with `no-multicast-peers` set and `allow-loopback-peers` not; an `allowed-peer-ip` inside a range undoes it. V17.1.2
   is credited in part when `user-quota` and `total-quota` are both above 0. Either short is a finding that says what;
   no relay settings say nothing. Six tests in `crates/sv-check/src/turn_relay/tests.rs`. Broken on purpose: with the
   range coverage always yes, the test that takes out each range went red; with `allowed-peer-ip` ignored, the test
   of an address let back in went red.
   **Part status:** done, 24 October 2026
4. **Malformed signaling messages, then a normal one** (V17.3.2, L2): probe. Medium.
   **Part status:** open
5. **Access tokens with too little scope, and for another person with the same email** (V10.3.2, V10.3.3, L2): probe,
   from the stand-in provider. Medium.
   V10.3.3 built 24 October 2026 (ADR-148): the test provider mints an access token for a different person who has
   the first person's email address, and `crates/sv-check/src/oidc/tokens.rs` sends it as a bearer token to `shows`,
   where the first person's own token shows what they saved (V10.5.2's question saves it through `create`). Reaching
   it is a finding; not reaching it, with the first person's own token showing it before and after, is credited in
   part. Three tests in `crates/sv-check/src/oidc/subject_tests.rs`, and the provider's contract test runs the script.
   Broken on purpose: with the control skipped, an app that saved nothing was credited; with the script minting the
   other person under the first one's subject, the contract test went red. V10.3.2 is the next pull request.
   **Part status:** claimed by claude/0625-5, 24 October 2026
6. **Authorization-server metadata and settings** (V10.4.16, V10.6.1; V10.4.3, V10.4.8): one read of the app's own
   `/.well-known` documents, and a settings check for django-oauth-toolkit and node-oidc-provider. Medium.
   The published details built 24 October 2026 (ADR-147): two read-only requests to the app's own copy,
   `/.well-known/oauth-authorization-server` and `/.well-known/openid-configuration`, read by
   `crates/sv-check/src/as_metadata.rs`. Weak client authentication (or none listed, which RFC 8414 reads as
   `client_secret_basic`) is a finding against V10.4.16, and only key-based methods are credited in part; a response
   type holding `token` is a finding against V10.6.1, and only the allowed types are credited in part; `plain` PKCE
   is a finding against V10.4.6, never a credit. Seven tests in `crates/sv-check/src/as_metadata/tests.rs`. Broken on
   purpose: without RFC 8414's default, the details that list no methods were not reported; with the response types
   always allowed, the implicit flow went unfound. The settings half is the next pull request.
   The settings built 24 October 2026 (ADR-147): `crates/sv-check/src/as_settings.rs` reads django-oauth-toolkit's
   `OAUTH2_PROVIDER` and node-oidc-provider's configuration. A code over ten minutes is a finding against V10.4.3, and
   one of a minute or less is credited in part; a django-oauth-toolkit refresh limit left unset or `None` (never
   expires, its default) and node-oidc-provider's `rotateRefreshToken: true` are findings against V10.4.8. Six tests in
   `crates/sv-check/src/as_settings/tests.rs`. Broken on purpose: with an unset limit read as one, the defaults test
   went red; with the one-minute line at ten, the five-minute code was credited.
   **Part status:** done, 24 October 2026
7. **Authorization-server probes with an owner-declared test client** (V10.4.1, V10.4.2, V10.4.5, V10.4.6, V10.4.10,
   V10.4.12, V10.4.13, V10.6.2): redirect address, code reuse, PKCE, refresh reuse, client authentication, response
   mode, PAR, sign-out without a hint. Large.
   **Part status:** open
8. **CodeQL's C/C++ suite for the unmanaged code that turns V1.4 on** (V1.4.2, L2): outside tool, finding only. Medium.
   **Part status:** open
9. **Upload probes** (V5.2.5, V5.2.6, V14.2.8): an archive with a link in it, an image huge in pixels, image metadata
   kept. Small to medium.
   **Part status:** open
10. **Response and caching probes** (V4.2.2, V14.2.5, V3.5.8, V3.5.7): Content-Length against the body; a private page
    with `.css` added and a missing static file answered with the main page; cross-site embedding headers on a private
    resource; script files compared signed in and signed out. Small to medium each.
    **Part status:** open
11. **Signed-in flow probes** (V1.2.10, V2.4.2, V6.3.5, V7.5.3, V8.3.2, V6.7.2): formula-starting values in a
    spreadsheet export; a declared flow submitted at machine speed; the suspicious sign-in notice in the mail catcher;
    owner-named sensitive actions without a fresh sign-in; a role taken away while a session is open; WebAuthn
    challenges. Medium each.
    **Part status:** open
12. **Static, finding only** (V6.5.8, V11.6.2, V6.3.6, V3.2.3): a one-time code checked against a time from the request;
    weak key-exchange parameters; email used as a sign-in factor; DOM-clobbering patterns. Small to medium.
    **Part status:** open

Already filed elsewhere: V10.5.3 is backlog 0620.

**The plan, 24 October 2026, at the owner's word** ("yes, write the plan into 0625 and start Phase A"). Checked
against `main` that day: every requirement in the twelve parts still reads "No check yet" in `docs/REQUIREMENTS.md`,
so none is overtaken. Level 3 is no longer waiting: backlog 0507 made it an opt-in (`[app] level = 3`, ADR-130), so
the level-3 parts can appear in a report, for an app that asks for level 3. The design entry's warning that V15.4's
`never` rule would be wrong at level 3 is not a live fault: a level-3 copy of `examples/flask-booking`, run that day,
shows V15.4.1 to V15.4.4 as not verified, not as not applying.

Each part is built as the Rust rules of backlog 0661 were: a claim of the part; a decision record first, as proposed,
where the part changes what `sv` runs or sends, adds a stackvet.toml entry, or credits anything; then the build, with
a test that breaks what it guards, regenerated coverage, and the example snapshots run after the last edit. One build
pull request at a time. In this order:

- *Phase A, static rules that only ever report, no record:* the stock default credentials (the first part), then
  LaTeX with shell escape (the second).
- *Phase B, a settings check that credits in part, with a record:* the TURN relay settings (the third).
- *Phase C, sign-in providers and authorization servers:* the server's own metadata and its settings (the sixth part,
  one record, two pull requests), the cheap first step the others build on; then tokens with too little scope or for
  another person with the same email (the fifth, a record, since its `resource-server` condition moves V10.3.1); then
  the authorization-server probe set (the seventh, a record for its test-client entry in stackvet.toml, built in four
  pull requests: redirect matching and code reuse; PKCE and refresh reuse; client authentication and response mode;
  PAR and sign-out without a hint).
- *Phase D:* malformed signaling messages (the fourth, a record for its new stackvet.toml entry); CodeQL's C/C++ suite
  (the eighth, a record, since it changes what `sv` runs).
- *Phase E, level-3 only:* the static rules of the twelfth part (no record), then the upload probes (the ninth), the
  response and caching probes (the tenth), and the signed-in flow probes (the eleventh), each with a record. Whether
  these are worth building now, with most apps held to level 1 or 2, is for the owner to say when Phase D is done.

About thirty build pull requests in all. **The owner's word on the eighth part, 24 October 2026:** "I approve of
fetching CodeQL's C/C++ pack, that's fine." That fetch is a change to what `sv` connects to, so its record (ADR-111,
whose exception is Semgrep fetching its rules, and `docs/ARCHITECTURE.md`'s fence) is written with the part's claim,
quoting these words. The owner asked for more about the seventh part's test client before deciding on it.
