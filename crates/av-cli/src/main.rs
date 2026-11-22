//! `av`, AgentVet's command (ADR-159; backlog 0705, part 2). Two commands: `av proxy`, the pass-through that writes
//! down each call an agent makes to a tool (`proxy`), and `av report <folder>`, which reads what
//! is already kept about the AI coding tool that worked in a folder and says what the record shows, at which
//! strength, and what it could not see. It decides nothing about the tool, and it never says a tool can be trusted.
//!
//! What it reads: the build-loop record (`sv`'s own list of the tool's calls to it, ADR-076), git's history, and, only
//! when the person asks for it with `--hook-record`, the hook record (the commands the tool ran and the files it
//! wrote, ADR-122). It reads no transcript, so it says nothing of what the tool claimed beyond what the hook record
//! keeps. It also reads the task envelope, `agentvet.toml`, when the folder has one, and says what it holds and what
//! it leaves as a question, and lists what the records show the tool did that falls outside it or touched a file
//! that guards oversight (backlog 0715). It writes nothing and opens no network connection.
//!
//! Exit codes follow `sv`'s (ADR-029): 0 when the report was made; 2 when there was nothing to read, so nothing was
//! said about the tool; 3 when `av` itself failed and there is no result.

use av_record::{build_loop, envelope, groups, held, hook_record, tool_behavior};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

// Everything `av` prints goes through `sv_report::visible`: a file name, a command, or a commit message is the
// tool's text, and an escape character in one must not reach the terminal. These shadow the standard macros in
// every module of this crate below them, as `sv`'s do; `proxy` relays through writers of its own, never these.
macro_rules! println {
    () => { ::std::println!() };
    ($($arg:tt)*) => { ::std::println!("{}", ::sv_report::visible(&::std::format!($($arg)*))) };
}
macro_rules! eprintln {
    ($($arg:tt)*) => { ::std::eprintln!("{}", ::sv_report::visible(&::std::format!($($arg)*))) };
}

mod checkpoint;
mod proxy;

const MADE: u8 = 0;
const NOTHING_READ: u8 = 2;
const FAILED: u8 = 3;

const USAGE: &str = "\
av: AgentVet says what the record shows an AI coding tool did.

Usage:
  av report <folder> [--hook-record] [--json]
  av proxy [--app <folder>] -- <the tool server's command>
  av checkpoint <folder>
  av --version

av report reads what is kept about the tool that worked in the folder: sv's record of the tool's calls to it, and
git's history. With --hook-record it also reads the record of the commands the tool ran and the files it wrote, which
a hook you installed keeps outside the folder (`sv connect claude --hook` prints it); it is read only when you ask,
each time. It says what each record shows and how far each can be relied on, and what it could not see. When the
folder holds a task file, agentvet.toml, it says what that allows and what it leaves unsaid, and lists what the tool
did that falls outside it. It reads no transcript, decides nothing, writes nothing, and opens no network connection.
--json prints the same as JSON.

av proxy sits between an agent and a tool server that speaks MCP over standard input and output: put it in front of
the server's command in the agent's settings. It starts that server, relays everything each way unchanged, and writes
down each call to a tool: the tool's name, how it was answered, a fingerprint and the size of what it was given, and a
short summary of that with any key cut out. Never an answer's text. The record is kept outside the folder, readable
only by you, for the app in --app (by default, the folder av proxy was started in). It opens no network connection.

av checkpoint, run by you at your own terminal, signs where each record of the app has got to, with the key sv review
signs with; it asks for that key's passphrase and keeps nothing of it. A later av report then says whether the lines
you signed for are still as they were. av never makes a key: sv review does.";

#[derive(Debug, PartialEq, Eq)]
struct Options {
    folder: PathBuf,
    hook_record: bool,
    json: bool,
}

