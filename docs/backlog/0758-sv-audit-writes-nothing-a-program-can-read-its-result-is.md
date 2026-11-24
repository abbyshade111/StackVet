# sv audit writes nothing a program can read: its result is words on the terminal and an exit status

**Status:** open

Filed 24 November 2026 by the cato-pipeline session, at the owner's word. That session reads `sv`'s `report.json`
in a continuous-authorization pipeline, and turns it into OSCAL and a plan of action. Seen with `sv` and `cv` built
from 06f5032.

**What was seen.** `sv audit [PATH] [--advisories DIR]` compares what an app ships with a local OSV database, and
its help names no other option. It prints its result as words and ends with a status (0, 1, 2, or 3). It writes no
file. `sv report` writes the same comparison into `report.json`, with a fingerprint for each finding and an
`examined` entry for `advisory.`, but only for a folder that holds a `stackvet.toml`.

**What it cost.** A repository with four lockfiles (the app's, its tests', and two for tools) wanted each compared
in its pipeline, with what was found tracked item by item. `sv audit` does the comparison for one lockfile in a
folder of its own, and its status is right. But there is nothing to track from: no finding's identity to hold from
one run to the next, and nothing to say the comparison covered the list. The pipeline now copies its own
`stackvet.toml` beside each lockfile, runs `sv report` there, and throws away what the report says about code and
about a missing `SECURITY.md`, which is about a folder holding one file.

**To do.** `sv audit --out DIR` (or `--json`), writing its findings in `report.json`'s own shape: `report_format`,
`findings` with fingerprints and `earlier_fingerprints`, and an `examined` entry for `advisory.` with the
database's folder, its newest record, and its count, as `sv report` writes them. One function for both, so they
cannot differ. Its words on the terminal and its status stay as they are. Related: backlog 0759, which is why a
repository has to do this at all.
