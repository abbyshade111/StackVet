# Setting StackVet up with your AI coding tool

These prompts are for someone who has used an AI chat but never built an app with one. You copy each prompt into
your AI coding tool, one at a time, and the tool does the setup with you: it looks at what your computer already has,
tells you what to install and where to get it, and asks before it installs or changes anything. If a step does not
work, it stops and tells you what happened in plain words.

If you have used a terminal or Docker before, [the guide](../GETTING-STARTED.md) has the same steps as commands.

**Tried so far:** in Cursor and in VS Code with GitHub Copilot, on a Mac, on 14 October 2026, from an empty folder
to a full report ([the record](../research/SETUP-PROMPT-TRIALS-2026-10-14.md)). The setup worked. Both did the prompts
as written, each with one gap now fixed in the prompts. Antigravity with the Gemini 3.8 Flash model, tried on 13 and 14
October 2026, rewrote code to make a check pass and then said it had not, so we don't recommend Antigravity with that
model. Tests hold the prompts to the image name StackVet is published under, to the options `sv connect` takes, and to
the Scoop commands that install it on Windows (`crates/sv-cli/tests/connect.rs`, `crates/sv-cli/tests/setup_prompts.rs`).
If anything here goes wrong, the guide has the same steps as commands. The website names only the AI tools these
prompts have really been tried in.

**Next:** the prompts as changed after that trial are tried again, in the same tools and in others.

## Before the first prompt: an AI tool that can run commands

A chat window that only writes answers cannot do this. You need an AI coding tool that can run commands on your
computer, in what it calls a terminal, asks you before it does, and can use MCP servers: MCP is a standard way for an
AI tool to use another program, and it is how your tool uses StackVet. Many tools can. These prompts name none of
them: prompt 3 asks your tool how its own app adds an MCP server, and has it add StackVet's.

Open your AI tool in a folder for your app: an empty folder in your home folder is a good one, called `my-app` or any
name you like. If you have not made one yet, prompt 2 offers to make it with you. Then go through the prompts in order.

While it works, your AI tool may ask whether it may read pages from github.com or raw.githubusercontent.com. Those are
StackVet's own instructions, which prompt 1 points it to, and reading them changes nothing: say yes.

**When your AI tool asks to run a command, allow it this one time.** Some tools also offer to always allow a whole
program, such as `colima`, `cat`, or `echo`. Say no to that: it would let the tool run anything that program can do
without asking again, which for some programs includes deleting things. For a command that only looks and changes
nothing, such as `docker info`, allowing that one command again in the same chat is fine.

| Prompt | What it does | Changes anything? |
|---|---|---|
| 1. Look at my computer | Says what is ready and what is missing | No |
| 2. Help me install what's missing | Makes a folder for your app if you need one, then walks you through each install, one at a time | Only with your yes, each time |
| 3. Set up StackVet here | Fetches StackVet, and has your AI tool add it to its own settings. One version for a Mac or Linux, one for Windows | This folder; anything else, and any install, only with your yes |
| 4. Check that it's connected | Asks StackVet itself whether everything is ready | No |
| 5. Start building | The first prompt of your app's build, with your app described at the top | Your app's folder, as it builds |
| 6. Make the full report | Installs or updates StackVet on your computer, then starts your app in a container and runs the security tools | Installs and runs only with your yes, each time |
| 7. Record my answers as mine | Goes through what is still only your AI tool's word, then tells you the one command to type yourself | No: you type the command |

## 1. Look at my computer

<!-- setup-step-1:start -->
> I'm new to this, so please explain things in plain words as we go. I want to set up StackVet, a tool that checks
> an app's security, so you can use it while we build an app together. This has to happen in a chat that runs on
> this computer, not in a cloud or background agent: if you are one of those, tell me and stop. StackVet is free and its code is public at
> https://github.com/abbyshade111/StackVet: its README and its guide (`docs/GETTING-STARTED.md` there) say what it
> needs and how it is set up, so read them there whenever you need to know something about StackVet. Read them, but
> don't run anything they say yet. For now, only look: don't install or change anything. Using your terminal, find
> out:
>
> Step 1. Which kind of computer this is: Mac, Windows, or Linux. On a Mac or Linux, show me what `uname -a` prints.
>
> Step 2. Whether git is installed (`git --version`).
>
> Step 3. On a Mac or Linux: whether Docker is installed (`docker --version`) and running (`docker info`). Either Docker
>    Desktop or Colima counts: both give the same `docker` command. If Colima is installed, check
>    `colima status` too.
>
> Step 4. On Windows: whether Scoop is installed (`scoop --version`).
>
> Step 5. Which folder you are working in (show me what `pwd` prints), whether it is empty, and whether it looks like a folder made for an app, or
>    like a scratch, temporary, or settings folder of your own, which is no place to keep an app.
>
> Then tell me, as a short list, what is ready and what is missing, and which of the next prompts I'll need. Don't
> fix anything yet.
<!-- setup-step-1:end -->