fn parse(args: &[String]) -> Result<Options, String> {
    let mut folder = None;
    let (mut hook_record, mut json) = (false, false);
    for arg in args {
        match arg.as_str() {
            "--hook-record" => hook_record = true,
            "--json" => json = true,
            other if other.starts_with("--") => {
                return Err(format!("av report does not know {other}"));
            }
            other if folder.is_none() => folder = Some(PathBuf::from(other)),
            other => return Err(format!("one folder at a time; {other} is a second")),
        }
    }
    Ok(Options {
        folder: folder.ok_or("av report needs a folder")?,
        hook_record,
        json,
    })
}

/// `av proxy`'s own options, then `--`, then the server's command, which is the server's and is never read as ours.
/// The app's folder is given in full, since the record is named by it.
fn parse_proxy(args: &[String]) -> Result<(PathBuf, Vec<String>), String> {
    let split = args
        .iter()
        .position(|a| a == "--")
        .ok_or("av proxy needs `--` and then the tool server's command")?;
    let (ours, server) = (&args[..split], &args[split + 1..]);
    if server.is_empty() {
        return Err("av proxy needs the tool server's command after `--`".to_owned());
    }
    let mut app = None;
    let mut rest = ours.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--app" => app = Some(PathBuf::from(rest.next().ok_or("--app needs a folder")?)),
            other => return Err(format!("av proxy does not know {other}")),
        }
    }
    let app = match app {
        Some(app) => app,
        None => std::env::current_dir()
            .map_err(|why| format!("the folder av proxy was started in: {why}"))?,
    };
    use sv_frameworks::paths::Canonical as _;
    let app = app
        .canonical()
        .map_err(|_| format!("{} is not a folder av can open", app.display()))?;
    Ok((app, server.to_vec()))
}

/// `av checkpoint <folder>`: only at a terminal, since a checkpoint is the owner's own act; a passphrase typed there
/// is what makes it one.
fn run_checkpoint(args: &[String]) -> u8 {
    use std::io::IsTerminal as _;
    use sv_frameworks::paths::Canonical as _;
    let [folder] = args else {
        eprintln!("av: av checkpoint needs one folder\n\n{USAGE}");
        return FAILED;
    };
    let Ok(app) = Path::new(folder).canonical() else {
        eprintln!("av: {folder} is not a folder av can open");
        return FAILED;
    };
    if !std::io::stdin().is_terminal() {
        eprintln!(
            "av: av checkpoint is for you to run at your own terminal: what it signs is your word, so it does not \
             read a passphrase handed to it by another program."
        );
        return FAILED;
    }
    let Some(keys) = sv_check::seal::Key::folder() else {
        eprintln!(
            "av: this computer has no folder for sv's keys (neither HOME nor XDG_CONFIG_HOME is set)"
        );
        return FAILED;
    };
    let stdin = std::io::stdin();
    // Written through a cleaning writer, as everything `av` prints is.
    let done = checkpoint::run(&app, &keys, &mut stdin.lock(), &mut Prompted);
    match done {
        Ok(true) => MADE,
        Ok(false) => NOTHING_READ,
        Err(why) => {
            eprintln!("av checkpoint: {why}");
            FAILED
        }
    }
}

/// Standard output, cleaned as `println!` here cleans it, for a command that writes as it goes: a question must be
/// on the screen before its answer is read.
struct Prompted;

impl std::io::Write for Prompted {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let text = String::from_utf8_lossy(buf);
        std::io::stdout()
            .lock()
            .write_all(sv_report::visible(&text).as_bytes())?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().lock().flush()
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--version" | "-V") => {
            println!("av {}", env!("CARGO_PKG_VERSION"));
            ExitCode::from(MADE)
        }
        Some("report") => match parse(&args[1..]) {
            Ok(options) => ExitCode::from(run(&options)),
            Err(why) => {
                eprintln!("av: {why}\n\n{USAGE}");
                ExitCode::from(FAILED)
            }
        },
        Some("checkpoint") => ExitCode::from(run_checkpoint(&args[1..])),
        Some("proxy") => match parse_proxy(&args[1..]) {
            Ok((app, server)) => match proxy::run(&app, &server) {
                Ok(code) => ExitCode::from(code),
                Err(why) => {
                    eprintln!("av proxy: {why}");
                    ExitCode::from(FAILED)
                }
            },
            Err(why) => {
                eprintln!("av: {why}\n\n{USAGE}");
                ExitCode::from(FAILED)
            }
        },
        Some("--help" | "-h" | "help") => {
            println!("{USAGE}");
            ExitCode::from(MADE)
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(FAILED)
        }
    }
}

