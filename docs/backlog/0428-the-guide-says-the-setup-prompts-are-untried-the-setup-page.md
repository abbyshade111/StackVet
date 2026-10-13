# The guide says the setup prompts are untried; the setup page says they were tried in Antigravity and the setup worked

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: usability. Severity: medium (a person is steered to the
harder path by a sentence that is no longer true). Verified against the two files by the reviewing session before
filing.

**What is wrong.** `docs/GETTING-STARTED.md:16-21` says the setup prompts "are new and have not yet been tried in any
AI coding tool, so the steps below are still the way that is known to work". `docs/prompts/setup.md:10-14` says "Tried
so far: in Antigravity, on a Mac, on 13 October 2026 ... The setup worked", with the honest nuance that the build
that followed went badly and Antigravity is not recommended until the prompts change (0407). The guide's version sends
the very person the prompts were written for (someone who has used an AI chat but never built an app) to the
terminal steps instead, and has neither half of the nuance.

**Fix.** The guide's paragraph says what the page says: "Tried once, in Antigravity on a Mac (13 October 2026): the
setup worked; the build that followed showed problems the prompts are being changed for, and they have not yet been
tried in Claude Code, Cursor, or VS Code." A small test holds the guide's "tried" sentence to the setup page's (as
`crates/sv-cli/tests/setup_prompts.rs` holds the page to `sv connect`), so the two cannot drift again. Must not lose:
the warning about the build. May be settled inside 0370's finish rather than on its own.

**Record.** None.
