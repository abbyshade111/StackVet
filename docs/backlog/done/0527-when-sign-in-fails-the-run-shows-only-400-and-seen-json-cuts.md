# When sign-in fails, the run shows only "(400)" and seen.json cuts the page before its error text

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity app. Severity: medium (the one
answer a person needs to fix the run is missing). Read by the reviewing subagent and the orchestrating session.

**What is wrong.** sv's sign-in to the Antigravity app answered 400, so 16 signed-in checks "stopped". The run step says
"signed in as A (400)" and nothing else; `seen.json` keeps 4,000 characters of the page, and the app's inline CSS runs
past that, so the error text (the only 400 branch is the CSRF check, `app/main.py:288-292`) is cut. Whether the fault
is the app's token or sv's token extraction cannot be told from the report.

**Fix.** For a failed sign-in, keep the whole page or pull the visible error text (`.error`, `.alert`, flash blocks,
`<title>`) into the run step, and say which form fields were sent (names only, never values). Then the person, or the
AI coding tool reading the report, can fix the manifest or the app.

**Record.** ADR-0NN: unchanged, because what seen.json keeps is a presentation matter; check the record that governs it.

**Done, 15 October 2026.** Two halves:
- **The words are kept.** An answer of 400 or above that runs past the 4,000-character cut keeps, after it, up to
  1,000 characters of the words a person would read on the page, styles and scripts left out
  (`kept_answer` in `crates/sv-run/src/docker.rs`; `sv_check::text::visible_words`, moved there from sv-report so
  both use one). So `seen.json` has a refused sign-in's error text, as it has a stack trace past the cut.
- **The run says why.** A sign-in answered 400 or above is a step of its own ("failed" from 500 up, since a crash is
  not a refusal, ADR-021): "signing in as A was refused (400, with
  the page's anti-forgery token): it sent the fields `csrf_token`, `password`, `username`, and the page said: “…”"
  (`crates/sv-check/src/signed_in/refused.rs`). When the first private page then does not open, which is also how a
  sign-in answered 200 with the form shown again is caught, the "not assessed" reason ends "To tell which: the sign-in
  answered 400; it sent the fields …, and the page said: “…”". Field names only, never values; a page that repeats the test password is not quoted at all (only a yes or no is
  taken from the password, so no text can carry it; CodeQL flagged the first build, which masked it in place), and characters that move or hide text are dropped; the page's words are cut to 300
  characters.

Not done: the visible-text pick is the page's words in order, not `.error` or `.alert` blocks by name; a page whose
error sits below 300 characters of navigation is quoted from its navigation. Tests:
`sv-run` `docker/error_words_tests.rs` (3) and `sv-check` `signed_in/refused_tests.rs` (6), one of them a run against
the fake app with a broken sign-in; each half broken on purpose turned its test red.
