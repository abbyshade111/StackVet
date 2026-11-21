# A test holds the example report to the reports page's list of files and to the current release

**Status:** open

Filed 21 November 2026, at the owner's word, from the whole-site review (backlog 0696). The reports page's example
report was made on 14 October by `sv` 0.1.0 and held 8 of the 14 files the page's table lists; nothing noticed until a
person read the page.

- **What to build:** a test that every file named in `docs/website/reports-page.md`'s table is in
  `docs/website/reports/notes-with-users/` and linked from the page, and that the version on the report's "Produced by"
  line is not older in its major or minor number than `Cargo.toml`'s. When it fails, it prints the command that makes
  the report again, from the page's own notes.
- **Done when:** deleting one file from the example folder, or writing an older version into its "Produced by" line,
  turns the test red. A version bump will then turn it red too, which is the point: the release says the example needs
  making again.
- **Order,** after 0696.
