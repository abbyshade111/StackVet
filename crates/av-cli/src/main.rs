//! `av`, AgentVet's command (ADR-159; backlog 0705, part 2). One command so far, `av report <folder>`, which reads what
//! is already kept about the AI coding tool that worked in a folder and says what the record shows, at which
//! strength, and what it could not see. It decides nothing about the tool, and it never says a tool can be trusted.
//!
//! What it reads: the build-loop record (`sv`'s own list of the tool's calls to it, ADR-076), git's history, and, only
//! when the person asks for it with `--hook-record`, the hook record (the commands the tool ran and the files it
//! wrote, ADR-122). It reads no transcript, so it says nothing of what the tool claimed beyond what the hook record
//! keeps. It also reads the task envelope, `agentvet.toml`, when the folder has one, and says what it holds and what
//! it leaves as a question; nothing is held to it yet (backlog 0715, parts 2 and 3). It writes nothing and opens no
//! network connection.
//!
//! Exit codes follow `sv`'s (ADR-029): 0 when the report was made; 2 when there was nothing to read, so nothing was
//! said about the tool; 3 when `av` itself failed and there is no result.

use av_record::{build_loop, envelope, hook_record, tool_behavior};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

// Everything `av` prints goes through `sv_report::visible`: a file name, a command, or a commit message is the
// tool's text, and an escape character in one must not reach the terminal. These shadow the standard macros in
// every module of this crate below them, as `sv`'s do.
macro_rules! println {
    () => { ::std::println!() };
    ($($arg:tt)*) => { ::std::println!("{}", ::sv_report::visible(&::std::format!($($arg)*))) };
}
macro_rules! eprintln {
    ($($arg:tt)*) => { ::std::eprintln!("{}", ::sv_report::visible(&::std::format!($($arg)*))) };
}

const MADE: u8 = 0;
const NOTHING_READ: u8 = 2;
const FAILED: u8 = 3;

const USAGE: &str = "\
av: AgentVet says what the record shows an AI coding tool did.

Usage:
  av report <folder> [--hook-record] [--json]
  av --version

av report reads what is kept about the tool that worked in the folder: sv's record of the tool's calls to it, and
git's history. With --hook-record it also reads the record of the commands the tool ran and the files it wrote, which
a hook you installed keeps outside the folder (`sv connect claude --hook` prints it); it is read only when you ask,
each time. It says what each record shows and how far each can be relied on, and what it could not see. When the
folder holds a task file, agentvet.toml, it says what that allows and what it leaves unsaid. It reads no transcript,
decides nothing, writes nothing, and opens no network connection. --json prints the same as JSON.";

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
    build_loop: sv_report::BuildLoop,
    /// The hook record, when the person asked for it.
    hook: Option<sv_report::hook::HookSeen>,
    behavior: sv_report::tool_behavior::ToolBehavior,
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
        "What the tool said it did, against what it did: av reads no transcript yet. The hook record keeps one kind of \
         claim, that the app's tests pass, and says above when the record shows otherwise."
            .to_owned(),
        match envelope::read(folder) {
            envelope::Read::Held(_) => {
                "Whether the tool kept to its task: the task file was read, and nothing is held to it yet."
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
    Read {
        envelope: envelope::read(folder),
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
        "It may not change{}: {}.",
        if e.guards.files.is_none() {
            " (the list av uses when the file names none)"
        } else {
            ""
        },
        listed(&e.guards())
    ));
    for question in e.questions() {
        out.push(format!("Not said, so not assumed either way: {question}"));
    }
    out.push(
        "Nothing is held to this yet: av reads the file, and does not yet compare what the tool did with it."
            .to_owned(),
    );
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
                "held_to": false,
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
