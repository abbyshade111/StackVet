# The SBOM is written with every report, and easy to find

**Status:** claimed by securevibe-trial-prep, 13 October 2026

From the owner's review of the report on 12 October 2026, after an inventory of what a run gathers against what
the report shows (`docs/website/home-page.md` is the words; the inventory is the session's page "StackVet Report
Inventory"). The owner: use "the observability features and scan findings to provide the actual evidence for what was
found where possible", and the outside tools' findings "have been the most important time and time again".
Read on the second loop trial's build `haiku-a-5` (`~/sv-loop2/runs/haiku-a-5-out` and `-tools-out`).

`sv sbom` writes the app's software bill of materials (CycloneDX JSON) on request, and `sv report` builds the same bill to
check it, but writes it nowhere a person can take it. At the owner's word: write it beside the report every time
(`stackvet-report/sbom.cdx.json`), name it in the report's first lines with how complete it is, and link it from
`report.html`. It changes what `sv` writes into someone's folder, so it carries a decision record.
