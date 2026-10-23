# StackVet's report on its own repository, 22 October 2026

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

**How it was made,** so it can be made again: from the repository's top folder, at commit `8423ce73` (`main` at
`082ca7fe` with `[app] level = 3` added to `stackvet.toml`), with `sv` built from `082ca7fe`, the outside tools that
were installed (Semgrep and Bandit), and without `--run`, since `sv` is not a web app:
`sv report . --out <a folder outside the repository>`. It took 50 seconds.

**What stackvet.dev shows of it.** A summary only, the owner's choice of 23 October 2026: the counts of what applies
and what is known of it, how many things to fix were found and how many things were not examined, and the bill of
materials. The things to fix are being gone through (backlog 0633), and their list is not published. Anyone can make the whole report by running
`sv` on this repository, as above.
