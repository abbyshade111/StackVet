# Three license questions for the owner from the credits reading: Brakeman, the Semgrep rules under Opengrep, and CodeQL's name

**Status:** done, 14 October 2026

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
   **The owner's decision, 14 October 2026:** accept it, on the same condition as the Semgrep rules: StackVet runs Brakeman, when the person has installed it, on the person's own app, and nothing built on it is sold. Recorded beside Semgrep's in ADR-018 (Later, 14 October 2026).
   **Part status:** done, 14 October 2026

2. **The Semgrep Rules License, under Opengrep.** `LICENSE` in semgrep/semgrep-rules says only "Semgrep Rules
   License v1.0. For more details, visit https://semgrep.dev/legal/rules-license", and that page could not be read
   from the session that checked: its network refused semgrep.dev. The owner read it on 26 September 2026 and
   accepted it while nothing is sold (ADR-018); on 3 October chose Opengrep to run the same rules when Semgrep is
   not installed. **The question:** the owner, reading that page, confirms that it allows the rules to be run by an
   engine other than Semgrep's own (Opengrep). Nothing here says it does not; it was not read.
   **The owner read the page, 14 October 2026.** Its answer to "Is it OK to run Semgrep or Semgrep, Inc. rules on my work projects?": "Yes! Semgrep is safe to run on your private code. The Semgrep Rules License v. 1.0 restrictions only come into effect if you are selling a product using rules provided in the Semgrep Registry. If that's the case, contact partnerships@semgrep.com to learn more." The restriction is on selling a product built with the registry's rules, not on which engine runs them, so running them under Opengrep is within the licence on the condition already recorded: nothing built on them is sold (ADR-018, Later). Confirmed.
   **Part status:** done, 14 October 2026

3. **CodeQL's name.** The GitHub CodeQL Terms and Conditions (`LICENSE.md` in github/codeql-cli-binaries): "These
   Terms do not grant any right or license to use any of GitHub's trademarks or logos, including, without
   limitation, the names GitHub and CodeQL ... except as allowed by GitHub's Logos and Usage Policy located at
   https://github.com/logos." The credits page names CodeQL and GitHub as the tool and its maker, as it names every
   other. **The question:** is naming them in the credits, with "StackVet is not made by or endorsed by ... GitHub",
   enough, or does the owner want GitHub's Logos and Usage Policy read first? Suggestion: read it before launch;
   naming a tool one runs is what credits do, and no logo is used.
   **The owner read a CodeQL licence, 14 October 2026:** the MIT License (copyright 2006-2025 GitHub, Inc.), which is the licence of `github/codeql`, the queries and libraries. The clause quoted above is from the GitHub CodeQL Terms and Conditions on `github/codeql-cli-binaries`, the command-line program `sv` runs, which are not MIT (and are why 0415 limits CodeQL to the uses they allow). MIT grants no trademark right either, so the naming question stands as written: read GitHub's Logos and Usage Policy before launch; the credits name the tool and its maker and use no logo.
   **The owner's decision, 14 October 2026:** the owner read GitHub's Logos and Usage Policy; naming CodeQL and GitHub in the credits, as the tool and its maker, with no logo and the line that StackVet is not made by or endorsed by GitHub, is fine. The owner also read the GitHub CodeQL Terms and Conditions in full the same day: the uses they allow are the ones 0415 names when a person turns CodeQL on, and StackVet ships none of the program.
   **Part status:** done, 14 October 2026

- How verified: read from each source on 14 October 2026; the Semgrep Rules License's own page not read (above).
- Needs a decision record?: yes for 1 and 2, a Later entry on ADR-018 with the owner's answer.
