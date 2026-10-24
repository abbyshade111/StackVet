# New checks and probes for the ASVS requirements StackVet says nothing about

**Status:** partly done: eleven parts, built in the order the plan below gives; the second (LaTeX with shell escape) is next

Filed at the owner's request, 20 October 2026, from DESIGN, "What could be added for the ASVS requirements StackVet
says nothing about (20 October 2026)", which has each requirement's proposal and why. Severity: low (each adds
coverage; none fixes a wrong answer). Each part is a new or extended rule: it needs a test that breaks what it guards,
regenerated coverage, and a decision record where it changes what `sv` runs or what a report concludes. None needs `sv`
to open a new network connection. The level 1 and 2 ones come first; 71 of the 103 requirements are level 3 and wait
for backlog 0507.

1. **Stock default credentials in connection addresses and compose files** (V13.2.3, L2): static, finding only; the
   fix backlog 0431 offered. Small.
   **Part status:** done, 24 October 2026
2. **LaTeX started with shell escape** (V1.2.8, L2): static, finding only. Small.
   **Part status:** open
3. **TURN relay settings** (V17.1.1, V17.1.2): static, from the coturn file, crediting in part. Small to medium.
   **Part status:** open
4. **Malformed signaling messages, then a normal one** (V17.3.2, L2): probe. Medium.
   **Part status:** open
5. **Access tokens with too little scope, and for another person with the same email** (V10.3.2, V10.3.3, L2): probe,
   from the stand-in provider. Medium.
   **Part status:** open
6. **Authorization-server metadata and settings** (V10.4.16, V10.6.1; V10.4.3, V10.4.8): one read of the app's own
   `/.well-known` documents, and a settings check for django-oauth-toolkit and node-oidc-provider. Medium.
   **Part status:** open
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
