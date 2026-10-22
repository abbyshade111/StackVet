//! What the AI coding tool did that is worth a look (backlog 0630, ADR-131), read for a report from git, the hook record
//! when the person consented to it for this run, and the last report the folder held, when `sv` can show it wrote it.
//!
//! It reads what changed, not what `sv` finds now, so a tool that games a check cannot make its own line vanish by
//! gaming it: the finding that went away is the line. It decides nothing, and says what it could not look for.

use serde_json::Value;
use std::path::Path;
use sv_check::git::changes::Commit;
use sv_report::tool_behavior::{Kind, Seen, ToolBehavior};

/// Commits read when there is no earlier report to read from, or its commit is not in HEAD's history.
pub const LAST_COMMITS: usize = 20;
/// Lines listed, all kinds together; the rest are counted.
pub const MOST_LINES: usize = 60;

/// The last report's findings and the commit it was made at, read before this report replaces it.
#[derive(Debug, Clone, Default)]
pub struct EarlierReport {
    pub commit: Option<String>,
    pub findings: Vec<Was>,
}

/// A finding of the last report, as much of it as the section says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Was {
    pub fingerprint: String,
    pub rule_id: String,
    pub title: String,
    pub file: String,
    pub line: u64,
}

/// The last report in `out_dir`, when `sv` can show it wrote it and left it unchanged.
pub fn earlier(out_dir: &Path) -> Option<EarlierReport> {
    let bytes = crate::report_seal::proven_report(out_dir).ok()?;
    let json: Value = serde_json::from_slice(&bytes).ok()?;
    Some(earlier_of(&json))
}

/// The same, from a report's JSON.
pub fn earlier_of(json: &Value) -> EarlierReport {
    let findings = json["findings"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|f| f["severity"].as_str() != Some("info"))
        .filter_map(|f| {
            Some(Was {
                fingerprint: f["fingerprint"]
                    .as_str()
                    .filter(|s| !s.is_empty())?
                    .to_owned(),
                rule_id: f["rule_id"].as_str().unwrap_or_default().to_owned(),
                title: f["title"].as_str().unwrap_or_default().to_owned(),
                file: f["location"]["file"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                line: f["location"]["line"].as_u64().unwrap_or(0),
            })
        })
        .collect();
    EarlierReport {
        commit: json["tool_behavior"]["at_commit"]
            .as_str()
            .map(str::to_owned),
        findings,
    }
}

/// The section for `report`, of the app at `app_dir`.
pub fn read(
    app_dir: &Path,
    earlier: Option<&EarlierReport>,
    report: &sv_report::Report,
) -> ToolBehavior {
    let commits = sv_check::git::changes::since(
        app_dir,
        earlier.and_then(|e| e.commit.as_deref()),
        LAST_COMMITS,
    );
    let done = report
        .hook_record
        .as_ref()
        .and_then(|_| crate::hook_record::done_for(app_dir));
    let now = held(&report.findings, &report.set_aside);
    let mut t = of(
        commits.as_ref().map(|(c, _)| c.as_slice()),
        done.as_deref(),
        earlier,
        &now,
    );
    t.at_commit = sv_check::git::head_commit(app_dir);
    t.commits_read = commits.as_ref().map(|(c, from_found)| {
        let n = c.len();
        let short = |h: &str| h.chars().take(8).collect::<String>();
        match (earlier.and_then(|e| e.commit.as_deref()), from_found) {
            (Some(at), true) if n == 0 => {
                format!("no commits since the last report (made at {})", short(at))
            }
            (Some(at), true) => format!(
                "the {} since the last report (made at {})",
                plural(n, "commit"),
                short(at)
            ),
            (Some(at), false) => format!(
                "{}, since the commit the last report was made at ({}) is not in this branch's history",
                last(n),
                short(at)
            ),
            (None, _) => format!(
                "{}, as there is no earlier report made at a known commit",
                last(n)
            ),
        }
    });
    if commits.is_none() {
        t.not_seen.push(
            "anything git would show, since the app's folder is not a git repository with commits, or git could not \
             be read"
                .to_owned(),
        );
    }
    match (&report.hook_record, &done) {
        (None, _) => t.not_seen.push(
            "the commands the tool ran and the files it wrote without committing them, since the hook record was not \
             read for this run (`sv report --hook-record` reads it, once its hook is installed)"
                .to_owned(),
        ),
        (Some(_), None) => t.not_seen.push(
            "the commands the tool ran, since there is no hook record for this app on this computer".to_owned(),
        ),
        _ => {}
    }
    if earlier.is_none() {
        t.not_seen.push(
            "findings that went away, since this folder held no earlier report that sv can show it wrote".to_owned(),
        );
    }
    t.not_seen.push(
        "packages installed that no lockfile holds, which this section does not look for yet (backlog 0630)"
            .to_owned(),
    );
    t
}

/// The fingerprints a report still holds: each finding's own and its earlier ones, and those of findings a person set
/// aside, which went from the findings by the person's word and not by the tool's change.
pub fn held<'a>(
    findings: &'a [sv_check::Finding],
    set_aside: &'a [sv_check::review::SetAside],
) -> Vec<&'a str> {
    findings
        .iter()
        .flat_map(|f| {
            std::iter::once(f.fingerprint.as_str())
                .chain(f.earlier_fingerprints.iter().map(String::as_str))
        })
        .chain(set_aside.iter().map(|s| s.finding.fingerprint.as_str()))
        .filter(|f| !f.is_empty())
        .collect()
}

