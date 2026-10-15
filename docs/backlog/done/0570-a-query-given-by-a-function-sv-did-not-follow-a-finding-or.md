# A query given by a function sv did not follow: a finding, or not verified (the owner's choice)

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Split from backlog 0525 on 15 October 2026, whose first half is built: a call to a Python function in the same file
whose one return is fixed text is judged as that text. Severity: medium (what counts as evidence).

**What is asked.** Backlog 0525's second half reports a query given by any other function as "built by a
function sv did not follow", not verified, instead of as a finding. Every such call is reported as a finding today,
and some are real (`execute(build_query(request.args))`), so turning them into not-verified lines changes what counts
as evidence for every app, and is the owner's call. The question, as asked on 15 October 2026:

- **A** (recommended): keep them as findings, as today, and close the item. A finding can be marked not a problem with
  `sv review`, and a real one is not hidden.
- **B**: report them as not verified, with a decision record.
- **C**: follow the function one level, as the first half does, and report only what still cannot be told as not
  verified.

**The owner's answer, 15 October 2026 (asked through the board's waiting line):** A: keep a query handed to a function sv did not follow as a finding, as today, and close the item; a false one is marked not a problem with sv review, and a real one is not hidden. C may be filed later if such findings prove noisy.
