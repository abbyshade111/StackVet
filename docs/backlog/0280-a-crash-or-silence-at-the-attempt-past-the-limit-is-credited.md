# A crash or silence at the attempt past the limit is credited as pushing back (V6.3.1, V6.6.3)

**Status:** claimed by builder-none, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** V6.3.1: "Verify that controls to prevent attacks such as credential stuffing and password brute force are implemented according to the application's security documentation." V6.6.3: "Verify that a code based out-of-band authentication mechanism is protected against brute force attacks by using rate limiting...". The check sets `status_changed = last.0 != first_status` and `pushed_back = status_changed || refused || slowed`, then credits (in part). So first attempts 401, attempt N+1 answered 500 (the app crashed) or 0 (no answer) reads as a limiter. The `real()` guard (`status != 0 && status < 500`) is applied only to the "one more let through" branch; the comment above it ("a crash ... must not read as a status that changed") is not what the code does.

**Where.** `crates/sv-check/src/signed_in/signin.rs:400-401` and `:460` (`probe.failed-sign-ins-unlimited`, V6.3.1); `crates/sv-check/src/signed_in/codes.rs:360-361` and `:420-426` (`probe.email-code-guessing-unlimited`, V6.6.3).

**Evidence.** `let status_changed = last.0 != first_status;` … `let real = |status: u16| status != 0 && status < 500;` used only inside `if !(status_changed || refused) && alike`, then `let pushed_back = status_changed || refused || slowed;`.

**How it bites.** An app that falls over at the 27th attempt is credited with a rate limit, against the project's own rule that a crash never turns a finding into a pass.

**Fix.** Require `real(last.0)` (and `real(answers[allowed+1].0)`) before crediting; a 5xx or no answer past the limit is "not assessed"; a fake-app test that answers 500 past the limit and asserts no credit, red before the fix.

**Record.** No record governs these files by name; ADR-053 governs in-part credit: an "unchanged, because" line.
