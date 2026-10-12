# Error-handler signals in the app's code, the static half of V16.5.4

**Status:** open

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
