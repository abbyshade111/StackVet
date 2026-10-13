# `sv connect --docker` on Windows, so StackVet can run in Docker there too

**Status:** open


Found on 12 October 2026 while building the setup prompt library (backlog 0370). On Windows, the settings that start
StackVet in Docker cannot be made today, for two reasons:

- `sv connect --docker` refuses any Docker path that does not start with `/` (`crates/sv-cli/src/connect.rs`), and
  Docker on Windows is `C:\Program Files\Docker\...\docker.exe`.
- Under that, the settings show the app's folder inside StackVet's box at the same path (`-v FOLDER:FOLDER`), and a
  Windows path such as `C:\Users\you\app` is not a path inside a Linux container.

**The owner's decision, 12 October 2026:** the Windows route is StackVet installed with Scoop and connected with
`stackvet connect TOOL` (no Docker), and this fix waits for later. That route also runs the outside tools, which `sv`
runs from the computer, not from Docker, wherever each is installed on Windows.

**What the fix would take**, when it is wanted: accept a Windows Docker path; show the folder inside the box at a
Linux-style path (for example `/c/Users/you/app`); and translate what StackVet hands back, the report's links and the
paths in findings, to the `C:\...` form the AI tool on Windows can open, or every link in a report points at a path
that is not there. Tried only by someone with a Windows computer and Docker Desktop set to Linux containers, since
GitHub's Windows runners have Windows containers alone. The `--run` checks are a separate question: they start the
app's containers themselves, already give Docker the short `C:\` form of a path, and are untried on Windows by hand.