/// How far a record can be relied on, in the words a report shows beside it (ADR-159, decision 3). Each of the
/// records read today is a file, or a history, that the tool could rewrite, and the report says so every time.
mod witness {
    pub const BUILD_LOOP: &str = "Kept by sv's MCP server in the folder's hidden .stackvet folder. The tool runs as \
                                  you and could rewrite it; its lines are chained, so a change to a line already \
                                  written shows, and a record replaced whole does not.";
    pub const HOOK: &str = "Kept by a hook in the tool's own settings, outside the folder. The tool could remove the \
                            hook or rewrite the record; its lines are chained, so a change to a line already written \
                            shows.";
    pub const GIT: &str = "Read from git's history. A commit counts as the tool's only when its message says so, and \
                           history can be rewritten.";
}

/// What one run of `av report` read, and what it did not.
#[derive(Debug)]
struct Read {
    /// The task envelope, as it was read.
    envelope: envelope::Read,
    /// What the records show against it, when there is one to hold them to.
    held: Option<held::Held>,
    build_loop: sv_report::BuildLoop,
    /// The hook record, when the person asked for it.
    hook: Option<sv_report::hook::HookSeen>,
    behavior: sv_report::tool_behavior::ToolBehavior,
    /// How each record stands against the owner's checkpoints: `None` for a record this run did not read.
    checkpoints: Vec<(
        av_record::checkpoint::Kind,
        Option<av_record::checkpoint::Standing>,
    )>,
    /// All of it, under the design's six groups.
    groups: Vec<groups::Gathered>,
    not_looked_at: Vec<String>,
}

impl Read {
    /// Whether any record held anything about the tool: a call, a commit of its, or a line of the hook record.
    fn anything(&self) -> bool {
        self.build_loop.calls > 0
            || self.behavior.tool_commits > 0
            || self
                .hook
                .as_ref()
                .is_some_and(|h| h.not_read.is_none() && h.commands + h.writes + h.turns > 0)
    }
}

