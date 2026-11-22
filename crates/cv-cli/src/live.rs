//! `cv live github <owner>/<repo>`: a repository's settings, read from GitHub with a token from the environment
//! (ADR-157, decisions 5 and 6, and its Later entry of 21 November 2026; backlog 0717). Never run by `cv check`.
//!
//! The token comes from `GH_TOKEN`, or else `GITHUB_TOKEN`; never from an option, since options are kept in shell
//! history. It is handed to `curl` on its standard input and appears nowhere else: the terminal and `report.json` show
//! only its first characters and its length.

use cv_live::github::{self, Reading, Repo};
use cv_live::judge::{judge, token_words};
use cv_rules::outcome::Evidence;
use std::path::{Path, PathBuf};

pub const USAGE: &str = "\
  cv live github <owner>/<repo> [--out <folder>] [--fail-on attention|not-assessed]

cv live github reads one GitHub repository's settings with the token in GH_TOKEN (or GITHUB_TOKEN): the rules on its
default branch, what workflows may do, which actions may run, its environments, and secret scanning. Every request is a
GET to api.github.com, and each is listed in the report. A classic token that can write is refused. GitHub does not
say a fine-grained token's permissions, so cv cannot confirm one is read-only: make it with read-only access to the
repository's Administration and Actions, and nothing else. It writes report.json into --out, by default ./cv-live-report.";

pub struct Options {
    pub repo: String,
    pub out: Option<PathBuf>,
    pub fail_on: super::FailOn,
}

pub fn parse(args: &[String]) -> Result<Options, String> {
    let mut rest = args.iter();
    match rest.next().map(String::as_str) {
        Some("github") => {}
        Some(other) => {
            return Err(format!(
                "cv live knows one provider, github; {other} is not one yet"
            ));
        }
        None => return Err("cv live needs a provider: cv live github <owner>/<repo>".to_owned()),
    }
    let mut repo = None;
    let mut out = None;
    let mut fail_on = super::FailOn::Nothing;
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--out" => out = Some(PathBuf::from(rest.next().ok_or("--out needs a folder")?)),
            "--fail-on" => {
                fail_on = match rest.next().map(String::as_str) {
                    Some("attention") => super::FailOn::Attention,
                    Some("not-assessed") => super::FailOn::NotAssessed,
                    _ => return Err("--fail-on takes `attention` or `not-assessed`".to_owned()),
                }
            }
            "--token" => {
                return Err("cv never takes a token as an option, since options are kept in shell history; put it \
                            in GH_TOKEN"
                    .to_owned());
            }
            other if other.starts_with("--") => {
                return Err(format!("cv live does not know {other}"));
            }
            other if repo.is_none() => repo = Some(other.to_owned()),
            other => return Err(format!("one repository at a time; {other} is a second")),
        }
    }
    Ok(Options {
        repo: repo.ok_or("cv live github needs a repository: <owner>/<repo>")?,
        out,
        fail_on,
    })
}

/// The token from the environment, `GH_TOKEN` first; `None` when neither holds one.
fn token() -> Option<String> {
    ["GH_TOKEN", "GITHUB_TOKEN"]
        .iter()
        .filter_map(|name| std::env::var(name).ok())
        .map(|t| t.trim().to_owned())
        .find(|t| !t.is_empty())
}

pub fn run(options: &Options) -> u8 {
    let repo = match Repo::parse(&options.repo) {
        Ok(repo) => repo,
        Err(why) => {
            eprintln!("cv: {why}");
            return super::FAILED;
        }
    };
    let Some(token) = token() else {
        eprintln!(
            "cv: no token in GH_TOKEN or GITHUB_TOKEN, so nothing was read. Make a fine-grained token with read-only \
             access to {} (Administration and Actions), and put it in GH_TOKEN.",
            repo.full()
        );
        return super::FAILED;
    };
    let mut curl = match cv_live::Curl::new() {
        Ok(curl) => curl,
        Err(why) => {
            eprintln!("cv: {why}");
            return super::FAILED;
        }
    };
    let reading = github::read(&repo, &token, &mut curl);
    drop(token);
    let evidence = judge(&reading);
    print!("{}", listing(&reading, &evidence));
    let out = options
        .out
        .clone()
        .unwrap_or_else(|| PathBuf::from("cv-live-report"));
    match write_report(&out, &reading, &evidence) {
        Ok(path) => println!("\nWrote {}", path.display()),
        Err(why) => {
            eprintln!("cv: {why}");
            return super::FAILED;
        }
    }
    exit_code(&reading, &evidence, &options.fail_on)
}

