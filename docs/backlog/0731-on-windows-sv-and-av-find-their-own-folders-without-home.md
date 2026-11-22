# On Windows, sv and av find their own folders without HOME, from the person's profile folder

**Status:** open

Filed 22 November 2026, at the owner's word ("Yes, file it"), found while building backlog 0728.

**What is wrong.** `sv` and `av` find the folders they keep things in outside an app's folder from `XDG_CONFIG_HOME`,
`XDG_STATE_HOME`, and `XDG_DATA_HOME`, or else from `HOME` (`.config`, `.local/state`, `.local/share` under it). A
Windows account usually sets none of them: PowerShell's `$HOME` is PowerShell's own name for the profile folder, not a
setting a program can read. Read in the code on 22 November 2026, not yet tried on a Windows computer: there, with
neither set, each of these finds no folder:

- `sv`'s config folder (`crates/sv-check/src/seal.rs`, `Key::config_root`): the review key, the list of trusted keys,
  the report key (`crates/sv-cli/src/report_seal.rs`), the switch that turns history on, and the board link.
- History (`crates/sv-cli/src/history.rs`).
- `av`'s records (`crates/av-record/src/hook_record.rs`, `folder`, and its neighbors `checkpoint.rs`, `proxy_seen.rs`,
  `asked.rs`): the hook record, the pass-through's record, the build-loop notes, and the owner's checkpoints.

So `sv` installed with Scoop may have nowhere to keep a review key, and `av report` on Windows nowhere to read a
record. What each says when that happens was not checked; 0728 makes `sv connect --docker --notes` refuse on Windows
with the reason, rather than accept a folder `av` will never read. The MCP server already falls back to `USERPROFILE`
for one thing, the home folder it refuses to serve (`crates/sv-cli/src/mcp/mod.rs`, `Server::new`).

- **To do:** first, on a Windows computer (or the Windows workflow), see what `sv review`, `sv history on`, and `av
  report` say with neither setting, and record it. Then, the owner's choice, asked with a recommendation: fall back to
  `USERPROFILE` when `HOME` is not set (the same `.config`, `.local/state`, `.local/share` under it, so one guide serves
  every system), or to Windows's own folders (`APPDATA` and `LOCALAPPDATA`). Recommended: `USERPROFILE`, since a person
  following the guide on a Mac or Linux finds the same folder names, and `sv connect` already prints those paths.
- **One place for it:** every one of these goes through one function that names the home folder, rather than each
  adding its own fallback.
- **Check:** a test, on every system, that with `HOME` and the `XDG_` settings unset and `USERPROFILE` set, each folder
  above is under `USERPROFILE`; and that with `HOME` set, nothing changes, so no folder on a Mac or Linux moves. On
  Windows, nothing kept moves either: a folder found today was found through `HOME` or an `XDG_` setting, which still
  come first.
- **Then:** `sv connect --docker --notes` on Windows works without the person setting anything, and its refusal goes.
- **Records:** where `sv` keeps things is ADR-057 (history), ADR-034 (the report key), ADR-122 (the hook record), and
  ADR-159 (`av`'s records); `adr_check.py` lists what is owed.
