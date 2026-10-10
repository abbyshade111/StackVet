# What sv saw of the running app, kept beside the report with credentials removed

**Status:** partly done: part 1: probe exchanges are kept, but no finding links to them (claimed by paper-facts, 10 October 2026)

The owner's decision A of the observability review (0226, part 3), 9 October 2026: "yes to A, C, and D"; recorded as
ADR-082. At the owner's word that evening ("yes, please go ahead"), an item of its own, as the backlog's rules now say
a review's findings are, its parts claimed and finished with `backlog.py claim` and `done`. Claimed by session
paper-facts, each part to land as a pull request of its own.

Today a credit or a finding from the running app can be followed only to the sentence `sv` wrote about it. Each
probe's response (`ProbeResponse`, `crates/sv-check/src/probes.rs`), what the stand-in services received, and the
app's own log are held in memory and dropped. ADR-082 says what to keep, how it is bounded, and that everything kept
passes through `secrets::redact_text` first.

1. **Each probe's exchange.** The request (method, path) and the response (status, headers, the body excerpt `sv`
   already keeps), each with an id, in one file in the report folder, sealed with the report; each finding and credit
   read from the running app names the ids it rests on. Bounded per exchange and in all, with what was left out said.
   A test plants a key built from pieces in a response and fails when it reaches the file.
   **Part status:** partly done: the stayed-up credit names no answers yet: its readings (docker inspect and the health path) are not kept in seen.json, which needs a decision on what the record keeps

2. **What the stand-in services received.** The test model's record of what it was sent and the paths it was asked
   for, the test sign-in provider's requests, and each mail's recipient, subject, and time (not its body), saved before
   their containers are removed.
   **Part status:** done, 9 October 2026

3. **The app's own log lines the log checks rest on.** The lines a logging check matched, and a short tail of the log,
   redacted, so a V16 credit can be checked by a person.
   **Part status:** done, 10 October 2026

4. **An outside tool's raw output, when asked for.** `--keep-tool-output` copies each tool's report, redacted, beside
   the report; its version, arguments, and exit code are recorded always (0226, part 2, item 14).
   **Part status:** done, 10 October 2026

**Said plainly, in every part:** the report says the record is there, what it holds, and that it is the app's own text,
which can hold personal data the app was given during the run (only `sv`'s own test accounts sign in).

**Part 3, checked 10 October 2026:** the AI feature's two log checks now keep the line they matched (the service's own failure, V16.5.2 and V16.5.3; the test tool call, C12.4.2), through the same redaction and cut as the signed-in lines (`crates/sv-check/src/ai.rs`, `crates/sv-cli/src/seen.rs`). Tests: `the_lines_the_ai_log_checks_matched_are_kept_for_a_person_to_read` and `the_lines_the_ai_feature_read_are_kept_and_a_key_in_them_is_not`; the first was broken on purpose and went red.
