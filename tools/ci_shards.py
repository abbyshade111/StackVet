#!/usr/bin/env python3
"""Deals `sv-cli`'s test files between the CI jobs that run them side by side (backlog 0192 part 1, ADR-106).

Until 12 October 2026 one CI job ran the whole suite, one test program after another, in about eighteen minutes;
`sv-cli`'s test files were 877 of the suite's 1,221 seconds. They are now dealt between two jobs, beside a third that
runs every other crate. The files are dealt in name order, one to each job in turn, so a new file joins a job without
anyone choosing one, and the two came out at 424 and 423 seconds on the run they were measured on.

    python3 tools/ci_shards.py args 1 2     # the `--test NAME` arguments for job 1 of 2
    python3 tools/ci_shards.py --self-test

A file in neither job, or in both, would be a test never run, or run twice; `crates/sv-cli/tests/ci_shards.rs` fails
when the jobs together are not every file exactly once.
"""

import pathlib
import sys

TESTS = pathlib.Path(__file__).resolve().parent.parent / "crates" / "sv-cli" / "tests"


def names(folder=TESTS):
    """Every test file's name, without `.rs`, in name order."""
    return sorted(p.stem for p in folder.glob("*.rs"))


def deal(files, job, jobs):
    """The files job `job` (from 1) of `jobs` runs: every `jobs`-th file in name order, from the `job`-th."""
    if not 1 <= job <= jobs:
        raise ValueError(f"job {job} of {jobs}")
    return [f for i, f in enumerate(sorted(files)) if i % jobs == job - 1]


def self_test():
    files = [f"t{i:02}" for i in range(7)]
    one, two = deal(files, 1, 2), deal(files, 2, 2)
    assert one == ["t00", "t02", "t04", "t06"], one
    assert two == ["t01", "t03", "t05"], two
    assert sorted(one + two) == files and not set(one) & set(two)
    # Order given does not matter: the deal is by name.
    assert deal(list(reversed(files)), 1, 2) == one
    for bad in [(0, 2), (3, 2)]:
        try:
            deal(files, *bad)
        except ValueError:
            continue
        raise AssertionError(f"job {bad} accepted")
    assert names(), "no test files found"
    print("ci_shards self-test: ok")


def main():
    args = sys.argv[1:]
    if args == ["--self-test"]:
        self_test()
        return
    if len(args) == 3 and args[0] == "args":
        job, jobs = int(args[1]), int(args[2])
        print(" ".join(f"--test {f}" for f in deal(names(), job, jobs)))
        return
    if args == ["list"]:
        print("\n".join(names()))
        return
    sys.exit("usage: ci_shards.py args JOB JOBS | list | --self-test")


if __name__ == "__main__":
    main()
