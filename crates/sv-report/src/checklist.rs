//! The checklist before going live (backlog 0589, ADR-121): `before-going-live.md`, and a section of `report.html`.
//!
//! What a person must do or confirm before the app is on the internet was already in the report, spread across it in
//! the order the requirements come: the "Before going live" list, the decisions nobody has made, the findings left,
//! the settings the run was given, and what could not be verified. This puts it in the order of a deployment, one
//! thing to do or confirm per line, each with the requirements it serves, from what the report holds and nothing else.
//!
//! It credits nothing. A line is a thing to do, not a check that passed, and the file says so at its top and its end;
//! the last part lists what the report could not verify at all, so a finished list is never read as a pass.

use crate::{Report, Status};
use serde::Serialize;

/// The file's name in the report folder.
pub const FILE: &str = "before-going-live.md";

/// One part of the checklist, in the order of a deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Part {
    /// Before the first deploy, at the host: what the app is given there.
    AtTheHost,
    /// After the first deploy, against the live address.
    AfterTheDeploy,
    /// The decisions that settle whether a requirement applies.
    Decide,
    /// What the report found and is still there, so the person goes live knowing it.
    SignOff,
}

impl Part {
    pub const ALL: [Part; 4] = [
        Part::AtTheHost,
        Part::AfterTheDeploy,
        Part::Decide,
        Part::SignOff,
    ];

    /// The part's heading.
    pub fn heading(self) -> &'static str {
        match self {
            Part::AtTheHost => "1. Before the first deploy, at your host",
            Part::AfterTheDeploy => "2. After the first deploy, at the live address",
            Part::Decide => "3. Decisions still open",
            Part::SignOff => "4. What is still there, to go live knowing it",
        }
    }

    /// What the part is, in a sentence, said when it has lines.
    fn about(self) -> &'static str {
        match self {
            Part::AtTheHost => {
                "What the live copy of the app is given where it runs: its settings, its keys, and its files."
            }
            Part::AfterTheDeploy => {
                "What only the app's own live site can answer. `sv probe` makes a handful of read-only requests to the \
                 address you give it and nothing else; it prints its answers and adds nothing to the report."
            }
            Part::Decide => {
                "Each of these decides whether a requirement applies to the app. Answer it in `stackvet.toml`, or with \
                 your AI coding tool, and write the report again."
            }
            Part::SignOff => {
                "What the checks found and is still there. Fix each one, or set it aside in `sv review` as a risk you \
                 accept, so the app goes live with you knowing it."
            }
        }
    }
}

/// One line of the checklist: one thing to do or confirm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Line {
    pub part: Part,
    /// What to do, in `sv`'s words.
    pub what: String,
    /// A command that does it or asks it, when there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// The file of the app it is about, when there is one: the app's own text, shown as code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    /// The requirements it serves; empty when it serves none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub requirements: Vec<String>,
}

/// The checklist of `report`, in the order of a deployment.
pub fn lines(report: &Report) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut handled = Vec::new();
    at_the_host(report, &mut lines, &mut handled);
    after_the_deploy(report, &mut lines);
    decide(report, &mut lines);
    sign_off(report, &handled, &mut lines);
    lines
}

/// How many things found are left for the last part: those the first part does not name.
fn left_count(report: &Report) -> usize {
    let mut handled = Vec::new();
    at_the_host(report, &mut Vec::new(), &mut handled);
    left(report, &handled).len()
}

/// The requirements among `ids` that apply to the app and that nothing has settled.
fn open(report: &Report, ids: &[&str]) -> Vec<String> {
    ids.iter()
        .filter(|id| {
            report
                .requirements
                .iter()
                .any(|r| r.id == **id && !matches!(r.status, Status::Checked | Status::ByHand))
        })
        .map(|id| (*id).to_owned())
        .collect()
}

/// The setting a gap says the run's copy of the app was loosened with (`loosened_for_the_run` in `sv-cli`), if it is
/// one of those gaps.
fn loosened_setting(gap: &crate::Gap) -> Option<&str> {
    if gap.reason != crate::GapReason::LeftOut {
        return None;
    }
    let rest = gap.why.strip_prefix("The copy `sv` ran had `")?;
    let (setting, _) = rest.split_once("` set in its start command")?;
    Some(setting)
}

