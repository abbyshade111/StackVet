# cv writes report pages, SARIF, the seal, and the OSCAL export, as sv does

**Status:** open

Filed 22 November 2026, at the owner's word. ADR-157, part 2 ("As built"): `cv check` writes a listing on the terminal
and `report.json`, with "the report pages, SARIF, the seal, and the OSCAL export waiting for a later part"; part 3's
"Not done" names the pages again. `cv live` writes the same.

- **To do:** reuse `sv-report`'s pages, statuses, and seal (ADR-034) and the OSCAL export (ADR-156) rather than a second
  copy (ADR-157, point 1: one rule has one home), with ConfigVet's pages in the field's own terms, each linked to the
  glossary the first time it appears (point 7). SARIF for each finding, as `sv` writes it.
- **Before any code:** which of `sv`'s pages `cv` gets (its reports have no app, no level, and no build loop), asked of
  the owner with a recommendation, and recorded on ADR-157.
- **Check:** a report `cv` writes is sealed and checked as `sv`'s are; nothing not assessed reads as a pass on any page.
