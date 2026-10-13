# Confirm the license of every tool and data source the website credits

**Status:** open

Before stackvet.dev goes live (backlog 0224, `docs/website/home-page.md`, "Credits"). The repository records two
licenses: the Semgrep Rules License, with the owner's condition that nothing built on it is sold (ADR-018), and FIRST's
CVSS calculator, BSD-2-Clause (ADR-033). The rest are to be read from each one's own source and written beside its
credit: CodeQL first (GitHub's terms limit some uses, which may need the same care as Semgrep's), Brakeman (its license
is not recorded here), then Bandit, gosec, Opengrep, OSV, and MITRE ATLAS. The OWASP standards are CC BY-SA 4.0,
which asks for credit, a link to the license, and a note of changes; the credits page says StackVet holds them as
data and words some of them plainly.
