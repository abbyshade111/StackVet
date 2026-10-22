# Long functions and long files beyond the recorded split: seven files over 3,000 lines and 68 non-test functions over 150

**Status:** claimed by claude/backlog-work, 20 October 2026

**Was queued on the coordination board** (backlog 0273) until the board was removed on 20 October 2026 (ADR-061, Later); it is now taken the normal way, by a claim. From the deep review of 12 October 2026 (backlog 0277), area: code quality. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** The `ast.rs` and `sbom.rs` split (0187) is done; these are the next along natural seams: `docker.rs` holds the fence, the helpers (mail, OIDC, model, browser), the install step, and the probe script runner; `signed_in/mod.rs` holds the HTTP/cookie/page helpers and the suite runner.

**Where.** Files: `crates/sv-run/src/docker.rs` 5,145; `crates/sv-check/src/signed_in/mod.rs` 4,792; `crates/sv-check/src/ai.rs` 3,924; `crates/sv-cli/src/main.rs` 3,576; `crates/sv-check/src/signed_in/sessions.rs` 3,275; `crates/sv-report/src/lib.rs` 3,239; `crates/sv-manifest/src/lib.rs` 3,010. Functions: `ai.rs:788 run` (712 lines), `signed_in/mod.rs:1741 run_checks` (675), `sv-report/src/html.rs:218 page` (660), `markdown.rs:171 compliance` (483), `production.rs:721 run` (450), `main.rs:1354 cmd_check` (449), `adapters.rs:1155 run_one_recorded` (379).

**Evidence.** Line counts above.

**How it bites.** Maintainability; each is the file a new session reads whole to change one thing.

**Fix.** Split `docker.rs` and `signed_in/mod.rs` first, along the seams named, in two or three short pull requests each, behavior unchanged; the ten `#[allow(clippy::too_many_arguments)]` (`sv-cli/src/review.rs` ×6, `signed_in/{account_changes,owner_field,uploads}.rs`, `sv-cli/src/{seen,static_scan}.rs`) mark where a parameter struct is owed.

**Record.** ADR-019 and ADR-052 govern `docker.rs`: a Later line naming the new files' names, since their Governs lists change.

**The owner's go, 14 October 2026 (evening):** once the assigned and ready work on the board is done, build this on its own, with nothing else in flight; the data-and-figures item 0568 follows it.

**Checked against `main`, 20 October 2026** (the backlog clean-up): every file still on the list has grown since it was written. The owner's go of 14 October waited for the board's work, which no longer exists; the refactor can be claimed the normal way.

**Checked against `main`, 21 October 2026, and the first split.** `docker.rs` (2,154 lines) and `signed_in/mod.rs`
(2,147) were split since this was written, into `docker/{fence,probes,helpers,install,containers}.rs` and the
`signed_in` modules; the rest grew: `sv-report/src/lib.rs` 4,317, `ai.rs` 4,048, `sv-cli/src/main.rs` 3,730,
`signed_in/sessions.rs` 3,510, `sv-manifest/src/lib.rs` 3,455, and new to the list `signed_in/fake_app.rs` 3,898,
`sv-cli/src/assemble.rs` 3,494, `sv-cli/src/review.rs` 3,334, `signed_in/uploads.rs` 3,139, and
`signed_in/passwords.rs` 3,133. The `#[allow(clippy::too_many_arguments)]` count is 18. First split: the build loop's
types and report lines out of `sv-report/src/lib.rs` into `sv-report/src/build_loop.rs`, still named from the crate's
root, and its three inline test modules into sibling files, leaving 3,650 lines. Next: `ai.rs` (`run`), then
`main.rs`.

**Second split, 21 October 2026.** `kill_switch` out of `ai.rs` into `ai/kill_switch.rs`, and the checks of what the
app's output recorded (`LogMarkers`, `logged`, and their rules' readers) into `ai/logged.rs`, still named from `ai`;
`ai.rs` goes from 4,048 lines to 3,240. ADR-071, ADR-073, ADR-074, ADR-075, ADR-079, and ADR-088 name the new files.
Next: `run` in `ai.rs` (756 lines), then `sv-cli/src/main.rs`.

**Third split, 21 October 2026.** `run` in `ai.rs` hands steps 7 (the MCP tool questions) and 8 (the rate check) to
`mcp_questions` and `rate_question`, shaped as its other steps' helpers are, and goes from 736 lines to 461. Next:
`sv-cli/src/main.rs`, then `run_checks` in `signed_in/mod.rs`.

