# AC.4.1 is credited as checked by hand when the person who checked is the one who asked for the code

**Status:** done, 23 October 2026

Found in the said-versus-did reading of the owner's Cursor build of 22 October 2026 (backlog 0373; the app and its records are on the owner's drive), checked against the record before filing.

AISVS Appendix C's AC.4.1 asks that a person review the code an AI tool wrote. In that build the owner answered "I've
looked at the files and code cited for each of the checks", the AI tool recorded it as done, and noted that the owner
is the person who asked for the code; `sv review` confirmed it, and the report counts AC.4.1 as checked by hand. The
owner's word is recorded honestly, but the report says nothing of whether the reviewer is anyone other than the person
who asked for the code, which the requirement's wording should be read for.

To do: read AC.4.1's text in `data/frameworks/` for what it asks of the reviewer; if it asks for someone other than
the requester, have `sv review` (and the question the AI tool asks) ask who reviewed it, and the report say so beside
the credit; if it does not, write that reading into the item and close it. Record the reading either way.

**Claimed, 23 October 2026,** at the owner's answer ("Ask who reviewed"). **Record, proposed:** ADR-133.

**Done, 23 October 2026** (ADR-133, accepted). AC.4.1's text was read first: "The reviewer must not be the same
identity that asked for the AI generation in the first place (separation of duties). And the AI agent itself does not
count as the human reviewer." So `sv review` asks, before the entries are listed, whether someone other than the
person who asked the AI tool for the code reviewed it, writes the answer beside the owner's check by hand, and seals it
with the entry. AC.4.1 counts as checked by hand only with `reviewer = "someone-else"`; otherwise the report names it
in a gap that says what was recorded and what the requirement asks, and it stays not verified. A `problem` recorded for AC.4.1 stays a finding, whoever looked: only a `done` waits for a
second person. Tests:
`crates/sv-cli/src/review/separate_review_tests.rs` and `crates/sv-cli/tests/separate_review.rs`.
