//! `cv`, ConfigVet's command (ADR-157; backlog 0687). Part 2 has one command, `cv check <folder>`, which reads the
//! folder's deployment files, prints what each rule settled, and writes `report.json`. It opens no network connection.
//!
//! Exit codes follow `sv`'s (ADR-029): 0 when the run finished; 1 only with `--fail-on attention` and a finding; 2 when
//! a deployment file could not be read or the folder holds none (always), or with `--fail-on not-assessed` and anything
//! not assessed; 3 when `cv` itself failed and there is no result. 1 outranks 2.

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
  cv check <folder> [--out <folder>] [--fail-on attention|not-assessed]
  cv --version

cv check reads the folder's Dockerfiles, compose files, Kubernetes manifests, and Terraform plans (as JSON), with the
app's package.json and Django settings, and says for each setting whether it is set safely, set unsafely, missing with
a safe or an unsafe default, or not assessed. It writes report.json into --out, by default <folder>/cv-report, and
opens no network connection.";

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
}

fn parse(args: &[String]) -> Result<Options, String> {
    let mut folder = None;
    let mut out = None;
    let mut fail_on = FailOn::Nothing;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
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
    let checked = check(&options.folder, &table);
    print!("{}", listing(&options.folder, &checked));
    let out = options
        .out
        .clone()
        .unwrap_or_else(|| options.folder.join("cv-report"));
    match write_report(&out, &checked) {
        Ok(path) => println!("\nWrote {}", path.display()),
        Err(why) => {
            eprintln!("cv: {why}");
            return FAILED;
        }
    }
    exit_code(&checked, &options.fail_on)
}

fn exit_code(checked: &Checked, fail_on: &FailOn) -> u8 {
    if *fail_on == FailOn::Attention && !checked.evidence.findings.is_empty() {
        return ATTENTION;
    }
    let gaps = !checked.unread.is_empty() || checked.read.is_empty();
    if gaps || (*fail_on == FailOn::NotAssessed && !checked.evidence.not_assessed.is_empty()) {
        return NOT_ASSESSED;
    }
    CLEAN
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
fn write_report(out: &Path, checked: &Checked) -> Result<PathBuf, String> {
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
    });
    let path = out.join("report.json");
    let text = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n")
        .map_err(|e| format!("{} could not be written: {e}", path.display()))?;
    Ok(path)
}