/// What the commits, the hook record's lines, and the last report show, against the fingerprints this report holds.
pub fn of(
    commits: Option<&[Commit]>,
    done: Option<&[crate::hook_record::Done]>,
    earlier: Option<&EarlierReport>,
    now: &[&str],
) -> ToolBehavior {
    let commits = commits.unwrap_or_default();
    let tools: Vec<&Commit> = commits.iter().filter(|c| c.tool.is_some()).collect();
    let mut seen = Vec::new();
    let earlier_files: Vec<&str> = earlier
        .map(|e| e.findings.iter().map(|f| f.file.as_str()).collect())
        .unwrap_or_default();
    for c in tools.iter().rev() {
        let place = place_of(c);
        let mut push = |kind, what: String| {
            seen.push(Seen {
                kind,
                what,
                place: place.clone(),
            })
        };
        let code_files = c
            .files
            .iter()
            .filter(|(_, p)| is_code(p) && !is_test(p))
            .count();
        for (status, path) in &c.files {
            if is_answers(path) {
                let lines = answer_lines(c, path);
                push(
                    Kind::AnswersChanged,
                    format!("{path} {}{lines}", changed(*status)),
                );
            }
            if is_test(path) && *status == 'D' {
                push(
                    Kind::TestsTouched,
                    format!("the test file {path} was deleted"),
                );
            }
            if is_test(path) && *status == 'M' && code_files > 0 {
                let expected = |lines: &[(String, String)]| {
                    lines.iter().any(|(f, l)| f == path && is_expectation(l))
                };
                if expected(&c.removed) && expected(&c.added) {
                    push(
                        Kind::TestsTouched,
                        format!(
                            "a line the test {path} expects was changed in the same commit as {} of code",
                            plural(code_files, "file")
                        ),
                    );
                }
            }
            if is_guard(path) {
                push(Kind::GuardEdited, format!("{path} {}", changed(*status)));
            }
        }
        for (file, line) in &c.added {
            let text = line.trim();
            if let Some(marker) = SKIP_MARKERS.iter().find(|m| text.contains(**m)) {
                push(
                    Kind::TestsTouched,
                    format!("a test was marked to be skipped in {file} ({marker})"),
                );
            }
            if let Some(marker) = LOOK_AWAY_MARKERS.iter().find(|m| text.contains(**m)) {
                let named = if earlier_files.contains(&file.as_str()) {
                    "; a finding in the last report named this file"
                } else {
                    ""
                };
                push(
                    Kind::GuardEdited,
                    format!(
                        "a marker that tells a check to look away ({marker}) was added in {file}{named}"
                    ),
                );
            }
        }
    }
    for d in done.unwrap_or_default() {
        match d {
            crate::hook_record::Done::Ran { at, command } => {
                if let Some(why) = risky(command) {
                    seen.push(Seen {
                        kind: Kind::Command,
                        what: format!("{why}: {command}"),
                        place: format!("the hook record, {at}"),
                    });
                }
            }
            crate::hook_record::Done::Wrote { at, file } => {
                let kind = if is_answers(file) {
                    Some(Kind::AnswersChanged)
                } else if is_guard(file) {
                    Some(Kind::GuardEdited)
                } else {
                    None
                };
                if let Some(kind) = kind {
                    seen.push(Seen {
                        kind,
                        what: format!("the tool wrote {file}"),
                        place: format!("the hook record, {at}"),
                    });
                }
            }
        }
    }
    for was in earlier.map(|e| e.findings.as_slice()).unwrap_or_default() {
        if now.contains(&was.fingerprint.as_str()) {
            continue;
        }
        let touched: Vec<String> = tools
            .iter()
            .rev()
            .filter(|c| c.files.iter().any(|(_, p)| *p == was.file))
            .map(|c| place_of(c))
            .collect();
        let wrote = done.unwrap_or_default().iter().any(
            |d| matches!(d, crate::hook_record::Done::Wrote { file, .. } if *file == was.file),
        );
        let place = match (touched.as_slice(), wrote, done.is_some()) {
            ([], true, _) => format!("{} was written by the tool, by the hook record", was.file),
            ([], false, true) => format!(
                "no commit by an AI coding tool changed {}, and the hook record shows the tool writing nothing to it",
                was.file
            ),
            ([], false, false) => format!("no commit by an AI coding tool changed {}", was.file),
            (commits, _, _) => format!("{} changed in {}", was.file, commits.join("; ")),
        };
        let at = if was.line > 0 {
            format!("{}:{}", was.file, was.line)
        } else {
            was.file.clone()
        };
        seen.push(Seen {
            kind: Kind::FindingGone,
            what: format!(
                "\"{}\" ({}) at {at} was in the last report and is not in this one",
                was.title, was.rule_id
            ),
            place,
        });
    }
    seen.sort_by_key(|s| s.kind);
    let more = seen.len().saturating_sub(MOST_LINES);
    seen.truncate(MOST_LINES);
    ToolBehavior {
        tool_commits: tools.len(),
        other_commits: commits.len() - tools.len(),
        seen,
        more,
        ..Default::default()
    }
}

