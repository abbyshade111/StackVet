<picture>
  <source media="(prefers-color-scheme: dark)" srcset="brand/stackvet-logo-dark.svg">
  <img alt="StackVet" src="brand/stackvet-logo.svg" width="320">
</picture>

# Building an app with StackVet alongside

This is for building an app from an empty folder with an AI coding tool, with StackVet checking it as
you go. You do not need to know how to program, and you do not need to know about security: the tool
writes the code, StackVet checks it against the OWASP security standards, and the two of them ask you
the questions only you can answer.

![Three lanes, you, your AI coding tool, and sv, across five steps: plan, build, check, questions, and report. You describe the app, and the tool writes stackvet.toml with you; sv works out which requirements apply. The tool writes the code a feature at a time and calls stackvet_check after each one; sv says first what it did not examine, and the tool fixes what it finds. The tool asks you the questions only you can answer, and you record your answers with sv review. Then you run sv report --run at a terminal; sv starts the app behind its fence and writes the report folder.](website/diagrams/how-it-works.svg)

It takes about fifteen minutes to set up, once. A word here you do not know is probably in
[the glossary](GLOSSARY.md).

**Easiest: let your AI coding tool do steps 1 to 3.** [The setup prompts](prompts/setup.md) are for someone who has used
an AI chat but never built an app with one. Copied into your AI coding tool one at a time, they have it look at your
computer, help you install what is missing, fetch StackVet (on Windows, install it with Scoop), and write the settings
file with the paths already filled in, asking you before it installs or changes anything outside the folder.
**Tried so far:** in Cursor and in VS Code with GitHub Copilot, on a Mac, on 14 October 2026. The setup worked. Both
did the prompts as written, each with one gap now fixed. In Antigravity with the Gemini 3.8 Flash model, the build that
followed went badly: it rewrote code to make a check pass, then said it had not, so the setup page does not recommend
it. They have not yet been tried in Claude Code, or the Claude desktop app. Steps 1 to 3 below do it by hand, if you
prefer or a prompt stops.

## What StackVet will and will not tell you

It never says your app is secure. It says what it checked, what it found, and, first of all, what it
did not look at. A short report is not a good sign unless the part saying what was not examined is
short too.

## 1. Install Docker, and start it

Did the setup prompts do this? Go to step 4.

StackVet runs inside [Docker](GLOSSARY.md#docker), so there is nothing else to install. (Checking your app while it
runs, a later step, needs StackVet installed on the computer itself: section 6.)

- **On a Mac:** Docker Desktop (docker.com), or Colima if you prefer something smaller.
- **On Linux:** Docker itself, or Docker Desktop. **On Windows,** skip Docker: see step 3.

**Docker has to be running before you open your AI tool.** If it is not, the StackVet tools are
missing, and nothing tells you why. After a restart, start Docker first.

