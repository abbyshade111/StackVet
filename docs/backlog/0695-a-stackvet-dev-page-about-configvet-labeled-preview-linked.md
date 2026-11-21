# A stackvet.dev page about ConfigVet, labeled preview, linked from the home page

**Status:** claimed by Claude Code session (ConfigVet page), 21 November 2026

Filed 21 November 2026, at the owner's word. The owner chose on 21 November 2026: one page on stackvet.dev about
ConfigVet (`cv`, ADR-157), labeled preview, for ops and DevSecOps engineers, linked from the home page, and read by
the owner before it merges.

- **Where it comes from:** `docs/website/configvet-page.md`, built by `tools/website.py` as the other pages are
  (ADR-132), each section's headline and words with its notes left out.
- **What it says, and only from these sources:** ADR-157 (its decisions, the owner's answers, "As built" parts 1 to
  3, and its Later entries), the design entry `docs/design/0396-configvet-a-companion-for-an-app-s-deployment-configuration.md`,
  `cv`'s usage text in `crates/cv-cli/src/main.rs`, and each tool's `note` in `data/cv-adapters.json`: what
  `cv check` reads today, the five outcomes of a setting, the outside tools it runs offline, what it never does (no
  network connection; live accounts come later, behind ADR-157's rules), and how it relates to `sv`.
- **Install, honestly:** today `cv` is built from source. Its Homebrew formula and Scoop manifest come with v0.3.0
  (backlog 0694), which is not tagged, so the page prints no install command as if it worked.
- **The build pull request is opened without auto-merge;** it merges only after the owner has read the draft and said
  so. Merging publishes nothing: `.github/workflows/site.yml` publishes only when the owner starts it.
