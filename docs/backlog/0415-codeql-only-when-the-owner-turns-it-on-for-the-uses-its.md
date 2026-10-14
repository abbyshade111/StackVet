# CodeQL only when the owner turns it on, for the uses its license allows

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

The owner's decision of 13 October 2026, option B of three: "agree, let's go with B please".

Asked whether CodeQL had ever run, the owner learned it never had on their own apps: it is not installed, and every
report says so under "What was not examined", with how to install it. Read the same day, the CodeQL terms
(github.com/github/codeql-cli-binaries, `LICENSE.md`) allow it for academic research, for demonstrating it, and on
an "Open Source Codebase", one "released under an OSI-approved License"; they do not allow it on any other code
without a paid GitHub Advanced Security license, nor in automated analysis or CI except on open source hosted on
GitHub. Most of StackVet's users build private apps. So every report today points them at a tool they may not use
on that app.

What CodeQL adds, from `docs/COVERAGE.md` on 13 October: it speaks to 31 ASVS requirements; 26 of them are reached
by `sv`'s own checks or another outside tool too; five by CodeQL alone: V1.2.2 (level 1), V1.2.9, V15.3.5, V15.3.6,
and V16.4.1 (level 2). It also follows data across files, which pattern matching does not.

The three options put to the owner: A, take CodeQL out; **B, keep it, off unless the owner turns it on**; C, keep
it as it is. The owner chose B, and asked that a report made without it neither says CodeQL did not run nor lists it
as a tool to install.

To build:
- **A setting in `stackvet.toml` turns CodeQL on,** for example `[tools] codeql = true`, with what the license
  allows written beside it in the setting's own description and in the guide: an app released under an open-source
  license, academic research, or a paid GitHub Advanced Security license. `sv` cannot tell which applies; the owner
  who turns it on is saying one does.
- **Off, CodeQL is not run and not named:** not in "What was not examined", not in "Outside tools", not in
  `sv doctor`'s list of tools to install, and not in the screen's line of tools that ran. Its requirements stay as
  whatever else shows them, and the five only it reaches stay not verified, as now when it is missing.
- **On and missing,** the report says so with how to install it, as today.
- **The docs:** the README, the guide, the website's credits and section 5, and `docs/COVERAGE.md` say CodeQL is
  opt-in and why. `tools/coverage.py` marks its routes as opt-in, so the counts say what a default run can reach.
- **Tests,** with deliberate breaks: off, no report file names CodeQL; on and missing, the gap is there; on and
  installed, it runs (the existing tests that run the real CodeQL when it is installed).

Decision record, proposed, numbered when it is written: **CodeQL runs only when the owner turns it on, for the uses
its license allows.** It changes what `sv` runs by default, and so what a default report can conclude. Later entries
on the records for the outside tools' default (ADR-111) and for CodeQL's languages (ADR-078).

The owner also said the same day that CodeQL is worth downloading for the paper's research, which its license
allows: the download is asked for on its own, with the file, the source, and the size.

**Claim released, 14 October 2026** (the owner's word): session securevibe-trial-prep was archived after the coordination trial with no branch pushed for this item, so the claim was stale. Open again, queued on the board.
