# A weekly check that names the long files and functions to split

**Status:** claimed by claude/0662, 25 October 2026
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

**Record, proposed: a weekly job lists the long files and functions to split, in its run's summary only.** It governs
the job's workflow, the counting it runs, and their test.

*Context.*

Backlog 0324 listed the Rust files and functions too long to work in comfortably, once, by hand, and each of the three
times it was checked again the files on it had grown. When it was marked done, on 24 October 2026, the owner asked:
"there should be a new weekly check for long files to split moving forward". A new scheduled job changes what CI runs,
which ADR-051 and ADR-104 record for the workflows they name.

Asked on 25 October 2026 where the job should write its list, the run's summary alone or also an issue it keeps up to
date (which writes to the repository's issues each week), the owner chose: "Run summary only".

*Decision.*

- **A workflow on a weekly schedule,** Mondays at 09:00 UTC, after the CodeQL, audit, and fuzz runs, and by hand
  (`workflow_dispatch`). It reads the code and writes nothing to the repository: read-only permissions, no issue, no
  commit, no backlog item.
- **What it lists,** largest first, in plain words in the run's summary: every Rust file in `crates/` over 3,000
  lines, and every function outside test code over 150 lines, the two numbers 0324 used. Test files and test fakes
  are listed apart, since a long test fake is a different job from a long module.
- **The two numbers are kept in one place,** the counting script, which the job runs and its test reads.
- **A function's length is read by its braces,** in a Python script, with strings, characters, and comments that hold
  braces skipped; its test shows it on a file made to hold all three.
- **It never fails.** A long file is a reason for a session to take the work the normal way, a backlog item each; the
  job does not file items itself, since a list every run adds to brings back the conflicts the one-file-per-item
  layout is for.

*Alternatives considered.*

- **An issue kept up to date with the list.** The owner chose the summary alone.
- **Failing the run, or `main`, when a file passes the line.** Rejected: a long file is work to plan, not a fault to
  stop on, and a red weekly run would be noise.
- **Counting with a Rust parser.** Rejected for now: the braces, with strings and comments skipped, are enough for a
  list a person reads, and the test holds the cases where they could go wrong.

*Consequences.*

- Each Monday's run shows what has grown past the lines, so a file is seen as it grows, not years later.
- Nothing is written to the repository, and nothing new is spent: the job runs on GitHub's free minutes for the
  repository, as the other weekly jobs do.
