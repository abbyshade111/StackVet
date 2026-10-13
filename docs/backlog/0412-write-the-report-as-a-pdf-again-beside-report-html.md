# Write the report as a PDF again, beside report.html

**Status:** claimed by securevibe-trial-prep, 13 October 2026

The owner's request of 13 October 2026, after the my-app report was sent as a zip: "at one point there was a pdf
version of reports in the bundle, can we start doing that again so there's a format people can easily read?"

That was v1. SecureVibe v1 wrote a PDF beside each HTML report with a writer of its own (v1's ADR-013, 25 September
2026, `0a59456`; paper size as a setting, `ed75b52`). The writer was TypeScript, so it did not come across when `sv`
was written in Rust, and no `sv` report has had a PDF since. Today the report folder holds `report.html`,
`security.md`, `compliance.md`, `low-findings.md`, `findings.sarif`, `sbom.cdx.json`, `report.json`, and `seen.json`.
Only the HTML is easy for a person to read, and only if they open it in a browser.

To build:
- **`sv report` writes `report.pdf`** into the report folder, and `sv bundle` carries it. It holds what a person reads:
  `compliance.md`'s order, with each finding written out in full where `compliance.md` lists only its title, as
  `report.html` does, so nothing appears twice. The owner agreed the same day, on learning that joining the two
  files would print their shared opening and "What was not examined" twice. It is made from the same text the
  other reports are written from, so the PDF cannot say something they do not.
- **`sv` writes the file itself, with no new dependency,** as v1 did: a small writer over the standard PDF fonts every
  reader has (Helvetica and Courier), so nothing is bundled and nothing is downloaded. It handles the shapes the reports
  use: headings, paragraphs, lists, tables that repeat their header row on each page and split long rows, links, bold
  text, and "Page n of N". Chapters become bookmarks.
- **US Letter by default,** as in v1, with A4 available (an option, or a `stackvet.toml` setting).
- **The limit, said plainly.** The standard fonts show Western European letters and common punctuation only. Any other
  character prints as `?`, and the PDF's first page says that the HTML report is the complete version. A test fails
  when a report starts using a character the fonts lack, so it is found, not printed wrongly.
- **Tests,** each with a deliberate break: every ordinary word of the security and compliance reports appears in the
  PDF; nothing runs past the margin on either paper size; a table's header row repeats on a new page; the file is in
  the report folder and in the bundle. One test reads the PDF back without any outside tool.
- **`report.html` and `security.md` name the PDF** in the list of what the folder holds.

Decision record, proposed, numbered when it is written: **the report is also written as a PDF, by `sv` itself.** It
changes what `sv` writes into someone's folder. Alternatives to record: a PDF library crate (full Unicode and embedded fonts, at the
cost of a new dependency tree; the change to make if the character limit matters); printing through the person's
browser (rejected in v1, because no file exists until somebody makes one, and it differs between machines); a PDF
only on request (rejected: the point is a file people can open without knowing to ask). The records on what `sv`
writes into the folder (ADR-017) and on the report folder's files (ADR-082) get Later entries.
