# stackvet.dev: AgentVet, StackVet's companion that vets the AI coding tool (preview)

The words of the page about AgentVet (`av`) (backlog 0725), as `configvet-page.md` keeps ConfigVet's:
`tools/website.py` reads them, shows each section's headline in bold as its heading, and leaves out each section's
last list, the notes on it. The owner asked on 22 November 2026 for a page about `av`, like ConfigVet's: labeled
preview, linked from the home page, read by the owner before it merges. Every claim comes from ADR-159 (its
decisions, the owner's answers, and "As built" parts 1 to 12), the design entry
`docs/design/0397-agentvet-a-companion-that-vets-the-agent-what-it-did-what-it.md`, `av`'s usage text in
`crates/av-cli/src/main.rs`, ADR-122 (the hook record), and `docs/INSTALL.md`'s section on `av`; the notes say which.
**Nothing here is live** until the owner publishes the site.

## The opening

**AgentVet: what the record shows your AI coding tool did. A preview.**

StackVet checks the app. AgentVet, `av`, looks at the AI coding tool that built it. It reads the records kept while
the tool worked, and says what each record shows the tool did, how far that record can be relied on, and what it
could not see.

It decides nothing about the tool, and it never says a tool can be trusted. Every line is something for you to
read: many of them are things a tool is right to do.

**This is a preview.** It has been tried on one computer, with one AI coding tool, and it reads nothing of what the
tool said beyond one kind of claim.

```
av report <folder> [--hook-record] [--proxy-record] [--json]
av proxy [--app <folder>] -- <the tool server's command>
av checkpoint <folder>
```

- ADR-159, decisions 1, 3, and 4, and the usage text in `crates/av-cli/src/main.rs`: "It reads no transcript, decides
  nothing". "One kind of claim" is that the app's tests pass (ADR-122; ADR-159, "As built", part 5).
- "Tried on one computer": the owner's trials of 21 and 22 November 2026, a Mac, Claude Code's desktop app, `sv`
  through Docker (ADR-159, "As built", parts 9, 11, and 12).

## What it reads

**Four records, each with how far it can be relied on.**

A report is only as good as its sources, so `av report` names each one and says who could have changed it. Each is
kept on your own computer. Two are read only when you ask, each time, because they hold a short account of what the
tool ran or was given.

| Record | What it holds | Who keeps it, and what could change it |
|---|---|---|
| The build-loop record | Each time the tool asked StackVet something | StackVet's MCP server, in the app's own folder. The tool runs as you and could rewrite it |
| Git's history | What the tool's commits changed | Git. A commit counts as the tool's only when its message says so, and history can be rewritten |
| The hook record (`--hook-record`) | Each command the tool ran, each file it wrote, each permission it asked you for, and whether the tests were run before it said they pass | A hook in the tool's own settings, kept outside the app's folder. The tool could take the hook out |
| The pass-through's record (`--proxy-record`) | Each call the tool made to another tool server, and how it was answered | `av proxy`, kept outside the app's folder. The tool could take it out of its settings |

Every line of every record is chained to the one before it, so a change to a line already written shows. A chain
does not show a record that was replaced whole. Two things do:

- **A note outside the folder.** Each time StackVet's server writes a line of the build-loop record, it notes,
  outside the app's folder, where the record has got to. A tool kept to the app's folder cannot change those notes.
- **A checkpoint you sign.** `av checkpoint`, run by you at your own terminal, signs where each record has got to,
  with your own key and its passphrase. A later report says whether the lines you signed for are still as they were.

No file contents, command output, or prompts are kept, and never an answer's text. What a tool ran or was given is
cut short, with any key cut out.

- ADR-159, decision 3 (evidence by who witnessed it) and "As built", parts 2, 6, 7, 8, and 12; the sentences beside
  each record are `av`'s own (`witness` in `crates/av-cli/src/main.rs`). ADR-076 (the build-loop record), ADR-122
  (the hook record, and its consent: read only for a run that asks).
- "A tool kept to the app's folder cannot change those notes; a tool that can write anywhere you can, can": part 8,
  said by the report each time. A checkpoint is the one thing that shows a change made by such a tool, and only to
  lines signed for (part 7).
- What is never kept: ADR-122 and its Later entry of 22 November 2026, and part 6 ("Never an answer's text"). For a
  tool server's tool, the summary is the start of what it was given, which can be text meant for a file; the page
  does not say otherwise.

## The task file

**You say what the task is. AgentVet lists what falls outside it.**

Put a file named `agentvet.toml` in the app's folder, saying what the tool's task is and what it may touch:

```
envelope-version = 1

[task]
goal = "Add a sign-out link to the header."

[may]
write = ["src/", "tests/"]
run = ["pytest", "npm"]
call = ["stackvet_check"]

[guards]
files = ["deploy/"]
```

`av report` then lists what the records show the tool did outside it: a file changed that `write` does not cover, a
program started that `run` does not name, a tool called that `call` does not name.

- **A line left out is a question, never a yes or a no.** Leave out `write` and the report asks which files the tool
  may write; it does not assume everything, or nothing.
- **Some files are always guarded,** whatever the file says: the tool's own instructions and settings, the hooks and
  servers it is started with, what runs in CI and who must review, and `agentvet.toml` itself. Your own list adds to
  these and takes none away.
- **The file does not say who wrote it.** The tool can write `agentvet.toml` as well as you can, so the report says
  it may be the tool's own account of its task.

Nothing is judged against a task file that cannot be read: the report says why, and holds the tool to nothing.

- ADR-159, decision 8, and "As built", parts 3 and 4 (`crates/av-record/src/envelope.rs`, `held.rs`); `[may] call` is
  part 9. The example is the module's own, with `call` added. "Your own list adds to these" is the owner's choice of
  21 November 2026.
- Not said on the page, and true: `[may] reach` and `ask-first` can be written, and nothing read yet can hold a tool
  to them; the report says so.

## The pass-through

**A record of each call to a tool server, kept where the tool's own account cannot reach.**

AI coding tools reach other tools through MCP servers. `av proxy` sits in front of one: put it before the server's
command in the tool's settings. It starts that server, passes everything each way unchanged, and writes down each
call: the tool's name, how it was answered, and a short summary of what it was given, with any key cut out.

`av report --proxy-record` then says which tools were called and how often, by session, which calls the server turned
down, and a call that was turned down and then made again with the same thing.

It adds no connection of its own, and if it cannot write its record it says so and keeps passing calls along, so a
full disk does not stop the tool's work.

- ADR-159, decision 10, and "As built", parts 6, 9, and 11. "By session": `av proxy` writes a line when it starts
  (part 11, after the owner's second trial paired a call across two sessions).
- "Turned down" is a server's error and a tool's own failure together, since a server may say no either way (part 9,
  from the first trial); the report says it does not tell them apart.
- It speaks MCP over standard input and output only; a server reached over HTTP is not covered, and the page's "Not
  there yet" says so.

## What you were asked, and whether it ran

**No record can show that you said no. This shows the nearest thing.**

When you refuse a call at your AI coding tool's own prompt, nothing is told: the call is never sent to a tool
server, and the tool's hooks are not told you refused. Two trials showed this before anything was built on it.

What a hook does see is that the tool is about to ask you, and that a tool ran. So the hook record keeps a line each
time you are asked, and `av report --hook-record` says how many times you were asked and after how many the tool then
ran. Each one it did not run after is listed, with the next thing the tool did in that session, so you can read what
it tried instead.

It says **asked, and did not run**, never that you refused: that is you saying no, or the prompt closed without an
answer, or the session ending with it open. No hook says which.

- ADR-159, "As built", part 12 (backlog 0718), and ADR-122's Later entry of 22 November 2026: what was read in Claude
  Code's hooks reference, what was tried on the owner's computer the same day, and what the record keeps for an
  asking. The hook is the one `sv connect claude --hook` prints; it is offered for Claude Code alone.
- "The next thing the tool did" is the next line the record holds for that session. Only the same tool given the same
  thing is matched as "tried again"; the same end reached another way is left for the person to read (part 12, "Not
  built").

## Six things it looks for

**The same lines, under the six things worth looking at. An empty one is not a clean one.**

The report ends by gathering everything under six headings. Each says what was seen, and, as plainly, what it could
not show.

- **What the tool said, against the record.** Today, one claim: that the app's tests pass, set against whether the
  tests were run, and passed, before it said so.
- **A check made to pass without a fix.** A test removed or skipped, an expectation changed, a comment telling a
  check to look away, in the tool's commits.
- **Outside what the task file allows.** From the task file, above.
- **Changes to what watches the tool.** A guarded file changed, a record's chain that no longer checks out, a record
  that no longer matches a note or a checkpoint.
- **Written in your name.** An entry in a tool's commit that says it is yours.
- **Trying again after a no.** A call that was turned down, or that you were asked about and did not run, and that
  came again.

A claim the record contradicts and a claim the record does not support are said as two different things, and a claim
that could not be checked is never listed as either.

- ADR-159, decision 4 (the three verdicts) and "As built", part 5 (`crates/av-record/src/groups.rs`), with parts 9,
  11, and 12 for the last group. The headings are the report's own labels.
- "An empty one is not a clean one" is the report's own sentence: "an empty kind is not a clean one: each says what it
  could not show".

## What it cannot see

**Said in every report, under "Not looked at".**

- **What the tool said it did,** beyond that the tests pass. AgentVet reads no transcript.
- **Whether the tool did its task, or did it well.** It sets what the tool wrote and ran against the task file, and
  reads nothing of what the work was for.
- **A no you gave at the tool's own prompt.** Only that you were asked, and the tool did not run.
- **What a program the tool started went on to do.** A command is read as far as its text shows.
- **A record replaced before you ever signed a checkpoint,** or a hook taken out so that nothing was written.
- **Anything the tool did some other way.** A tool server with no pass-through in front of it, or work done outside
  the hooks.

AgentVet does not stop anything. It reports; stopping an action is a later decision of its own.

- The first three are `av`'s own "Not looked at" lines (`crates/av-cli/src/main.rs`, and
  `groups::NO_AT_THE_PROMPT`). The fourth and sixth are what the groups say they could not show (`held.rs`: "What a
  program the tool started went on to do"; `groups.rs`: "A tool called some other way than through av proxy"). The
  fifth is the oversight group's.
- "Does not stop anything": ADR-159, decision 11, and the owner's answer ("start with report, work towards stopping
  an action in certain circumstances").

## How it relates to StackVet

**The same rules of evidence, in a program of its own.**

AgentVet is built in StackVet's workspace and shares its records: the build-loop record and the hook record are the
ones StackVet already keeps, and StackVet's own report still reads them as it did. Every rule of evidence holds
unchanged: something not looked at is never a pass and never a failure, and a key is shown by its first four
characters and its length, never whole.

- **It opens no network connection.** `av report` reads files on your computer and writes nothing. `av proxy` starts
  one program, the tool server you name, and what that server connects to is the server's own.
- **It runs on your computer, not in StackVet's container.** It starts the tool server, reads records in your own
  folders, and takes your key's passphrase at your terminal.
- **It never makes a key.** A checkpoint is signed with the key StackVet's `sv review` uses.

StackVet is written for people who are not programmers. AgentVet's first reader is someone who builds with an AI
coding tool and wants to know what it did.

- ADR-159, decisions 2 and 10, and "As built", parts 1, 2, and 7; `docs/ARCHITECTURE.md`, "The fence", for what `av`
  opens and starts. A test holds what `av`'s own source can show: nothing in it connects, and one file starts a
  program.
- "Not in StackVet's container": ADR-108's Later entry of 22 November 2026.
- "First reader": the owner's answer of 21 November 2026 (a solo builder who is a security engineer); the page says
  it without naming anyone.

## Not there yet

**What the preview doesn't do, and what has and hasn't been tried.**

- **Tried on one set-up only:** a Mac, Claude Code's desktop app, and StackVet through Docker. The hook is offered
  for Claude Code alone. The pass-through has run on Windows only in the release's own check, not in a real session.
- **No transcript is read,** so only one kind of claim is checked.
- **A tool server reached over HTTP is not covered** by the pass-through.
- **The same end reached another way is not matched.** A call that did not run and was then tried with something
  else is shown only as what came next.
- **It does not stop an action.**

Until these change, AgentVet stays a preview, and this page says what each part really does as it lands.

- ADR-159, "As built", parts 9, 10, 11, and 12, each one's "Not shown by anything here" and "What this does not
  cover"; the release workflow's check of `av` on Windows is ADR-108's Later entry of 22 November 2026.

## Trying it

**Released with StackVet, as a package of its own.**

`av` comes from the same release as StackVet, with the same version number, and installs separately. On a Mac or
Linux, Homebrew builds it on your computer:

```
brew install abbyshade111/stackvet/av
```

On Windows, Scoop installs the signed download from the release:

```
scoop bucket add stackvet https://github.com/abbyshade111/scoop-stack-vet
scoop install stackvet/av
```

Or build it from StackVet's repository with Rust's `cargo`, and the program is `av`, in `target/release/`:

```
cargo build --release --locked -p av-cli
```

The hook that keeps the hook record runs a copy of StackVet's `sv` installed on your computer, not the one in
StackVet's container.

If something is wrong or confusing, tell us in a [GitHub issue](https://github.com/abbyshade111/StackVet/issues).

- Each line is `docs/INSTALL.md`'s, which a test holds this page to, as it holds ConfigVet's. StackVet v0.3.1,
  published 22 November 2026, is the first release with `av`'s own Windows zip (#2533, backlog 0724), signed beside
  `sv`'s and `cv`'s. The tap's `Formula/av.rb` (homebrew-stackvet#7) and the bucket's `av.json` (scoop-stack-vet#4)
  are the packages these lines install; both had merged, with the tap's install jobs for `av` required, before this
  page printed them. Neither install has been tried by hand. `scoop bucket add` is only needed once, and is the same
  line as for `sv`.
- "A copy of `sv` installed on your computer": ADR-122 (`sv connect claude --hook` is not offered for `sv` in its
  container). Contact is GitHub issues only, as on the home page.
