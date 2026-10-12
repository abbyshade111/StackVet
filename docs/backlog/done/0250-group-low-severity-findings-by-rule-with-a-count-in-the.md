# Group low-severity findings by rule with a count in the report, and keep the full list in its own file

**Status:** done, 11 October 2026

From the same report as backlog 0248: 76 of its 89 findings are low severity, and one line per occurrence made the
report long. Proposal: in the report's findings section, one line per rule with its count and the files it fired in,
and the full list of occurrences in its own file beside the report. Nothing is dropped; only the first screen changes.
Check first that the report's machine-readable file (`report.json`) keeps every occurrence, which it does now.

Built 11 October 2026 (ADR-095): `security.md` and the report page show low-severity findings one line per rule, with the count and the files, most first; every one is in full in `low-findings.md`, the report folder's seventh file. `report.json` and the SARIF file keep every occurrence. Breaks: grouping nothing, leaving the full file empty, and dropping the html lines each fail a test.
