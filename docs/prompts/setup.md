# Setting StackVet up with your AI coding tool

These prompts are for someone who has used an AI chat but never built an app with one. You copy each prompt into
your AI coding tool, one at a time, and the tool does the setup with you: it looks at what your computer already has,
tells you what to install and where to get it, and asks before it installs or changes anything. If a step does not
work, it stops and tells you what happened in plain words.

If you have used a terminal or Docker before, [the guide](../GETTING-STARTED.md) has the same steps as commands.

**Tried so far:** in Antigravity, on a Mac, on 13 October 2026, from an empty folder to StackVet connected (prompts 1
to 4). That trial is where most of these words come from; prompt 3 was tried in a wording that borrowed one app's
settings for StackVet's server, and the form below, which asks StackVet for it in no app's format, came out of it.
Tests hold the prompts to the image name StackVet is published under, to the options `sv connect` takes, and to the
Scoop commands that install it on Windows (`crates/sv-cli/tests/connect.rs`, `crates/sv-cli/tests/setup_prompts.rs`).
If anything here goes wrong, the guide has the same steps as commands. The website names only the AI tools these
prompts have really been tried in.

## Before the first prompt: an AI tool that can run commands

A chat window that only writes answers cannot do this. You need an AI coding tool that can run commands on your
computer, in what it calls a terminal, asks you before it does, and can use MCP servers: MCP is a standard way for an
AI tool to use another program, and it is how your tool uses StackVet. Many tools can. These prompts name none of
them: prompt 3 asks your tool how its own app adds an MCP server, and has it add StackVet's.

Open your AI tool in a folder for your app: an empty folder called `my-app` in your home folder is a good one. If you
have not made one yet, prompt 2 offers to make it with you. Then go through the prompts in order.

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

## 1. Look at my computer

<!-- setup-step-1:start -->
> I'm new to this, so please explain things in plain words as we go. I want to set up StackVet, a tool that checks
> an app's security, so you can use it while we build an app together. StackVet is free and its code is public at
> https://github.com/abbyshade111/StackVet: its README and its guide (`docs/GETTING-STARTED.md` there) say what it
> needs and how it is set up, so read them there whenever you need to know something about StackVet. Read them, but
> don't run anything they say yet. For now, only look: don't install or change anything. Using your terminal, find
> out:
>
> 1. Which kind of computer this is: Mac, Windows, or Linux.
> 2. Whether git is installed (`git --version`).
> 3. On a Mac or Linux: whether Docker is installed (`docker --version`) and running (`docker info`). Either Docker
>    Desktop or Colima counts: both give the same `docker` command. If Colima is installed, check
>    `colima status` too.
> 4. On Windows: whether Scoop is installed (`scoop --version`).
> 5. Which folder you are working in, whether it is empty, and whether it looks like a folder made for an app, or
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
> a scratch, temporary, or settings folder of yours), offer to make one called `my-app` in my home folder. Ask
> before you make it. Then tell me, step by step, how to open that folder in the app I'm using now, in that app's
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
> 1. Tell me which computer this is (Mac, Linux, or Windows). If it is Windows, stop: there is a Windows version of
>    this prompt, and I will use that one instead.
> 2. Check that Docker is installed and running with `docker info` (Docker Desktop or Colima: either works). If it
>    is not installed, tell me where to get Docker Desktop or Colima and wait for me. If it is installed but not
>    running, ask me to start it (with Colima, ask before running `colima start`) and wait.
> 3. Fetch StackVet: `docker pull ghcr.io/abbyshade111/stackvet-sv`
> 4. If this folder is not a git repository yet, run `git init` here.
> 5. Find docker's full path with `command -v docker`.
> 6. Print StackVet's MCP server for this folder, putting docker's full path from step 5 after `--docker`:
>    `docker run --rm --network none -v "$PWD":"$PWD" -w "$PWD" ghcr.io/abbyshade111/stackvet-sv connect --docker DOCKER_PATH`
>    On Linux, also add `--user "$(id -u):$(id -g)"` at the end, so the files StackVet writes are mine. It prints the
>    server's name, `stackvet`, that it talks over standard input and output, and its command and arguments.
> 7. Find out, from your own app's documentation, how this app is given a local MCP server that talks over standard
>    input and output: which settings file, where it is, and its format. If you are not sure, say so and stop,
>    rather than guessing.
> 8. If that file is in this folder, good. If it is one file for my whole account, tell me which file, and that
>    StackVet will then be set up for this folder only, and ask me before you read it or change it: it may hold
>    settings, or keys, for other programs.
> 9. Read the file once, if it is there, and tell me what is in it. Then show me what you would write: an entry named
>    `stackvet`, with StackVet's command and arguments exactly as printed, in your app's format, added to what is
>    already there and keeping all of it. Write it only after I say yes, and then read it back once to check it.
> 10. Tell me what I need to do so you load the new settings, in this app's own words: for some apps it is a
>    restart, for others the new server has to be started or allowed. If you are not sure how this app does it, say
>    so, rather than guessing. Then tell me to paste prompt 4, "Check that it's connected", from the setup page, and
>    which settings file you wrote, since a new chat may not remember it.
<!-- setup-prompt:end -->

