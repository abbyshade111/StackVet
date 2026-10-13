# An "Outside tools" section: which ran, their versions, what each found, read, and skipped

**Status:** open

From the owner's review of the report on 12 October 2026, after an inventory of what a run gathers against what
the report shows (`docs/website/home-page.md` is the words; the inventory is the session's page "StackVet Report
Inventory"). The owner: use "the observability features and scan findings to provide the actual evidence for what was
found where possible", and the outside tools' findings "have been the most important time and time again".
Read on the second loop trial's build `haiku-a-5` (`~/sv-loop2/runs/haiku-a-5-out` and `-tools-out`).

Each outside tool's run (program, version, arguments, exit code, time) is in `report.json` (`Examined.tool`) and nowhere
a person reads. To build: a section in `report.html` and `compliance.md`, one row per tool: ran or not and why, its
version, how many findings (and how many merged into `sv`'s own), what it was given to read, and what it looked away
from. A tool that did not run says how to install it, as the gap does now.
