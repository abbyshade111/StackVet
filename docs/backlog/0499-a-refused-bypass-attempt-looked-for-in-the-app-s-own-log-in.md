# A refused bypass attempt looked for in the app's own log, in part, with the log destinations the code names shown

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V16 Security Logging and Error Handling (V16.1.1, V16.2.3, V16.3.3, V16.4.3)**

- **V16.1.1** (L2), a log inventory. Notes question exists. Route 4: the logging libraries, their transports and
  destinations, and the log files the code names (winston transports, `logging.handlers`, `log/` paths, Sentry,
  Datadog, syslog, CloudWatch SDKs), listed beside the question. Same as partial-checks. Tier: *documented by the
  owner*.
- **V16.2.3** (L2), logs go only where the inventory says. Hand check exists. The same list, plus the hosts the fenced
  run saw the app try to reach that look like log shipping (Sentry's, Datadog's, Logtail's, Papertrail's hostnames),
  shown as "the app tried to send to these during the run". Comparing with the inventory needs the inventory in a
  readable form; the notes are free text, so this stays *checked by hand* with the observation beside it. No new credit.
- **V16.3.3** (L2), the app logs attempts to bypass its controls. Hand check exists. Route 2, cheap because the
  mechanism exists: the run already reads the app's output for V16.3.1 and V16.3.2 (`probe.authentication-logged`,
  `probe.authorization-failure-logged`); the probes that already attempt a bypass (`probe.validation-only-in-the-browser`,
  `probe.flow-step-skipped`, `probe.action-done-twice`, `probe.cross-site-request-accepted`) carry a marker, and the
  marker found in the app's output after the refused attempt is *checked in part* ("one kind of bypass, the app's own
  output"); not found is a finding, as V16.3.2's absence is. Tier: `sv`'s own, in part. Same as partial-checks; the
  reuse of the two existing log readers is the point.
- **V16.4.3** (L2), logs sent to a separate system. Design question exists. Route 4: the log-shipping SDK or
  transport found, with its line, beside the question; absence is nothing (a host-level agent ships logs with no trace
  in the code). Same as partial-checks. Tier: *attested by the owner*.