## 2. Help me install what's missing

Skip this one if prompt 1 said nothing was missing.

<!-- setup-step-2:start -->
> Please help me get ready, one thing at a time.
>
> First, the folder. If the folder you are working in is not an empty folder made for this app (for example, it is
> a scratch, temporary, or settings folder of yours), offer to make one in my home folder, called `my-app` or any
> name I choose: ask me which. Ask before you make it. Then tell me, step by step, how to open that folder in the app I'm using now, in that app's
> own words for its menus and buttons, and wait for me to say I'm done: the rest of the setup happens there. If you
> are not sure how this app does it, say so and ask me what I see, rather than guessing. Opening it may start a new
> chat with you: if it does, I'll paste the first prompt there, then this one again, and you can skip what is
> already done.
>
> Then install what you found missing, one thing at a time. If nothing is missing but Docker is not running, start
> it as below. For each one:
>
> - Tell me in one sentence what it is and why StackVet needs it.
> - Give me its official download page, and only that one: Docker Desktop at https://www.docker.com/products/docker-desktop/,
>   Colima at https://github.com/abiosoft/colima, git at https://git-scm.com/downloads, Scoop at https://scoop.sh.
> - For Docker on a Mac or Linux: if Docker Desktop or Colima is already installed, use that one, and don't offer the
>   other unless I ask. If neither is, tell me there are two choices that both work with StackVet: Docker Desktop, an
>   app with a window, which is the easier one to start with; or Colima, which is smaller and runs from the terminal.
>   Let me choose. On Windows, StackVet does not need Docker for this setup, so skip it there.
> - If I have to download and click through an installer, tell me what to click, step by step, and wait for me to say
>   I'm done.
> - If you can install it with a command instead, show me the command and what it will do, and run it only after I
>   say yes. Tell me first if it will ask for my computer's password or for administrator permission.
>
> After each one, check that it worked. For Docker, it has to be running, not only installed: check with `docker info`,
> and if it is not running, ask me to open Docker Desktop and wait, or, with Colima, ask before running
> `colima start`. StackVet needs Docker to run Linux containers: if I'm asked to choose, help me keep
> that. If anything fails, stop and tell me plainly what happened. Don't go on to the
> next thing until this one works.
<!-- setup-step-2:end -->

## 3. Set up StackVet here

Use the version for your computer.

### On a Mac or Linux

