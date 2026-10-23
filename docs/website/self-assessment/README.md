# StackVet's report on its own repository, 23 October 2026

The report `sv` wrote of this repository, held to ASVS and AISVS level 3, for stackvet.dev's page "StackVet checks
itself" (backlog 0376, ADR-132). The paper's own self-assessment, of 4 October 2026, is `docs/paper/self-assessment-v2`
and stays as it was: the paper cites it.

**What is here.** `summary.md`, the report's counts copied out of it, and `sbom.cdx.json`, its bill of materials. The
report's finding-by-finding files (`compliance.md`, `fix-now.md`, `your-decisions.md`, `not-examined.md`,
`before-going-live.md`, `low-findings.md`, and `seen.json`) were kept here from 22 October 2026 and removed from `main`
on 23 October at the owner's word: they listed each finding in StackVet's own code by file and line, and the owner chose
not to publish them while they are gone through (backlog 0633). They are in the repository's history, as an older
version since changed. The rest of what `sv` wrote (`report.html`, `report.pdf`, `report.json`, `security.md`,
`information.md`, and `stackvet.sarif`) was never kept.

**How it was made,** so it can be made again: from the repository's top folder, with `stackvet.toml` as committed
here (the owner's `sv review` records in it), with `sv` built from `8233c364` (`main` that day), the outside tools that were
installed (Semgrep and Bandit), and without `--run`, since `sv` is not a web app: `sv report . --out <a folder outside
the repository>`. It took about a minute. Since the report of 22 October (312 things to fix), backlog 0643 to 0646 fixed
the rules that were wrong for StackVet's own code and read Rust test modules as test code, and the owner confirmed 12
findings as false alarms in `sv review` (backlog 0633). The tests' folders stay checked, with their findings listed
apart: the owner tried leaving them out (`[[left-out]]`) on 23 October and took it back the same day, so the full scan
is the one shown.

**What stackvet.dev shows of it.** A summary only, the owner's choice of 23 October 2026: the counts of what applies
and what is known of it, how many things to fix were found and how many things were not examined, and the bill of
materials. The things to fix are being gone through (backlog 0633), and their list is not published. Anyone can make the whole report by running
`sv` on this repository, as above.
