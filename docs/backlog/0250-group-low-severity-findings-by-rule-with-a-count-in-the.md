# Group low-severity findings by rule with a count in the report, and keep the full list in its own file

**Status:** claimed by securevibe-trial-prep, 11 October 2026

From the same report as backlog 0248: 76 of its 89 findings are low severity, and one line per occurrence made the
report long. Proposal: in the report's findings section, one line per rule with its count and the files it fired in,
and the full list of occurrences in its own file beside the report. Nothing is dropped; only the first screen changes.
Check first that the report's machine-readable file (`report.json`) keeps every occurrence, which it does now.