**Fourth split, 21 October 2026.** `sv-cli/src/main.rs`'s six inline test modules into sibling files
(`main_tests.rs`, `main_dependency_gap_tests.rs`, and the rest), from 3,730 lines to 3,167. Next: `main.rs`'s
`Command` table and `cmd_probe`, then `run_checks` in `signed_in/mod.rs`.

**Fifth split, 21 October 2026.** `run_checks` in `signed_in/mod.rs` hands its checks 9b to 9h to `late_checks`, word
for word, from 819 lines to about 720. `check_guard_tests` reads both bodies now: with it reading `run_checks` alone, a
check called bare in `late_checks` passed every test, and with both it is named. Next: `main.rs`'s `Command` table and
`cmd_probe`, then `html.rs`'s `page` (863 lines) and `markdown.rs`'s `compliance_as` (650).

**Sixth split, 21 October 2026.** `page` in `sv-report/src/html.rs` hands six of its sections to functions of their
own (the requirement tables, the AI process, the threats, what only the owner can check, the tests to write, and what
sits outside the tables), word for word, from 863 lines to 467; a report.html made before and after on the example app
differs only in the run's id and its timings. Next: `compliance_as` in `markdown.rs` (650), then `main.rs`'s
`Command` table and `cmd_probe`.

**Seventh split, 21 October 2026.** `compliance_as` in `sv-report/src/markdown.rs` hands five of its sections to
functions of their own, word for word, from 650 lines to 355; the four pages made before and after on the example app
differ only in their timings. Next: `main.rs`'s `Command` table and `cmd_probe`.

**Eighth split, 21 October 2026.** `main.rs`'s `Command` struct and `COMMANDS` table into `sv-cli/src/commands.rs`,
word for word; `sv --help`, each command's `--help`, and the refusal of an unknown option read the same before and
after. Next: `cmd_probe` and `cmd_check` in `main.rs`, `the_owners_word` and `running_app` in `assemble.rs`, and
`review` in `review.rs`.

**Ninth split, 21 October 2026.** `the_owners_word` in `sv-cli/src/assemble.rs` hands the gaps it says for the notes
file, the design decisions, the design questions, and the checks done by hand to functions of their own, word for
word, from 590 lines to 399. With each call taken out, only the notes' went red in any test: no test held the other
three's words. `assemble_owner_gaps_tests.rs` now holds each, red with its function emptied. Next: `running_app` in
`assemble.rs`, `review` in `review.rs`, and `cmd_check` and `cmd_probe` in `main.rs`.

**Tenth split, 22 October 2026.** `running_app` in `sv-cli/src/assemble.rs` hands what the run's note says of the copy
to `run_note_details`, and what the app's own tests showed to `own_tests`, from 496 lines to 301. With either returning
nothing, every test in sv-cli passed, those that start the app among them: nothing held how the app's own tests become
credit. `assemble_own_tests_tests.rs` now holds `own_tests` (a suite that passed credits the requirement its test names;
one that failed credits nothing and keeps its output; one stopped credits nothing and says so; none declared is said).
`run_note_details` is still held by nothing: it needs a whole `sv_run::RunOutcome` and `RunPlan`, which no test can
make yet. Next: `review` in `review.rs`, and `cmd_check` and `cmd_probe` in `main.rs`.

**Eleventh split, 22 October 2026.** `review` in `sv-cli/src/review.rs` held the four confirmations asked last and the
checklist twice, word for word; they are one `ask_last`, called from both places, and `review` goes from 515 lines to
433. The parameter struct the `too_many_arguments` allows marked as owed is `Asking`, which `confirm_scope`,
`confirm_not_offered`, `confirm_policy`, `confirm_capabilities`, and `tick_checklist` now take, their bodies word for
word: five allows of nine in `review.rs` and `review/ticks.rs` gone, and four left (`review_then_report`, `sign_again`,
`record_finding`, `record_confirmation`). With `ask_last` emptied, more than twenty review tests go red. Next: the four
allows left, then `cmd_check` and `cmd_probe` in `main.rs`.

**Twelfth split, 22 October 2026.** Three of `review.rs`'s four allows left: `record_finding`'s was never needed (it
takes seven, and clippy's limit, with no `clippy.toml`, is more than seven); `sign_again` and `record_confirmation` take
`Asking` with what is their own. `review_then_report` keeps its allow: three of its eight are handed straight to
`report_again::write_again`, so a struct for them belongs there first. Next: `cmd_check` and `cmd_probe` in `main.rs`,
then the allows in `signed_in/{account_changes,owner_field,uploads}.rs` and `sv-cli/src/{seen,static_scan}.rs`.