fn place_of(c: &Commit) -> String {
    let subject: String = c.subject.chars().take(60).collect();
    format!(
        "commit {} \"{subject}\", by {}",
        c.hash.chars().take(8).collect::<String>(),
        c.tool.as_deref().unwrap_or("a tool not named")
    )
}

fn changed(status: char) -> &'static str {
    match status {
        'A' => "was added",
        'D' => "was deleted",
        _ => "was changed",
    }
}

/// "the last commit", or "the last 3 commits".
fn last(n: usize) -> String {
    if n == 1 {
        "the last commit".to_owned()
    } else {
        format!("the last {n} commits")
    }
}

fn plural(n: usize, word: &str) -> String {
    if n == 1 {
        format!("1 {word}")
    } else {
        format!("{n} {word}s")
    }
}

/// The lines of an answers file the commit changed that lower what the app is held to or switch something off.
fn answer_lines(c: &Commit, path: &str) -> String {
    let keys = ["audience", "categories", "level", "= false"];
    let pick = |lines: &[(String, String)]| -> Vec<String> {
        lines
            .iter()
            .filter(|(f, l)| f == path && keys.iter().any(|k| l.contains(k)))
            .map(|(_, l)| l.trim().to_owned())
            .take(3)
            .collect()
    };
    let (was, is) = (pick(&c.removed), pick(&c.added));
    let mut out = String::new();
    if !was.is_empty() {
        out.push_str(&format!("; it read {}", was.join(", ")));
    }
    if !is.is_empty() {
        out.push_str(&format!("; it now reads {}", is.join(", ")));
    }
    out
}

/// The owner's answers, under either of the names `sv` reads them by (ADR-062).
fn is_answers(path: &str) -> bool {
    matches!(file_name(path), "stackvet.toml" | "securevibe.toml")
}

fn file_name(path: &str) -> &str {
    path.rsplit(['/', '\\']).next().unwrap_or(path)
}

/// A test file, by where it is or what it is called.
pub fn is_test(path: &str) -> bool {
    let name = file_name(path).to_ascii_lowercase();
    path.split(['/', '\\'])
        .rev()
        .skip(1)
        .any(|d| matches!(d, "test" | "tests" | "__tests__" | "spec" | "specs"))
        || name.starts_with("test_")
        || name.contains("_test.")
        || name.contains(".test.")
        || name.contains(".spec.")
        || name.contains("_spec.")
}

fn is_code(path: &str) -> bool {
    const KINDS: [&str; 12] = [
        ".py", ".js", ".jsx", ".ts", ".tsx", ".go", ".rb", ".java", ".kt", ".php", ".rs", ".cs",
    ];
    KINDS.iter().any(|k| path.ends_with(k))
}

