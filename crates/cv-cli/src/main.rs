//! `cv`, ConfigVet's command (ADR-157; backlogs 0687 and 0693). One command, `cv check <folder>`, which reads the
//! folder's deployment files, prints what each rule settled, runs the outside tools over the same files (hadolint,
//! Checkov, and Trivy, `tools`), and writes `report.json`. Neither it nor the tools as it runs them open a network
//! connection.
//!
//! Exit codes follow `sv`'s (ADR-029): 0 when the run finished; 1 only with `--fail-on attention` and a finding; 2 when
//! a deployment file could not be read or the folder holds none (always), or with `--fail-on not-assessed` and anything
//! not assessed; 3 when `cv` itself failed and there is no result. 1 outranks 2.

mod live;
mod tools;

use cv_rules::outcome::Outcome;
use cv_rules::{Checked, check, defaults::Table};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const CLEAN: u8 = 0;
const ATTENTION: u8 = 1;
const NOT_ASSESSED: u8 = 2;
const FAILED: u8 = 3;

const USAGE: &str = "\
cv: ConfigVet reviews the files that say how an app is deployed.

Usage:
  cv check <folder> [--out <folder>] [--fail-on attention|not-assessed] [--no-tools]
  cv live github <owner>/<repo> [--out <folder>] [--fail-on attention|not-assessed]
  cv live cloudflare <zone> [--out <folder>] [--fail-on attention|not-assessed]
  cv --version

cv check reads the folder's Dockerfiles, compose files, Kubernetes manifests, and Terraform plans (as JSON), with the
app's package.json and Django settings, and says for each setting whether it is set safely, set unsafely, missing with
a safe or an unsafe default, or not assessed. It then runs hadolint, Checkov, and Trivy over the same files, where
they are installed, unless --no-tools is given; a tool that is not installed is said so, never counted as a pass. It
writes report.json into --out, by default <folder>/cv-report, and opens no network connection.

cv live github and cv live cloudflare, only when asked for by name, read one GitHub repository's settings with the
token in GH_TOKEN, or one Cloudflare zone's with the token in CLOUDFLARE_API_TOKEN: every request a GET to
api.github.com or api.cloudflare.com, each listed in its report. See `cv live --help`.";

#[derive(Debug, PartialEq, Eq)]
enum FailOn {
    Nothing,
    Attention,
    NotAssessed,
}

#[derive(Debug)]
struct Options {
    folder: PathBuf,
    out: Option<PathBuf>,
    fail_on: FailOn,
    tools: bool,
}

fn parse(args: &[String]) -> Result<Options, String> {
    let mut folder = None;
    let mut out = None;
    let mut fail_on = FailOn::Nothing;
    let mut tools = true;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--no-tools" => tools = false,
            "--out" => out = Some(PathBuf::from(rest.next().ok_or("--out needs a folder")?)),
            "--fail-on" => {
                fail_on = match rest.next().map(String::as_str) {
                    Some("attention") => FailOn::Attention,
                    Some("not-assessed") => FailOn::NotAssessed,
                    _ => return Err("--fail-on takes `attention` or `not-assessed`".to_owned()),
                }
            }
            other if other.starts_with("--") => {
                return Err(format!("cv check does not know {other}"));
            }
            other if folder.is_none() => folder = Some(PathBuf::from(other)),
            other => return Err(format!("one folder at a time; {other} is a second")),
        }
    }
    Ok(Options {
        folder: folder.ok_or("cv check needs a folder")?,
        out,
        fail_on,
        tools,
    })
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--version" | "-V") => {
            println!("cv {}", env!("CARGO_PKG_VERSION"));
            ExitCode::from(CLEAN)
        }
        Some("check") => match parse(&args[1..]) {
            Ok(options) => ExitCode::from(run(&options)),
            Err(why) => {
                eprintln!("cv: {why}\n\n{USAGE}");
                ExitCode::from(FAILED)
            }
        },
        Some("live") => match args.get(1).map(String::as_str) {
            Some("--help" | "-h") | None => {
                println!("Usage:\n{}", live::USAGE);
                ExitCode::from(if args.len() > 1 { CLEAN } else { FAILED })
            }
            _ => match live::parse(&args[1..]) {
                Ok(options) => ExitCode::from(live::run(&options)),
                Err(why) => {
                    eprintln!("cv: {why}\n\nUsage:\n{}", live::USAGE);
                    ExitCode::from(FAILED)
                }
            },
        },
        Some("--help" | "-h" | "help") => {
            println!("{USAGE}");
            ExitCode::from(CLEAN)
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(FAILED)
        }
    }
}

