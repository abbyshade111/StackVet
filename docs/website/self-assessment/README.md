# StackVet's report on its own repository, 26 October 2026

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
here (the owner's `sv review` records in it), with `sv` built from `5a1af08f` (`main` that day), the outside tools that
were installed (Semgrep ran; Bandit had no Python of the app's to read, since `stackvet.toml`'s `[repository]
not-the-app` names `tools/`), and without `--run`, since `sv` is not a web app: `sv report . --out <a folder outside
the repository>`. It took under two minutes. It found 1 thing to fix, and the owner's 12 false alarms of 23 October
still set aside; the report of 23 October had 151 things to fix (backlog 0672).

Two things to get right when it is made again, both learned on 26 October 2026:

- **Run it in the repository's own folder.** A finding the owner set aside in `sv review` is sealed for the folder it
  was recorded in, so a copy elsewhere (made with `git archive`, say) shows those findings as things to fix again, and,
  holding no `.git`, misses what the repository's history holds.
- **First take out what git ignores but `sv` would read.** `sv` reads the folder as it is on disk: a Claude Code
  worktree left under `.claude/worktrees/` added a whole second copy of the repository (1,293 findings), and the site
  built into `exports/site/` added its pages. Both were removed before this report, at the owner's word; `git status
  --short --ignored` lists what is there.

**What stackvet.dev shows of it.** A summary only, the owner's choice of 23 October 2026: the counts of what applies
and what is known of it, how many things to fix were found and how many things were not examined, and the bill of
materials. The things to fix are being gone through (backlog 0633), and their list is not published. Anyone can make the whole report by running
`sv` on this repository, as above.
