# What sv in its container could gain from its records folder (22 November 2026)

Backlog 0719, an investigation the owner asked for on 22 November 2026: would anything `sv` does, besides the
build-loop notes, gain from reaching `sv`'s own folders from inside its container? Everything below was read in the
code at `main` that day, not from memory, and nothing was built.

## What the container is given today

`sv connect <tool> --docker` prints a `docker run -i --rm --network none` with the app's folder mounted, and, only when
asked for:

- `--keys DIR`: the one file `allowed_signers`, the list of trusted keys, read-only, with `XDG_CONFIG_HOME` pointed at
  the folder holding it (`crates/sv-cli/src/connect.rs`, backlog 0581).
- `--notes DIR`: the one folder `loop-notes`, where the MCP server notes where an app's build-loop record has got to,
  with `XDG_STATE_HOME` pointed at it (backlog 0716, part 4; not on Windows).

The container is removed when its session ends (`--rm`), and its own file system can be written. So anything `sv`
writes into its config, data, or state folders inside the container, other than the two mounts, is written into the
container and lost with it.

## Everything `sv` keeps outside the app's folder, and what happens to each in the container

| What | Where on the computer | Who writes and reads it | In the container today |
|---|---|---|---|
| The report key, an HMAC key that seals each report folder as `sv`'s (ADR-034, `crates/sv-cli/src/report_seal.rs`) | `~/.config/stackvet/`, beside the review key | `sv report` and the MCP server's `stackvet_write_report` write and seal; the MCP server checks the seal before it offers a report as "a report sv wrote" | **A gap.** Made new in each session and lost with it, so a report the MCP server wrote in an earlier session is never offered again. The code says so: a report "written … in a container since gone, cannot be checked here, and is not offered". |
| The review key, which signs what the owner records with `sv review` (ADR-026, ADR-043, `crates/sv-check/src/seal.rs`) | `~/.config/stackvet/` | `sv review`, at the owner's terminal only | Never given, on purpose: the list of trusted keys is given so the container can *check* the owner's records, never sign. Keep it so. |
| History, a record of each `sv report` run, switched on by a file `keep-history` beside the review key (ADR-057, `crates/sv-cli/src/history.rs`) | `~/.local/share/stackvet/history` | `sv report` writes; `sv history` and the dashboard read | Never on: `keep-history` is not visible in the container, so a run there is not recorded even when the person turned history on. A convenience, never evidence. |
| The dashboard's link to the backlog board (`crates/sv-cli/src/board_link.rs`) | `~/.config/stackvet/backlog-board` | Read by the dashboard only | Not needed: the dashboard is the owner's tool, on the computer. |
| The build-loop notes (ADR-159, decision 12) | `~/.local/state/stackvet/loop-notes` | The MCP server writes; `av report` reads | Given with `--notes`, alone. Done (backlog 0716). |
| The hook record (ADR-122, `crates/av-record/src/hook_record.rs`) | `~/.local/state/stackvet/hook-records` | The hook runs `sv hook-record` after each of the AI tool's commands; `sv report` and `av report` read | **A gap**, and `sv` says so: "--hook is not offered for sv in its container yet: the hook runs sv after each of the tool's commands, and needs a copy of sv installed on your computer". |
| The pass-through's record, and the owner's checkpoints (ADR-159, decisions 10 and 12) | `~/.local/state/stackvet/…` | `av proxy` and `av checkpoint`, which run on the computer | Not `sv`'s in the container to touch; `av` runs outside it. |

## What a person with only Docker cannot do, because of it

1. **Have the AI tool read a report from an earlier session through the MCP server.** The report is in the app's
   folder, but its seal was made with a key the container has thrown away, so the server will not vouch for it. The
   tool has to ask for a new report each session. This one is not in any message `sv` prints to a Docker user today;
   it is only in `report_seal.rs`'s own notes.
2. **Keep the hook record.** Said by `sv connect --hook`.
3. **Keep history.** Said nowhere to a Docker user: history is off there without a word.

## What each candidate would cost

Each folder or file given to the container is one step nearer for an app's own files, which `sv` reads there. Asked
of each: mounted to read or to write, what it holds, and what a fault in `sv` or a hostile app's files could do.

