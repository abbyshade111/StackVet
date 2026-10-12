# The report, and the prompt that reads it, lead with the most important things to fix

**Status:** open

The owner's choice of 12 October 2026, from the website's words (backlog 0224, `docs/website/home-page.md`,
sections 1, 3, and 4): the report "starts with the most important things to fix, then what was checked, and what
nobody has looked at yet." Today `report.html` opens with "Read this first", then "What was not examined", then the
things to fix, and the report-reading prompt (`docs/prompts/report.md`) starts with what was not checked. Both are to
lead with the most important things to fix; what was not examined stays prominent, directly after.

This changes what a report puts first, so it is a decision with a record: an ADR or a Later entry on the report's own
record, in the pull request that builds it, and `crates/sv-cli/tests/report_prompt.rs` kept true. The site does not go
live saying this until it is built.
