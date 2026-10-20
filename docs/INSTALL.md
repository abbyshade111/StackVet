# Installing StackVet on your computer, and keeping it up to date

The guide's steps 1 to 5 ([`GETTING-STARTED.md`](GETTING-STARTED.md)) need none of this: StackVet runs in Docker,
started by your AI coding tool. This page is for the day you want the checks of the running app
(`sv report --run`), which need StackVet installed on your computer itself, and for keeping either copy up to date.

## Installing StackVet on your computer, for `--run`

You only need this for `sv report --run`. Everything in steps 1 to 5 keeps working through Docker as it
is, and your AI tool keeps using the container: the copy you build here is for typing at a terminal.

On Windows there is a ready-made download, installed with Scoop (below). On a Mac or Linux you build it from its
source code (the program written out as text, which a builder turns into a program you can run), with Homebrew or by
hand. It is done once and takes a few commands. The hand steps were tried on a Mac on 5 October 2026, from a fresh
copy of StackVet. The Linux steps have not been tried by hand, though StackVet is built on Linux every time its code
changes.

**On Windows: Scoop.** [Scoop](https://scoop.sh) is a free installer for Windows programs. StackVet's Windows program
is published with each release, zipped with its data and signed where GitHub built it, and Scoop downloads it, checks
it against the published checksum, and puts it on your path. Nothing is built on your computer. In PowerShell:

```powershell
scoop bucket add stackvet https://github.com/abbyshade111/scoop-stack-vet
scoop install stackvet/sv
stackvet --version
```

**In PowerShell, type `stackvet` wherever this guide says `sv`.** PowerShell already uses `sv` as a short name for
one of its own commands (`Set-Variable`), so plain `sv` there does nothing visible. Scoop installs the program under
both names; `sv.exe` works too, and in the older Command Prompt plain `sv` does. `scoop update sv` updates it.

This install is tried automatically on a Windows computer at GitHub every week and on every change to the bucket
([github.com/abbyshade111/scoop-stack-vet](https://github.com/abbyshade111/scoop-stack-vet)): Scoop installs the
latest release and it checks an example app. It has not yet been tried by hand on a person's own Windows computer.
For `--run`, Docker Desktop must be set to Linux containers (its default); if it is set to Windows containers,
StackVet says so and what to change, and the run is reported as not assessed. That has not been tried by hand on
Windows either.

**The quickest way, on a Mac or on Linux: Homebrew.** If you have [Homebrew](https://brew.sh), one command builds
StackVet on your computer, fetching what the build needs by itself, and puts `sv` on your path with its data beside
it. The first install takes a few minutes:

```bash
brew install --HEAD abbyshade111/stackvet/sv
sv --version
```

`--HEAD` means the latest StackVet, built from `main`, until the formula moves to a release newer than v0.1.0;
`brew upgrade --fetch-HEAD sv` updates it.
This is tried automatically on a Mac and on Linux every week and on every change to the formula
([github.com/abbyshade111/homebrew-stackvet](https://github.com/abbyshade111/homebrew-stackvet)), and two AI coding
tools installed it this way on a Mac in the trial of 14 October 2026; it has not yet been tried by hand on Linux. If it fails, the steps below do the same thing by hand.

**1. The basic tools.** On a Mac, in a terminal:

```bash
xcode-select --install
```

This installs Apple's command-line tools, which include `git` and the parts Rust needs to finish
building a program. If it says they are already installed, go on. On Linux (Debian or Ubuntu):
`sudo apt install build-essential git curl`.

**2. Install Rust.** Rust is the programming language StackVet is written in. Installing it gives you
`cargo`, the program that builds StackVet. The official installer, from rust-lang.org:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When it asks how to install, press Enter for the standard choice. Then close the terminal window and
open a new one, so it sees what was installed, and type `cargo --version`. It should print a version
number: StackVet needs 1.95 or newer. If yours is older, `rustup update` brings it up to date.

**3. Get StackVet's source code**, into a folder named `stackvet` in your home folder:

```bash
cd ~
git clone https://github.com/abbyshade111/StackVet.git stackvet
```

Or, without `git`: on StackVet's GitHub page, choose **Code**, then **Download ZIP**, unzip it,
rename the folder it makes (`StackVet-main`) to `stackvet`, and move it into your home folder.
Built that way, `sv --version` says `commit unknown` rather than which version of the code it is, and
updating means downloading it again.

**4. Build and install it:**

```bash
cd ~/stackvet
sh tools/install.sh
```

The first time, `cargo` downloads the pieces StackVet is made from, and the build takes a few
minutes. Then the script copies the program and the files it reads into a folder of their own,
`~/.local/share/stackvet`, and puts a link to the program at `~/.local/bin/sv`. It ends by printing
the version and `Installed.` If `~/.local/bin/sv` is already there and is not its own link, it stops
and says so rather than replace it.

**5. Let the terminal find it.** When you type a command, the terminal looks for it in a list of
folders called your `PATH`. This adds the folder with the link to that list, for every terminal you
open from now on. On a Mac:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
```

On Linux, the same line with `~/.bashrc` at the end instead of `~/.zshrc`. Then close the terminal
window and open a new one.

**6. Check it:**

```bash
sv --version
sv check ~/stackvet/examples/flask-booking
```

The first prints `sv 0.1.0` and the version of the code it was built from, and on a second line the
folder of files it reads (`data: …/.local/share/stackvet/data`). The second checks one of the example apps that come with it: it
should say what it read and what it found, not `Error`. If it says `command not found: sv`, step 5 has
not taken effect: open a new terminal window, or look for the line at the end of `~/.zshrc`.

Then, in your app's folder, `sv doctor` says in one line each whether everything `--run` needs is in
place: the folder in git, `stackvet.toml` there and readable, how to start the app written in it, and
Docker running. It writes nothing and opens no network connection, so it cannot say whether a newer
StackVet is out.

**The installed copy does not need the `stackvet` folder.** Each time StackVet runs, it reads more
than a dozen of its own files (the security standards and its rules). The installed copy reads the ones
the script put beside it, so moving, renaming, or deleting the folder you built it in does not stop it.
Use `~/.local/bin/sv` wherever a full path is asked for, as in your AI tool's settings; it stays the same
when you build again. A copy of the program on its own, without its `data` folder beside it, falls back to
the folder it was built in; once that is gone, it cannot find those files and says where it looked.

**The build folder can be deleted.** Building leaves a folder named `target` inside `~/stackvet`, of 1 to 7 GB,
which the installed copy does not use. To get the space back:

```bash
rm -rf ~/stackvet/target
```

That removes only the build's leftovers: `sv` keeps working, and so does your AI tool's link to it. The next
time you update StackVet, the build takes its few minutes again, as it did the first time.

**Docker or Colima has to be running** for `--run`, as in [step 1 of the guide](GETTING-STARTED.md#1-install-docker-and-start-it), because that is what
starts your app. With Colima on a Mac, your app's folder has to be inside your home folder (Colima
shares only that unless you tell it otherwise); if it is not, the report says so and why.

Your AI tool tells you the exact command to type. It looks like this, with your own app folder:

```bash
sv report /Users/you/code/my-app --run
```

To update StackVet later: `cd ~/stackvet`, then `git pull`, then `sh tools/install.sh` again. It
replaces the program and its files together.

## An image of your own

`install = true` downloads Python packages (from `requirements.txt`) and Node packages (from `package.json`)
only. An app in another language, or one that needs something else installed first, can instead start from
an image of its own: a ready-made box that already holds the language and the packages. You make it once,
with Docker, and name it in `stackvet.toml`.

Do not put the download in `build` (for example `build = "pip install -r requirements.txt"` or
`build = "npm ci"`). The build step runs inside the same fence as the app, with no internet, so it fails, and
`sv preflight` says so before you try.

To make the image, ask your AI tool to write a file named `Dockerfile` in the app's folder that starts from the
language's own image and installs the packages, and nothing more. For a Python app it reads:

```
FROM python:3.12-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
```

Then, in a terminal in the app's folder, run `docker build -t my-app-packages .` (any name will do), and in
`stackvet.toml` name it under `[stack.run]`: `image = "my-app-packages"`, with no `install = true` line. StackVet
still puts the app's own files in and runs it behind the fence; the image only brings the packages. Build the
image again whenever the list of packages changes, since StackVet runs whatever the image holds.

## Keeping StackVet up to date

StackVet changes often, and a new container image is published each time a change is added to it.
Your computer keeps the copy it fetched until you fetch again. To get the newest, in a terminal:

```bash
docker pull ghcr.io/abbyshade111/stackvet-sv
```

**Then restart your AI tool, or at least StackVet inside it.** The tool starts StackVet's container
when it starts the StackVet server, and keeps the one it started until the server starts again. In the
Claude desktop app, quit it and open it again; in Claude Code, end the session and start a new one. In VS Code, open the Command Palette (Cmd+Shift+P on a
Mac, Ctrl+Shift+P elsewhere), choose "MCP: List Servers", pick `stackvet`, and choose Restart.

**To see which version you have**, ask the copy itself:

```bash
docker run --rm ghcr.io/abbyshade111/stackvet-sv --version
```

It prints a line such as `sv 0.1.0 (commit 3f9c…)`, the commit being 40 letters and digits long: it is
the exact version of StackVet's code the copy was built from, and is what to quote if you report a problem. If you also built StackVet on
your computer (for `--run`, above), `sv --version` prints the same line for that copy.

**Keep the two at the same version.** The container your AI tool uses and a copy built on your
computer are updated separately, and two versions can disagree: a check one of them has, or a fix, the
other may not. When you update one, update the other: `docker pull` as above, and for the copy on your
computer, `cd ~/stackvet`, then `git pull`, then `sh tools/install.sh`. Compare the two `--version`
lines afterwards. They can still differ for a short while after a change, because the image is
published only once the change has passed its tests.

**To stay on one version on purpose**, for example while you finish an app, use the commit's own
image instead of the newest. Every image is also published under the commit it was built from, so
`ghcr.io/abbyshade111/stackvet-sv:` followed by the whole commit that `--version` prints always
means the same version. Put that name in place of
`ghcr.io/abbyshade111/stackvet-sv` in your `.mcp.json` and in the commands above, and nothing
changes until you change it back.

**Old copies take up space.** Each `docker pull` that fetches a new version keeps the old one on your
disk, no longer named. `docker image prune` deletes the ones no longer named, and asks first; it
does not touch the copy you are using.

## If you set StackVet up before 5 October 2026

The guide had you use the program in the build folder then. If your `PATH` or your AI tool's settings name
`…/stackvet/target/release/sv`, change them to `~/.local/bin/sv`, the installed copy, which keeps working when the
build folder moves or goes.