fn exit_code(reading: &Reading, evidence: &Evidence, fail_on: &super::FailOn) -> u8 {
    if *fail_on == super::FailOn::Attention && !evidence.findings.is_empty() {
        return super::ATTENTION;
    }
    // A reading that stopped settled nothing, whatever was asked.
    if reading.stopped.is_some()
        || (*fail_on == super::FailOn::NotAssessed
            && (!evidence.not_assessed.is_empty() || !reading.unread.is_empty()))
    {
        return super::NOT_ASSESSED;
    }
    super::CLEAN
}

/// What the terminal says: the token first, before any finding (ADR-157, decision 5), then each setting.
fn listing(reading: &Reading, evidence: &Evidence) -> String {
    let mut out = format!("cv live github {}\n", reading.repository);
    out.push_str(&format!(
        "Token: {} ({}). {}\n",
        reading.token.token.as_str(),
        match reading.token.kind {
            github::TokenKind::Classic => "a classic token",
            github::TokenKind::FineGrained => "a fine-grained token",
            github::TokenKind::Installation => "a GitHub App's token",
            github::TokenKind::Unknown => "a token of a kind cv does not know",
        },
        token_words(reading)
    ));
    out.push_str(&format!(
        "Asked GitHub {} times, each a GET to api.github.com.\n",
        reading.requests.len()
    ));
    if let Some(why) = &reading.stopped {
        out.push_str(&format!("Stopped: {why}\n"));
    }
    for f in &evidence.findings {
        out.push_str(&format!("\n  finding        {} ({})\n", f.title, f.rule_id));
        out.push_str(&format!("                 {}\n", f.description));
    }
    for v in &evidence.verified {
        out.push_str(&format!(
            "\n  checked in part {} ({})\n",
            v.scope, v.check_id
        ));
    }
    for n in &evidence.not_assessed {
        out.push_str(&format!(
            "\n  not assessed   {} ({}): {}\n",
            n.unit, n.rule_id, n.why
        ));
    }
    for u in &reading.unread {
        out.push_str(&format!("  not read       {u}\n"));
    }
    out.push_str(&format!(
        "\n{} findings, {} credits (each in part), {} not assessed.\n",
        evidence.findings.len(),
        evidence.verified.len(),
        evidence.not_assessed.len()
    ));
    out
}

/// Writes `report.json` into `out`, refusing a folder that holds anything else, as `cv check` does.
fn write_report(out: &Path, reading: &Reading, evidence: &Evidence) -> Result<PathBuf, String> {
    if out.exists() {
        let entries = std::fs::read_dir(out)
            .map_err(|e| format!("{} could not be opened: {e}", out.display()))?;
        for entry in entries.flatten() {
            if entry.file_name() != "report.json" {
                return Err(format!(
                    "{} holds files cv did not write, so cv will not write its report there; give --out a new folder",
                    out.display()
                ));
            }
        }
    } else {
        std::fs::create_dir_all(out)
            .map_err(|e| format!("{} could not be made: {e}", out.display()))?;
    }
    let report = serde_json::json!({
        "tool": "cv",
        "version": env!("CARGO_PKG_VERSION"),
        "live": {
            "reading": reading,
            "token_said": token_words(reading),
            "evidence": evidence,
        },
    });
    let path = out.join("report.json");
    let text = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n")
        .map_err(|e| format!("{} could not be written: {e}", path.display()))?;
    Ok(path)
}
