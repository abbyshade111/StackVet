# stackvet.dev: the How StackVet works page

The words of the page for people who want the detail (backlog 0376, ADR-132), as `home-page.md` keeps the home page's:
`tools/website.py` reads them, shows each section's headline in bold as its heading, and leaves out each section's last
list, the notes on it. After the last section it shows the description from `docs/paper/HOW-SV-WORKS.md`. **Nothing
here is live.**

## The path

**How StackVet works**

The path from "I built an app" to "I have a report": what you do, what your AI coding tool does, and what StackVet
does.

![Three lanes, you, your AI coding tool, and StackVet, across five steps: plan, build, check, questions, and
report.](diagrams/how-it-works.svg)

- The drawing is `docs/website/diagrams/how-it-works.svg`, made by `make_how_it_works.py` beside it (backlog 0436).

## From a real build

**When only you can answer, your AI tool stops and asks.**

Step 4 in a real build: StackVet's check found that the app gave nobody a way to report a security problem, and the
AI tool would not make up an address to put there.

*From a real build in VS Code with GitHub Copilot, 14 October 2026, in the tool's own words:*

> **Your AI tool:** StackVet found one real issue: there’s no way to report a security problem. I shouldn’t invent a
> contact address, so I’m pausing to get one before creating the required security-contact information.

- The VS Code build of the setup prompt trial (`docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md`), at the owner's
  word on 22 October 2026. Quoted from the chat as it is kept on the owner's drive, with its curly apostrophes. The
  owner's answer, a contact address, is left out.

## In detail

**In detail**

This is the account written for the research paper about StackVet, of StackVet as it was on 4 October 2026, when it
was still called SecureVibe and its settings file `securevibe.toml`. The way it works is the same; some details, such
as the number of commands, have grown since. The current map of the code is
[docs/ARCHITECTURE.md](https://github.com/abbyshade111/StackVet/blob/main/docs/ARCHITECTURE.md).

- The paper's description is read from `docs/paper/HOW-SV-WORKS.md`, "Description", and shown after this.
