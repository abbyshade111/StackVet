# Error-handler signals in the app's code, the static half of V16.5.4

**Status:** open

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
