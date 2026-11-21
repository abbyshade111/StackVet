# The number check does not flag a claim that the build's pull request already contains

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word, after #2478 failed "Decision records" on its own claim. CLAUDE.md asks
for a claim in a pull request of its own, merged before building. A build branch cut from the claim's branch while the
claim is still open carries the claim's commit, so it adds the same backlog file, and `tools/record_numbers.py check
--github` reads that as two pull requests taking one number: "backlog 0706 is also added by pull request #2476, opened
first". The claim then merges, and the build's check stays red until someone with the owner's sign-in reruns it,
since a new push would dismiss the owner's approval (`dismiss_stale_reviews_on_push`).

- **To build:** for each other open pull request, the check also reads its head commit (`head.sha`, already in the
  list it fetches). When that commit is in this pull request's history (`git merge-base --is-ancestor`), and the file
  with the number is the same path, or the same item moved under `done/`, it is the same item, not a clash, and is
  not reported. A head commit that is not in this checkout (a fork, a shallow fetch) is treated as not contained, so
  the check never passes what it could not see.
- **Check:** the self-test covers a build that contains its claim (passes), one that shares the number but not the
  commit (fails, as now), and one whose claim commit is missing from the checkout (fails). Turning off the history
  test turns the first case red.
- **Records:** ADR-061's Later entries say what the check reads; one more says this.

**Done, 21 November 2026:** `tools/record_numbers.py` reads each open pull request's head commit and does not report
one this branch holds, adding the same item; an unknown commit is never held. Replayed on #2478's commits it cleared
the message #2478 failed with. The self-test covers the three cases asked for and a fourth (the commit held, a
different file); three breaks turned it red. ADR-061, Later, which also says why a build still starts from `main`
after its claim merges.
