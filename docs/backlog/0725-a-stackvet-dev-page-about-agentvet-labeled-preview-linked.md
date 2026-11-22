# A stackvet.dev page about AgentVet, labeled preview, linked from the home page

**Status:** claimed by Claude Code session (AgentVet), 22 November 2026

Filed 22 November 2026, at the owner's word ("an update to stackvet.dev about av (similar to cv)"), to come after the
packaging (backlog 0724). ConfigVet's page was backlog 0695, and this follows it.

**Order,** by the owner's word: after 0724.

- **Where it comes from:** a new `docs/website/agentvet-page.md`, built by `tools/website.py` as the other pages are
  (ADR-132), and a line on the home page that links to it, as ConfigVet's does. `tools/website.py` is a code-owned
  file: a cloud session writes the change on its branch and pushes it, and a session on the owner's Mac opens the
  pull request as the sessions' App.
- **What it says, and only from these sources:** ADR-159 (its decisions, the owner's answers, and "As built" parts 1
  to 12), the design entry `docs/design/0397-agentvet-a-companion-that-vets-the-agent-what-it-did-what-it.md`, and
  `av`'s usage text in `crates/av-cli/src/main.rs`. What `av report` reads and how far each record can be relied on;
  the task file; the pass-through; checkpoints; what you were asked and whether it ran; the six kinds of thing it
  looks for; and, as plainly, what it cannot see and that it decides nothing and never says a tool can be trusted.
- **Labeled preview,** as ConfigVet's is. What has been shown is two trials on one computer (a Mac, Claude Code's
  desktop app, `sv` through Docker with Colima); the page says so and names nothing else as tried.
- **Install, honestly:** only what works when the page is written. If 0724's packages are not published yet, the
  page prints no install command as if it worked (the test that holds ConfigVet's install lines to
  `docs/INSTALL.md`, `crates/sv-cli/tests/website_configvet_install.rs`, is the model for one here).
- **The owner reads the draft before it merges,** as with ConfigVet's page: the build pull request is opened without
  auto-merge. Merging publishes nothing: `.github/workflows/site.yml` publishes only when the owner starts it.
- The health-tracking app stays anonymous, and the owner's own folders and app names from the trials are not
  written on the page.
