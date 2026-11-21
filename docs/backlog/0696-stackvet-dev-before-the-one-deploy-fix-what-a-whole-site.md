# stackvet.dev before the one deploy: fix what a whole-site review found (21 November 2026)

**Status:** claimed by Claude Code session (site review), 21 November 2026

Filed 21 November 2026, at the owner's word. A whole-site review of stackvet.dev, as `tools/website.py` builds it from
`main` at 472ab1f1, found sixteen things to change before the owner runs the site's deploy once; the owner chose to
make all of them. Merging publishes nothing: `.github/workflows/site.yml` publishes only when the owner starts it
(ADR-132).

- **Wrong or saying too much:** the check-a-project page says the run talks to GitHub and GitLab only, though Semgrep
  fetches its rules in the same run (ADR-111, ADR-154); How it works shows the paper's "the one network connection
  sv makes of its own" and `--tools` with no word that both changed; StackVet checks itself says its findings "are
  being gone through", though backlog 0633 is done and the report found none to fix; the credits leave CodeQL's C and
  C++ out; the ConfigVet page names an MCP server `cv` does not have.
- **Out of date:** the ConfigVet page's "not packaged yet" (the tap has a head-only `cv` formula); the reports page's
  example report, made by sv 0.1.0 with 8 of the 14 files; the home page's invented example lines and level 3
  examples, which its notes say are replaced and checked before launch; a leftover note about the PDF; the
  self-assessment, made by sv 0.1.0.
- **Missing:** hadolint, Checkov, and Trivy in the credits; why the home page counts 191 AISVS requirements and the
  What it checks page 259; a link to GitHub issues.
- **Wording:** an "It" that reads as StackVet where Semgrep is meant; the ConfigVet page's kube-linter line;
  "43 found a problem" beside "0 things to fix"; `--locked` in the ConfigVet build line.
- **Not here:** `docs/INSTALL.md`, the release workflow, the tap, and the bucket, which the v0.3.0 session moves.
- **The build pull request is opened without auto-merge;** it merges only on the owner's word.
