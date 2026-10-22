# The AI checks keep the prompt StackVet wrote, so What StackVet saw shows what was asked as well as what came back (20 October 2026)

**Status:** done, 22 October 2026

Filed from backlog 0381 at the owner's choice of 20 October 2026, asked in a session. Severity: low (evidence that
shows the answer and not the question it answered).

**What is missing.** Every AI finding now names the questions whose answers showed it (0381), and "What StackVet saw"
quotes those answers. It does not show what was asked: `crates/sv-check/src/signed_in/recording.rs` keeps no
request's headers or body, rightly, since they carry the test accounts' passwords and session cookies. So a finding
such as "a flagged prompt injection is passed on" shows the app's reply and not the injection it was replying to, and
a reader can neither judge it nor try it again.

**The owner's choice, 20 October 2026:**

- **Keep the prompt StackVet wrote, from StackVet's side, and nothing else.** The string the AI suite composes
   (`ask`, `send_filled`'s prompt, before the form's own fields are filled in) is recorded beside the question's id,
   as an optional field the AI suite alone fills. Never the request's body or headers, and never the fields
   `send_filled` takes from the app's page, which can hold a session's token. The test accounts' passwords are in no
   prompt; the few prompts with test data use data `sv` made up for the run (a planted note's title).
- **Quoted word for word, escaped, and labeled.** "What StackVet saw" shows it under "StackVet sent", through the
   same escaping the app's answers go through: several prompts carry invisible characters on purpose (the
   hidden-input checks) or a textbook injection, and they are shown as text, never acted on. The attack prompts are
   quoted as they were sent, not described.
- **Recorded:** a Later entry on ADR-082 saying the records now hold `sv`'s own prompt for an AI question, and still
   no request body.

**Break it and watch:** a test that records a question with a prompt and a form field holding a token must show the
prompt under "StackVet sent" and the token nowhere; and a prompt with a zero-width character must be shown escaped.
