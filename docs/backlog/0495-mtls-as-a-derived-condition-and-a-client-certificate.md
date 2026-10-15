# mTLS as a derived condition, and a client certificate accepted without a trust check reported

**Status:** claimed by honesty, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V12 Secure Communication (V12.1.3)**

- **V12.1.3** (L2), mTLS client certificates validated before their identity is used. No question; applies to a
  typical app only in name, since almost none uses client certificates. Route 1, in two steps: a derived condition
  `mtls` (code: `requestCert: true`, `ssl_verify_client`, `ClientAuth: tls.Require...`, `SSLVerifyClient`,
  `ssl.CERT_REQUIRED` with `verify_mode`), so for the app that has none it is *does not apply* with the reason "the app
  asks for no client certificate" (derived, like `xml`); and for the app that has it, finding-only:
  `RequireAnyClientCert` or `RequestClientCert` without `ClientCAs`, `requestCert: true` with
  `rejectUnauthorized: false`, nginx `ssl_verify_client optional_no_ca`. Tier: applicability (a reason, not a status),
  and `sv`'s own for the finding. Partial-checks proposes the finding; the derived condition is new, and it takes the
  requirement off every ordinary app's "not verified" list honestly.

**The owner's choice, 14 October 2026:** build it first of the three: the derived condition (does not apply, with the reason, for the app that asks for no client certificate) and the finding for a certificate accepted without a trust check. A Later line on the applicability record.

**First part built, 15 October 2026 (honesty):** the derived condition `mtls`, gating V12.1.3 alone, with the reason
"the app asks for no client certificate" (ADR-015, Later). Still to build: the finding for a client certificate
accepted without a trust check (the ast rule for Go and Node, and nginx's `optional_no_ca`).

**Second part built, 15 October 2026 (honesty):** `ast.client-certificate-untrusted` (V12.1.3, only ever a finding),
for Go's `ClientAuth` set to `tls.RequestClientCert` or `tls.RequireAnyClientCert` and Node.js's `requestCert: true`
beside `rejectUnauthorized: false` (ADR-018, Later). Left: nginx's `ssl_verify_client optional_no_ca`, which needs a
check in Rust since no grammar reads nginx settings, and Python's `CERT_OPTIONAL` with `getpeercert()`, which one
file cannot tie together.
