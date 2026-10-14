# Confirm the license of every tool and data source the website credits

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Before stackvet.dev goes live (backlog 0224, `docs/website/home-page.md`, "Credits"). The repository records two
licenses: the Semgrep Rules License, with the owner's condition that nothing built on it is sold (ADR-018), and FIRST's
CVSS calculator, BSD-2-Clause (ADR-033). The rest are to be read from each one's own source and written beside its
credit: CodeQL first (GitHub's terms limit some uses, which may need the same care as Semgrep's), Brakeman (its license
is not recorded here), then Bandit, gosec, Opengrep, OSV, and MITRE ATLAS. The OWASP standards are CC BY-SA 4.0,
which asks for credit, a link to the license, and a note of changes; the credits page says StackVet holds them as
data and words some of them plainly.

**The owner's word, 14 October 2026:** a session does the reading (each tool's and data source's licence from its own source, CodeQL first) and brings the owner only the ones that need a judgment, with the text that raises it. Queued on the board.

**Done, 14 October 2026.** Each credited tool's and data source's license was read from its own source and written
beside its credit in `docs/website/home-page.md`: CodeQL (GitHub's CodeQL Terms and Conditions, which allow
analysis of open-source code, academic research, and demonstration; `sv` runs it only when the person turns it on,
backlog 0415; its queries are MIT), Semgrep and Opengrep (LGPL 2.1, the rules under the Semgrep Rules License,
ADR-018), Bandit and gosec (Apache 2.0), Brakeman (the Brakeman Public Use License, non-commercial), OSV (each
source's own terms), and MITRE ATLAS (Apache 2.0). The three that need the owner's judgment, with the text that
raises each, are backlog 0513: Brakeman's non-commercial clause, the Semgrep Rules License's own page (not read: the
session's network refused semgrep.dev), and CodeQL's name. Not built: no code changed; no license was decided.
