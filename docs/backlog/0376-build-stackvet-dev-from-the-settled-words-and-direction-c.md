# Build stackvet.dev from the settled words and direction C

**Status:** open

**Was queued on the coordination board** (backlog 0273) until the board was removed on 20 October 2026 (ADR-061, Later); it is now taken the normal way, by a claim.

From backlog 0224 (done 12 October 2026): the home page's words and its look are settled in
`docs/website/home-page.md`, direction C in the logo's own colors. To build: the page itself, the credits page, and
stackvet.app redirecting to stackvet.dev, with the coverage numbers read from `docs/COVERAGE.md` when the site is built
rather than copied in. Where it is hosted, and how the owner's two addresses point at it, are the owner's to choose
(hosting may cost money, so it is asked first).

**It does not go live** while any of backlog 0370 to 0374 is open: the setup prompts tried and the tools named
(0370), the report leading with what to fix (0371), requirements in plain language (0372), the walk-through (0373),
and every credited license confirmed (0374). The research section stays a placeholder until the owner's paper is
published. Nothing is live from this item until the owner says so.

**Two more pages, at the owner's word on 12 October 2026:**
- **How StackVet works**, for people who want the detail: built from the inventory of what a run gathers against what
  the report shows (the owner: "this inventory can serve dual purposes"), and from `docs/paper/HOW-SV-WORKS.md`.
- **StackVet checks itself**: StackVet's own report on its own repository (`docs/paper/self-assessment-v2`), its SBOM
  (backlog 0382), and the rest, linked to where each lives on GitHub.

**The owner's answer, 15 October 2026:** after the paper's data is done.

**Checked against `main`, 20 October 2026** (the backlog clean-up): every condition to build is met (0224, 0433, 0370 to 0374, 0568); nothing is built. It now also has the reports page (0413) and the framework pages (0619, `tools/framework_pages.py`, ADR-129) to take in. Hosting and the two addresses are the owner's to choose, and nothing goes live while 0373 is open or before the owner's word. `docs/website/home-page.md`'s list of what is not yet true is brought up to date with this clean-up.
