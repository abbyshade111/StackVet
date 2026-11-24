# cv's report.json carries what a pipeline keys on: a format number, a fingerprint for each finding, and what each tool looked at

**Status:** open

Filed 24 November 2026 by the cato-pipeline session, at the owner's word. That session reads `sv`'s `report.json`
in a continuous-authorization pipeline, and turns it into OSCAL and a plan of action. Seen with `sv` and `cv` built
from 06f5032.

Related to backlog 0735 (the report pages, SARIF, the seal, and the OSCAL export for `cv`). This is about
`report.json` itself, which 0735 does not name.

**What was seen.** `cv check`'s `report.json` has four top-level names: `checked`, `outside_tools`, `tool`, and
`version`. Beside `sv`'s it lacks three things a pipeline keys on:

- **A format number.** `sv`'s has `report_format`, so a reader can refuse a shape it was not written for.
- **A fingerprint for each finding.** A `cv` finding has a rule id and a place (`file` and `line`). A line number
  moves when the file is edited, so an item tracked by it is closed and opened again by an edit that fixes nothing.
- **What was looked at, by rule family.** `sv`'s `examined` says, for each family, `ran`, `partly`, `not-run`,
  or `nothing-to-examine`, which is what lets a reader tell a finding that was fixed from one not looked for. `cv`
  has the parts (`outside_tools.tools`, and `checked.read` and `unread`), in its own shape.

**What it means.** `cv check` already does most of what a pipeline would otherwise wire by hand: Checkov, hadolint,
and Trivy offline, each kept from hiding a rule, with a severity on Checkov's findings. The pipeline that filed this
would use it in place of its own Checkov step, and cannot yet, for these three alone.

**To do.** `report_format`; `fingerprint` on each finding, made as `sv`'s `v2-` ones are, so one function makes
both; and an `examined` list in `sv`'s shape, one entry for `cv.` and one for each tool's prefix. Whether SARIF
alone would do was asked: it carries fingerprints, and not the list of what was looked at.
