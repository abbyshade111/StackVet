# The no-network rule is stated six times with two different exception lists since ADR-111, and wants one home and a picture

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing. The report's own "Outside tools" sentence, the coverage tables, and `sv bundle --help` are the same staleness in the code, filed as one item of their own.

- Where: `CONTRIBUTING.md:53-57` ("with two exceptions the person asks for by name: `sv probe` ... and ... the
  install step"); `SECURITY.md:30-31` ("designed to open none, apart from `sv probe <address>` and the install step
  with `install = true`, each asked for by name"); `docs/ARCHITECTURE.md:135-136` ("`sv` opens no network connection
  of its own but `sv probe` and the install step"); `README.md:94` ("with two exceptions you ask for by name") followed
  at `:99-104` by Semgrep fetching its packs "every time it runs"; `CLAUDE.md:37-41` (three exceptions, Semgrep named);
  `docs/website/home-page.md:146-151` ("There are three exceptions. Two you turn on yourself ... The third is on unless
  you turn it off").
- What is wrong: Since 13 October (ADR-111, item 0394 done) the outside tools run by default, and Semgrep downloads its
  rule packs from semgrep.dev on every `sv report` unless `--no-tools` is passed. CLAUDE.md and the website say three;
  CONTRIBUTING, SECURITY, and ARCHITECTURE still say two and name neither Semgrep nor `--no-tools`. SECURITY.md is the
  page that tells a reporter what counts as a vulnerability: by its text, a report reaching semgrep.dev is one. 0330
  (done) added the Semgrep sentence beside the README's rule; 0296 (done) corrected CONTRIBUTING's two exceptions on
  12 October, before ADR-111 landed, so it was right for a day.
- What to do: State the fence once, as a picture and a table, and have every other page say one sentence and link
  to it. The picture: a layered box, the app's container at the center (read-only, no capabilities, on a network with
  no gateway), the sidecar that sends every request from inside, the helpers (mail, test identity provider, test
  model, headless browser) inside the fence, `sv` itself outside it with **three** arrows crossing to the internet,
  each labeled with what turns it on: `sv probe <address>` (at most four requests, five with `--api`, one DNS
  question), `install = true` (PyPI or npm, the dependency files only, a container that never sees the code), and the
  outside tools unless `--no-tools` (Semgrep's rule packs from semgrep.dev; Docker's own image pulls for `sv run`).
  Home: `docs/ARCHITECTURE.md`'s "The fence" bullet, or the "How it works" page proposed in section 3. Then fix the
  three stale statements to "three exceptions, listed in <link>". Must not lose: that the exceptions are opt-in
  (two) and opt-out (one), which is the point the website already makes well.
- How verified: read; grep across the eleven person-facing files for "no network connection".
- Overlap: 0330 (done): the README sentence only; 0296 (done): CONTRIBUTING before ADR-111; 0322 (open) is about
  which `tools/*.py` need the network, not `sv`. New: SECURITY.md, CONTRIBUTING.md, and ARCHITECTURE.md are stale
  against ADR-111, and the six statements have no single home.
- Needs a decision record?: no new decision; ADR-111 and ADR-027 and ADR-052 govern and are unchanged: the pages are
  corrected to them ("unchanged, because" lines).