fn read(folder: &Path, with_hook: bool) -> Read {
    // The build-loop record. Findings a person set aside are in sv's report, not here, so none are told apart.
    let mut loop_seen = build_loop::read(folder);
    build_loop::settle(&mut loop_seen, &[]);
    build_loop::look_again(&mut loop_seen, folder, &[]);

    // The hook record, only for a run that asked for it (ADR-122: consent is given each time).
    let hook = with_hook.then(|| hook_record::read_for(folder, None));
    let done = if with_hook {
        hook_record::done_for(folder)
    } else {
        None
    };

    // Git, and what the two together show. No earlier report and no findings are handed over: comparing with sv's
    // last report is sv's own section, which can show it wrote that report.
    let commits = sv_check::git::changes::since(folder, None, tool_behavior::LAST_COMMITS);
    let commit_list = commits.as_ref().map(|(c, _)| c.as_slice());
    let mut behavior = tool_behavior::of(commit_list, done.as_deref(), None, &[]);
    let installs = done.as_deref().unwrap_or_default().iter().any(
        |d| matches!(d, hook_record::Done::Ran { command, .. } if !tool_behavior::installed(command).is_empty()),
    );
    let components = if installs {
        sv_check::sbom::build(folder).components
    } else {
        Vec::new()
    };
    behavior.seen.extend(tool_behavior::installs_and_secrets(
        commit_list,
        done.as_deref(),
        &components,
        &[],
    ));
    behavior.seen.sort_by_key(|s| s.kind);
    behavior.more += behavior
        .seen
        .len()
        .saturating_sub(tool_behavior::MOST_LINES);
    behavior.seen.truncate(tool_behavior::MOST_LINES);
    behavior.commits_read = commits.as_ref().map(|(c, _)| match c.len() {
        1 => "the last commit".to_owned(),
        n => format!("the last {n} commits"),
    });

    let mut not_looked_at = vec![
        "What the tool said it did, beyond that the app's tests pass: av reads no transcript yet."
            .to_owned(),
        match envelope::read(folder) {
            envelope::Read::Held(_) => {
                "Whether the tool did its task, or did it well: av sets what it wrote and ran against the task \
                 file, and reads nothing of what the work was for."
            }
            envelope::Read::Unreadable(_) => {
                "Whether the tool kept to its task: the task file could not be read, so there is nothing to hold \
                 it to."
            }
            envelope::Read::NotThere => {
                "Whether the tool kept to its task: there is no task file (agentvet.toml) to hold it to."
            }
        }
        .to_owned(),
        "Which findings that went away a person set aside, and which findings are there now: those are in sv's \
         report, whose own section compares this report with the last (`sv report --hook-record`)."
            .to_owned(),
    ];
    if !with_hook {
        not_looked_at.push(
            "The commands the tool ran and the files it wrote: the hook record is read only when you ask, with \
             --hook-record."
                .to_owned(),
        );
    }
    if commits.is_none() {
        not_looked_at.push(
            "Git's history: the folder is not a git checkout, or git could not say.".to_owned(),
        );
    }
    let envelope = envelope::read(folder);
    let held = match &envelope {
        envelope::Read::Held(e) => {
            // A commit names its files from the top of its repository, and the task file names them from its own
            // folder: the two are the same only when this folder is that top.
            let history = match &commits {
                None => Err("the folder is not a git checkout, or git could not say"),
                Some(_) if !folder.join(".git").exists() => Err(
                    "this folder is not the top of its repository, so a commit's file names cannot be set \
                     against the task file yet",
                ),
                Some((list, _)) => Ok(list.as_slice()),
            };
            Some(held::against(e, history, done.as_deref()))
        }
        _ => None,
    };
    // What the owner signed for (ADR-159, decision 12). A record is set against its checkpoints only when this run
    // may read it: the hook record when asked for, and the pass-through's not yet.
    use sv_frameworks::paths::Canonical as _;
    let real = folder.canonical().unwrap_or_else(|_| folder.to_path_buf());
    let checker = sv_check::seal::Checker::for_app(&real);
    let checkpoints: Vec<_> = av_record::checkpoint::Kind::ALL
        .into_iter()
        .map(|kind| {
            let may_read = match kind {
                av_record::checkpoint::Kind::BuildLoop => true,
                av_record::checkpoint::Kind::Hook => with_hook,
                av_record::checkpoint::Kind::Proxy => false,
            };
            (
                kind,
                may_read.then(|| av_record::checkpoint::standing(&real, kind, &checker)),
            )
        })
        .collect();
    let read_standings: Vec<_> = checkpoints
        .iter()
        .filter_map(|(kind, standing)| Some((*kind, standing.clone()?)))
        .collect();
    let gathered = groups::gather(&groups::Read {
        checkpoints: &read_standings,
        build_loop: &loop_seen,
        hook: hook.as_ref(),
        behavior: &behavior,
        held: held.as_ref(),
        commits: commit_list,
    });
    Read {
        envelope,
        held,
        checkpoints,
        groups: gathered,
        build_loop: loop_seen,
        hook,
        behavior,
        not_looked_at,
    }
}

