# A 500 on a well-formed sign-up is set aside as "neither reported nor credited" and never named as a weakness

**Status:** claimed by honesty, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the VS Code app. Severity: low. Read by the
reviewing subagent, checked in `seen.json` by the orchestrating session.

**What is wrong.** `signup-long` (a 73-character password) answered 500; the app log shows zxcvbn 4.5.0 refusing
passwords over 72 characters, while the app states a limit of 1,024. The report says a crash is not an answer and
credits nothing, which is honest, but a crash on a request a person could make is itself a weakness (CWE-755), and
the Cursor app has the same path (`familygames/passwords.py:10, 26`).

**Fix.** Keep the non-credit and add a low finding, "the app crashed (500) on a request a person could make", with the
request named (field and length, never the value), under the error-handling requirement it speaks to (read V16.5's
text first).

**Record.** None, or a Later line on the probe's record if a new finding counts as evidence.

**Built, 15 October 2026 (honesty):** a 500 on one of `sv`'s ordinary sign-ups (A, B, the control account, and the
long password) is now a low finding, `probe.crashed-on-ordinary-request`, under V16.5.3 (its text read first: "fails
gracefully and securely, including when an exception occurs"). It names the request, its status, and its fields by name
and length, never a value. What the crash would have answered stays withheld. ADR-021, Later entry of 15 October
2026. Fake-app case: `crashes_on_long_password`.
