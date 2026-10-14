# Three license questions for the owner from the credits reading: Brakeman, the Semgrep rules under Opengrep, and CodeQL's name

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From backlog 0374 (14 October 2026), which read every credited tool's and data source's license from its source and
wrote each beside its credit in `docs/website/home-page.md`. These three need the owner's judgment; the rest do not.
Each quotes the text that raises it. The site's item 5 ("every credited tool's and data source's license confirmed")
stays open until these are answered.

1. **Brakeman's license allows non-commercial use only.** `LICENSE.md` in presidentbeef/brakeman, the Brakeman Public
   Use License: "Commercial Uses ... of the Software for commercial purposes require a commercial, non-free
   license." Its examples of commercial use include "Using the Software as a component of a value-added
   service/product"; of uses that are not, "Using the Software to analyze Licensee's software." `sv` runs Brakeman,
   when the person has installed it, on the person's own app, and does not ship it (StackVet's image installs no
   outside tool). **The question:** does the owner read StackVet's running of Brakeman as fine on the same condition
   as the Semgrep rules (StackVet is not sold), or ask the maintainer, as the license invites ("if Licensee is
   unsure whether it needs to purchase a Commercial Use license, contact the maintainer")? Suggestion: the same
   condition as Semgrep's, recorded in ADR-018 beside it.
   **Part status:** open

2. **The Semgrep Rules License, under Opengrep.** `LICENSE` in semgrep/semgrep-rules says only "Semgrep Rules
   License v1.0. For more details, visit https://semgrep.dev/legal/rules-license", and that page could not be read
   from the session that checked: its network refused semgrep.dev. The owner read it on 26 September 2026 and
   accepted it while nothing is sold (ADR-018); on 3 October chose Opengrep to run the same rules when Semgrep is
   not installed. **The question:** the owner, reading that page, confirms that it allows the rules to be run by an
   engine other than Semgrep's own (Opengrep). Nothing here says it does not; it was not read.
   **Part status:** open

3. **CodeQL's name.** The GitHub CodeQL Terms and Conditions (`LICENSE.md` in github/codeql-cli-binaries): "These
   Terms do not grant any right or license to use any of GitHub's trademarks or logos, including, without
   limitation, the names GitHub and CodeQL ... except as allowed by GitHub's Logos and Usage Policy located at
   https://github.com/logos." The credits page names CodeQL and GitHub as the tool and its maker, as it names every
   other. **The question:** is naming them in the credits, with "StackVet is not made by or endorsed by ... GitHub",
   enough, or does the owner want GitHub's Logos and Usage Policy read first? Suggestion: read it before launch;
   naming a tool one runs is what credits do, and no logo is used.
   **Part status:** open

- How verified: read from each source on 14 October 2026; the Semgrep Rules License's own page not read (above).
- Needs a decision record?: yes for 1 and 2, a Later entry on ADR-018 with the owner's answer.
