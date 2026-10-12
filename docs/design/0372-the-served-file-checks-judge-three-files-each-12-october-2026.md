# The served-file checks judge three files each (12 October 2026)

Backlog 0232, part 4 (group D of the one-sample inventory, in part), and ADR-053, "Later, 12 October 2026" (the third).

**The problem.** `served_upload_checks` uploaded one `.php` file for V5.3.1 and one `.html` file for V3.2.1 and credited
each in part from it. An app may refuse or neutralize one extension and not its neighbors.

**What was built.** Each check has a set of three files: `sv-probe.php`, `.phtml`, and `.php5` (ids `upload-code`,
`upload-code-1`, `upload-code-2`), and `sv-probe.html`, `.htm`, and `.xhtml` (`upload-page-file` and on). Each is sent,
fetched back from `serves-at`, and judged as before; a file run or rendered is a finding naming it. The credit lists
every file judged safe, with where and how, and is whole only when all three were. The render check counts
`application/xhtml+xml` as a page as well as `text/html`. The crash guard lists `upload-code-` and
`upload-page-file-`, so a crash at any of the new requests is never read as a file served safely.

**The stand-in app.** Three new flaws, each touching one file alone: `runs_uploaded_php5`, `renders_uploaded_xhtml`,
and `refuses_phtml`. Crash scenarios for the first two.

**Tests.** `crates/sv-check/src/signed_in/uploads/served_set_tests.rs` (four), and `a_correct_app_confirms_all_four_upload_questions`
now expects these two credits whole.

**Not built.** More than one upload route (the manifest's `upload` is one entry); the one-file checks (size, the
antivirus test file, the SVG); the archives.
