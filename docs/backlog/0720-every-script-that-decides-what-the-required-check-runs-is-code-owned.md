# Every script that decides what the required check runs is code-owned

**Status:** open

Filed 21 November 2026, at the owner's word, from the security review of 21 November (finding A5). Title and text are
kept to the fix, as the review asked.

`.github/CODEOWNERS` asks for the owner's review on the workflows and on the scripts that judge the checks (ADR-051,
Later, 14 October 2026), so a change to how a check is judged waits for the owner. Some files that decide what the
required `test` check runs, or what it holds the repository to, are not in it. On 21 November #2509 changed
`tools/coverage.py` and merged with no review asked for, as the rules allow today.

- **To build:** CODEOWNERS names each of these, after confirming on `main` what each one decides:
  - `tools/ci_shards.py`, which deals the test files between the suite's jobs (`rust.yml`);
  - `tools/coverage.py`, whose lists the `test` job holds the findings to (`rust.yml`), and whose `--check` the
    website tests run;
  - `tools/py7zr-requirements.txt`, the pinned, hashed packages the archive tests install (`rust.yml`,
    `windows.yml`; ADR-089);
  - `tools/image_smoke.py`, which drives the image before it is published (`rust.yml`);
  - `tools/website.py`, which builds the pages the site's deploy publishes;
  - `tools/prompt_stamps.py` and `docs/prompts/`, the setup prompts and the fingerprints that hold them.
  `crates/sv-cli/tests/codeowners.rs` already fails on a CODEOWNERS path that does not exist.
- **And a floor under `test`:** the required `test` job fails when the suite ran fewer test programs than a floor
  written in the workflow, so the number of programs run cannot fall without the job saying so. The floor is taken
  from a real run on `main`, a little under it, with how to move it said beside it.
- **Check:** a pull request from the sessions' App that changes one of the files above is seen waiting for the owner's
  approving review; and with the floor raised above the real count in a trial, `test` fails.
- **Records:** ADR-051, a Later entry; `.github/CODEOWNERS`'s own comment says what was added and why.
