# Setting StackVet up with your AI coding tool

These prompts are for someone who has used an AI chat but never built an app with one. You copy each prompt into
your AI coding tool, one at a time, and the tool does the setup with you: it looks at what your computer already has,
tells you what to install and where to get it, and asks before it installs or changes anything. If a step does not
work, it stops and tells you what happened in plain words.

If you have used a terminal or Docker before, [the guide](../GETTING-STARTED.md) has the same steps as commands.

**Tried so far:** not yet in any AI coding tool. Tests hold the prompts to the image name StackVet is published under,
to the options `sv connect` takes, and to the Scoop commands that install it on Windows
(`crates/sv-cli/tests/connect.rs`, `crates/sv-cli/tests/setup_prompts.rs`). Until someone has gone through them from an
empty folder, follow the guide if anything here goes wrong. The website names only the AI tools these prompts have
really been tried in.

## Before the first prompt: an AI tool that can run commands

A chat window that only writes answers cannot do this. You need an AI coding tool that can run commands on your
computer, in what it calls a terminal, and asks you before it does. Claude Code (in the Claude desktop app, or on its
own), Cursor, and GitHub Copilot's agent mode in VS Code all can, and StackVet has settings ready for each. Another tool
that can run commands may work too, but StackVet has no settings ready for it yet, and prompt 3 stops and says so.

Make an empty folder for your app, for example a folder called `my-app` in your home folder, and open it in your AI
tool. Then go through the prompts in order.

| Prompt | What it does | Changes anything? |
|---|---|---|
| 1. Look at my computer | Says what is ready and what is missing | No |
| 2. Help me install what's missing | Walks you through each install, one at a time | Only with your yes, each time |
| 3. Set up StackVet here | Fetches StackVet and connects it to your AI tool. One version for a Mac or Linux, one for Windows | Only this folder, and installs with your yes |
| 4. Check that it's connected | Asks StackVet itself whether everything is ready | No |

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
> 5. Which folder you are working in, and whether it is empty.
>
> Then tell me, as a short list, what is ready and what is missing, and which of the next prompts I'll need. Don't
> fix anything yet.
<!-- setup-step-1:end -->

## 2. Help me install what's missing

Skip this one if prompt 1 said nothing was missing.

<!-- setup-step-2:start -->
> Please help me install what you found missing, one thing at a time. For each one:
>
> - Tell me in one sentence what it is and why StackVet needs it.
> - Give me its official download page, and only that one: Docker Desktop at https://www.docker.com/products/docker-desktop/,
>   Colima at https://github.com/abiosoft/colima, git at https://git-scm.com/downloads, Scoop at https://scoop.sh.
> - For Docker on a Mac or Linux, tell me there are two choices that both work with StackVet: Docker Desktop, an app
>   with a window, which is the easier one to start with; or Colima, which is smaller and runs from the terminal. Let
>   me choose. On Windows, StackVet does not need Docker for this setup, so skip it there.
> - If I have to download and click through an installer, tell me what to click, step by step, and wait for me to say
>   I'm done.
> - If you can install it with a command instead, show me the command and what it will do, and run it only after I
>   say yes. Tell me first if it will ask for my computer's password or for administrator permission.
>
> After each one, check that it worked. For Docker, it has to be running, not only installed: check with `docker info`,
> and if it is not running, ask me to open Docker Desktop and wait, or, with Colima, ask before running
> `colima start`. If Docker Desktop asks which kind of containers to
> use, tell me to keep Linux containers. If anything fails, stop and tell me plainly what happened. Don't go on to the
> next thing until this one works.
<!-- setup-step-2:end -->

## 3. Set up StackVet here

Use the version for your computer.

### On a Mac or Linux

<!-- setup-prompt:start -->
> Please set up StackVet in this folder so you can use its tools while we build. StackVet runs in Docker. Work
> through these steps in order, using your terminal. Ask me before you install anything, and before you change
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
> 6. Print the settings for yourself. Use `claude` if you are Claude, `vscode` if you are GitHub Copilot in VS Code,
>    or `cursor` if you are Cursor, and put docker's full path from step 5 after `--docker`. If you are none of
>    these, stop and tell me which tool you are: StackVet has no settings ready for it yet.
>    `docker run --rm --network none -v "$PWD":"$PWD" -w "$PWD" ghcr.io/abbyshade111/stackvet-sv connect TOOL --docker DOCKER_PATH`
>    On Linux, also add `--user "$(id -u):$(id -g)"` at the end, so the files StackVet writes are mine.
> 7. It names the settings file to put this in (`.mcp.json`, `.vscode/mcp.json`, or `.cursor/mcp.json`). Show me
>    the file, then write it into this folder exactly as printed. If the file already exists, show me both and ask
>    before changing it.
> 8. Tell me to restart you (or, in VS Code, to click **Start** above `stackvet` in `.vscode/mcp.json`), and that
>    the next prompt is "Check that it's connected".
<!-- setup-prompt:end -->

### On Windows

On Windows, StackVet is installed on the computer itself, with Scoop, rather than run in Docker. Nothing is built,
and the checks that read your app's code need no Docker at all. (Docker Desktop is needed later only for the checks
that start your app.)

<!-- setup-windows:start -->
> Please set up StackVet in this folder so you can use its tools while we build. On Windows it is installed with
> Scoop, a free installer, and runs on this computer. Use PowerShell in your terminal. Work through these steps in
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
> 6. Print the settings for yourself, in this folder. Use `claude` if you are Claude, `vscode` if you are GitHub
>    Copilot in VS Code, or `cursor` if you are Cursor: `stackvet connect TOOL`. If you are none of these, stop and
>    tell me which tool you are: StackVet has no settings ready for it yet.
> 7. It names the settings file to put this in (`.mcp.json`, `.vscode/mcp.json`, or `.cursor/mcp.json`). Show me
>    the file, then write it into this folder exactly as printed. If the file already exists, show me both and ask
>    before changing it.
> 8. Tell me to restart you (or, in VS Code, to click **Start** above `stackvet` in `.vscode/mcp.json`), and that
>    the next prompt is "Check that it's connected".
<!-- setup-windows:end -->

## 4. Check that it's connected

After your AI tool has restarted, in the same folder:

<!-- setup-step-4:start -->
> Which `stackvet_` tools can you call? List their names. If you can call `stackvet_status`, call it and tell me in
> plain words what it says is ready and what is not. If you can't call any `stackvet_` tools, StackVet is not
> connected: say so, and check the settings file you wrote, and, on a Mac or Linux, that Docker (or Colima) is
> running.
<!-- setup-step-4:end -->

If your AI tool lists the `stackvet_` tools, you are set up. Go on to step 4 of [the guide](../GETTING-STARTED.md),
"Start the build with this prompt", which is where you describe the app you want.

**Already have StackVet installed** (for example with Homebrew)? Then the settings can use it directly instead of
Docker: `sv connect claude` (or `vscode`, or `cursor`) in the app's folder prints them, starting your installed copy by
its own path.
