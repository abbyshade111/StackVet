# `sv connect --docker` on Windows, so StackVet can run in Docker there too

**Status:** claimed by claude/0375, 25 October 2026

**Was queued on the coordination board** (backlog 0273) until the board was removed on 20 October 2026 (ADR-061, Later); it is now taken the normal way, by a claim.


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

**Checked against `main`, 20 October 2026** (the backlog clean-up): still nothing built (`crates/sv-cli/src/connect.rs` refuses a `--docker` path that does not start with `/`). It waits on the owner's word to take it up (ADR-108, Later, 12 October 2026), and on a Windows computer with Docker Desktop to try it.

**Reopened at the owner's word, 25 October 2026:** "reopen 0375 to fix the Windows docker issue(s)". Asked how to try
it, since GitHub's Windows runners have Windows containers alone, the owner chose "Build, you try it": built and
unit-tested here, with a short guide for the owner to try it on Windows, and this item open until that try confirms
it. Asked what it covers, the owner chose "connect --docker only": `sv report --run` on Windows is not part of it.
ADR-153, proposed with this claim, says how.

**Built 25 October 2026** (ADR-153, accepted): `sv connect --docker` takes Docker's Windows path, mounts the folder at
`/c/...` and tells the container the Windows one in `SV_HOST_FOLDER`, and the MCP server turns paths both ways at its
wire (`crates/sv-cli/src/mcp/host_paths.rs`). Tested here with Windows paths given as text; not run on Windows.

**For the owner to try, on a Windows computer with Docker Desktop set to Linux containers:**

1. Install StackVet the Windows way, with Scoop, as `docs/INSTALL.md` says (`scoop bucket add stackvet
   https://github.com/abbyshade111/scoop-stack-vet`, then `scoop install stackvet/sv`), so a Windows copy writes the
   settings: only a copy running on Windows reads Docker's Windows path.
2. Get the image Docker will start: `docker pull ghcr.io/abbyshade111/stackvet-sv`.
3. Find Docker's full path, `where.exe docker` (for example `C:\Program Files\Docker\Docker\resources\bin\docker.exe`),
   and in the app's folder write the settings with it:
   `stackvet connect claude --docker "C:\Program Files\Docker\Docker\resources\bin\docker.exe" > .mcp.json`.
4. Open `.mcp.json`: its `args` should hold `-v C:\...\app:/c/.../app`, `-e SV_HOST_FOLDER=C:\...\app`, and
   `--root /c/.../app`.
5. Open the folder in Claude Code and ask it to run StackVet's status, then a report. The report's links, and the files
   named in findings, should be `C:\...` paths that open.

What would show it working: steps 4 and 5 as said. Anything else, the exact message is what to bring back. This item
is done once the try works.