/// A line that says what a test expects.
fn is_expectation(line: &str) -> bool {
    ["assert", "expect(", "toBe(", "toEqual(", "should."]
        .iter()
        .any(|w| line.contains(w))
}

/// A check's settings, or the workflow that runs the checks.
pub fn is_guard(path: &str) -> bool {
    let name = file_name(path);
    path.starts_with(".github/workflows/")
        || path.starts_with(".gitlab-ci")
        || matches!(
            name,
            ".semgrepignore"
                | ".semgrep.yml"
                | ".semgrep.yaml"
                | ".flake8"
                | ".bandit"
                | "ruff.toml"
                | ".ruff.toml"
                | ".pre-commit-config.yaml"
                | ".gitleaks.toml"
                | ".gitleaksignore"
                | ".snyk"
                | ".trivyignore"
                | ".eslintignore"
        )
        || name.starts_with(".eslintrc")
        || name.starts_with("eslint.config.")
}

/// Markers that tell a test runner to skip a test.
const SKIP_MARKERS: [&str; 9] = [
    "@pytest.mark.skip",
    "@unittest.skip",
    "pytest.skip(",
    "it.skip(",
    "test.skip(",
    "describe.skip(",
    "xit(",
    "xdescribe(",
    "t.Skip(",
];

/// Markers that tell a scanner, linter, or type checker to look away from a line or a file.
const LOOK_AWAY_MARKERS: [&str; 8] = [
    "nosec",
    "nosemgrep",
    "eslint-disable",
    "@ts-ignore",
    "@ts-nocheck",
    "type: ignore",
    "noqa",
    "NOSONAR",
];

/// What a command does that is worth a look, when it is one of these: it destroys work, steps around a guard, runs a
/// script straight from the network, or runs as the computer's administrator.
pub fn risky(command: &str) -> Option<&'static str> {
    let c = format!(
        " {} ",
        command.split_whitespace().collect::<Vec<_>>().join(" ")
    );
    let has = |s: &str| c.contains(s);
    if has(" git push ") && (has(" --force") || has(" -f ")) {
        return Some("a force-push, which can replace commits already pushed");
    }
    if has(" git reset --hard") {
        return Some("a hard reset, which throws away changes not committed");
    }
    if has(" git clean -")
        && c.split(" git clean ")
            .nth(1)
            .is_some_and(|r| r.contains('f'))
    {
        return Some("a git clean, which deletes files git does not track");
    }
    if has(" --no-verify ") {
        return Some("a commit or push that steps around the repository's own hooks");
    }
    if has(" git branch -D ") {
        return Some("a branch deleted whether or not it was merged");
    }
    if (has(" curl ") || has(" wget ")) && command.contains('|') {
        let after = command.rsplit('|').next().unwrap_or_default();
        let runner = after.split_whitespace().next().unwrap_or_default();
        if matches!(
            runner,
            "sh" | "bash" | "zsh" | "python" | "python3" | "node" | "sudo"
        ) {
            return Some("a script run straight from the network");
        }
    }
    if has(" chmod 777 ") || has(" chmod -R 777 ") || has(" chmod a+rwx ") {
        return Some("a file opened to every user of the computer");
    }
    if c.trim_start().starts_with("sudo ") || has(" && sudo ") || has("; sudo ") {
        return Some("a command run as the computer's administrator");
    }
    if let Some(rest) = c.split(" rm ").nth(1) {
        let words: Vec<&str> = rest
            .split_whitespace()
            .take_while(|w| !matches!(*w, "&&" | ";" | "|"))
            .collect();
        let recursive = words
            .iter()
            .any(|w| w.starts_with('-') && (w.contains('r') || w.contains('R')));
        let targets: Vec<&&str> = words.iter().filter(|w| !w.starts_with('-')).collect();
        if targets.iter().any(|t| is_test(t)) {
            return Some("a test file removed");
        }
        if recursive && targets.iter().any(|t| !is_build_folder(t)) {
            return Some("a folder deleted with everything in it");
        }
    }
    None
}

/// A folder a build or a package manager makes again, whose deletion is housekeeping.
fn is_build_folder(path: &str) -> bool {
    let last = path
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(path);
    matches!(
        last,
        "target"
            | "node_modules"
            | "dist"
            | "build"
            | ".next"
            | "__pycache__"
            | ".pytest_cache"
            | ".venv"
            | "venv"
            | "coverage"
            | ".mypy_cache"
            | ".ruff_cache"
    )
}