/// The findings between the first check and the last, from the build-loop record alone: how many went, and of those
/// which still read as they did. A changed line is not called a fix: nothing here read the change.
fn findings_lines(b: &sv_report::BuildLoop) -> Vec<String> {
    let Some(moved) = b.findings_moved else {
        return vec![
            "Findings that went away between the tool's checks: not shown. The record needs two checks that kept \
             every finding's fingerprint."
                .to_owned(),
        ];
    };
    let mut out = vec![format!(
        "Between the tool's first check and its last, {} no longer found, and {} new.",
        match moved.no_longer_found {
            1 => "1 finding was".to_owned(),
            n => format!("{n} findings were"),
        },
        match moved.new {
            1 => "1 was".to_owned(),
            n => format!("{n} were"),
        }
    )];
    if b.looked_again > 0 || b.not_looked_again > 0 {
        out.push(format!(
            "Of those no longer found, {} looked for again in the code{}. {} with the line unchanged, so it went \
             some other way than a change to that line; a line that did change is not shown to be a fix.",
            match b.looked_again {
                1 => "1 was".to_owned(),
                n => format!("{n} were"),
            },
            match b.not_looked_again {
                0 => String::new(),
                n => format!(", and {n} past the limit were not"),
            },
            match b.went_same.len() {
                0 => "None still reads as it did".to_owned(),
                1 => "1 still reads as it did, listed below,".to_owned(),
                n => format!("{n} still read as they did, listed below,"),
            }
        ));
    }
    for w in &b.went_same {
        out.push(format!(
            "  no longer found, line unchanged: {} at {}:{}",
            sv_report::one_line(&w.rule),
            sv_report::one_line(&w.file),
            w.line
        ));
    }
    out
}