/// The findings whose rule is one of `rules`, or starts with one ending in a dot.
fn findings_of<'a>(report: &'a Report, rules: &[&str]) -> Vec<&'a sv_check::Finding> {
    report
        .findings
        .iter()
        .filter(|f| !f.apart())
        .filter(|f| {
            rules.iter().any(|r| match r.strip_suffix('.') {
                Some(prefix) => f.rule_id.starts_with(&format!("{prefix}.")),
                None => f.rule_id == *r,
            })
        })
        .collect()
}

fn place_of(f: &sv_check::Finding) -> Option<String> {
    f.location
        .is_file()
        .then(|| format!("{}:{}", f.location.file, f.location.line))
}

fn at_the_host(report: &Report, lines: &mut Vec<Line>, handled: &mut Vec<String>) {
    let mut line = |what: String, place: Option<String>, requirements: Vec<String>| {
        lines.push(Line {
            part: Part::AtTheHost,
            what,
            command: None,
            place,
            requirements,
        })
    };

    // HTTPS is set at the host before the live site can answer anything about it.
    if report.on_the_internet {
        let tls = open(report, &["V12.2.1", "V12.2.2", "V12.1.1"]);
        if !tls.is_empty() {
            line(
                "Turn on HTTPS at your host, with a certificate browsers trust, send visitors who come over plain \
                 HTTP to HTTPS, and turn off the old versions of TLS (1.0 and 1.1). Most hosts have these on one \
                 settings page."
                    .to_owned(),
                None,
                tls,
            );
        }
    }

    // The settings the copy `sv` ran was started with, which the live copy must not have.
    let mut settings: Vec<(&str, Vec<String>)> = Vec::new();
    for gap in &report.gaps {
        let Some(setting) = loosened_setting(gap) else {
            continue;
        };
        match settings.iter_mut().find(|(s, _)| *s == setting) {
            Some((_, requirements)) => requirements.extend(gap.requirements.iter().cloned()),
            None => settings.push((setting, gap.requirements.clone())),
        }
    }
    for (setting, mut requirements) in settings {
        requirements.sort();
        requirements.dedup();
        line(
            format!(
                "Make sure the live app does not run with `{setting}`. The copy `sv` ran had it in its start \
                 command, which loosens the app, so the checks it bears on were not credited."
            ),
            None,
            requirements,
        );
    }

    // A key in the code: the live app gets its own, set at the host.
    for f in findings_of(report, &["secrets."]) {
        handled.push(f.fingerprint.clone());
        line(
            format!(
                "Take the key out of the code and set it at your host instead, then replace it with a new one: one \
                 that has been in the code is no longer secret. ({})",
                f.title
            ),
            place_of(f),
            f.requirement_ids.clone(),
        );
    }

    // Debug mode shows the app's insides to anyone who causes an error.
    for f in findings_of(report, &["config.debug-mode", "ast.debug-mode-on"]) {
        handled.push(f.fingerprint.clone());
        line(
            "Turn debug mode off for the live app: it shows the app's insides to anyone who causes an error."
                .to_owned(),
            place_of(f),
            f.requirement_ids.clone(),
        );
    }

    // The exact versions the app was tested with are the ones the live app installs.
    let pinned = findings_of(report, &["config.versions-pinned"]);
    if let Some(f) = pinned.first() {
        handled.extend(pinned.iter().map(|f| f.fingerprint.clone()));
        line(
            "Keep a lockfile with the app, naming the exact version of every package, so the live app installs \
             what you tested."
                .to_owned(),
            place_of(f),
            f.requirement_ids.clone(),
        );
    }

    // Somewhere for people to tell the owner about a problem they find.
    let contact = findings_of(
        report,
        &[
            "config.security-contact",
            "config.security-contact-placeholder",
        ],
    );
    if let Some(f) = contact.first() {
        handled.extend(contact.iter().map(|f| f.fingerprint.clone()));
        line(
            "Put a real address in SECURITY.md where people can tell you about a security problem they find."
                .to_owned(),
            place_of(f),
            f.requirement_ids.clone(),
        );
    }
}

