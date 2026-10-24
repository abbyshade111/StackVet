# Plain-language lines for ASVS's level-3 requirements, now that an app can be held to level 3

**Status:** done, 24 October 2026

Filed 21 October 2026, while 0507's catalogs were opened to level 3. Severity: low (a level-3 requirement in a report
shows the standard's words with no plain line beside them).

Backlog 0372 gave every requirement in the report a plain-language line beside the framework's own words
(`data/plain-requirements.json`), except ASVS's 92 level-3 requirements, "not to be written yet: level 3 is held until
backlog 0507 decides" whether `sv` holds any app to it. 0507 has: `[app] level = 3` (ADR-130). So an app held to level 3
now has 92 rows with no plain line, and `crates/sv-report/tests/plain_requirements.rs` still exempts them.

**To do:** a line for each, held to its requirement's words as the others are, and the exemption in the test removed,
so a level-3 requirement without one fails it by name.

**The owner's word, 23 October 2026,** among the things to do after backlog 0633: "add plain language for any
requirements (assuming level 3) that don't have it yet". On that day, all 92 of ASVS's level-3 requirements still had none,
and every other requirement had one.

**Order,** at the owner's word of 23 October 2026: after 0650.

**Built, 24 October 2026.** A plain line for each of the 92, written from its requirement's own words and read
against them, in the standard's order in `data/plain-requirements.json`; a term a person may not know is explained
where it first comes (Subresource Integrity, sender-constrained tokens, TURN, SRTP). The exemption in
`crates/sv-report/tests/plain_requirements.rs` is gone, and its setup now asserts level-3 requirements were loaded.
Broken on purpose: with V17.2.8's line removed and V6.3.6's replaced by words it shares none of, both tests went red,
each naming its requirement. As with the others, the test proves a line exists and shares words, not that it is
faithful; that rests on the reading.
