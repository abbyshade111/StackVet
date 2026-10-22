# Two tests in moved.rs canonicalize the expected data folder after deleting it, and fail on macOS when the temporary folder is under /var

**Status:** open

Filed 21 October 2026 at the owner's word ("yes, file the macOS path test fix"), found while building backlog 0324's
fourth split. Severity: low (a test fault on macOS only; CI runs on Linux, where both pass).

**What is wrong.** `a_copy_of_sv_reads_the_data_beside_it_and_checks_an_app_with_it` and
`the_install_script_puts_sv_and_its_data_in_a_folder_of_their_own`, in `crates/sv-cli/tests/moved.rs`, delete their
scratch folder (`std::fs::remove_dir_all(&root)`) and only then compute the expected data folder as
`home.join("data").canonical().unwrap_or(home.join("data"))`. With the folder gone, `canonical()` fails and the
expected path is the one as written, while `sv` reports the path it resolved.

**Where.** `crates/sv-cli/tests/moved.rs`, the assertions at about lines 70 and 414 on 21 October 2026.

**Evidence.** On macOS, where `/var` is a link to `/private/var`, with the temporary folder under `/var/folders/…`
(the Mac's own, outside Claude Code's sandbox): "the copy read some other data", left
`/private/var/folders/…/sv-moved-installed-…/share/stackvet/data`, right `/var/folders/…/share/stackvet/data`; the
install-script test fails the same way. Inside the sandbox, whose temporary folder is not behind a link, both pass.

**How it bites.** A full local run on a Mac reports two failures that are not faults in `sv`, so a real failure among
them is easier to miss.

**Fix.** Compute the expected path before the folder is removed (or remove it after the assertions), so `canonical()`
resolves it; and assert first that it did, since a fallback that quietly keeps the unresolved path is how this hid.
Break it to check: canonicalize after the removal again and watch both tests go red on a Mac with `TMPDIR` under `/var`.

**Record.** None: a test's setup, not what `sv` reads.