<!-- setup-prompt:start -->
> Please set up StackVet in this folder so you can use its tools while we build. StackVet runs in Docker, and you
> will use it as an MCP server. Work through these steps in order, using your terminal. Ask me before you install anything, and before you change
> anything outside this folder. If a step fails or you are unsure, stop and tell me what happened in plain words:
> do not carry on as though StackVet were set up.
>
> Step 1. Tell me which computer this is (Mac, Linux, or Windows). If it is Windows, stop: there is a Windows version of
>    this prompt, and I will use that one instead.
>
> Step 2. Check that Docker is installed and running with `docker info` (Docker Desktop or Colima: either works). If it
>    is not installed, tell me where to get Docker Desktop or Colima and wait for me. If it is installed but not
>    running, ask me to start it (with Colima, ask before running `colima start`) and wait.
>
> Step 3. Fetch StackVet: `docker pull ghcr.io/abbyshade111/stackvet-sv`
>
> Step 4. If this folder is not a git repository yet, run `git init` here.
>
> Step 5. Find docker's full path with `command -v docker`.
>
> Step 6. Print StackVet's MCP server for this folder, putting docker's full path from step 5 after `--docker`:
>    `docker run --rm --network none -v "$PWD":"$PWD" -w "$PWD" ghcr.io/abbyshade111/stackvet-sv connect --docker DOCKER_PATH`
>    On Linux, also add `--user "$(id -u):$(id -g)"` at the end, so the files StackVet writes are mine. It prints the
>    server's name, `stackvet`, that it talks over standard input and output, and its command and arguments.
>
> Step 7. Find out, from your own app's documentation, how this app is given a local MCP server that talks over standard
>    input and output: which settings file, where it is, and its format. If you are not sure, say so and stop,
>    rather than guessing.
>
> Step 8. If that file is in this folder, good. If it is one file for my whole account, tell me which file, and that
>    StackVet will then be set up for this folder only, and ask me before you read it or change it: it may hold
>    settings, or keys, for other programs.
>
> Step 9. Read the file once, if it is there, and tell me what is in it. Then show me what you would write: an entry named
>    `stackvet`, with StackVet's command and arguments exactly as printed, in your app's format, added to what is
>    already there and keeping all of it. Write it only after I say yes, and then read it back once to check it.
>    Paste the `stackvet` entry you wrote into this chat, and nothing else from the file, so the chat keeps a record.
>
> Step 10. Tell me what I need to do so you load the new settings, in this app's own words: for some apps it is a
>    restart, for others the new server has to be started or allowed. If you are not sure how this app does it, say
>    so, rather than guessing. Then tell me to paste prompt 4, "Check that it's connected", from the setup page, and
>    which settings file you wrote, since a new chat may not remember it.
<!-- setup-prompt:end -->

### On Windows

On Windows, StackVet is installed on the computer itself, with Scoop, rather than run in Docker. Nothing is built,
and the checks that read your app's code need no Docker at all. The checks that start your app (`sv report --run`)
have not been tried on a Windows computer, even with Docker Desktop running: without them, the report says which
requirements only they reach, and a Mac or Linux computer, or a cloud runner, is the known way to run them.

<!-- setup-windows:start -->
> Please set up StackVet in this folder so you can use its tools while we build. On Windows it is installed with
> Scoop, a free installer, and runs on this computer, and you will use it as an MCP server. Use PowerShell in your terminal. Work through these steps in
> order. Ask me before you install anything, and before you change anything outside this folder. If a step fails or
> you are unsure, stop and tell me what happened in plain words: do not carry on as though StackVet were set up.
>
> Step 1. Check that this is Windows. If it is a Mac or Linux, stop: there is a version of this prompt for those, and I
>    will use that one instead.
>
> Step 2. Check that Scoop is installed with `scoop --version`. If it is not, tell me what Scoop is, and ask me before
>    installing it the way https://scoop.sh says, with these two commands:
>    `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser` and
>    `Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression`. Tell me first that the first one lets this
>    computer run scripts I have chosen to download, for my own account only.
>
> Step 3. Install StackVet: `scoop bucket add stackvet https://github.com/abbyshade111/scoop-stack-vet`, then
>    `scoop install stackvet/sv`.
>
> Step 4. Check it with `stackvet --version`. In PowerShell, always type `stackvet`, never `sv`: PowerShell already uses
>    `sv` as a short name for one of its own commands, so `sv` there does nothing visible.
>
> Step 5. If this folder is not a git repository yet, run `git init` here.
>
> Step 6. Print StackVet's MCP server for this folder: `stackvet connect`. It prints the server's name, `stackvet`, that it
>    talks over standard input and output, and its command and arguments.
>
> Step 7. Find out, from your own app's documentation, how this app is given a local MCP server that talks over standard
>    input and output: which settings file, where it is, and its format. If you are not sure, say so and stop,
>    rather than guessing.
>
> Step 8. If that file is in this folder, good. If it is one file for my whole account, tell me which file, and that
>    StackVet will then be set up for this folder only, and ask me before you read it or change it: it may hold
>    settings, or keys, for other programs.
>
> Step 9. Read the file once, if it is there, and tell me what is in it. Then show me what you would write: an entry named
>    `stackvet`, with StackVet's command and arguments exactly as printed, in your app's format, added to what is
>    already there and keeping all of it. Write it only after I say yes, and then read it back once to check it.
>    Paste the `stackvet` entry you wrote into this chat, and nothing else from the file, so the chat keeps a record.
>
> Step 10. Tell me what I need to do so you load the new settings, in this app's own words: for some apps it is a
>    restart, for others the new server has to be started or allowed. If you are not sure how this app does it, say
>    so, rather than guessing. Then tell me to paste prompt 4, "Check that it's connected", from the setup page, and
>    which settings file you wrote, since a new chat may not remember it.
>
> Step 11. Tell me that on Windows, StackVet's checks that start the app (`stackvet report --run`) have not been tried
>    yet: the checks that read the code work here, and the report will say what the others would have reached. Don't
>    promise me the full report on this computer.
<!-- setup-windows:end -->

