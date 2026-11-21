# Nightly: Rust and Windows fail: the board page test reads partly done items

**Status:** open

Recorded by the nightly CI routine on 21 November 2026. One test failed on `main` at commit `2b6aa2b` (pull request
#2435, "Give each workflow test fixture a folder of its own (backlog 0679)"), on three runs:

- Rust, push, `suite (cli-2)`: <https://github.com/abbyshade111/StackVet/actions/runs/44857797794>
- Rust, nightly schedule, `suite (cli-2)`: <https://github.com/abbyshade111/StackVet/actions/runs/44860636410>
- Windows, push: <https://github.com/abbyshade111/StackVet/actions/runs/44857797803> (4462 passed, 1 failed)

- **What failed:** `the_board_page_sorts_the_items_by_status_and_carries_the_tools_counts`, in
  `crates/sv-cli/tests/docs_page.rs`, at line 226: "no --claimed item, and its heading says None." The same test
  and message on Linux and on Windows.
- **Why, as the fix's pull request tells it:** the board page puts claimed and partly done items under "Being worked
  on", but the test asked only for claimed items. After #2434 made 0625 partly done and #2435 finished 0679, the last
  claimed item, the heading listed 0625 where the test expected "None.". Each pull request passed alone; only the two
  together turned `main` red.
- **Was it there the night before?** No: the nightly runs of 20 November 2026 (commit `f467ebe`) passed.
- **Is `main` red?** No longer. Pull request #2437 ("Fix main: the board test reads partly done items as being worked
  on") merged at 03:35 UTC, about 50 minutes after #2435, and every Rust and Windows run on `main` since passed,
  including the Windows nightly on `4d3d4c4` (4463 passed, 0 failed). The last pull request merged before `main` went
  red was #2435.
- **What to do:** most likely nothing more: check that #2437's fix holds, and mark this done. This routine records; it
  fixes nothing.
