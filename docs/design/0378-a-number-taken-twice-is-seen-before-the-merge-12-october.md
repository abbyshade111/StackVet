# A number taken twice is seen before the merge (12 October 2026)

Backlog 0265, ADR-061, "Later, 12 October 2026". Asked for by the owner in place of a merge queue (backlog 0192, part 2,
declined).

**The problem.** Each session numbers a new record, backlog item, or design entry from what it can see, so two open
pull requests could take the same number, and only the second to merge found out. On 11 and 12 October it happened to
ADR-094, ADR-096, backlog 0248, and design entry 0376. Two files with different names and one number do not conflict in
git: the two 0376 entries merged side by side, and were noticed only because a person read the list.

**What was built.** `tools/record_numbers.py`. `check` reads the numbered files (`docs/adr/ADR-NNN.md`,
`docs/backlog/NNNN-*.md` with `done/`, `docs/design/NNNN-*.md`) and fails on three things: two files with one number
in the branch, one of them not on the base branch (a pair a merge brought in); a number the branch adds (`new_numbers`:
held at its head, not where it branched from, so an item only moved keeps its number) that the base branch already
has; and, with `--github`, one that another open pull request opened before it adds too. It names the other file or
pull request; the one opened first keeps the number. The "Decision records" job, which `main` requires, runs its
self-test and the check on every pull request, even when the records step before it failed, and may now read pull
requests to get their file lists. `next adr|backlog|design` gives one past the highest on this branch and every branch
pushed to origin, and names that branch; `tools/backlog.py new` and `tools/design_entry.py new` use it for the real
folders (this entry took 0378, past the 0377 on the 0249 branch, while this branch's own highest was 0376).
`CLAUDE.md` names the tool and where a record's number comes from.

**The numbers already taken twice.** Run on `main` that day, the pair rule named 36 numbers: backlog 0218 and 35 design
entries from 8 to 10 October, as many as six files on one number (0340). Their files are cited by path, so they stay,
and the check leaves a pair alone when both files are on the base branch.

**Tests.** The tool's self-test (one of its cases in a scratch git repository), run by the workflow and by
`crates/sv-cli/tests/record_numbers.rs`, which also holds `next` past every design entry in the tree. Run on the 0249
branch as it was before `main` was merged in, the check names design 0376 and the file on `main` that has it.

**Broken on purpose, each put back:** the pair rule off; old pairs not left alone; the base branch not read (the 0376
case then missed too); an earlier pull request ignored; the later one keeping the number; its own entry taken for
another's; and a moved item counted as new (green at first, so the scratch-repository case was added; then red). Each
turned the self-test red. Not held by a test: that `backlog.py new` and `design_entry.py new` ask `next`, since their
tests run on folders with no branches; it was seen working when this entry was made.

**Not seen.** A number a session holds without having pushed it, as the weekly review session held ADR-100 and 101 that
day. Pushing the claim with its proposed record early is what makes a number visible.
