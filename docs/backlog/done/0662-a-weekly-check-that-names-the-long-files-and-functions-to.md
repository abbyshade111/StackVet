# A weekly check that names the long files and functions to split

**Status:** done, 25 October 2026
**Asked for by the owner, 24 October 2026**, when 0324 (long functions and files beyond the recorded split) was marked
done: "there should be a new weekly check for long files to split moving forward". 0324 was a list written once and
checked by hand three times, and each time the files on it had grown; a check that runs each week finds a file as
it grows.

**What it is.** A job on a weekly schedule, Mondays beside the Audit, CodeQL, and fuzz runs in `.github/workflows/`,
that lists every Rust file in `crates/` over a line count and every non-test function over a length, largest first,
in the run's summary, in plain words. 0324 used 3,000 lines for a file and 150 for a function; start there, and keep
the two numbers in one place the job and its test both read.

**What it is not.** It reports; it does not fail `main`, edit code, or split anything. A file on the list is work a
session takes the normal way, a backlog item each, and the job never files items itself (a list every run adds to
brings back the conflicts the one-file-per-item layout is for). Test files and test fakes are listed apart, as
findings in test code are, since a long test fake is a different job from a long module.

**To decide in the claim.** Where the job writes its list (the run summary alone, or also an issue it keeps up to
date, which writes to the repository's issues and so needs the owner's word first); whether a function's length is
read by a parser or by braces (a Python tool reading braces is enough if its test shows it on a file with strings
and comments that hold braces); and the decision record, since a new scheduled job changes what CI runs (ADR-051
and ADR-104 name the workflows).

**Done when.** The job runs on its schedule and by hand, its list matches a count made another way on today's `main`,
a test holds the counting on a small file made to be long, and breaking the count (a threshold read wrong) turns that
test red.

**Decided in the claim, 25 October 2026.** The owner's word on where the list goes: "Run summary only". The record
below is proposed here, with the claim, and goes into `docs/adr/` as the next record, accepted, in the pull request
that builds the job, when the files it governs exist to be named.

**The record is ADR-152**, accepted in the pull request that built the job; it was proposed here until the files it
governs existed.

**Built 25 October 2026** (ADR-152): `tools/long_files.py`, `.github/workflows/long-files.yml`, and its test
`crates/sv-cli/tests/long_files.rs`. On `main` that day the script's fourteen files over 3,000 lines matched `wc -l`,
and two of its 116 long functions were checked by their closing braces. Broken on purpose: either limit read as "at
or over", the braces in strings and comments counted, and a test module not marked each turned the test red. Run by hand
from the Actions tab on 25 October 2026, once it was on `main`
(<https://github.com/abbyshade111/StackVet/actions/runs/40224997909>): it passed, and the 134 lines of its list in
the run's summary were the same, line for line, as the script's on `main` here.
