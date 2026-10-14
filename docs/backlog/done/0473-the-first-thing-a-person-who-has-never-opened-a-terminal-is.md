# The first thing a person who has never opened a terminal is told to do is open a terminal, and nothing says how

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: usability. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `docs/GETTING-STARTED.md:41-45` ("Then fetch StackVet, in a terminal: `docker pull …`"), 49-54, 63-71
  (``connect claude --docker "$(command -v docker)"``), 252-267 (the `docker run --rm -it … -e XDG_CONFIG_HOME=…
  review .` line); `docs/GLOSSARY.md:11-14` ("Terminal: A window where you type commands instead of clicking. On
  a Mac it is the app called Terminal").
- What is wrong: the glossary says what a terminal is and not how to get one (on a Mac: Spotlight, type
  Terminal; on Windows: Start, type PowerShell), nor how to get one "in the app's folder" (Finder: right-click
  the folder, "New Terminal at Folder", or drag the folder onto the Terminal icon), which five of the guide's
  commands assume. The connect line uses `"$(command -v docker)"`, a shell substitution, where the guide's own
  advice two lines later is to type `which docker` and copy what it prints. The `sv review` line for "only
  Docker" is 190 characters with a volume and an environment variable, typed each time.
- What to do: the glossary's Terminal entry gains "how to open one, and how to open one in a folder", per system,
  in four lines; the guide's three sections lead with "let your AI tool do steps 1 to 3" once F5 is honest, with
  the typed steps kept as the fallback they are; the connect line becomes two lines (`which docker`, then the
  command with the path pasted in). For `sv review` with only Docker, the cheapest relief is the setup page's
  prompt 6 (which installs `sv` natively) followed by plain `sv review .`; say so beside the long line ("once you
  have installed StackVet for the full report, this is just `sv review .`").
- How verified: read.
- Overlap: 0217 part 7 (done: the glossary, which has the entry but not the "how"), 0119 ("getting to step one"),
  0370 (claimed). The specific wording is new.
- Needs a decision record?: no.

**Done 14 October 2026 (session documentation).** The glossary's Terminal entry now says how to open a terminal on a
Mac, Windows, and Linux, and how to open one in the app's folder. The guide's connect step takes Docker's path from
`which docker`, pasted after `--docker`, rather than `"$(command -v docker)"`. The long `sv review` line for Docker alone
is followed by "Once StackVet is installed on the computer itself (section 6), this is just `sv review .`." To stay in
the guide's 4,200-word budget, three sentences were shortened and one dropped ("The examples below show what it
prints."). Not done: leading sections 1 to 3 with the AI tool doing them, which waited on "F5", a finding no committed
file names. That is backlog 0512.
