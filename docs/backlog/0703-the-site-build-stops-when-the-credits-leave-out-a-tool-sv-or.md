# The site build stops when the credits leave out a tool sv or cv runs

**Status:** claimed by Claude Code session (packaging and ConfigVet), 21 November 2026

Filed 21 November 2026, at the owner's word, from the whole-site review (backlog 0696). The credits are written by
hand in `docs/website/home-page.md`, so they drifted: they named CodeQL for three languages when `data/adapters.json`
also runs it on C and C++, and named none of `cv`'s tools after `data/cv-adapters.json` gained hadolint, Checkov, and
Trivy. Nothing failed.

- **What to build:** `tools/website.py` stops the build when a tool `name` in `data/adapters.json` or
  `data/cv-adapters.json` (CodeQL's entries read as one tool and its languages) is not named in the credits' shown
  words, and says which. The licenses stay written by hand, since each is read from its source (backlog 0374).
- **Done when:** removing hadolint, or CodeQL's "C and C++", from the credits makes the build and
  `crates/sv-cli/tests/website.rs` fail, and putting it back makes them pass.
- **Order,** after 0696.