- **The report key, to write and read.** It is a secret: whoever holds it can seal a forged report that the MCP server
  then offers as `sv`'s, the thing ADR-034 stops. Today the container holds a fresh key for one session, so a leak
  lasts one session. Mounted, a leak would last until the person makes a new key, and if it were the computer's own
  key, a forged report would be offered by the computer's `sv` too. If it is done at all, it should be **a key of the
  container's own**, in a folder of its own (for example `~/.local/state/stackvet/container-report-key`), never the
  computer's report key and never the folder that holds the review key, mounted as one file as `allowed_signers` is.
  Then a leak forges only what the container's server offers.
- **The hook record, to write.** It would need the hook to start a container after every command the AI tool runs.
  Not measured, but a container start is far slower than the installed `sv` the hook is written for, and the hook
  runs often. The hook record also holds every command the tool ran; mounting it to write gives an app's files a way
  to the record that is meant to watch them. Not worth it while installing `sv` (Homebrew, Scoop) is one command.
- **History, to write.** It holds a dated list of each app's weaknesses, kept outside the app's folder for exactly
  that reason (ADR-057). Low value, and it would need the `keep-history` switch to reach the container too. Not worth
  it now; worth one sentence in what `sv connect --docker` prints, so a person who turned it on is not surprised.

## One folder, or one for each use

**Keep one mount for each use, never the whole records folder.** The notes were mounted alone so the container does
not see the hook record, the pass-through's record, or the owner's checkpoints: three records written to watch what
the AI tool and its servers did, which an app's files, read by `sv` in the container, should not be able to reach or
rewrite. The same holds for anything added later: each gets its own mount, its own option, off unless asked for, with
a record.

## Windows

The app's folder has another path inside the container (ADR-153), and a note is filed under the folder's path, so
`av report` on the computer would not find it; that is why `--notes` is refused on Windows. But the container is
already told the computer's own path of the folder, in `SV_HOST_FOLDER` (`connect.rs`, `HOST_FOLDER_ENV`), and the
MCP server already turns the container's paths back into the computer's with it, for what it shows the AI tool
(`crates/sv-cli/src/mcp/host_paths.rs`). The notes do not: `av-record`'s `outside_note::file_of` names a note by the
SHA-256 of the app's path as the process sees it, the container's path there. Naming a note by the computer's path when
`SV_HOST_FOLDER` is set would file it where the computer's `av report` looks, and let `--notes` work on Windows. The
one thing to get right is that both sides write the path the same way: `av report` on the computer names it from its
own canonical path, which on Windows can carry a `\\?\` prefix and another case for the drive letter than the path
`sv connect` printed, so the test must take a real Windows path through both. Not tried; it is the one concrete build
this investigation turns up.

## Recommendation

1. **Build: name the build-loop notes by `SV_HOST_FOLDER` when it is set,** and then offer `--notes` on Windows, with a
   test on a Windows path. Small, and it finishes what backlog 0716 started.
2. **Say it, don't mount it, for history:** one line in what `sv connect --docker` prints, that history is not kept
   for runs in the container.
3. **The owner's choice: the report key.** Either leave it (the AI tool asks for a new report each session, and a
   leaked key lasts one session), or give the container a report key of its own, as one file in a folder of its own,
   written by the container and never the computer's key. Recommended: leave it, and say it in what `sv connect
   --docker` prints, unless Docker-only use of the MCP server grows enough that a report surviving the session matters.
4. **Keep:** the hook record, the review key, the pass-through's record, and the checkpoints stay out of the
   container; one mount for each use.

Each of 1 and 2 would be a backlog item of its own; 3 would be a decision with a record (ADR-101, ADR-153, and the
fence in `docs/ARCHITECTURE.md`).

- Read on 22 November 2026: `crates/sv-cli/src/connect.rs` (the `docker run` arguments, `--hook`'s refusal,
  `HOST_FOLDER_ENV`), `crates/sv-cli/src/mcp/host_paths.rs`, `crates/sv-cli/src/report_seal.rs` (its notes on a container since gone), `crates/sv-check/src/seal.rs`,
  `crates/sv-cli/src/history.rs`, `crates/sv-cli/src/board_link.rs`, and `crates/av-record/src/` (`hook_record.rs`,
  `outside_note.rs`, `checkpoint.rs`, `proxy_seen.rs`, `asked.rs`). Nothing was run in a container for this.