## 4. Check that it's connected

After your AI tool has loaded the new settings, in the same folder. If it is a new chat, add the settings file prompt
3 wrote, which it told you, where this prompt says.

<!-- setup-step-4:start -->
> Which `stackvet_` tools can you call? List their names. If you can call `stackvet_status`, call it with this
> folder's full path (`pwd` prints it) and tell me in plain words what it says is ready and what is not, and which
> folder it says it serves. If that is not this folder, or it says this folder is not inside the folder the server was
> started for, a StackVet server started for an earlier app is still running: on a Mac or Linux, show me what
> `docker ps` lists, stop the old StackVet container only after I say yes, and tell me to restart you. If you can't
> call any `stackvet_` tools, StackVet is not
> connected: say so, and check two things. First, the settings file prompt 3 wrote, which is
> *(the file prompt 3 named, if you know it)*: read that one file, and don't search my computer for others. Second,
> on a Mac or Linux, whether Docker is running, with `docker info`. Your own commands may be kept away from Docker
> even when it is running, so if `docker info` fails, tell me what it said and ask me, rather than telling me to
> start Docker.
<!-- setup-step-4:end -->

**Already have StackVet installed** (for example with Homebrew)? Then its server can start your installed copy instead
of Docker: `sv connect` in the app's folder prints it, by your installed copy's own path, and your AI tool adds it the
same way as in prompt 3.

## 5. Start building

Once your AI tool lists the `stackvet_` tools, you are set up. Write a few sentences about the app you want where the
prompt says, and paste it into the same chat. It is the prompt in step 4 of [the guide](../GETTING-STARTED.md), word for
word.

<!-- setup-step-5:start -->
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
<!-- setup-step-5:end -->

What happens next:

- **It writes `stackvet.toml` first.** That file says what your app is and does, and StackVet decides which security
  rules apply from it. `stackvet_status` said it was missing; that is expected until now.
- **It builds a feature at a time, and checks each one with StackVet.** When it fixes something StackVet found, it
  checks again.
- **It asks you questions only you can answer**, such as who may see what, or how long someone may stay signed in.
  "I'm not sure" is a fine answer.
- **When the first version works, ask it for the report:** "Call `stackvet_write_report`, then tell me where the report
  is and walk me through it." It writes a folder called `stackvet-report` in your app's folder; open `report.html` in
  it in your web browser. It starts with the most important things to fix. The report your AI tool writes covers your
  code; the outside security tools and the checks that start your app run in the full report, which prompt 6 walks
  you through.
- **If you ask it to push to GitHub, say so in these words:** "push it, and do not bypass any branch protection or
  ruleset; if the only way to push is to bypass one, stop and ask me first." Better still, ask it for a pull request,
  and merge it yourself.

## 6. Make the full report

The report your AI tool writes through StackVet's tools reads your code. The full report also starts your app in a
container of its own, with no internet, tries things against it (signed in as test users too, when `stackvet.toml`
names them), and runs the security
tools each language already has, such as Semgrep and Bandit. It is made at the terminal, by StackVet installed on your
computer itself; your AI tool can walk you through it. Paste this into the same chat, in your app's folder:

