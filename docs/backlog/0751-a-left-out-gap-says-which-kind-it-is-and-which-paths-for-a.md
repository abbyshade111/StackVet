# A left-out gap says which kind it is and which paths, for a program that reads report.json

**Status:** open

Filed 24 November 2026 by the cato-pipeline session, at the owner's word. That session reads `sv`'s `report.json`
in a continuous-authorization pipeline, and turns it into OSCAL and a plan of action. Seen with `sv` and `cv` built
from 06f5032.

**What was seen.** A gap with `reason: "left-out"` is written for six different things, in `sv-cli`'s
`left_out.rs`, `lib.rs`, and `assemble.rs`: a `[[left-out]]` folder; the folders named as not the app; a symbolic
link not followed; a folder skipped as installed or built code; an outside tool that was not run; and a setting the
copy `sv` ran had loosened. They mean different things to whoever reads the report with a program:

- For three of them a finding can go missing while `examined` still says `ran`: a confirmed `[[left-out]]` folder,
  a folder skipped as installed code, and a link (`bandit.` stays `ran`; backlog 0752).
- For the others nothing goes missing: the folders named as not the app still count, and a tool that was not run has
  its own `examined` entry saying so.

The only way to tell them apart is to read the sentence in `what`, and the only place the paths are is the sentence
in `why`, which names five and then says "and N more".

**What it cost.** The pipeline's reader matched the two sentences it had seen and refused any other, so as not to
close a plan-of-action item for a finding that was never fixed. So an app with a symbolic link in it, or a virtual
environment named `env/`, would have failed the pipeline's step. That fault stood for six weeks, until a run over
such an app found it; the pipeline's own repository has neither, and its fixtures had neither. The reader now
matches four sentences with patterns, and holds every item open when a sentence cannot name all its paths.

**To do.** Two fields on a gap, both left out when empty, so nothing that reads the report today changes:
- `kind`, one word for which of the six it is; and
- `paths`, every path it is about, not the first five.

Say them in `docs/ARCHITECTURE.md` where `report.json` is described. A test that fails when a new place writes
`GapReason::LeftOut` with no kind.
