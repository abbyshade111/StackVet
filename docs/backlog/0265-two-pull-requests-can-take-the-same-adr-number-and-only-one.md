# Two pull requests can take the same ADR number, and only one of them finds out

**Status:** claimed by backlog-org-0265, 12 October 2026

Asked for by the owner on 12 October 2026, in place of a merge queue (backlog 0192, part 2, declined): "might be worth
adding protections for ADR number collisions". On 11 and 12 October 2026 a session took ADR-094 and then ADR-096 for a
record while another session's open pull request had taken the same number. Each time it was found only when `main`
was merged into the branch and the two files met, and once a record was overwritten in a working copy before anyone
noticed. Backlog numbers collide the same way (0248 was taken twice on 11 October).

What to build: a way for a pull request to see the numbers the other open pull requests have already taken, before it
merges. Two shapes to weigh: a step in the "Decision records" check that lists the new ADR files in every open pull
request (CI can ask GitHub; `sv` itself opens no connection) and fails when two take the same number; or a number
handed out by a small tool that reads every open branch, as `tools/backlog.py new` reads the highest on this branch
only. Either way it says which pull request holds the number, so the later one renumbers before it merges, and it
covers backlog numbers too.

**Scope of this claim (12 October 2026), `Status: proposed` as a Later entry on ADR-061:** both shapes. (1) A step in
the "Decision records" check lists the files each other open pull request adds (CI asks GitHub, with the workflow
given read access to pull requests; `sv` itself opens nothing) and fails when this one adds an ADR, backlog item, or
design entry whose number another open pull request added first, naming it; the one opened first keeps the number.
(2) A tool that reads every pushed branch and gives the next free number of each kind, for `tools/backlog.py new` and
`tools/design_entry.py new` to use. Design entries are in scope because their numbers collide the same way. Neither
sees a number a session holds without having pushed it (on 12 October the cloud review session held ADR-100 and 101
that way); the claim is what makes a number visible, so the tool says to push a claim before relying on a number.
