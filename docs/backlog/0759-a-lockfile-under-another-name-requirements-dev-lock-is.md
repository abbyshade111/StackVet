# A lockfile under another name (requirements-dev.lock) is neither read nor named, so its packages are compared with nothing

**Status:** open

Filed 24 November 2026 by the cato-pipeline session, at the owner's word. That session reads `sv`'s `report.json`
in a continuous-authorization pipeline, and turns it into OSCAL and a plan of action. Seen with `sv` and `cv` built
from 06f5032.

**What was seen.** A repository with `pyproject.toml` and four hash-pinned lockfiles made by `uv pip compile`:
`requirements.lock`, `requirements-dev.lock`, `requirements-mirror.lock`, and `requirements-sv-tools.lock`. The last
two are for tools, each compiled from an `.in` file of its own, and installed into a container image and onto a
machine that reaches the internet.

- With `certifi 2026.7.22` (GHSA-6q48-6c7w-9gw2, critical) put into `requirements-mirror.lock` alone,
  `sv report --advisories` found nothing, and `sbom.cdx.json` held only the version in `requirements.lock`.
- `report.json` does not hold the name of any of the three other lockfiles: not in `gaps`, not in what the bill of
  materials says it read.
- `sv audit` on that one file, copied into a folder of its own as `requirements.txt`, found it.

**Where it comes from.** `crates/sv-scan/src/ecosystems.rs` reads `requirements.lock` beside `pyproject.toml`
(commit 83403590, "Read requirements.lock beside pyproject.toml"), and knows a requirements file under another name when it
ends in `.txt` (`requirements-dev.txt`, `requirements/prod.txt`: ADR-037, Later, 6 October 2026). A `.lock` under
another name matches neither, so it is passed over without a word. `uv pip compile -o requirements-dev.lock` is a
common way to name one.

**What it means.** Three lists of what this repository installs were compared with no advisory, and the report
did not say so. That is the thing the first rule is against: not assessed, and not said. The repository now audits
each in its own pipeline (backlog 0758).

**To do, for the owner to choose between the first two:**
- read `requirements*.lock` as `requirements*.txt` is read, each a list of its own, into the advisory comparison,
  and say in the bill of materials which list a package came from; or
- do not read them, and name each in the bill of materials and in a gap as a list of packages that was not
  compared, so `advisory.` is `partly` while one is there.
- Either way a test with a second lockfile holding a package with a known advisory, which today passes unseen.