fn after_the_deploy(report: &Report, lines: &mut Vec<Line>) {
    for item in &report.before_going_live {
        lines.push(Line {
            part: Part::AfterTheDeploy,
            what: match &item.by_hand {
                Some(by_hand) => format!("{}. {by_hand}", item.what),
                None => format!("{}.", item.what),
            },
            command: item.command.clone(),
            place: None,
            requirements: vec![item.id.clone()],
        });
    }
    if report.on_the_internet {
        lines.push(Line {
            part: Part::AfterTheDeploy,
            what: "If the app has accounts: on the live site, sign up, sign in, sign out, and change a password the \
                   way a person would, and see each works. The checks before going live tried them on a copy on \
                   this computer, not on your host."
                .to_owned(),
            command: None,
            place: None,
            requirements: Vec::new(),
        });
    }
}

fn decide(report: &Report, lines: &mut Vec<Line>) {
    for un in &report.undecided {
        lines.push(Line {
            part: Part::Decide,
            what: format!(
                "{} (It decides whether this applies: {})",
                un.blocked_on.join(" Or: "),
                un.description
            ),
            command: None,
            place: None,
            requirements: vec![un.id.clone()],
        });
    }
}

/// The things to fix that the first part does not already name, in the reports' order.
fn left<'a>(report: &'a Report, handled: &[String]) -> Vec<&'a sv_check::Finding> {
    let (app, _) = crate::app_then_tests(report);
    let (to_fix, _) = crate::fix_then_notes(app);
    to_fix
        .into_iter()
        .filter(|f| !handled.contains(&f.fingerprint))
        .collect()
}

fn sign_off(report: &Report, handled: &[String], lines: &mut Vec<Line>) {
    let mut low = 0;
    for f in left(report, handled) {
        if f.severity == sv_check::Severity::Low && crate::accepted_note(report, f).is_none() {
            low += 1;
            continue;
        }
        let what = match report.set_aside.iter().find(|s| {
            s.verdict == sv_check::review::ACCEPTED_RISK
                && s.finding.fingerprint == f.fingerprint
                && s.finding.rule_id == f.rule_id
        }) {
            Some(accepted) => format!(
                "{} ({}). Accepted as a risk by {} on {}: confirm you still accept it for the live app.",
                f.title,
                f.severity.name(),
                accepted.by,
                accepted.on
            ),
            None => format!(
                "{} ({}). Fix it, or accept it as a risk.",
                f.title,
                f.severity.name()
            ),
        };
        lines.push(Line {
            part: Part::SignOff,
            what,
            command: None,
            place: place_of(f),
            requirements: f.requirement_ids.clone(),
        });
    }
    if low > 0 {
        lines.push(Line {
            part: Part::SignOff,
            what: format!(
                "{low} low-severity {} not listed here one by one: look through {}, and fix or accept each.",
                if low == 1 { "finding," } else { "findings," },
                crate::LOW_FINDINGS_FILE
            ),
            command: None,
            place: None,
            requirements: Vec::new(),
        });
    }
}

/// What the report could not verify at all, as plain lines with no box: the last part, so a finished checklist is not
/// read as a pass.
pub fn not_verified(report: &Report) -> Vec<String> {
    let c = &report.counts;
    let mut out = vec![format!(
        "{} of the {} requirements that apply to this app were not verified by anything: not by a check, a test, \
         your notes, or your answers. compliance.md lists each.",
        c.not_verified, c.applicable
    )];
    if let Some(line) = crate::bluf::not_run_line(report) {
        out.push(line);
    }
    let gaps = report.gaps.len();
    if gaps > 0 {
        out.push(format!(
            "{gaps} {} what this report could not examine, and why: \"What was not examined\" in report.html and \
             compliance.md.",
            if gaps == 1 { "limit says" } else { "limits say" }
        ));
    }
    out
}

/// What the file says first: what it is, and what it is not.
fn opening(report: &Report) -> String {
    let mut out = format!(
        "This is what to do or confirm before people use {} on the internet, in the order you would do it, drawn from \
         what this report found. It credits nothing: a line done here is not a check, and nothing on it counts toward \
         any requirement. Ticking a box in this file records nothing; the file is written again with \
         each report. `sv before-live` prints it again from the last report.",
        report.app_name
    );
    if !report.on_the_internet {
        out.push_str(
            " stackvet.toml does not say the app will be on the internet, so what only a live site can answer is \
             left out; set `deployment = \"internet\"` there and write the report again if it will be.",
        );
    }
    out
}

