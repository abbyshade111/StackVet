# A reports page: what each report file is for, who reads it, and an example of each

**Status:** claimed by securevibe-trial-prep, 13 October 2026

The owner's request of 13 October 2026. Looking at the my-app report, the owner found that `security.md` and
`compliance.md` "often look very similar if not the same". They are not the same: `security.md` is the findings in
full (where, the code, how sure, why it matters, what to do, how to set one aside) and what was not examined, for
whoever fixes the code; `compliance.md` is every requirement that applies, by chapter, with what is known of each,
the threats, what only the owner can check, and the tests worth writing, for whoever needs the whole picture against
the standards. On a small app they look alike because they open the same way: the same first lines, the same
paragraph on whether `sv` was used during the build, and the same "What was not examined", word for word. The owner:
"we should make note in GitHub and on the website what the difference is and the intended audience. Maybe a reports
page on the website that says the intended audience and shows an example of each type of report."

The docs say otherwise today: the README calls `compliance.md` and `security.md` "the same in Markdown", and the
getting-started guide says they "say the same in plain text for your AI tool". Both are wrong.

To build, after 0412 (the PDF), so the page can show the PDF too:
- **README and `docs/GETTING-STARTED.md`:** a short table of the report folder's files, each with what it holds and who
  it is for (a person reading, the person or AI tool fixing the code, an editor or GitHub, `sv` itself on the next
  run), in place of "the same". The report folder's own `report.html` says the same in its list of files.
- **A reports page for the website,** `docs/website/reports-page.md`, in the home page's style (words first, settled
  by the owner, nothing live): each file, its audience, and when to open it, with a link to an example of each.
- **Example reports** from one of the example apps in `examples/` (one with findings and several statuses), made with
  the released `sv` and kept with the page, with the command that made them and the date, so they can be made again.
  Never the health-tracking app. The home page's section 4 links the page, and its invented example lines can point
  at the real examples.
- **A test** that the README's table names every file `sv report` writes, so a new file cannot be left out (break:
  drop a row and see it fail).