Then fetch StackVet, in a [terminal](GLOSSARY.md#terminal):

```bash
docker pull ghcr.io/abbyshade111/stackvet-sv
```

## 2. Make a folder for the app, and put it in git

Make an empty folder for the app, for example `~/code/my-app`, and in a terminal there start
[git](GLOSSARY.md#git):

```bash
git init
```

The check for a password or key ever saved into your project's history only runs in a git folder.
Without git, that check is reported as [*not assessed*](GLOSSARY.md#not-assessed).

## 3. Connect StackVet to your AI tool

Your AI tool talks to StackVet over [MCP](GLOSSARY.md#mcp), a standard way for AI tools to use other programs. You add
one small file to the app's folder. In each example, replace `/Users/you/code/my-app` with your app
folder's full path, in all three places; `pwd` in a terminal there prints it.

StackVet can print the file with the paths filled in. In a terminal in the app's folder, `which docker` prints
Docker's full path; run this with yours after `--docker`, and `claude`, `vscode`, or `cursor` for your tool:

```bash
docker run --rm --network none -v "$PWD":"$PWD" -w "$PWD" ghcr.io/abbyshade111/stackvet-sv connect claude --docker /usr/local/bin/docker
```

It prints the settings and the name of the file they go in, and writes nothing. On Linux, add
`--user "$(id -u):$(id -g)"` at the end.

**On Windows, the Docker settings do not work yet:** `sv connect --docker` takes only a Mac or Linux path for
Docker. Install StackVet with Scoop instead ("On Windows: Scoop", in section 6), and in the app's folder, in
PowerShell, run `stackvet connect claude` (or `vscode`, or `cursor`). Its settings start the installed copy, without
Docker. Not yet tried by hand; the checks that start your app, not at all
(section 6).

### Claude (the desktop app and Claude Code)

A file named `.mcp.json` in the app's folder. One like it was tried in the Claude desktop app with
StackVet installed directly; this container version only by a test that talks to it as the tool does.

```json
{ "mcpServers": { "stackvet": {
  "command": "/opt/homebrew/bin/docker",
  "args": ["run", "-i", "--rm", "--network", "none",
           "-v", "/Users/you/code/my-app:/Users/you/code/my-app",
           "ghcr.io/abbyshade111/stackvet-sv", "mcp", "--root", "/Users/you/code/my-app"] } } }
```

- `command` is the full path to `docker`, because an app started from the Dock often cannot find it.
  With Docker Desktop it is often `/usr/local/bin/docker`.
- The first time you open the folder, the tool asks whether to use the new server. Say yes.
- On Linux, add `"--user", "1000:1000"` (your own `id -u` and `id -g`) before the image name, so the
  files it writes are yours. Without it the image runs as a user of its own, never root, and that user
  cannot write into your folder. On a Mac, Docker Desktop makes what it writes yours either way.

### VS Code (GitHub Copilot)

**Tried on 27 September 2026, by the owner, start to finish:** Copilot's agent called StackVet's
tools, asked every question one at a time, fixed the path findings `stackvet_check` reported, and ran
the check again to confirm the fix. The answers were
saved in the app's folder, and a fresh report showed them and the fix. That was with `sv` installed
directly; the container settings below have not been tried in VS Code yet.

You need the GitHub Copilot Chat extension, with the chat in **Agent** mode (the mode that can use
other programs), and VS Code 1.102 or newer. If you use a paid Copilot plan, each request may count
against its monthly allowance.

**Let VS Code write the settings file for you.** A file typed or pasted by hand was not picked up the
first time it was tried: a wrong folder, or quotes turned curly by a text editor, breaks it without any
message. Instead:

1. Open the app's folder in VS Code (*File → Open Folder*).
2. Open the Command Palette (Cmd-Shift-P on a Mac, Ctrl-Shift-P elsewhere), and choose
   **MCP: Add Server…**, then **Command (stdio)**.
3. For the command, give StackVet's full path, then `mcp --root ${workspaceFolder}`. With `sv`
   installed: `/full/path/to/sv mcp --root ${workspaceFolder}` (`which sv` in a terminal prints the
   path). VS Code fills in `${workspaceFolder}` with the open folder, so there is no path to type.
4. Name it `stackvet`, and save it for the **Workspace** (this app only).
5. In the `.vscode/mcp.json` it opens, click **Start** above `stackvet`. In the chat, the tools
   button should now list the twelve `stackvet_` tools.

The file it writes looks like this:

```json
{ "servers": { "stackvet": {
  "type": "stdio",
  "command": "/full/path/to/sv",
  "args": ["mcp", "--root", "${workspaceFolder}"] } } }
```

For the container instead (not yet tried in VS Code), `command` is the full path to `docker` and `args` are
`["run", "-i", "--rm", "--network", "none", "-v", "${workspaceFolder}:${workspaceFolder}",
"ghcr.io/abbyshade111/stackvet-sv", "mcp", "--root", "${workspaceFolder}"]`.

If **MCP: Add Server…** is not in the list, check the VS Code version (*Code → About*), search Settings
for `mcp` in case it is switched off, and, if your Copilot comes through work or school, ask whether
your organization has turned these tools off. If StackVet does not appear or will not start,
**MCP: List Servers → stackvet → Show Output** says why.

### Other tools

The same `command` and `args` go in the tool's own MCP settings file. **Cursor has been tried once**, on
one app built from an empty folder (see [the record](design/0368-the-app-built-in-cursor-from-an-empty-folder-11-october-2026.md)),
with the settings below. In Cursor, turn the StackVet server on under Settings, then MCP, before its tools appear;
until then the chat lists none. If it does not work for you, tell us:

- **Cursor:** `.cursor/mcp.json` in the app's folder, with the same `mcpServers` block as for Claude, above.

### Did it connect?

Check before you start building. If StackVet is not connected, the tool does not say so: it builds
without it, and nothing gets checked. In the tool's chat, ask:

> Which `stackvet_` tools can you call? List their names.

It should list twelve, among them `stackvet_spec` and `stackvet_check`. If it lists none, or says it
has no such tools, StackVet is not connected. If it lists them, `stackvet_status` says whether the rest
is ready: the folder in git, `stackvet.toml`, and how to start the app. Then, in this order: make sure Docker is running (step
1); check the app folder's path in the settings file, in all three places; and restart the tool, since
most read their MCP settings only when they start. In Claude Code, `/mcp` lists each server and
whether it connected. In VS Code, **MCP: List Servers → stackvet → Show Output** says why it did
not start.

If the tool stops mentioning StackVet later, ask again: after a restart, Docker may not have
started yet.
The prompt in step 4, and the rules file below (`sv rules`) for any tool that reads `AGENTS.md`, tell
the tool to stop and tell you when the tools are missing.

### A tool without MCP

Every step still works by copying and pasting. Instead of the tool calling StackVet, you run it in a
terminal in the app's folder and paste what it prints into the chat:

```bash
docker run --rm ghcr.io/abbyshade111/stackvet-sv init
docker run --rm --network none -v "$PWD":"$PWD" -w "$PWD" ghcr.io/abbyshade111/stackvet-sv check .
docker run --rm --network none -v "$PWD":"$PWD" -w "$PWD" ghcr.io/abbyshade111/stackvet-sv questions .
docker run --rm --network none -v "$PWD":"$PWD" -w "$PWD" ghcr.io/abbyshade111/stackvet-sv rules .
```

On Linux, add `--user "$(id -u):$(id -g)"` after `docker run` in each line, for the same reason as
above. The last one writes the security rules for your tool into `AGENTS.md` in the app's folder, which many
tools read on their own; see "Rules your AI coding tool follows while it codes" in the README.

## 4. Start the build with this prompt

Open the app's folder in your AI tool and paste this, with your app described at the top:

> I want to build: *(describe the app in a few sentences: who uses it, what they do, what it keeps
> about them)*.
>
> We are using StackVet to check it as we go. If you cannot call the `stackvet_` tools, stop
> and tell me before writing any code: it means StackVet is not connected. Before writing any code:
>
> Step 1. Call `stackvet_spec` and write `stackvet.toml` for this app, from what it will really do.
>    Its capability lines start commented out: answer each one you can with true or false, and
>    **leave a line commented out rather than guessing `false`**. A line left out is reported as not
>    assessed; a wrong `false` switches whole sets of checks off. **Ask me before you decide where the
>    app runs, whether anyone signs in, or how it is reached** (`auth`, `tls`, and the `[stack.run]`
>    lines), and tell me whenever you change one. **Ask me, too, who uses the app (`audience`) and
>    what it keeps about people (`[data] categories`):** they decide whether it is held to ASVS
>    level 1 or the stricter level 2. Name the app's pages and routes in it, under
>    `[stack.run.users]`, not only `/` and the health path, so the checks of the running app reach them.
>    Fill in `owned` there too when the app has a record one person makes that another must not see:
>    it lets StackVet try to read it as somebody else and to submit a forged form.
>    If signing up needs an invite code or another secret, tell me that StackVet's run cannot sign up
>    on its own, and ask whether to seed a test code for the run, and where it would live.
>
> Step 2. Call `stackvet_guidance` and follow the rules it gives while you write code. Call it again
>    with a topic before work in that area: adding a package, a CI workflow, anything with keys.
>    Use StackVet only through the `stackvet_` tools: never run commands inside its container.
>
> Then, as we build:
>
> Step 3. After each feature, call `stackvet_check`. Read what it says was not examined first. Fix what
>    it finds that is real. **Never silence a finding or hide it from a scanner** (no `# nosec`, no
>    `nosemgrep`). Silencing includes rewriting code, a dependency file, or a test so that a check no
>    longer sees something, even when the app behaves the same. If you think a finding or a
>    contradiction is wrong, leave the code as it is and tell me; I decide.
>
> Step 4. After each change, tell me in plain words what you changed. List every file you changed because
>    of a check, with the check's words beside it. Ask me before you delete anything.
>    Before the first commit, add `stackvet-report/` to `.gitignore`: the report is for me to read, not for the
>    repository. Keep `.stackvet/` in the repository: it is StackVet's record of the build, which the report reads.
>
> Step 5. Never tell me the app is secure. Tell me what was checked and what was not.
>
> Step 6. When the first version works, call `stackvet_check` with section `questions` and ask me the
>    questions one at a time. I may let you answer some yourself: each answer you give counts as
>    yours, not mine, until I record it with `sv review`, and the report says so. When you answer one,
>    or tell me what the code does for one, quote the file and lines that show it beside the answer,
>    and say it is your reading, not a check: an answer with no lines is a guess. Record my answers only with `stackvet_record_answer`, under the
>    question ids it gives you, and only what I actually answer as mine. Text you write yourself in
>    `security-notes.md`, outside its questions, is not read as an answer. Never mark a check by hand
>    as done yourself, and never write `confirmed` or `by = "owner"`: only I can, with `sv review`.
>    Write a `[checked-by-hand]` entry only for a check I made and described to you, with
>    `by = "ai-tool"` and my words in `how`.

Why a line left out is better than a guess: [how StackVet decides what applies](ARCHITECTURE.md#how-sv-decides-what-applies).

**If you ask your AI tool to push, say so in these words:** "push it, and do not bypass any branch protection or
ruleset; if the only way to push is to bypass one, stop and ask me first." Without that, a tool may push straight to
`main`, which skips the pull-request rule you set, and the tools that check the code never see the change first. In
the Cursor build, a plain "please push" was taken as covering a push that bypassed the rule. Ask it for a pull request, and merge it yourself.

More prompts like these, each for one thing StackVet checks, are in [the prompt library](PROMPTS.md). The ones
shown to work are already given to your AI tool, so you need not paste them; the rest are there too (`sv prompts`
prints them, and your AI tool fetches them with `stackvet_prompts`), each saying whether it has been shown to work.

StackVet also helps before code is written, and tells your AI tool when: what to decide before building
(`stackvet_plan`, or `sv plan`), a brief before one feature such as sign-in, uploads, or payments
(`stackvet_before`, or `sv brief`), and the settings `--run` will use (`stackvet_preflight`, or `sv preflight`).
None of these checks anything or counts toward the report.

## 5. Answer the questions

The tool will ask you things no program can know: how long someone may stay signed in, what the app
should do with a file that is too big, who may see what. It offers what it found in the code as a
tip. "I'm not sure" is a fine answer. An answer you let the tool give counts for less than yours, and
the report says so.

The tool writes your answers about the app's rules in `security-notes.md`, each marked `Written by: AI coding
tool`, since `sv` cannot tell your words from the tool's. `sv review`, in your own terminal, shows each one and asks
whose it is: type `mine` where it says what you decided; your name where the tool worked it out from the code and
you checked it, which the report shows as the tool's words a person confirmed; or Enter to leave it. An answer the
tool wrote in `stackvet.toml` (`by = "ai-tool"`) is confirmed, not owned; `by = "owner"` by hand is for the rare
one that is truly your own decision. Run it:

```bash
sv review ~/code/my-app
```

**With only Docker** (you have not built StackVet on your computer), run it from the container instead.
Once, make the folder it keeps its key in, so that the folder is yours and only yours:

```bash
mkdir -p ~/.config/stackvet && chmod 700 ~/.config/stackvet && touch ~/.config/stackvet/allowed_signers
```

Then, each time, in a terminal in the app's folder:

```bash
docker run --rm -it --network none -v "$PWD":"$PWD" -w "$PWD" -v "$HOME/.config/stackvet":/sv-config/stackvet -e XDG_CONFIG_HOME=/sv-config ghcr.io/abbyshade111/stackvet-sv review .
```

On Linux, add `--user "$(id -u):$(id -g)"` after `docker run`, as in section 3.

With StackVet installed on the computer itself (section 6), this is just `sv review .`.

It shows each answer that does not yet count as yours, asks your name or `owner`, and signs it with a key of its own,
kept in your own settings folder; the first time, it asks whether to protect that key with a passphrase (yes or no; Enter
means yes). Only then does the report count the answer as yours: a line saying `owner` that was never recorded
this way still counts as the tool's word, since a tool could write it too. A real
finding is fixed in the code and checked again; recording it does not make it go away. When you record something,
`sv review` then writes the report again with the options of the last one, `--run` included, so your answers count
without a second run; where it cannot, it says so and gives you the command. `sv review` needs a terminal
someone is typing in, so your AI tool cannot run it for you. [`docs/REVIEW.md`](REVIEW.md) is the whole of it, with a
picture of how a signature is made and checked.

**If your AI tool uses the container** (section 3), give it your list of trusted keys, `allowed_signers`, one file,
read-only, and not the folder, which also holds the key that signs as you. The `.mcp.json` from section 3 becomes,
with your own paths (this has not been tried in an AI tool yet):

```json
{ "mcpServers": { "stackvet": {
  "command": "/opt/homebrew/bin/docker",
  "args": ["run", "-i", "--rm", "--network", "none",
           "-v", "/Users/you/code/my-app:/Users/you/code/my-app",
           "-v", "/Users/you/.config/stackvet/allowed_signers:/sv-config/stackvet/allowed_signers:ro",
           "-e", "XDG_CONFIG_HOME=/sv-config",
           "ghcr.io/abbyshade111/stackvet-sv", "mcp", "--root", "/Users/you/code/my-app"] } } }
```

Make the file first, with the `mkdir` line above, or Docker makes a folder in its place. Without this line, recorded
answers count as the tool's word in those reports. For CI, see [`docs/REVIEW.md`](REVIEW.md#setting-it-up-where-the-report-is-made).

Some questions are checks to make by hand, such as opening the live site and looking at the padlock.
The tool walks you through them and records what you saw; that record, too, counts as yours once you
have recorded it with `sv review`.

## 6. What you get without anything more, and what you do not

With the steps above, StackVet reads the code, the settings, and the list of packages the app uses,
and asks you the questions. It lists those packages but does not compare them with known vulnerabilities:
that needs a downloaded copy of the list of known ones, at a terminal ([Checking the packages against known
vulnerabilities](KNOWN-VULNERABILITIES.md)). **It does not start the app.** The checks that need a running app, such
as what it sends to a browser, whether signing out really ends the session, and whether one person
can see another's data, are reported as *not assessed*, and so are your app's own tests, which run only when
the app is started. That is honest, not a pass.

Those checks need `sv report --run` at a terminal, with StackVet installed directly on your computer
rather than in Docker, because starting your app means starting containers of its own. On Windows that is a
download, installed with Scoop, though these checks are untried on Windows: a Mac or Linux computer is the known way.
On a Mac or Linux it means building StackVet, which Homebrew does for you. The steps, and updates,
are on [Installing StackVet on your computer](INSTALL.md).

If your app uses packages, which most do, starting it also needs the line `install = true` under `[stack.run]` in
`stackvet.toml`. The app runs with no internet, so it cannot fetch its own packages; this line lets StackVet
download them first, in a separate box that sees only the list of packages, never your code. That download is the
one time StackVet uses the internet for your app, and the report says when it did. Without it, such an app does not
start, and what needs it running is *not assessed*.

**The full check is that one command.** `sv report --run` also runs the other security tools installed on your
computer, such as Bandit and Semgrep, and puts their findings in the same report.
Your AI coding tool's StackVet connection never runs them or starts the app, since each runs code, which stays your
decision at a terminal; so its reports and checks leave both out, and say so. To leave them out, add `--no-tools`. CodeQL runs only if `stackvet.toml` has
`[tools] codeql = true`, which its license allows only for an open-source app, research, or a paid GitHub license.

**Where the report is.** When your AI tool writes the report (`stackvet_write_report`), or you run
`sv report`, it goes in a folder named `stackvet-report` inside the app's folder. Open `report.html` in
a browser. Each file in the folder has a reader of its own:

| File | Who it is for |
|---|---|
| `report.html` | You, in a browser: double-click it |
| `compliance.md` | Anyone who needs the whole picture against the standards, you or someone you show it to |
| `security.md` | Whoever fixes the code, you or your AI coding tool |
| `low-findings.md` | You or your AI coding tool, when you want the detail |
| `stackvet.sarif` | Code editors and GitHub's code scanning, which show a finding beside its line |
| `report.json` | Programs, among them sv review, sv compare, and sv dashboard |
| `seen.json` | You, to see what a finding rests on |
| `sbom.cdx.json` | Tools that read a bill of materials |
| `report.pdf` | You, or whoever you hand it to |

On a small app `security.md` and `compliance.md` open alike, but they are not the same:
[the reports page](website/reports-page.md) shows an example of each. To have your AI tool
read it with you, starting with what was not checked, give it [the report prompt](prompts/report.md) (not yet tried
in an AI tool).

Each requirement in it has one of these words, strongest first:

- *needs attention*: something was found wrong;
- *checked*: a check of StackVet's own looked, and found nothing wrong in what it tried;
- *checked in part*: a check looked at only part of what the requirement asks;
- *tested by the app's own tests*: a test your AI tool wrote names it, and passed;
- *documented by the owner*: you wrote down your decision;
- *checked by hand by the owner*: you looked for yourself, and said what you saw;
- *attested by the owner*: you said yes to a question about how the app is built;
- *stated by the AI coding tool*: the tool answered, and you have not made the answer yours;
- *not verified*: nothing speaks to it yet.

None of them means "passed": even *checked* means one check found nothing wrong, not that the whole requirement
is met. Only StackVet's own checks can make a requirement *checked*; your word counts once you record it with
`sv review`, and the tool's word counts least. [A picture of the nine](ARCHITECTURE.md#the-nine-statuses-strongest-evidence-first)
shows who can grant each.

If you later run StackVet in an automatic check (CI), [a table in `docs/ARCHITECTURE.md`](ARCHITECTURE.md#the-exit-codes)
says what the number it ends with means.

## Later

- **The checks of the running app**, and keeping StackVet up to date: [`INSTALL.md`](INSTALL.md). An app in a language
  `install = true` does not cover can start from [an image of its own](INSTALL.md#an-image-of-your-own).
- **Several apps on one page**, their history, and what changed between two reports: [`ALL-YOUR-APPS.md`](ALL-YOUR-APPS.md).
- **The packages against known vulnerabilities:** [`KNOWN-VULNERABILITIES.md`](KNOWN-VULNERABILITIES.md).
- **If you set StackVet up before 5 October 2026:** [`INSTALL.md`](INSTALL.md#if-you-set-stackvet-up-before-5-october-2026).
