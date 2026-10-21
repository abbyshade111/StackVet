# Error-handler signals in the app's code, the static half of V16.5.4

**Status:** done, 21 October 2026

**Was queued on the coordination board** (backlog 0273) until the board was removed on 20 October 2026 (ADR-061, Later); it is now taken the normal way, by a claim.

From 0085, part 3, whose done note lists this as "not done from the proposals"; split out on 11 October 2026 when
0085 was closed, so it stays visible. The running half of V16.5.4 is built: `probe.app-stopped-during-questions`
(`crates/sv-check/src/running.rs`) finds an app that stopped, restarted, or went quiet while it was asked.

**What is left.** The static, supporting half proposed in `docs/PARTIAL-CHECKS.md` (V16.5.4): signs in the app's own
code that it has a last-resort error handler, such as `process.on('uncaughtException')` or `'unhandledRejection'`,
Express error middleware with four arguments, Flask `errorhandler(Exception)`, FastAPI
`exception_handler(Exception)`, Spring `@ControllerAdvice`, ASP.NET `UseExceptionHandler`, and Go recover
middleware. Supporting only: a handler found is a sign, not proof that it works, so it may add to what the running
half says but never credit V16.5.4 alone, and its absence is not a finding (a framework may supply one by default).
Read the requirement's own text before citing it. Builds on `data/ast-rules.json` and `crates/sv-check/src/ast.rs`.

**Set aside on 12 October 2026, the owner's choice when asked.** Claimed that day to build the signs as information
beside V16.5.4, and found before any code was written that there is nowhere to put them: V16.5.4 is a level 3
requirement, and `sv` holds every app to level 1 or 2, so no report has a line for it (requirement lines are built only
for the app's level). A finding at `info` would land in the list of things to fix, which reads as a fault for what is
good news, and the section for findings above the app's level is for faults too. The running half shows the same
shape today: a stop is an ordinary finding, and a clean run's credit names no requirement. Build this when `sv` first
holds an app to level 3, where the signs can sit beside the V16.5.4 line as supporting evidence
(`RequirementLine.supported_by`), never a credit.

**Checked against `main`, 20 October 2026** (the backlog clean-up): still nothing built. It stays set aside, at the owner's choice of 12 October, until `sv` holds an app to level 3: backlog 0507 carries that, and 0263 is taken with it.

**Taken up, 21 October 2026,** now that `[app] level = 3` holds an app to level 3 (ADR-130, backlog 0507), so a
report has a V16.5.4 line for the signs to sit beside.

**Proposed decision** (Status: proposed with this claim; accepted as a Later entry on ADR-130 in the pull request that
builds it):

- **V16.5.4 joins `manualOnly`** in `data/knowledge/applicability.json`. That a last-resort handler catches *all*
  unhandled exceptions is not something reading code or a bounded run can show. The running half,
  `probe.app-stopped-during-questions`, only ever finds a failure, so nothing it says changes.
- **The signs are a static check of `sv`'s own,** reading the app's code for a last-resort handler. That includes
  `process.on('uncaughtException')` and `'unhandledRejection'`, Express error middleware with four parameters, Flask's
  `errorhandler(Exception)` and `register_error_handler(Exception, ...)`, FastAPI's `exception_handler(Exception)`,
  Django's `handler500`, Spring's `@ControllerAdvice`, ASP.NET's `UseExceptionHandler`, and Go `recover()` in
  middleware. It names each place it finds. A handler found is a credit for V16.5.4 with what it saw as its scope.
  V16.5.4 being manual-only, the report shows it beside the line as supporting evidence (`supported_by`), never as
  *checked*. None found is neither a finding nor said as a fault, since a framework may supply a handler by default.
- **A design question for V16.5.4,** since `human_checks.rs` holds that every requirement only a person can settle is
  explained somewhere. Does a last-resort handler catch every unhandled exception, log what happened, and keep the app
  running? It points at the places the check found.
- Level 3 only, as V16.5.4 is: an app below level 3 has no line for it, and sees nothing of this.

