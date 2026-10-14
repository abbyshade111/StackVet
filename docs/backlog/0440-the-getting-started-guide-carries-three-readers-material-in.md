# The getting-started guide carries three readers' material in one 6,300-word file, and the setup is the smallest third

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing.

- Where: `docs/GETTING-STARTED.md:29-305` (steps 1-5, about 2,400 words); `:307-591` (section 6, 2,745 words:
  what you get, an image of your own, the dashboard and `sv compare` (342), the OSV downloads (228), installing
  StackVet for `--run` (1,297)); `:593-633` (keeping up to date, 410); `:635-639` (known problems, 48, and its two
  sentences say nothing is open).
- What is wrong: The guide's own first lines promise "about fifteen minutes to set up, once", and that promise is
  kept by the first 2,400 words. The next 3,200 are for later days: the week the person wants the running-app
  checks (Homebrew, Scoop, rustup, `PATH`, `xcode-select`), the month they have two apps (dashboard, history,
  compare), and maintenance (two copies to keep at one version). A reader on their first afternoon, on a phone
  (the owner often is), scrolls past the install of Rust to find "Where the report is" (`:328`). Section 6's heading
  ("What you get without anything more, and what you do not") hides the install under it.
- What to do: Split by the day the reader needs it. **GETTING-STARTED** keeps the picture from F2, steps 1 to 5,
  "What you get without anything more", "Where the report is", and the tier diagram's link: at most 3,000 words.
  **INSTALL.md** (or `docs/install/`): Scoop, Homebrew, by hand, "the installed copy does not need the folder", the
  build folder, keeping up to date, versions and pinning, old copies. **A reference page each** for the dashboard
  and history, `sv compare`, and the OSV downloads (the table at `:425-432` is already right). At the top of the
  guide, one flow: Docker running → a folder in git → connect (`sv connect`) → "Did it connect?" → the starting
  prompt → answer the questions and `sv review` → the report; with one branch off the last box, "want the checks of
  the running app? → INSTALL.md → `sv report --run`". Must not lose: the honesty paragraph at `:23-27` stays first;
  "Docker has to be running before you open your AI tool" stays at step 1; every "not yet tried by hand" note moves
  with its section.
- How verified: read; word counts per heading.
- Overlap: 0217 (partly done) built the setup prompt, the glossary, the report prompt, and `sv doctor`, and its part
  8 (watch someone do it) is the owner's; 0370 (claimed) is the setup prompt library; 0120 (partly done) the install
  routes. None restructures the guide, and 0217's recommendation did not consider its length.
- Needs a decision record?: no.
