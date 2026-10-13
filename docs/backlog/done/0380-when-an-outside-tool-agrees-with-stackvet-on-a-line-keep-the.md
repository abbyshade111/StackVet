# When an outside tool agrees with StackVet on a line, keep the tool's words

**Status:** done, 12 October 2026

From the owner's review of the report on 12 October 2026, after an inventory of what a run gathers against what
the report shows (`docs/website/home-page.md` is the words; the inventory is the session's page "StackVet Report
Inventory"). The owner: use "the observability features and scan findings to provide the actual evidence for what was
found where possible", and the outside tools' findings "have been the most important time and time again".
Read on the second loop trial's build `haiku-a-5` (`~/sv-loop2/runs/haiku-a-5-out` and `-tools-out`).

`merge_same_place` keeps the more severe of two findings on one line that share a CWE, and at equal severity `sv`'s
own; the other's message, fix, and title are dropped, leaving "Also reported by: `semgrep.x`". To build: the kept
finding shows each tool that also reported it by name, with the tool's own message, so a second opinion is read as one.
The tool's text is still redacted, as everything is.
