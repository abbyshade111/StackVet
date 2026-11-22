//! `cv live github <owner>/<repo>`: a repository's settings, read from GitHub with a token from the environment
//! (ADR-157, decisions 5 and 6, and its Later entry of 21 November 2026; backlog 0717). `cv live cloudflare <zone>`: a
//! zone's settings, read from Cloudflare the same way (its Later entry of 22 November 2026; backlog 0732). Never run by
//! `cv check`.
//!
//! The token comes from the environment, never from an option, since options are kept in shell history: `GH_TOKEN`, or
//! else `GITHUB_TOKEN`, for GitHub; `CLOUDFLARE_API_TOKEN` alone for Cloudflare. It is handed to `curl` on its standard
//! input and appears nowhere else: the terminal and `report.json` show only its first characters and its length.

use cv_live::cloudflare::{self, Zone};
use cv_live::github::{self, Reading, Repo};
use cv_live::judge::{judge, token_words};
use cv_rules::outcome::Evidence;
use std::path::{Path, PathBuf};

pub const USAGE: &str = "\
  cv live github <owner>/<repo> [--out <folder>] [--fail-on attention|not-assessed]
  cv live cloudflare <zone> [--out <folder>] [--fail-on attention|not-assessed]

cv live github reads one GitHub repository's settings with the token in GH_TOKEN (or GITHUB_TOKEN): the rules on its
default branch, what workflows may do, which actions may run, its environments, and secret scanning. Every request is a
GET to api.github.com, and each is listed in the report. A classic token that can write is refused. GitHub does not
say a fine-grained token's permissions, so cv cannot confirm one is read-only: make it with read-only access to the
repository's Administration and Actions, and nothing else.

cv live cloudflare reads one Cloudflare zone's settings with the token in CLOUDFLARE_API_TOKEN: its SSL/TLS mode, the
lowest TLS version, TLS 1.3, Always Use HTTPS, HSTS, Automatic HTTPS Rewrites, and its cache rules. Every request is a
GET to api.cloudflare.com, and each is listed in the report. A Global API Key and Wrangler's own sign-in token are
refused before anything is sent. Cloudflare does not show a token its own permissions unless it may read every API
token, which cv does not ask for, so cv cannot confirm the token is read-only: make it with Zone Read, Zone Settings
Read, and Cache Rules Read for the one zone, and nothing else.

Each writes report.json into --out, by default ./cv-live-report.";

/// Which account to read.
pub enum Target {
    GitHub(String),
    Cloudflare(String),
}

pub struct Options {
    pub target: Target,
    pub out: Option<PathBuf>,
    pub fail_on: super::FailOn,
}

