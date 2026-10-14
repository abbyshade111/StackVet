# The browser-validation probe reads any answer under 400 as "accepted", so a refusal shown on a 200 page is a finding

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the VS Code app. Severity: medium (a false medium
finding on V2.2.2). Verified against `seen.json` and the code by the orchestrating session.

**What is wrong.** The probe sends a sign-up with a value that breaks the form's own `maxlength=254` on `email`. The VS
Code app refuses it the way Flask apps do: it renders the form again with "Enter a valid email address." and status
200 (`app.py:323-324, 355`), while the control sign-up answered 302. `crates/sv-check/src/signed_in/uploads.rs` sets
`accepted = response.status < 400` and files "A rule the form states is not applied on the server" (V2.2.2). The
control's status is in hand and not compared.

**Fix.** Accepted means "answered the way the control did" (same status class, same redirect target), and a 200 where
the control redirected is "could not tell" unless signing in as the broken account works. Say in the step what was
compared. Fake-app case: a sign-up that re-renders the form with 200 on a bad value, and watch the finding go.

**Record.** ADR-0NN: unchanged, because the probe's bar is unchanged; only its reading of an answer is corrected.
(Check which record governs `signed_in/uploads.rs` before the build.)

**Built by the sweep:** this is the first case of backlog 0560 (ADR-118); it closes with it.
