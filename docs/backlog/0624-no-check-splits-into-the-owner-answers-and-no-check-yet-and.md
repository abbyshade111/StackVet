# No check splits into the owner answers and no check yet, and a class is shown only where one was set

**Status:** claimed by claude/backlog-work, 20 October 2026

Filed at the owner's request, 20 October 2026, asked in a session. Severity: medium (a coverage list that says
*No check* where StackVet asks the owner, and a class nobody chose, mislead the reader about what StackVet does).

**What is wrong.** `docs/REQUIREMENTS.md`, the coverage chart, the framework pages (ADR-129), and the workbooks
exported from the same data say *No check* for every requirement no automated check speaks to, whatever else StackVet
does for it. Of the 144 ASVS 5.0 requirements marked *No check*, 39 have a design question, a security-notes section,
or a by-hand instruction (41 when this was filed, from a count that took `security-notes.json`'s `elsewhere` list,
which names requirements it does *not* ask, as asked), and 21 of those also have a locator that points the owner at the code (V1.1.1: a design
question, and `locate.decoding` lists every decoding call). The status in `tools/coverage.py` (`STATUS`,
`requirement_rows`) counts automated checks only. AISVS 1.0 has 145 more rows marked *No check*, read the same way.
Beside it, the verification class reads *ai-assistable* for every requirement no rule classes, because that is
`defaultVerificationClass` in `data/knowledge/applicability.json`: a judgment nobody made about that requirement, which
contradicts *No check* beside it.

**Proposed decision** (Status: proposed; accepted in the pull request that builds it, as a new ADR or a Later entry on
ADR-129):

- *No check* becomes two statuses: **The owner answers** (a design question, a security-notes section, or a by-hand
  instruction exists; the detail names which, and adds "`sv` shows where to look" when a locator points at the code)
  and **No check yet** (nothing in StackVet speaks to it). The evidence tiers do not change: an owner's answer is
  still *documented*, *checked by hand*, or *attested*, never *checked*.
- A verification class is shown only where a rule or the manual-only list sets one; elsewhere it reads *not classed*.
- The same words everywhere the coverage is shown: `docs/REQUIREMENTS.md`, `docs/COVERAGE.md` and its charts, the
  framework pages, and the workbook export.

**The parts.** The proposals for each requirement are in DESIGN, "What could be added for the ASVS requirements
StackVet says nothing about (20 October 2026)"; the new checks and probes it lists are backlog 0625.

1. **The two statuses and the class.** `tools/coverage.py` reads `data/design-questions.json`,
   `data/security-notes.json`, `data/human-checks.json`, and `data/locators.json` for the status; the framework pages
   and the export take it from there. The counts in the charts and the tables move from *no check* to the two new
   rows. Done when V1.1.1 reads *The owner answers: a design question; `sv` shows where to look*, and a requirement
   with nothing reads *No check yet*.
   **Part status:** open
2. **The nine requirements that ask for a document and have no notes section.** V3.1.1, V8.1.3, V8.1.4, V11.1.4,
   V13.1.2, V13.1.3, V13.1.4, V15.1.4, V15.1.5, all level 3, so they were outside `security-notes.json`'s `elsewhere`
   list, which covers levels 1 and 2. A section each, with `facts` from locators `sv` already has. The other seven
   whose words mention documentation (V3.2.3, V3.5.8, V3.7.5, V8.2.4, V13.2.6, V13.3.4, V15.2.5) ask the app to
   behave as documented, so a notes section is the wrong tool; they take the design entry's proposal.
   **Part status:** done, 20 October 2026
3. **Applicability that asks apps for what they cannot have.** Gate V6.6.4 on `out-of-band-auth`, V12.3.5 and V8.3.3
   on `multiple-services`; a derived condition `cryptographic-authenticator` for section V6.7; claim conditions
   `several-identity-providers` (V6.8.1) and `resource-server` (V10.3). Each changes which requirements apply, so a
   decision record.
   **Part status:** open
4. **V15.4's and C5.1's `never` rules.** Their reason, "level 3 only, above the level this app targets", is true while
   `sv` holds every app to level 1 or 2 and false the day an app is held to level 3. Moved to backlog 0507, which
   replaces them, at the owner's word of 20 October 2026.
   **Part status:** done, 20 October 2026 (moved to 0507)
5. **The data entries the design entry lists as quick wins.** Design questions (the twelve at level 1 or 2 first, V3.5.4 and V17.2.1 among them), the
   six by-hand instructions, the locators with their questions, the citation of V11.2.5 by
   `ast.check-passes-on-error`, and the cookie-size check on the running app (V3.3.5). Each passes its catalog's words
   test, and each credit is seen withheld (ADR-059).
   **Part status:** open
