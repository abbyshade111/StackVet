# After a rate limiter's wait, an AI finding names the limiter's answer as its evidence, not the answer it judged

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-check`. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/ai.rs:2418-2430` (`ask(http, 32, "PLAIN", AFTER)` twice, the second after
  `http.wait`), `:2404` and `:2464` (evidence `question_id("PLAIN", 32)`), `:2571-2583` and `:2557`, `:2627` (the
  same for 52), `:2998-3003` (`hidden_input_questions`' `asked` closure asks again under the same `n`);
  `crates/sv-check/src/signed_in/recording.rs:16, 48-58` (a repeated id is kept as `…-2`).
- What is wrong: the second ask is recorded as `ai-plain-32-2`, but `FAILURE_HANDLED`, `FAILURE_SHOWN`,
  `HANG_HANDLED`, `HIDDEN_INPUT`, and `CHARSET` name `ai-plain-32` (or `ai-smuggle-16`, `ai-oddchars-17`): "What
  StackVet saw" then quotes the 429 that was waited out rather than the answer the finding rests on. The `LANGUAGES`
  finding (`:2901-2906`) names all four injection questions whatever `through` holds. `evidence_tests.rs` only
  checks that every id named was recorded, which `ai-plain-32` is.
- What to do: have `ask` return the id it was recorded under (or number the retry `n` with a suffix the finding
  can name), and name the question that got through in `LANGUAGES`. Must not lose: the first question staying
  recorded.
- How verified: read.
- Overlap: 0381 (partly done; built the recording and the evidence ids). This is a gap in that build; new.
- Needs a decision record?: no.