/// A list of an envelope's entries as a report shows it: each on one line, or that there are none.
fn listed(entries: &[String]) -> String {
    if entries.is_empty() {
        "nothing".to_owned()
    } else {
        entries
            .iter()
            .map(|e| sv_report::one_line(e))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// What the task envelope holds, in the report's words. Every word of the file shown is shown on one line: the
/// file may be the tool's own.
fn envelope_lines(read: &envelope::Read) -> Vec<String> {
    let e = match read {
        envelope::Read::NotThere => {
            return vec![
                "There is none in this folder, so nothing says what the tool's task was or what it might touch."
                    .to_owned(),
            ];
        }
        envelope::Read::Unreadable(why) => {
            return vec![
                format!("Not read: {}.", sv_report::one_line(why)),
                "Nothing is judged against a file that cannot be read.".to_owned(),
            ];
        }
        envelope::Read::Held(e) => e,
    };
    let mut out = vec![
        envelope::MAY_BE_THE_TOOLS.to_owned(),
        format!("The task: {}", sv_report::one_line(&e.task.goal)),
    ];
    for (what, entries) in [
        ("It may write", e.may.write.as_deref()),
        ("It may start", e.may.run.as_deref()),
        ("It may connect to", e.may.reach.as_deref()),
        ("It must ask first to", e.ask_first.as_deref()),
    ] {
        if let Some(entries) = entries {
            out.push(format!("{what}: {}.", listed(entries)));
        }
    }
    out.push(format!(
        "It may not change, whatever the file says: {}.",
        listed(
            &envelope::DEFAULT_GUARDS
                .iter()
                .map(|g| (*g).to_owned())
                .collect::<Vec<_>>()
        )
    ));
    let own: Vec<String> = e
        .guards()
        .into_iter()
        .filter(|g| !envelope::DEFAULT_GUARDS.contains(&g.as_str()))
        .collect();
    if !own.is_empty() {
        out.push(format!("The file adds to those: {}.", listed(&own)));
    }
    for question in e.questions() {
        out.push(format!("Not said, so not assumed either way: {question}"));
    }
    out
}

/// What the records show against the envelope, in the report's words: each line with the record it was read from.
fn held_lines(held: &held::Held) -> Vec<String> {
    let mut out = Vec::new();
    if held.seen.is_empty() {
        out.push(
            "Held to it, nothing read here falls outside it or touched a guarded file. That is what these records \
             show, not that nothing did."
                .to_owned(),
        );
    } else {
        out.push(format!(
            "Held to it, {} worth a look. Each is what a record shows, for you to read: a line here is not a \
             finding, and many are right to do.",
            match held.seen.len() + held.more {
                1 => "1 thing is".to_owned(),
                n => format!("{n} things are"),
            }
        ));
    }
    for seen in &held.seen {
        out.push(format!(
            "  {}: {} ({}; {})",
            seen.group.label(),
            sv_report::one_line(&seen.what),
            sv_report::one_line(&seen.place),
            match seen.witness {
                held::Witness::Git => "from git's history",
                held::Witness::Hook => "from the hook record",
            }
        ));
    }
    if held.more > 0 {
        out.push(format!("  {} more not listed here.", held.more));
    }
    out
}

/// How each record stands against the owner's checkpoints, in the report's words.
fn checkpoint_lines(
    all: &[(
        av_record::checkpoint::Kind,
        Option<av_record::checkpoint::Standing>,
    )],
) -> Vec<String> {
    use av_record::checkpoint::{Kind, Standing};
    let mut out = vec![
        "A checkpoint is your signature, made with `av checkpoint` at your own terminal, on where a record had got \
         to. It shows a change to lines you signed for; it says nothing of lines written since."
            .to_owned(),
    ];
    for (kind, standing) in all {
        let lines = |n: &usize| match n {
            1 => "its first line stands".to_owned(),
            n => format!("its first {n} lines stand"),
        };
        out.push(match standing {
            None => format!(
                "{}: not read, so not set against a checkpoint{}.",
                kind.said(),
                match kind {
                    Kind::Hook => "; it is read only when you ask, with --hook-record",
                    _ => "; av report does not read it yet",
                }
            ),
            Some(Standing::None) => format!(
                "{}: no checkpoint. Every line rests on its chain alone, which a tool that runs as you could make \
                 again.",
                kind.said()
            ),
            Some(Standing::Kept { at, lines: n, signed }) => format!(
                "{}: {} as you signed for them at {} ({}). Lines after them rest on the chain alone.",
                kind.said(),
                lines(n),
                sv_report::one_line(at),
                sv_report::one_line(signed)
            ),
            Some(Standing::NotKept { at, lines: n }) => format!(
                "{}: CHANGED since your checkpoint of {}. Of the {} you signed for, some were changed or cut.",
                kind.said(),
                sv_report::one_line(at),
                match n {
                    1 => "1 line".to_owned(),
                    n => format!("{n} lines"),
                }
            ),
            Some(Standing::Unchecked { at, why }) => format!(
                "{}: a checkpoint of {} is there and could not be taken as yours: {}. It is read as no checkpoint.",
                kind.said(),
                sv_report::one_line(at),
                sv_report::one_line(why)
            ),
        });
    }
    out
}

/// The six groups, as the report's last table: how many lines each holds, the lines no section above has shown, and
/// what the group could not show. An empty group always says why.
fn group_lines(all: &[groups::Gathered]) -> Vec<String> {
    let mut out = Vec::new();
    for g in all {
        // The lines of these two groups come from what was held to the task file and from git, both listed above.
        let listed_above = |line: &groups::Line| match g.group {
            groups::Group::Gaming | groups::Group::Outside => true,
            groups::Group::Oversight => {
                !line.place.contains("of the record") && !line.place.contains("record's own")
            }
            _ => false,
        };
        let above = g.lines.iter().filter(|l| listed_above(l)).count();
        out.push(format!(
            "{}: {}{}",
            g.group.label(),
            match g.lines.len() {
                0 => "nothing seen in what was read".to_owned(),
                1 => "1 line".to_owned(),
                n => format!("{n} lines"),
            },
            match above {
                0 => "",
                _ => " (listed above)",
            }
        ));
        for line in g.lines.iter().filter(|l| !listed_above(l)) {
            out.push(format!(
                "    {}{} ({}; from {})",
                line.verdict
                    .map(|v| format!("{}: ", v.label()))
                    .unwrap_or_default(),
                sv_report::one_line(&line.what),
                sv_report::one_line(&line.place),
                line.from
            ));
        }
        for gap in &g.not_shown {
            out.push(format!("    could not be shown: {gap}"));
        }
    }
    out
}

fn print(read: &Read, folder: &Path) {
    println!(
        "What the record shows the AI coding tool did in {}",
        folder.display()
    );
    println!(
        "av decides nothing about the tool. It says what each record shows, and what it could not see."
    );

    println!();
    println!("The task (agentvet.toml)");
    for line in envelope_lines(&read.envelope) {
        println!("  {line}");
    }
    if let Some(held) = &read.held {
        for line in held_lines(held) {
            println!("  {line}");
        }
    }

    println!();
    println!("The tool's calls to sv (the build-loop record)");
    println!("  How far to rely on it: {}", witness::BUILD_LOOP);
    println!("  {}", sv_report::build_loop_said(&read.build_loop));
    for line in findings_lines(&read.build_loop) {
        println!("  {line}");
    }

    println!();
    println!("What the tool ran and wrote (the hook record)");
    match &read.hook {
        None => println!("  Not read: it is read only when you ask, with --hook-record."),
        Some(seen) => {
            println!("  How far to rely on it: {}", witness::HOOK);
            let (lines, command) = sv_report::hook::sentences(seen, &[]);
            for line in lines {
                let line = match command {
                    Some(c) => line.replace(sv_report::hook::COMMAND, &sv_report::one_line(c)),
                    None => line,
                };
                println!("  {line}");
            }
        }
    }

    println!();
    println!("What is worth a look (from git, and the hook record when it was read)");
    println!("  How far to rely on it: {}", witness::GIT);
    for line in sv_report::tool_behavior::sentences(&read.behavior) {
        println!("  {line}");
    }
    for seen in &read.behavior.seen {
        println!(
            "  {}: {} ({})",
            seen.kind.label(),
            sv_report::one_line(&seen.what),
            sv_report::one_line(&seen.place)
        );
    }

    println!();
    println!("What you signed for (checkpoints)");
    for line in checkpoint_lines(&read.checkpoints) {
        println!("  {line}");
    }

    println!();
    println!("By kind of thing");
    println!(
        "  The same lines under the six things av looks for. A line is for you to read, and an empty kind is not a \
         clean one: each says what it could not show."
    );
    for line in group_lines(&read.groups) {
        println!("  {line}");
    }

    println!();
    println!("Not looked at");
    for gap in &read.not_looked_at {
        println!("  {gap}");
    }
}

fn json(read: &Read) -> serde_json::Value {
    serde_json::json!({
        "av": env!("CARGO_PKG_VERSION"),
        "envelope": match &read.envelope {
            envelope::Read::NotThere => serde_json::json!({ "read": "not-there" }),
            envelope::Read::Unreadable(why) => serde_json::json!({ "read": "unreadable", "why": why }),
            envelope::Read::Held(e) => serde_json::json!({
                "read": "held",
                "whose": envelope::MAY_BE_THE_TOOLS,
                "envelope": e,
                "guards": e.guards(),
                "questions": e.questions(),
                "held_to": read.held,
            }),
        },
        "build_loop": {
            "witness": witness::BUILD_LOOP,
            "record": read.build_loop,
        },
        "hook_record": read.hook.as_ref().map(|seen| serde_json::json!({
            "witness": witness::HOOK,
            "record": seen,
        })),
        "worth_a_look": {
            "witness": witness::GIT,
            "seen": read.behavior,
        },
        "checkpoints": read.checkpoints.iter().map(|(kind, standing)| serde_json::json!({
            "record": kind,
            "read": standing.is_some(),
            "standing": standing,
        })).collect::<Vec<_>>(),
        "groups": read.groups,
        "not_looked_at": read.not_looked_at,
    })
}

fn run(options: &Options) -> u8 {
    if !options.folder.is_dir() {
        eprintln!("av: {} is not a folder", options.folder.display());
        return FAILED;
    }
    let read = read(&options.folder, options.hook_record);
    if options.json {
        match serde_json::to_string_pretty(&json(&read)) {
            Ok(text) => println!("{text}"),
            Err(why) => {
                eprintln!("av: the report could not be written as JSON: {why}");
                return FAILED;
            }
        }
    } else {
        print(&read, &options.folder);
    }
    if read.anything() {
        MADE
    } else {
        if !options.json {
            println!();
            println!(
                "Nothing here is about the tool: no call to sv was written down, no commit names a tool{}. That is \
                 not a sign that no tool worked here.",
                if options.hook_record {
                    ", and the hook record holds nothing"
                } else {
                    ""
                }
            );
        }
        NOTHING_READ
    }
}

#[cfg(test)]
mod tests;
