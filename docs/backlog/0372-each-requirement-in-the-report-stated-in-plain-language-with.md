# Each requirement in the report stated in plain language, with checks named in words

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

The owner's choice of 12 October 2026, from the website's words (backlog 0224, `docs/website/home-page.md`,
section 4): the report "states every requirement in plain language." Today each requirement's row in `report.html`
("what it asks for") is OWASP's own formal wording, and the checks beside a status are named by their internal ids
(`config.versions-pinned: ...`). To build: a plain-language line for each requirement, shown beside OWASP's text, not
in place of it (the formal text is what the requirement means, and a citation is a claim), and each check named in
words. Several hundred requirements: it can start with those that come up most in reports, saying in the report which
have no plain line yet. Plain lines drift from what they paraphrase, so a test holds each to its requirement, as
`crates/sv-check/tests/notes.rs` does for the questions.

**Partly done, 14 October 2026:** `data/plain-requirements.json` holds a plain-language line for each of ASVS 5.0's 70
level-1 requirements, written from each requirement's own words. report.html shows the line above the standard's words
("In its own words: …"); compliance.md shows it in a fourth column, with the standard's words in its appendix as before;
both say how many rows have a plain line, while some do not. A check beside a status is named by what it covered first,
its id after it in parentheses. `crates/sv-report/tests/plain_requirements.rs` holds every line to a real requirement
whose words it shares, and every ASVS level-1 requirement to having one. What remains: the requirements above level 1,
AISVS, and the Secure by Design checklist.

**Level 2, 14 October 2026:** at the owner's word of 14 October ("the plain lines for the level-2 requirements first;
AISVS and the Secure by Design checklist after, or as a remainder note if they would take more than a day"), every one
of ASVS 5.0's 183 level-2 requirements has a plain line too, written from its own words, and the test now holds levels 1
and 2 to having one. **What remains** was given here as ASVS's 92
level-3 requirements, AISVS 1.0's 235 and its Appendix C's 82, and the Secure by Design checklist's 36 controls. The
report already says how many of its rows have a line, so a row without one shows the standard's words alone.

**AISVS and the Secure by Design checklist, 14 October 2026:** the counts above were wrong: the data holds 191 AISVS
requirements and 68 in Appendix C. Those 259, and the checklist's 36 controls, now each have a plain line, 295 in
all, written from each one's own words and read against them one by one. Where a line first dropped part of a
long requirement (a list of what is covered, or "in place of"), it was put back. The test now holds every requirement
but ASVS's level-3 ones to having a line, and first checks that each framework's requirements loaded. **What remains:**
ASVS's 92 level-3 requirements, which the owner did not ask for in this round.
