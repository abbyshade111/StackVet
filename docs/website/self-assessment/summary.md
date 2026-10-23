# StackVet's report on itself, in summary

The lines below are copied from the report `sv` wrote of this repository on 22 October 2026 (see README.md), for
stackvet.dev's page "StackVet checks itself". The report's finding-by-finding files were removed from `main` on 23
October 2026 at the owner's word; they are in the repository's history, and `sv report .` makes them again.

Produced by `sv` 0.1.0 (commit 082ca7feb89f) on 2026-10-22 20:42:01 UTC, run 44f4f1614a4d. ASVS level 3.

Of the requirements that apply to this app:

- **33** — something found a problem
- **1** — an automated check looked and found nothing wrong
- **1** — an automated check tried part of what each asks and found nothing wrong there
- **379** — nothing has looked at these at all

Held to ASVS level 3. Not in these numbers: 5 not yet placed because nobody has answered the question that decides them. Level 3 because stackvet.toml asks for level 3 (`[app] level`), above the level 1 its other answers set: answers in stackvet.toml, which your AI coding tool usually writes and nobody has confirmed, so check them, and confirm them with `sv review`.

Things to fix: 312

Things not examined: 27