<!-- setup-step-6:start -->
> I'd like the full StackVet report for this app: the one that starts the app in its own container, with no
> internet, and runs the security tools each language has. Your `stackvet_` tools can't make it: it is made at the
> terminal, by StackVet installed on this computer itself. StackVet's page on GitHub (`docs/INSTALL.md`, the
> section "Installing StackVet on your computer, for `--run`") says how. Walk me through it one step at a time, and
> ask me before you install, update, or run anything. If a command fails with "operation not permitted", or your own
> sandbox keeps it from running, tell me, and run it outside your sandbox only after I say yes.
>
> Step 1. Check whether StackVet is installed on this computer: `sv --version` on a Mac or Linux, `stackvet --version` in
>    PowerShell. If it is not, install it the way the guide says for this computer, and tell me what each step does
>    and how long it takes before you run it. If you find an older or kept copy of StackVet, or a link to one, tell
>    me, and install with Homebrew (Scoop on Windows) unless I say otherwise.
>
> Step 2. If it is installed, compare the commit `sv --version` prints with the latest on StackVet's GitHub page, and tell
>    me how old it is: StackVet never checks for itself. If it is older, offer to update it: `brew upgrade --fetch-HEAD
>    sv` if Homebrew installed it, `scoop update sv` on Windows, or the guide's steps for a copy built by hand. After
>    installing or updating, `sv --version` must name the commit you expected, the latest on StackVet's GitHub page:
>    if it does not, stop and tell me.
>
> Step 3. On a Mac or Linux, your `stackvet_` tools use a second copy, StackVet's container image. Offer to fetch its latest
>    with `docker pull ghcr.io/abbyshade111/stackvet-sv`. Don't stop or restart any container for it: a new chat with
>    you starts the new one.
>
> Step 4. In this folder, run `sv doctor` (`stackvet doctor` in PowerShell) and tell me in plain words what is ready for the
>    full report and what is not. If it says Docker is installed but this shell cannot reach it, your commands are
>    kept from Docker, and nothing needs installing: ask me to allow it, or to run `sv doctor` in my own terminal.
>
> Step 5. On Windows, tell me first that this step has not been tried on a Windows computer, and that a Mac or Linux
>    computer is the known way to run it. Ask me, then run `sv report . --run` here (`stackvet report . --run` in
>    PowerShell). It starts the app in a
>    container with no internet, tries things against it (signed in as test users too, when stackvet.toml names
>    them), and runs the security tools that
>    are installed; Semgrep downloads its rules from semgrep.dev as it runs. It takes several minutes. It needs Docker:
>    if your own commands are kept from Docker, say so and ask me to allow it, or tell me the command to type myself.
>    Never give StackVet's container access to Docker, and never run commands inside a container.
>
> Step 6. Tell me where the report is, and open `report.html` in this computer's own web browser for me (`open` on a Mac,
>    `start` in PowerShell, `xdg-open` on Linux): not in a browser pane inside this tool, which closes with it, and
>    not in this chat, where it shows as code. Walk me through it from the top: the
>    most important things to fix first, then what was not examined. Never tell me the app is secure. Ask me before
>    you delete the report's folder, or anything else.
>
> Step 7. Last, tell me that until I run `sv review` myself (prompt 7 on the setup page), every answer recorded so far
>    counts as yours, not mine, and the report says so.
<!-- setup-step-6:end -->

**If you stop here,** every answer your AI tool recorded still counts as its word, not yours, and the report says so,
until you run `sv review` in step 7.

## 7. Record my answers as mine

**Your answers count as yours only once you record them yourself.** The report shows the answers your AI tool wrote
as the tool's word, even when it was writing down what you said, until you record them with `sv review` in your app's
folder (`stackvet review` in PowerShell). It asks your name, shows each answer, and signs the ones you confirm with a
key of your own, which you can protect with a passphrase. It runs only in a terminal you type in, so your AI tool
cannot run it for you. Your AI tool can still
walk you to it. Not yet tried in any AI tool. Paste this into the same chat, in your app's folder:

<!-- setup-step-7:start -->
> Read stackvet.toml and security-notes.md and tell me, one at a time and in plain words, each answer or finding that
> is still only your word or your proposal, and what I should look at before I say it is mine. Don't change either
> file to say an answer is mine: only I can do that, with `sv review`. Then tell me the one command to type in my
> terminal (`sv review .`, or `stackvet review .` in PowerShell), say that you cannot run it for me, and wait. When I
> say I am done, run stackvet_check again and tell me what changed.
<!-- setup-step-7:end -->

When you record something, `sv review` then writes the report again itself, with the options of the last one,
`--run` included, so the answers you recorded count as yours without a second run. Where it cannot do that (`--no-report`,
a report it cannot show it wrote, or a computer that cannot start your app), it says so, and you make the full report
again (prompt 6).

Everything else StackVet can do, from answering its questions to keeping it up to date, is in
[the guide](../GETTING-STARTED.md), from step 5.
