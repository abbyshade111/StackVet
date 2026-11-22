# The build-loop notes are named by the computer's path of the app's folder, so --notes works on Windows

**Status:** done, 22 November 2026

Filed 22 November 2026, at the owner's word: recommendation 1 of the investigation for backlog 0719, design entry
0398 ("What sv in its container could gain from its records folder").

`sv connect --docker --notes DIR` lets the MCP server in its container note where an app's build-loop record has got
to, for `av report` on the computer to read (backlog 0716, part 4). It is refused on Windows
(`crates/sv-cli/src/connect.rs`, `check_notes_on`): a note is named by the SHA-256 of the app's path as the process
sees it (`crates/av-record/src/outside_note.rs`, `file_of`), and inside the container that is the Linux-style path
(`/c/Users/you/app`), not the one `av report` names it by on the computer. But on Windows the container is already told
the computer's path, in `SV_HOST_FOLDER` (`connect.rs`, `HOST_FOLDER_ENV`, set only for a Windows folder), and the MCP
server already turns its paths back with it (`crates/sv-cli/src/mcp/host_paths.rs`).

- **To do:** when `SV_HOST_FOLDER` is set, name a note by that path rather than the container's; then offer `--notes`
  on Windows, with `make_notes`'s advice given in a form PowerShell runs (or the folder's making said in words), and
  the refusal's text and the `connect.rs` notes at its top changed to match.
- **The one thing to get right:** both sides must write the path the same way. `av report` on the computer names it
  from its own path of the folder, which on Windows can carry a `\\?\` prefix, another case for the drive letter, or a
  short name (`RUNNER~1`, backlog 0120) that the path `sv connect` printed into `SV_HOST_FOLDER` does not. Settle one
  form both sides reduce a Windows path to, in one function both call, rather than two that agree by accident.
- **Check:** a test takes one real Windows path (`C:\Users\you\app`, and the same folder as `\\?\c:\users\you\app`)
  through both sides and finds the same note file; the Windows workflow runs it. Break it (name by the container's path
  again) and watch the test go red. A note named the old way is left as it is; nothing moves what was kept.
- **Not here:** mounting anything else into the container (design entry 0398, "One folder, or one for each use").
- **Records:** ADR-159 (decision 12) and ADR-153 get a Later entry; `adr_check.py` lists any others owed.