### On Windows

On Windows, StackVet is installed on the computer itself, with Scoop, rather than run in Docker. Nothing is built,
and the checks that read your app's code need no Docker at all. (Docker Desktop is needed later only for the checks
that start your app.)

<!-- setup-windows:start -->
> Please set up StackVet in this folder so you can use its tools while we build. On Windows it is installed with
> Scoop, a free installer, and runs on this computer, and you will use it as an MCP server. Use PowerShell in your terminal. Work through these steps in
> order. Ask me before you install anything, and before you change anything outside this folder. If a step fails or
> you are unsure, stop and tell me what happened in plain words: do not carry on as though StackVet were set up.
>
> 1. Check that this is Windows. If it is a Mac or Linux, stop: there is a version of this prompt for those, and I
>    will use that one instead.
> 2. Check that Scoop is installed with `scoop --version`. If it is not, tell me what Scoop is, and ask me before
>    installing it the way https://scoop.sh says, with these two commands:
>    `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser` and
>    `Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression`. Tell me first that the first one lets this
>    computer run scripts I have chosen to download, for my own account only.
> 3. Install StackVet: `scoop bucket add stackvet https://github.com/abbyshade111/scoop-stack-vet`, then
>    `scoop install stackvet/sv`.
> 4. Check it with `stackvet --version`. In PowerShell, always type `stackvet`, never `sv`: PowerShell already uses
>    `sv` as a short name for one of its own commands, so `sv` there does nothing visible.
> 5. If this folder is not a git repository yet, run `git init` here.
> 6. Print StackVet's MCP server for this folder: `stackvet connect`. It prints the server's name, `stackvet`, that it
>    talks over standard input and output, and its command and arguments.
> 7. Find out, from your own app's documentation, how this app is given a local MCP server that talks over standard
>    input and output: which settings file, where it is, and its format. If you are not sure, say so and stop,
>    rather than guessing.
> 8. If that file is in this folder, good. If it is one file for my whole account, tell me which file, and that
>    StackVet will then be set up for this folder only, and ask me before you read it or change it: it may hold
>    settings, or keys, for other programs.
> 9. Read the file once, if it is there, and tell me what is in it. Then show me what you would write: an entry named
>    `stackvet`, with StackVet's command and arguments exactly as printed, in your app's format, added to what is
>    already there and keeping all of it. Write it only after I say yes, and then read it back once to check it.
> 10. Tell me what I need to do so you load the new settings, in this app's own words: for some apps it is a
>    restart, for others the new server has to be started or allowed. If you are not sure how this app does it, say
>    so, rather than guessing. Then tell me to paste prompt 4, "Check that it's connected", from the setup page, and
>    which settings file you wrote, since a new chat may not remember it.
<!-- setup-windows:end -->

## 4. Check that it's connected

After your AI tool has loaded the new settings, in the same folder. If it is a new chat, add the settings file prompt
3 wrote, which it told you, where this prompt says.

<!-- setup-step-4:start -->
> Which `stackvet_` tools can you call? List their names. If you can call `stackvet_status`, call it and tell me in
> plain words what it says is ready and what is not. If you can't call any `stackvet_` tools, StackVet is not
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
> 1. Call `stackvet_spec` and write `stackvet.toml` for this app, from what it will really do.
>    Its capability lines start commented out: answer each one you can with true or false, and
>    **leave a line commented out rather than guessing `false`**. A line left out is reported as not
>    assessed; a wrong `false` switches whole sets of checks off.
> 2. Call `stackvet_guidance` and follow the rules it gives while you write code. Call it again
>    with a topic before work in that area: adding a package, a CI workflow, anything with keys.
>
> Then, as we build:
> 3. After each feature, call `stackvet_check`. Read what it says was not examined first. Fix what
>    it finds that is real. If a finding looks wrong, tell me instead of rewriting working code to
>    make it go away.
> 4. Never tell me the app is secure. Tell me what was checked and what was not.
> 5. When the first version works, call `stackvet_check` with section `questions` and ask me the
>    questions one at a time. Record my answers with `stackvet_record_answer`, and only what I
>    actually answer as mine.
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
  code; the outside security tools and the checks that start your app run when you make the report at a terminal, as
  [the guide](../GETTING-STARTED.md) says.
- **If you ask it to push to GitHub, say so in these words:** "push it, and do not bypass any branch protection or
  ruleset; if the only way to push is to bypass one, stop and ask me first." Better still, ask it for a pull request,
  and merge it yourself.

Everything else StackVet can do, from answering its questions to keeping it up to date, is in
[the guide](../GETTING-STARTED.md), from step 5.
