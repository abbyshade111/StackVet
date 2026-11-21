# Nightly: Windows fails: workflows::tests::untrusted_text_passed_through_env_or_safe_fields_is_not_found

**Status:** done, 20 November 2026

Recorded by the nightly CI routine on 28 October 2026. The Windows workflow's push run on `main` for commit `2da1540`
(pull request #2385, "Claim 0678", a change to backlog items only) ended red on 27 October 2026:
<https://github.com/abbyshade111/StackVet/actions/runs/40525578053>.

- **What failed:** one test in `sv-check`'s library tests, of 4,417 run: "4416 passed, 1 failed, in 344 groups (cargo
  exit 101)". The test is `workflows::tests::untrusted_text_passed_through_env_or_safe_fields_is_not_found`.
- **What the failure said:** `crates\sv-check\src\workflows.rs:876:64: called Result::unwrap() on an Err value: Os {
  code: 3, kind: NotFound, message: "The system cannot find the path specified." }`. Line 876 is the test helper
  `app` writing a workflow file into the folder it had just made.
- **A likely cause, not yet proved:** the helper names its folder `sv-workflows-<name>-<process id>` under the
  temporary folder, and two tests in the same process pass the same name, `"env"`: this one, and
  `each_way_of_handing_out_every_secret_is_found_where_it_is_written` (through `the_finding_in("env", ...)`). Run at
  the same moment, one test's `remove_dir_all` can delete the folder while the other is writing into it. Windows, where
  files are slower to create, is where that race would show first. A fix would give each test a folder of its own
  (for instance, a counter or the test's name in the folder's name); show it first by making the race happen.
- **Was it there before?** No: every other Windows run on `main` in the 26 hours before this routine ran passed,
  including the next push (`c930319`, pull request #2386), the push after it (`96eb360`, pull request #2387), and the
  scheduled runs on 27 and 28 October. No backlog item named this test before this one. Pull request #2385 changed
  only backlog items, so it did not bring the fault; the test code it runs was already on `main`.
- **Is `main` red?** No. Windows is not a required check, and it went green on the next push.

This routine records; it fixes nothing.

**Done, 20 November 2026.** The cause suggested above holds, and is wider: two pairs of tests in
`crates/sv-check/src/workflows.rs` passed one name to the folder helper, `"env"` (this test's table and
`each_way_of_handing_out_every_secret_is_found_where_it_is_written`) and `"inherit"` (`run("inherit", ...)` and the
same secrets test). The helper `app` now counts its calls into the folder's name, so every call gets a folder of its
own whatever name it is passed. Shown first by making the race certain: the new witness,
`crates/sv-check/src/workflows/folder_tests.rs`, makes two fixtures with one name and failed on every run before the
counter ("two fixtures given one name share the folder"); it passes with it. Locally: the workflow tests passed 30 runs
of 16 threads each, and `cargo test -p sv-check --lib` passed 2,241. Not run on Windows here; the Windows workflow runs
on the pull request's merge.