pub fn parse(args: &[String]) -> Result<Options, String> {
    let mut rest = args.iter();
    let provider = match rest.next().map(String::as_str) {
        Some(p @ ("github" | "cloudflare")) => p,
        Some(other) => {
            return Err(format!(
                "cv live knows two providers, github and cloudflare; {other} is not one yet"
            ));
        }
        None => {
            return Err(
                "cv live needs a provider: cv live github <owner>/<repo>, or cv live cloudflare <zone>".to_owned(),
            );
        }
    };
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
                return Err(format!(
                    "cv never takes a token as an option, since options are kept in shell history; put it in {}",
                    if provider == "github" {
                        "GH_TOKEN"
                    } else {
                        cloudflare::TOKEN_ENV
                    }
                ));
            }
            other if other.starts_with("--") => {
                return Err(format!("cv live does not know {other}"));
            }
            other if repo.is_none() => repo = Some(other.to_owned()),
            other => return Err(format!("one at a time; {other} is a second")),
        }
    }
    let target = match provider {
        "github" => {
            Target::GitHub(repo.ok_or("cv live github needs a repository: <owner>/<repo>")?)
        }
        _ => {
            Target::Cloudflare(repo.ok_or("cv live cloudflare needs a zone, such as stackvet.dev")?)
        }
    };
    Ok(Options {
        target,
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
    match &options.target {
        Target::GitHub(repo) => run_github(repo, options),
        Target::Cloudflare(zone) => run_cloudflare(zone, options),
    }
}

fn run_github(repo: &str, options: &Options) -> u8 {
    let repo = match Repo::parse(repo) {
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
    let mut curl = match cv_live::Curl::new(github::HEADERS) {
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

fn run_cloudflare(zone: &str, options: &Options) -> u8 {
    let zone = match Zone::parse(zone) {
        Ok(zone) => zone,
        Err(why) => {
            eprintln!("cv: {why}");
            return super::FAILED;
        }
    };
    let Some(token) = std::env::var(cloudflare::TOKEN_ENV)
        .ok()
        .map(|t| t.trim().to_owned())
        .filter(|t| !t.is_empty())
    else {
        eprintln!(
            "cv: no token in {}, so nothing was read. Make a Cloudflare API token with Zone Read, Zone Settings Read, \
             and Cache Rules Read for {} only, and put it in {}.",
            cloudflare::TOKEN_ENV,
            zone.0,
            cloudflare::TOKEN_ENV
        );
        return super::FAILED;
    };
    let mut curl = match cv_live::Curl::new(cloudflare::HEADERS) {
        Ok(curl) => curl,
        Err(why) => {
            eprintln!("cv: {why}");
            return super::FAILED;
        }
    };
    let reading = cloudflare::read(&zone, &token, &mut curl);
    drop(token);
    let evidence = cv_live::cloudflare_judge::judge(&reading);
    let said = cv_live::cloudflare_judge::token_words(&reading);
    print!("{}", cloudflare_listing(&reading, &evidence, &said));
    let out = options
        .out
        .clone()
        .unwrap_or_else(|| PathBuf::from("cv-live-report"));
    let report = serde_json::json!({
        "tool": "cv",
        "version": env!("CARGO_PKG_VERSION"),
        "live": {
            "reading": &reading,
            "token_said": said,
            "evidence": &evidence,
        },
    });
    match write_json(&out, &report) {
        Ok(path) => println!("\nWrote {}", path.display()),
        Err(why) => {
            eprintln!("cv: {why}");
            return super::FAILED;
        }
    }
    if options.fail_on == super::FailOn::Attention && !evidence.findings.is_empty() {
        return super::ATTENTION;
    }
    if reading.stopped.is_some()
        || (options.fail_on == super::FailOn::NotAssessed
            && (!evidence.not_assessed.is_empty() || !reading.unread.is_empty()))
    {
        return super::NOT_ASSESSED;
    }
    super::CLEAN
}

/// What the terminal says for a Cloudflare zone: the token first, before any finding, then each setting as read.
fn cloudflare_listing(reading: &cloudflare::Reading, evidence: &Evidence, said: &str) -> String {
    let mut out = format!("cv live cloudflare {}\n", reading.zone);
    out.push_str(&format!(
        "Token: {}. {said}\n",
        reading.token.token.as_str()
    ));
    out.push_str(&format!(
        "Asked Cloudflare {} times, each a GET to {}.\n",
        reading.requests.len(),
        cloudflare::HOST
    ));
    if let Some(why) = &reading.stopped {
        out.push_str(&format!("Stopped: {why}\n"));
    }
    if !reading.settings.values.is_empty() {
        out.push_str("\nAs read:\n");
        for (id, value) in &reading.settings.values {
            out.push_str(&format!(
                "  {id:<26} {}\n",
                cv_live::cloudflare_judge::shown(value)
            ));
        }
    }
    if let Some(rules) = &reading.settings.cache_rules {
        out.push_str(&format!(
            "  {:<26} {} rule(s)\n",
            "cache rules",
            rules.len()
        ));
        for rule in rules {
            out.push_str(&format!(
                "    {}{}: {}\n",
                if rule.description.is_empty() {
                    "(no name)"
                } else {
                    &rule.description
                },
                if rule.enabled { "" } else { " (off)" },
                rule.expression
            ));
        }
    }
    findings_and_credits(&mut out, evidence);
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

/// The findings, the credits, and what was not assessed, as both providers' listings show them.
fn findings_and_credits(out: &mut String, evidence: &Evidence) {
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
    findings_and_credits(&mut out, evidence);
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

/// Writes GitHub's `report.json` into `out`.
fn write_report(out: &Path, reading: &Reading, evidence: &Evidence) -> Result<PathBuf, String> {
    let report = serde_json::json!({
        "tool": "cv",
        "version": env!("CARGO_PKG_VERSION"),
        "live": {
            "reading": reading,
            "token_said": token_words(reading),
            "evidence": evidence,
        },
    });
    write_json(out, &report)
}

/// Writes `report` as `report.json` into `out`, refusing a folder that holds anything else, as `cv check` does.
fn write_json(out: &Path, report: &serde_json::Value) -> Result<PathBuf, String> {
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
    let path = out.join("report.json");
    let text = serde_json::to_string_pretty(report).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n")
        .map_err(|e| format!("{} could not be written: {e}", path.display()))?;
    Ok(path)
}
