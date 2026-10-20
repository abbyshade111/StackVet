# The PDF word test fails on main: flask-booking's report.pdf is said to lack the word lt (20 October 2026)

**Status:** done, 20 October 2026

Found by the fourth weekly review of the decision records (0616, 19 October 2026, in a Linux cloud container) and
seen again on 20 October 2026 on the owner's Mac at `main` `15e3308e`. Severity: medium (a test of the report fails,
and CI does not show it).

**What fails.** `every_ordinary_word_of_the_security_and_compliance_reports_is_in_the_pdf`
(`crates/sv-cli/tests/report_pdf.rs`, line 89): `flask-booking: not in the PDF: ["lt"]`. The review guessed an escaped
`<` read as the word "lt" on one side of the comparison but not the other. Not investigated beyond that.

**What is odd.** `main`'s CI is green, and the test has no gate on an outside tool: each PDF is read back by
`sv_report::pdf::back`. Why CI passes while two other computers fail is not known; find that first (which shard
runs `report_pdf`, and whether its example's report differs there), since a test CI does not run guards nothing.

**Break it and watch:** once fixed, a word removed from the PDF writer's output must fail the test on CI as well.

**Built, 20 October 2026.** The cause: `flask-booking`'s report lists the commits that reached the branch directly, read
from the repository it sits in, and each ends `<noreply@anthropic.com>`; `markdown::inert` writes the `<` as `&lt;`,
the PDF prints `<`, and the test's `words` read the Markdown's `&lt;` as the word "lt". The PDF was right and the test
was wrong. CI checks out two commits (`fetch-depth: 2`), so the line was never in a report there, which is why CI
stayed green. `words` now reads the five entities `inert` and the renderers write as the characters they stand for,
`&amp;` last. A new test, `an_angle_bracket_and_an_ampersand_in_the_apps_own_text_reach_the_pdf_as_themselves`, puts
a `<` and a `&` into the app's name, so the path is tried on every run, whatever the history; with the decoding taken
out it fails with the same `["lt"]`, as it would on CI.
