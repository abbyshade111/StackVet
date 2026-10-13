# The report, and the prompt that reads it, lead with the most important things to fix

**Status:** done, 12 October 2026

The owner's choice of 12 October 2026, from the website's words (backlog 0224, `docs/website/home-page.md`,
sections 1, 3, and 4): the report "starts with the most important things to fix, then what was checked, and what
nobody has looked at yet." Today `report.html` opens with "Read this first", then "What was not examined", then the
things to fix, and the report-reading prompt (`docs/prompts/report.md`) starts with what was not checked. Both are to
lead with the most important things to fix; what was not examined stays prominent, directly after.

This changes what a report puts first, so it is a decision with a record: an ADR or a Later entry on the report's own
record, in the pull request that builds it, and `crates/sv-cli/tests/report_prompt.rs` kept true. The site does not go
live saying this until it is built.

**Done 12 October 2026 by session securevibe-trial-prep**, in two pull requests (ADR-110 and its Later entry). #1609:
the reports, the reading prompt, and the dashboard lead with the things to fix, worst first, then what was set aside,
how the report was made (renamed from "Read this first"), and what was not examined, in the owner's order. #1614: a
note worth knowing (an `Info` finding in the app's own code) is shown under "Worth knowing" and never counted as a
thing to fix; no AI-made commit outside a merge is a check that found nothing, not a finding (ADR-092, Later); and the
headline, "What to do next", and the "N things to fix" heading count the same set. The sample report on the "notes with
users" example now reads "3 things were found", "Fix the 3 things that were found", and "3 things to fix".
