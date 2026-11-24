# cv check does not read stackvet.toml, so findings in folders named as not the app are listed with the app's own

**Status:** open

Filed 24 November 2026 by the cato-pipeline session, at the owner's word. That session reads `sv`'s `report.json`
in a continuous-authorization pipeline, and turns it into OSCAL and a plan of action. Seen with `sv` and `cv` built
from 06f5032.

**What was seen.** Over a repository whose `stackvet.toml` says `[repository] not-the-app = ["tests/fixtures"]`,
`cv check` listed 52 findings from Trivy. 50 of them were in `tests/fixtures/deploy/app/deployment.yaml`, a sample
manifest kept for the repository's own tests. They are listed with the two that are about the app's Dockerfiles,
with nothing to tell them apart. `cv-cli` and `cv-rules` do not depend on `sv-manifest`, so `cv` never reads
`stackvet.toml`.

**What it means.** `sv` has a rule for this, and it is a careful one (ADR-031): a folder named as not the app is
still checked, and its findings still count, but they are "listed apart", since "naming a folder there must never
hide a vulnerability". `cv` does half of it. It hides nothing, which is right. It does not list apart, so on a
repository with fixtures the app's own two findings are under fifty that are not the app's.

**To do.** `cv check` reads `[repository] not-the-app` from `stackvet.toml` when there is one, as `sv` resolves
it (the guard that refuses a list which would set aside everything included), and lists those findings apart, on the
terminal and in `report.json`. They still count, and `--fail-on` still sees them. With no `stackvet.toml`, as now.
This is a new thing `cv` reads, so a "Later" entry on ADR-157.
