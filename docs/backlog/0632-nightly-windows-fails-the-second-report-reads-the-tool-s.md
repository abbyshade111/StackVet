# Nightly: Windows fails: the_second_report_reads_the_tool_s_commit_since_the_first_and_the_record_only_when_asked

**Status:** open

Recorded by the nightly CI routine on 22 October 2026. One test failed on Windows, and only there: the Windows
workflow's push runs on `main` for two commits went red at the step "The whole test suite", with 4143 passed and
1 failed, in 327 groups (cargo exit 101):

- commit `633415a` (pull request #2195, "The report says what the AI coding tool did that is worth a look"):
  <https://github.com/abbyshade111/StackVet/actions/runs/39699093547> (22 October 2026, 00:20 UTC);
- commit `0110dae` (pull request #2196, sv-report's build-loop lines, backlog 0324's first split):
  <https://github.com/abbyshade111/StackVet/actions/runs/39700616858> (00:35 UTC).

The failing test is `the_second_report_reads_the_tool_s_commit_since_the_first_and_the_record_only_when_asked`, and
the failure said:

    crates\sv-cli\tests\tool_behavior_report.rs:129:9:
    Error: what the hook handed over is not JSON
    Caused by: invalid escape at line 1 column 101

- **Was it there the night before?** No. The test is new in #2195, and the scheduled Windows run of 21 October
  (`1eb7c5e`) passed.
- **Which pull request brought it:** #2195 (`633415a`); the run before it, #2194 (`9ee85a1`), passed. #2196 only
  inherited it.
- **Already fixed forward:** #2197 (`b7c5f05`, commit `a002757`, "The tool-behavior report test runs on Windows")
  builds the hook's JSON rather than pasting a Windows path, whose backslashes are JSON escapes, into it. Every
  Windows run on `main` from #2197 on passed, and so did the scheduled run of 22 October on `755ac4d`
  (<https://github.com/abbyshade111/StackVet/actions/runs/39756541737>): 4147 passed, 0 failed.
- **What is left:** nothing found by this routine. The next session can read #2197 and close this item as done. The
  Rust workflow (Linux) was green on both commits, so `main`'s required checks never went red.
