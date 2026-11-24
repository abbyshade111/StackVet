# A symbolic link not followed leaves an outside tool's family saying ran, while sv's own say partly

**Status:** open

Filed 24 November 2026 by the cato-pipeline session, at the owner's word. That session reads `sv`'s `report.json`
in a continuous-authorization pipeline, and turns it into OSCAL and a plan of action. Seen with `sv` and `cv` built
from 06f5032.

**What was seen.** A small Python app, with Bandit 1.9.4 installed, and one symbolic link in it (`sync-link.py`,
pointing at `tools/sync.py`). `sv report` said, in `examined`:

| Family | State | Why |
|---|---|---|
| `ast.` | partly | 1 symbolic link(s) in the app were not followed, such as `sync-link.py` |
| `secrets.` | partly | the same |
| `config.` | partly | the same (in a second app) |
| `bandit.` | **ran** | (nothing) |

The gap says "What it points at was not read by any check". If that is true of Bandit, its family is `partly` too. If
Bandit does read through the link, the sentence is wrong for it. Which was not found here: `sv` hands Bandit its
files by name, and whether a link is among them was not read in the code.

**What it means.** A program that closes a finding when its family `ran` and the finding is gone will close a
Bandit finding in a file that has become a link, though nothing read the file. `sv`'s own families are protected by
`partly`; an outside tool's is not.

**To do.** Find which is true for each outside tool. Then either its family says `partly` with the same reason, or
the gap's sentence says which checks did read what the link points at. A test with a link in the app, in a sibling
file, broken on purpose.
