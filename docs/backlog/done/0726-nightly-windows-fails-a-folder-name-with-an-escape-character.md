# Nightly: Windows fails: a_folder_name_with_an_escape_character_does_not_reach_the_terminal

**Status:** done, 22 November 2026

Recorded by the nightly CI routine on 22 November 2026. This routine records; it does not fix.

**What failed.** The Windows workflow (`windows.yml`) on `main`, step "The whole test suite", in 36 push runs in a row,
from commit 7c7cbf5c (21 November, 22:04 UTC) to commit 06e7253e (22 November, 02:36 UTC). One test failed each time,
`a_folder_name_with_an_escape_character_does_not_reach_the_terminal` in `crates/av-cli/tests/report.rs`; the first run
counted 4,499 passed and 1 failed, the last 4,623 passed and 1 failed.

- First failing run: https://github.com/abbyshade111/StackVet/actions/runs/44954796889 (commit 7c7cbf5c)
- Last failing run: https://github.com/abbyshade111/StackVet/actions/runs/44976909834 (commit 06e7253e)

**What the failure said.** Making the test's folder was refused:

    crates\av-cli\tests\report.rs:217:35: called `Result::unwrap()` on an `Err` value:
    Os { code: 123, kind: InvalidFilename, message: "The filename, directory name, or volume label syntax is incorrect." }

Windows allows no control character in a file or folder name, so the test's setup cannot be made there.

**Where it came from.** The test arrived with pull request #2481 (AgentVet part 2, backlog 0705), merged as 7c7cbf5c;
the run before it, on 691b65b6 (pull request #2482), passed. It was not there the night before: the scheduled Windows
run of 21 November passed.

**Already fixed on `main`.** Pull request #2517 (commit 78134547, merged as 5ca6d415) makes the test not run on Windows,
and says the cleaning of what `av` prints stays held there by `proxy_report.rs`. Every Windows run since, the scheduled
one of 22 November included (run 44986172656: 4,645 passed, 0 failed), passed. Nothing is left to do here but keep the
record, so this item is closed as done the day it was opened.
