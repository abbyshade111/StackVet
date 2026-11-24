# The OSCAL export names the findings behind a requirement by their count alone, so a pipeline cannot join them to report.json

**Status:** open

Filed 24 November 2026 by the cato-pipeline session, at the owner's word. That session reads `sv`'s `report.json`
in a continuous-authorization pipeline, and turns it into OSCAL and a plan of action. Seen with `sv` and `cv` built
from 06f5032.

**What was seen.** In the file `sv report --oscal` writes, the observation for a requirement that needs attention
reads, in `description`: "An automated check of sv's found something and named this requirement. What it rests on:
ast.shell-command. Findings named against it in the report: 4." The rule family and the count are in a sentence.
Nothing in the observation or the finding names which four: no rule id, no fingerprint, no link.

**What it means.** A pipeline that already tracks each of `sv`'s findings from `report.json`, by rule and
fingerprint, cannot say which of its tracked items are behind an objective the export concludes is not satisfied,
except by reading `report.json`'s `requirements` again and matching ids. The export names the report by its
SHA-256, so the two are tied as files; they are not tied finding by finding.

**To do.** On the observation, properties in `sv`'s own namespace (`https://stackvet.dev/ns/oscal`), one for each
finding named against the requirement, holding its rule id and its fingerprint; or OSCAL's own
`relevant-evidence` with the same. No description, fix, or code: the export says no more than the report does
(ADR-156), and these two are already the report's names for a finding. ADR-156 is the record. The schema test stays
as it is; a new one checks every `not-satisfied` finding names at least one.