fn run(options: &Options) -> u8 {
    if !options.folder.is_dir() {
        eprintln!("cv: {} is not a folder", options.folder.display());
        return FAILED;
    }
    let table = match Table::load() {
        Ok(table) => table,
        Err(why) => {
            eprintln!("cv: the table of defaults could not be read: {why}. Nothing was checked.");
            return FAILED;
        }
    };
    let mut checked = check(&options.folder, &table);
    let tools = if options.tools {
        match (tools::load(), sv_check::secrets::SecretRules::shared()) {
            (Ok(adapters), Ok(rules)) => Some(tools::run(
                &adapters,
                &options.folder,
                &checked.read,
                &mut checked.evidence.findings,
                rules,
            )),
            (Err(why), _) => {
                eprintln!(
                    "cv: the outside tools' descriptions could not be read: {why:#}. Nothing was checked."
                );
                return FAILED;
            }
            (_, Err(why)) => {
                eprintln!(
                    "cv: the rules that keep secrets out of a tool's words could not be read: {why}. Nothing was checked."
                );
                return FAILED;
            }
        }
    } else {
        None
    };
    print!("{}", listing(&options.folder, &checked));
    print!("{}", tool_listing(tools.as_ref()));
    let out = options
        .out
        .clone()
        .unwrap_or_else(|| options.folder.join("cv-report"));
    match write_report(&out, &checked, tools.as_ref()) {
        Ok(path) => println!("\nWrote {}", path.display()),
        Err(why) => {
            eprintln!("cv: {why}");
            return FAILED;
        }
    }
    exit_code(&checked, tools.as_ref(), &options.fail_on)
}

fn exit_code(checked: &Checked, tools: Option<&tools::Tools>, fail_on: &FailOn) -> u8 {
    let tool_findings = tools.is_some_and(|t| !t.findings.is_empty());
    if *fail_on == FailOn::Attention && (!checked.evidence.findings.is_empty() || tool_findings) {
        return ATTENTION;
    }
    let gaps = !checked.unread.is_empty() || checked.read.is_empty();
    // A tool that did not run, or ran over part of what it was given, left something not assessed.
    let tool_gaps = tools.is_some_and(|t| t.tools.iter().any(|l| l.status != "ran"));
    if gaps
        || (*fail_on == FailOn::NotAssessed
            && (!checked.evidence.not_assessed.is_empty() || tool_gaps))
    {
        return NOT_ASSESSED;
    }
    CLEAN
}

/// What the terminal says of the outside tools: each one's run, then what they found that `cv`'s rules did not.
fn tool_listing(tools: Option<&tools::Tools>) -> String {
    let Some(tools) = tools else {
        return "\nThe outside tools were not run (--no-tools).\n".to_owned();
    };
    let mut out = String::from("\nOutside tools\n");
    for line in &tools.tools {
        let version = line
            .version
            .as_deref()
            .map(|v| format!(" ({v})"))
            .unwrap_or_default();
        out.push_str(&format!("  {:<10} {}{version}", line.name, line.status));
        if line.status != "not run" {
            out.push_str(&format!(
                ": {} findings, {} of them also cv's own",
                line.findings, line.agreed
            ));
        }
        out.push('\n');
        for said in &line.said {
            out.push_str(&format!("  {:<10} {said}\n", ""));
        }
    }
    for f in &tools.findings {
        out.push_str(&format!(
            "  {} ({}) at {} line {}\n",
            f.title, f.rule_id, f.location.file, f.location.line
        ));
    }
    out
}

/// What a person reads on the terminal: what was read, then each setting with its outcome.
fn listing(folder: &Path, checked: &Checked) -> String {
    let mut out = format!("cv check {}\n", folder.display());
    if checked.read.is_empty() {
        out.push_str(
            "No Dockerfile, compose file, Kubernetes manifest, or Terraform plan was found, so no deployment setting \
             was assessed.\n",
        );
    } else {
        out.push_str(&format!("Read {} deployment files:\n", checked.read.len()));
        for (layer, file) in &checked.read {
            out.push_str(&format!("  {file} ({})\n", layer.name()));
        }
    }
    for unread in &checked.unread {
        out.push_str(&format!("Could not read {}: {}\n", unread.file, unread.why));
    }
    let mut last_rule = "";
    for one in &checked.assessed {
        if one.rule_id != last_rule {
            out.push_str(&format!("\n{} ({})\n", one.setting, one.rule_id));
            last_rule = one.rule_id;
        }
        let detail = match &one.outcome {
            Outcome::SetSafe { value, at } | Outcome::SetUnsafe { value, at } => {
                format!("`{value}` at {} line {}", at.file, at.line)
            }
            Outcome::MissingDefaultSafe { defaults }
            | Outcome::MissingDefaultUnsafe { defaults } => defaults
                .iter()
                .map(|d| format!("{}: {} ({})", d.subject, d.default, d.sources[0].url))
                .collect::<Vec<_>>()
                .join("; "),
            Outcome::Unknown { why } => why.clone(),
        };
        out.push_str(&format!(
            "  {:<24} {}: {detail}\n",
            one.outcome.name(),
            one.unit
        ));
        if let Some(by) = one.reported_by {
            out.push_str(&format!("  {:<24} `sv check` reports this as {by}\n", ""));
        }
    }
    out.push_str(&format!(
        "\n{} findings, {} credits (each in part), {} not assessed.\n",
        checked.evidence.findings.len(),
        checked.evidence.verified.len(),
        checked.evidence.not_assessed.len()
    ));
    out
}

/// Writes `report.json` into `out`, refusing a folder that holds anything else, so `cv` never writes over a person's
/// own files.
fn write_report(
    out: &Path,
    checked: &Checked,
    tools: Option<&tools::Tools>,
) -> Result<PathBuf, String> {
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
        "checked": checked,
        "outside_tools": tools,
    });
    let path = out.join("report.json");
    let text = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n")
        .map_err(|e| format!("{} could not be written: {e}", path.display()))?;
    Ok(path)
}
