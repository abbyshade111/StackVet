# List test-file information findings in full in the markdown and html reports, not only in the headline (ADR-091)

**Status:** done, 11 October 2026

Built 11 October 2026: `security.md` lists each one with its rule, file, line, title and fingerprint, after the findings, and says how to make a real one count (ADR-091, Later). The html report is unchanged. Breaks: each of the two places the section is written fails a test.
