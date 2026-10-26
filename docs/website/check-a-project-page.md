# stackvet.dev: checking somebody else's project

The words of the page for somebody deciding whether to use a project they found (backlog 0671), for the feature backlog
0670 built (ADR-154), as `home-page.md` keeps the home page's: `tools/website.py` reads them, shows each section's
headline in bold as its heading, and leaves out each section's last list, the notes on it. Every claim is checked
against what 0670 built; what was not tried is said in the notes, not on the page. **Nothing here is live** until the
owner publishes the site.

## The opening

**Thinking of using somebody else's code? Check it first.**

You found a project on GitHub or GitLab that does what you need: a library, a template, a whole app to start from.
Before you build on it, give StackVet its address. StackVet downloads the code, checks it the way it checks your own
app, and writes a report for you, the person deciding whether to use it, not for the person who wrote it.

```
sv report https://github.com/OWNER/PROJECT
```

- Built in backlog 0670, in three pull requests (#2362, #2363, #2364), 25 October 2026; ADR-154.
- The command works for `https://github.com/OWNER/PROJECT` and `https://gitlab.com/GROUP/PROJECT`, with
  `/tree/BRANCH` or `/commit/COMMIT` to name one version.

## What happens

**One download, and nothing in it runs.**

- **It fetches one version.** StackVet downloads the host's own archive of the project, one commit, and nothing else
  leaves your computer: no account, no password, and nothing about your own files. It talks to GitHub and GitLab
  only, over a secure connection, and stops at 500 MB.
- **It unpacks the files itself.** The archive is plain files, never a program. StackVet writes each one into a folder
  of its own, `stackvet-fetched`, where you started it, with no permission to run. Anything that tries to land outside
  that folder, or is a shortcut to somewhere else, is left out, and StackVet tells you what it left out.
- **It names the version it checked.** A project can change tomorrow. The report says exactly which version, by its
  commit, it looked at.
- **It checks the code by reading it,** with the same checks your own app gets: StackVet's own rules, the search for
  keys and passwords left in the code, and the security tools each language already has, when they are on your
  computer.

Then it writes the report, in a folder named `stackvet-report` inside the project's folder.

- The download is through `curl` with HTTPS only, to github.com, gitlab.com, and codeload.github.com, each redirect
  checked before it is followed, at most three, 500 MB, and ten minutes (ADR-154; `docs/ARCHITECTURE.md`, "The
  fence", row 4). "Nothing about your own files" is the request carrying only the archive's address.
- The unpacking: at most 50,000 files and 2 GB; links, names with `..`, a root, a drive, `\`, or `:`, and a second
  file of the same name are left out and named in the terminal.

## The report

**A report written for the person weighing the risk.**

A report on your own app tells you what to fix. You can't fix somebody else's project, so this one is written
differently.

- **Every page says whose project it is,** with its address and the version, at the very top.
- **It counts risks to weigh, not things to fix.** For each one, the report says what it is and why it matters, so you
  can decide whether it matters for how you would use the project.
- **"Nothing found" never means "safe."** When nothing turns up, the report says so in those words: "That does not
  make it safe to use." It also says how many of the security requirements nothing looked at, and how many depend on
  answers only the project's author could give.
- **It doesn't guess the author's answers.** A lot of security depends on things only the author knows: who may do
  what, what data the app keeps, how it signs people in. You can't answer those for somebody else's project, so they
  stay open, and the report says which requirements that leaves open. If the project comes with its own StackVet
  settings, the report says those answers are the author's word, not yours, and that StackVet didn't check them.

Start with `report.html`: it has the whole report on one page.

- `crates/sv-report/src/fetched.rs` writes the banner, the short version, and the next steps; the quoted sentence is
  its headline when nothing was found. The questions left open are counted from the report's own "not verified" and
  "nobody has placed" numbers.

## Running it, only if you ask

**Starting a stranger's code is your call, and it happens last.**

Reading code is safe. Running it means running code nobody has vetted. So StackVet never starts a project you fetched
unless you ask it to, with `--run`, and even then:

- **Only after every check that reads the code has finished.**
- **Only if the project says how to start itself,** in its own StackVet settings. If it doesn't, StackVet doesn't
  guess, and the report says it wasn't started.
- **Inside StackVet's fence,** as your own app would be: a container with no way out to the internet, its files
  read-only, and every extra permission switched off.
- **Its packages are downloaded only if you also add `--install`.** The project's author can ask for that in their
  settings, but it's your decision, not theirs.

When a project was started, the first line of every page says so: code nobody vetted was run.

```
sv report https://github.com/OWNER/PROJECT --run --install
```

- `--run` needs Docker or Colima, as for any app (`docs/INSTALL.md`). The install step is the fence's second exception
  (ADR-052, Later, 25 October 2026: the owner's choice, "Only with --install"). Not tried yet: starting a project
  fetched from GitHub or GitLab for real.

## What it can't tell you

**Honest about what a check can't see.**

StackVet's report is one piece of evidence, not a verdict. It can't tell you whether the author is trustworthy,
whether the project is still maintained, or what a newer version will change. It checks the version it names, as
written, and says plainly what it did not look at. What you do with that is yours to decide.

- Tried on 25 October 2026 against one real project on each host: a GitHub project and a GitLab project, each fetched,
  its commit named and confirmed, and reported on.
