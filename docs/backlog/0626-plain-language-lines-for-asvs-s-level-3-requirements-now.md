# Plain-language lines for ASVS's level-3 requirements, now that an app can be held to level 3

**Status:** open

Filed 21 October 2026, while 0507's catalogs were opened to level 3. Severity: low (a level-3 requirement in a report
shows the standard's words with no plain line beside them).

Backlog 0372 gave every requirement in the report a plain-language line beside the framework's own words
(`data/plain-requirements.json`), except ASVS's 92 level-3 requirements, "not to be written yet: level 3 is held until
backlog 0507 decides" whether `sv` holds any app to it. 0507 has: `[app] level = 3` (ADR-130). So an app held to level 3
now has 92 rows with no plain line, and `crates/sv-report/tests/plain_requirements.rs` still exempts them.

**To do:** a line for each, held to its requirement's words as the others are, and the exemption in the test removed,
so a level-3 requirement without one fails it by name.
