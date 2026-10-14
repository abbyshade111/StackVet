# stackvet.dev: the reports page

What each file of a StackVet report is for, who reads it, and an example of each (backlog 0413). Words first, as the
home page's are (`home-page.md`); **nothing here is live**, and the owner settles the words before any design. The
home page's section 4 links here.

## What you get

**One folder, eight files, each with a reader of its own.**

When StackVet checks your app, it writes a folder named `stackvet-report` inside the app's folder. Start with
`report.html`: it is the whole report on one page, and it ends with this same list.

| File | What it holds | Who it is for |
|---|---|---|
| `report.html` | The whole report on one page: what to fix first, what was not examined, and every requirement that applies | You, in a browser: double-click it |
| `compliance.md` | Every requirement that applies, chapter by chapter, with what is known of each; the threats; what only you can check; and the tests worth writing | Anyone who needs the whole picture against the standards, you or someone you show it to |
| `security.md` | What the checks found, each in full (where, the code, how sure, why it matters, what to do, and how to set it aside), then what was not examined | Whoever fixes the code, you or your AI coding tool |
| `low-findings.md` | Every low-severity finding in full, where the other reports group them by rule | You or your AI coding tool, when you want the detail |
| `stackvet.sarif` | The findings as SARIF, a standard format for findings (named findings.sarif before 14 October 2026) | Code editors and GitHub's code scanning, which show a finding beside its line |
| `report.json` | The report's data as JSON | Programs, among them sv review, sv compare, and sv dashboard |
| `seen.json` | What sv saw of the running app, with every key and sign-in in it cut short | You, to see what a finding rests on |
| `sbom.cdx.json` | The app's bill of materials, every package it uses with its version, as CycloneDX JSON | Tools that read a bill of materials |

## Two reports that look alike, and are not

**`security.md` is what to fix. `compliance.md` is the whole picture.**

On a small app the two open with the same lines: the app's name, when the report was made, and whether StackVet was
used while the app was built. After that they part.

- **`security.md`** is for whoever fixes the code. Each thing to fix is set out in full: the file and line, the code
  itself, how sure StackVet is, why it matters, and what to do. Then it lists what was not examined, so nobody takes
  a short list of findings for a clean bill of health. Give it to your AI coding tool when you ask it to fix what was
  found.
- **`compliance.md`** is for anyone who needs to see the app against the standards: every requirement that applies,
  chapter by chapter, with what is known of each, from *needs attention* to *not verified*. It also has what could go
  wrong with an app like yours, the questions only you can answer, and the tests worth writing. Open it before
  launch, or when someone asks how the app measures up.

Each opens with a line saying what it holds and who it is for.

## An example of each

*A real report, made by StackVet from one of its own example apps.*

The example is **Notes with users** (`examples/notes-with-users`), a small notes app with accounts and an admin page,
written for StackVet's own tests. Its report has two things to fix, one of them low severity, and requirements
in several states: *needs attention*, *checked*, *checked in part*, *not verified*, and *not assessed*.

- [`report.html`](reports/notes-with-users/report.html)
- [`compliance.md`](reports/notes-with-users/compliance.md)
- [`security.md`](reports/notes-with-users/security.md)
- [`low-findings.md`](reports/notes-with-users/low-findings.md)
- [`stackvet.sarif`](reports/notes-with-users/stackvet.sarif)
- [`report.json`](reports/notes-with-users/report.json)
- [`seen.json`](reports/notes-with-users/seen.json)
- [`sbom.cdx.json`](reports/notes-with-users/sbom.cdx.json)

- **What it leaves out, and says so:** it was made without starting the app (no `--run`), because the machine that
  made it had no Docker running, and without the outside security tools (`--no-tools`). So `seen.json` is nearly
  empty, nothing was asked of the running app, and the report's "What was not examined" says both. A report made
  with `--run` has more in each file; this one shows what each file is, not everything StackVet can find.
- **How it was made,** so it can be made again: from the repository's top folder, with `sv` built from the commit
  the report names on its second line,
  `sv report examples/notes-with-users --no-tools --out docs/website/reports/notes-with-users`, on 14 October 2026.
  The health-tracking app built for a friend is never used as an example.

## Not yet on this page

- **Two diagrams:** the software development lifecycle, and where Secure by Design, ASVS, and AISVS touch it. The
  owner drew both on 13 October 2026; they are kept outside the repository for now, and are backlog 0514.
- **The report as a PDF,** beside `report.html`, once backlog 0412 builds it: a row in the table above, and in
  `report.html`'s own list.
