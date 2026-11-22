# Notes with users — every low-severity finding

Produced by `sv` 0.3.0 (commit 7cb7878b2ec7) on 2026-11-22 03:24:15 UTC, run 90bae36f9345.

**This file:** Every low-severity finding in full, where the other reports group them by rule. **For:** You or your AI coding tool, when you want the detail. The end of `report.html` lists every file in this folder and who each is for.

1 finding, grouped by rule in `security.md` and the report page. Each counts as a finding.

### [low] There is no way to report a security problem

**Where:** `SECURITY.md` line 1

*How sure: confirmed.*

*Fingerprint: `882dedc677bff5a5`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

No SECURITY.md or security.txt was found, so somebody who finds a problem in this app has nowhere obvious to say so.

**Why it matters.** Problems found by outsiders get reported publicly, or not at all.

**What to do.** Add a SECURITY.md saying where to send a report and how long a reply should take.