/// The checklist as `before-going-live.md`.
pub fn render(report: &Report) -> String {
    use crate::markdown::{code, inert};
    let mut out = format!(
        "# Before going live: {}\n\n",
        inert(&report.app_name).replace(['\n', '\r'], " ")
    );
    // Every form of the report names the `sv` that wrote it, and when: the checklist is that report's.
    match &report.generated {
        Some(when) => out.push_str(&format!(
            "Produced by `sv` {} on {when}.\n\n",
            report.sv.describe()
        )),
        None => out.push_str(&format!("Produced by `sv` {}.\n\n", report.sv.describe())),
    }
    out.push_str(&crate::folder::markdown_line(FILE));
    out.push_str(&inert(&opening(report)));
    out.push_str("\n\n");
    let all = lines(report);
    for part in Part::ALL {
        out.push_str(&format!("## {}\n\n", part.heading()));
        let mine: Vec<&Line> = all.iter().filter(|l| l.part == part).collect();
        if mine.is_empty() {
            out.push_str("Nothing here for this app.\n\n");
            continue;
        }
        out.push_str(&inert(part.about()));
        out.push_str("\n\n");
        for line in mine {
            out.push_str("- [ ] ");
            out.push_str(&inert(&line.what));
            if let Some(command) = &line.command {
                out.push_str(&format!(" Run {}.", code(command)));
            }
            if let Some(place) = &line.place {
                out.push_str(&format!(" At {}.", code(place)));
            }
            if !line.requirements.is_empty() {
                out.push_str(&format!(" ({})", line.requirements.join(", ")));
            }
            out.push('\n');
        }
        out.push('\n');
    }
    out.push_str("## 5. What this report could not verify\n\n");
    out.push_str(
        "Nothing on the list above covers these. Finishing it does not make the app secure; it means you go live \
         knowing what was checked and what was not.\n\n",
    );
    for line in not_verified(report) {
        out.push_str(&format!("- {}\n", inert(&line)));
    }
    out
}

/// `sv`'s words for a page: escaped, with each `` `span` `` shown as code.
pub(crate) fn text(words: &str) -> String {
    words
        .split('`')
        .enumerate()
        .map(|(i, piece)| {
            let piece = crate::html::escape(piece);
            if i % 2 == 1 {
                format!("<code>{piece}</code>")
            } else {
                piece
            }
        })
        .collect()
}

/// The checklist as a section of `report.html`.
pub fn html(report: &Report) -> String {
    use crate::html::escape;
    let mut b = String::from("<h2>The checklist before going live</h2>\n");
    b.push_str(&format!(
        "<p>{} It is also <a href=\"{FILE}\">{FILE}</a> in this folder.</p>\n",
        text(&opening(report))
    ));
    let all = lines(report);
    for part in Part::ALL {
        let mine: Vec<&Line> = all.iter().filter(|l| l.part == part).collect();
        b.push_str(&format!("<h3>{}</h3>\n", escape(part.heading())));
        if mine.is_empty() {
            b.push_str("<p>Nothing here for this app.</p>\n");
            continue;
        }
        b.push_str(&format!(
            "<p>{}</p>\n<ul class=\"checklist\">\n",
            text(part.about())
        ));
        if part == Part::SignOff {
            // The page lists each finding in full above; here they are counted, not repeated.
            let n = left_count(report);
            b.push_str(&format!(
                "<li>&#9744; {}</li>\n</ul>\n",
                text(&format!(
                    "{n} {} still there: fix each one, or accept it as a risk, from the things to fix above. `{FILE}` \
                     lists them one by one.",
                    if n == 1 { "thing found is" } else { "things found are" }
                ))
            ));
            continue;
        }
        for line in mine {
            let mut li = format!("&#9744; {}", text(&line.what));
            if let Some(command) = &line.command {
                li.push_str(&format!(" Run <code>{}</code>.", escape(command)));
            }
            if let Some(place) = &line.place {
                li.push_str(&format!(" At <code>{}</code>.", escape(place)));
            }
            if !line.requirements.is_empty() {
                li.push_str(&format!(" ({})", escape(&line.requirements.join(", "))));
            }
            b.push_str(&format!("<li>{li}</li>\n"));
        }
        b.push_str("</ul>\n");
    }
    b.push_str("<h3>5. What this report could not verify</h3>\n<ul>\n");
    for line in not_verified(report) {
        b.push_str(&format!("<li>{}</li>\n", text(&line)));
    }
    b.push_str("</ul>\n");
    b
}
