# StackVet's report on its own repository, 22 October 2026

The report `sv` wrote of this repository, held to ASVS and AISVS level 3, for stackvet.dev's page "StackVet checks
itself" (backlog 0376, ADR-132). The paper's own self-assessment, of 4 October 2026, is `docs/paper/self-assessment-v2`
and stays as it was: the paper cites it.

**What is here, and what is not.** The owner chose on 22 October 2026 to keep only the files that stay small, since
nothing committed is ever removed from the history: `compliance.md`, `fix-now.md`, `your-decisions.md`,
`not-examined.md`, `before-going-live.md`, `low-findings.md`, `seen.json`, and `sbom.cdx.json`, about 1.1 MB. The
rest of the folder `sv` wrote (`report.html`, `report.pdf`, `report.json`, `security.md`, `information.md`, and
`stackvet.sarif`) came to about 40 MB, most of it the 3,258 findings in test and sample code, and is left out. Because
the folder is not whole, `sv` cannot check its seal; the files are as `sv` wrote them.

**How it was made,** so it can be made again: from the repository's top folder, at commit `8423ce73` (`main` at
`082ca7fe` with `[app] level = 3` added to `stackvet.toml`), with `sv` built from `082ca7fe`, the outside tools that
were installed (Semgrep and Bandit), and without `--run`, since `sv` is not a web app:
`sv report . --out <a folder outside the repository>`. It took 50 seconds.

**Not yet reviewed.** It has 312 things to fix. Most look like StackVet's rules firing on StackVet's own detection
code, such as file paths built from a value in the code that reads the folder it is pointed at, or a program started
without a shell. None has been looked at by a person, and none is set aside: going through them is backlog 0633, and
the page is refreshed after it.
