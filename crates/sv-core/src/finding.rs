//! What a check found, and what it is evidence about.
//!
//! Two things are deliberate here and both come from v1's rules.
//!
//! **A secret never travels in a finding.** `Secret` cannot be constructed with the value visible: it
//! redacts on the way in, and there is no accessor that gives the original back. A finding is written to
//! reports, logs and SARIF, and a scanner that reports a credential has copied it somewhere new.
//!
//! **A check that knows which requirements it verifies says so.** `requirement_ids` is not decoration: it
//! is the link between "this rule fired" and "this ASVS requirement has evidence about it". A finding with
//! an empty list is evidence about nothing in particular, which is a fair thing to be, but it has to be
//! visible rather than assumed.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn name(self) -> &'static str {
        match self {
            Severity::Critical => "critical",
            Severity::High => "high",
            Severity::Medium => "medium",
            Severity::Low => "low",
            Severity::Info => "info",
        }
    }
}

/// How sure the rule is, kept separate from how bad it would be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

/// A credential that has been found. The value is redacted on construction and cannot be recovered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Secret {
    /// Enough to recognize it in the file, never enough to use it.
    redacted: String,
    /// How long the original was, which is sometimes the only way to tell two findings apart.
    length: usize,
    /// Whether the original was made of words, such as `violet-canoe-marble-ledger`, rather than random characters
    /// (`crate::secrets::reads_as_words`). Kept for `Finding::is_test_information`, never written to a report.
    #[serde(skip)]
    reads_as_words: bool,
}

impl Secret {
    /// Keeps the first four characters and says how much was dropped. Four is enough to match a key
    /// against the one in a password manager, and far short of enough to authenticate with.
    ///
    /// Never more than a third of the value, though: four characters of a four-character password in
    /// a web address were the whole of it (the review of 8 October 2026, item 6).
    pub fn redact(value: &str) -> Self {
        let total = value.chars().count();
        Secret {
            redacted: crate::text::cut_with_count(value, 4.min(total / 3)),
            length: total,
            reads_as_words: crate::words::reads_as_words(value),
        }
    }

    /// Whether the original value was made of words rather than random characters.
    pub fn reads_as_words(&self) -> bool {
        self.reads_as_words
    }

    pub fn as_str(&self) -> &str {
        &self.redacted
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

/// The rules whose findings are shown beside a requirement's credit rather than over it, and whose
/// review as a false alarm leaves that credit standing. Each says so in its own text; a rule goes on
/// this list only when it does, so the report never contradicts the finding it shows.
pub const INFORMATION_ONLY: &[&str] = &[crate::rule_ids::NAME_MISMATCH];

/// The rules whose findings in test code are information, not findings (ADR-091, approved by the owner on
/// 11 October 2026): Bandit's assert (B101) and its password-looking strings (B105, B106, B107). An assert is how
/// a test is written, and a password string in a test file is usually a fixture. Such a finding is listed in its
/// own section, for the owner to review, and it does not count. The secrets scan's rules are not on this list,
/// so a real-looking key in a test file still counts. One more is information, by the owner's answer of
/// 15 October 2026 (backlog 0529; ADR-091, Later): a credential assignment in test code whose value is made of words,
/// such as `violet-canoe-marble-ledger` (`Finding::is_test_information`).
pub const TEST_FILE_INFORMATION: &[&str] =
    &["bandit.B101", "bandit.B105", "bandit.B106", "bandit.B107"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Location {
    pub file: String,
    /// 1-indexed, as an editor counts.
    pub line: usize,
}

impl Location {
    /// The place given for a finding about the running app, which has no file and no line.
    pub const RUNNING_APP: &'static str = "the running app";
    /// The place given for a finding in what the running app printed.
    pub const RUNNING_APP_OUTPUT: &'static str = "the running app's output";

    /// A finding about the running app. Line 1, because every report shows a line; it means none.
    pub fn running_app() -> Self {
        Location {
            file: Self::RUNNING_APP.to_owned(),
            line: 1,
        }
    }

    /// A finding in the running app's output.
    pub fn running_app_output() -> Self {
        Location {
            file: Self::RUNNING_APP_OUTPUT.to_owned(),
            line: 1,
        }
    }

    /// Whether this names a file of the app, rather than the running app or its output. A tool
    /// that wants a file address (SARIF's) must not be handed one of these as if it were a path.
    pub fn is_file(&self) -> bool {
        !matches!(
            self.file.as_str(),
            Self::RUNNING_APP | Self::RUNNING_APP_OUTPUT
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub title: String,
    pub severity: Severity,
    pub confidence: Confidence,
    pub location: Location,
    /// What was found, redacted. Absent for rules that report a situation rather than a value.
    pub secret: Option<Secret>,
    /// The requirements this finding is evidence about. Empty is allowed and means exactly that.
    pub requirement_ids: Vec<String>,
    pub cwe: Vec<String>,
    /// Plain language, for somebody who is not a programmer.
    pub description: String,
    pub impact: String,
    pub fix: String,
    /// Other rules that reported the same kind of weakness on the same line, merged into this one so
    /// the owner reads it once. See `merge_same_place`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub also_reported_by: Vec<String>,
    /// The answers the running app gave that this finding rests on, by their ids in `seen.json`
    /// (`home`, `cors`, `signed-in-3`): what a person checks the finding against (ADR-082, backlog
    /// 0229, part 1). Empty for a finding that rests on no answer the running app gave.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<String>,
    /// What an owner's review names the finding by: see `crate::review::fingerprint`. Empty until the
    /// report fills it in.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub fingerprint: String,
    /// What the same finding was named before its fingerprint changed form (deep review A2): the
    /// earlier form, when it differs from `fingerprint`. For a tool that tracks findings by
    /// fingerprint across runs (cato-pipeline's POA&M), so it can carry an item over rather than
    /// close it and open another. Identical lines shared one earlier fingerprint, so it can name
    /// several findings; a tracker should give it to the first and treat the rest as new.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub earlier_fingerprints: Vec<String>,
    /// Known to be test or sample code from more than its file's name: on a line Rust builds only for
    /// its tests (`mark_rust_test_code`), or in a folder the manifest says is not the app
    /// (`mark_not_the_app`). False until the report looks.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub marked_test_code: bool,
    /// The library, and its version when it says, when the file is a copy of another project's
    /// library kept in the app, such as `jQuery 3.6.1` in `public/js/jquery.min.js`
    /// (`bundled::mark_bundled_libraries`). Listed apart from the app's own code, never hidden:
    /// it still counts. `None` until the report looks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundled_library: Option<String>,
    /// The other problems found on the same line of the same file, each whole, gathered into this
    /// one so the owner reads the line once (`one_per_line`). This one is the most severe of them;
    /// it names every requirement and CWE of the others, and each keeps its own rule, words, and
    /// fingerprint here. Empty until the report gathers them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub also_on_this_line: Vec<Finding>,
    /// The outside tool's own page about the rule that fired, from its SARIF `helpUri`, when that is a plain web
    /// address (`crate::adapters::kept_help_uri`); `None` for `sv`'s own rules and a tool that gives none (backlog
    /// 0378).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_uri: Option<String>,
    /// Why the report lists the finding apart though nothing about its file or rule says so: it is
    /// about requirements the app is not held to, or an outside tool's finding about requirements
    /// `sv`'s own check of the running app verified in the same run (`Outranked`). `None` until the
    /// report looks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outranked: Option<Outranked>,
}

/// Why a finding is listed apart from the ones that count (the owner's decision of 6 October 2026;
/// ADR-023, Later). Either way it is shown in full and still named in every report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "why", rename_all = "kebab-case")]
pub enum Outranked {
    /// Every requirement it names is one the app is not held to: above its target level, or not
    /// applying to it. It never decided an applicable requirement's status.
    NotHeldTo,
    /// An outside tool's finding about requirements `sv`'s own check of the running app verified in
    /// the same run, named here. It no longer keeps them from being credited.
    CheckedWhileRunning { check: String },
    /// An outside tool's finding from a rule written for a web framework, named here, that the app's
    /// package lists say it does not use (backlog 0521; ADR-111, Later). It decides nothing.
    OtherFramework { framework: String },
    /// An outside tool's finding from a rule written for the languages named here, in a file of another: Semgrep's
    /// insecure-WebSocket rule is a text pattern that matched `ws://` in a Rust comment (backlog 0615; ADR-111, Later).
    /// It decides nothing.
    OtherLanguage { languages: Vec<String> },
    /// Semgrep's rule for credentials written to a log, on a logger call whose every logged value is an id, named
    /// here: its message mentions a secret, and the line logs none (backlog 0583; ADR-111, Later). It decides nothing.
    LogsOnlyIds { names: Vec<String> },
    /// Bandit's B104, an app listening on every address, in an app stackvet.toml runs in a container (`[stack.run]`),
    /// where `sv preflight` tells it to listen on every address, since that is the only way to be reached there
    /// (backlog 0658; ADR-111, Later). Unlike the others, it still counts: outside a container the same line puts the
    /// app on the local network, so it is worth a look, not nothing.
    ListensInContainer,
}

/// Semgrep rules whose findings are listed apart as "worth a look" (Semgrep follow-up 5, the owner's
/// decision of 6 October 2026; ADR-023, Later). In the false-alarm measurement of 4 October 2026
/// (`docs/semgrep-false-alarms.csv`) these made 280 findings, of which one was real (NodeGoat's
/// `var-in-href` in `app/views/profile.html`). Listed apart, shown in full, and still counted, as test
/// code's are: never hidden, since one in 280 was real.
pub const WORTH_A_LOOK: &[&str] = &[
    "semgrep.javascript.lang.security.audit.unsafe-dynamic-method.unsafe-dynamic-method",
    "semgrep.javascript.lang.security.audit.detect-non-literal-regexp.detect-non-literal-regexp",
    "semgrep.javascript.jquery.security.audit.prohibit-jquery-html.prohibit-jquery-html",
    "semgrep.html.security.plaintext-http-link.plaintext-http-link",
    "semgrep.generic.html-templates.security.var-in-href.var-in-href",
    "semgrep.javascript.express.security.audit.xss.ejs.var-in-href.var-in-href",
    "semgrep.javascript.express.security.audit.xss.pug.var-in-href.var-in-href",
    "semgrep.ruby.rails.security.audit.xss.templates.var-in-href.var-in-href",
];

/// The rule that reports a file opened at a path built from any value, and the one that reports a path built from a
/// request's input (backlog 0653, ADR-138). Where the second reads the language, a finding of the first that nothing
/// else reported is only worth a look: the path is a value, but not one traced to a request.
pub const PATH_FROM_VALUE: &str = "ast.file-path-from-value";
pub const PATH_FROM_REQUEST: &str = "ast.file-path-from-request";

/// The languages `PATH_FROM_REQUEST` reads, by `sv_scan`'s names; a test holds this to the rule's own queries. In any
/// other language a `PATH_FROM_VALUE` finding stays a thing to fix, since a request's input there is not looked for.
pub const PATH_FROM_REQUEST_LANGUAGES: &[&str] = &[
    "csharp",
    "go",
    "java",
    "javascript",
    "kotlin",
    "php",
    "python",
    "ruby",
    "rust",
    "shell",
    "typescript",
];

/// The most of one value from an app's answer (a header, mostly) a finding quotes.
pub const QUOTED_CHARS: usize = 200;

/// `value` as a finding quotes it: on one line, and cut at `QUOTED_CHARS` characters with how long
/// it was in all. The app chooses its headers: one of 100 KB quoted whole would bury the report,
/// and a line break in one would start a line of its own in it (the review of 8 October 2026,
/// item 6).
pub fn quoted(value: &str) -> String {
    let one_line: String = value
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect();
    let count = one_line.chars().count();
    if count <= QUOTED_CHARS {
        return one_line;
    }
    let kept: String = one_line.chars().take(QUOTED_CHARS).collect();
    format!("{kept}… ({count} characters in all)")
}

/// A finding a check made, as it leaves the check. In a debug build (the test suite), with
/// `SV_CREDIT_LOG` set, it is written to that file's sibling `SV_CREDIT_LOG.withheld` as one line: the
/// check's id and the place in the code that made it, the caller's when the maker is a helper marked
/// `#[track_caller]`. `tools/coverage.py --withheld` reads it beside the credits, to tell which checks
/// the suite saw credit and never saw withhold (backlog item 32). In a release build it does nothing.
#[track_caller]
pub fn found(finding: Finding) -> Finding {
    #[cfg(debug_assertions)]
    withheld_census(&finding.rule_id, std::panic::Location::caller());
    finding
}

#[cfg(debug_assertions)]
pub(crate) fn withheld_census(check_id: &str, at: &std::panic::Location) {
    use std::io::Write;
    let Some(log) = std::env::var_os("SV_CREDIT_LOG") else {
        return;
    };
    let mut log = std::path::PathBuf::from(log).into_os_string();
    log.push(".withheld");
    let line = format!("{check_id}\t{}:{}\n", at.file(), at.line());
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log)
    {
        let _ = file.write_all(line.as_bytes());
    }
}

impl Finding {
    /// How sure `sv` is that this is a real problem, in the owner's words: "confirmed" when the rule
    /// is sure (or the running app was seen doing it), "likely" when it usually is, and "possible"
    /// when it is worth a look before any code is changed. Every one of them still needs attention;
    /// this says how much to trust it, not whether to count it.
    pub fn certainty(&self) -> &'static str {
        match self.confidence {
            Confidence::High => "confirmed",
            Confidence::Medium => "likely",
            Confidence::Low => "possible",
        }
    }

    /// Whether this finding keeps a requirement it names from being *checked*: true for every finding
    /// but the few that say, in their own text, that they leave the credit alone, and an outside
    /// tool's finding that `sv`'s own check of the running app outranked (`Outranked`).
    ///
    /// Those are listed by name in `INFORMATION_ONLY`, and must also be `Severity::Info`, so raising
    /// one's severity makes it count again rather than quietly staying beside the credit. A finding
    /// another rule was merged into counts whatever its own rule is. Every other finding counts,
    /// including a tool's at `info`: a tool's lowest level is still the tool saying something is
    /// wrong, and nothing in its text says otherwise. (BACKLOG, family-hub item 6, 4 October 2026.)
    pub fn withholds_credit(&self) -> bool {
        if matches!(
            self.outranked,
            Some(
                Outranked::CheckedWhileRunning { .. }
                    | Outranked::OtherFramework { .. }
                    | Outranked::OtherLanguage { .. }
                    | Outranked::LogsOnlyIds { .. }
            )
        ) {
            return false;
        }
        !(self.severity == Severity::Info
            && self.also_reported_by.is_empty()
            && INFORMATION_ONLY.contains(&self.rule_id.as_str()))
    }

    /// Whether the finding is in code that tests the app or shows how to use it, rather than in the
    /// app itself. Said beside it, never used to hide it: test code can hold a real key, and sample
    /// code gets copied.
    pub fn in_test_code(&self) -> bool {
        self.marked_test_code || sv_scan::files::is_test_path(&self.location.file)
    }

    /// Whether this is information rather than a finding: one of `TEST_FILE_INFORMATION`, in test code, that no
    /// other tool also reported (ADR-091); or a credential assignment in test code whose value is made of words,
    /// reported by no other rule but those (backlog 0529). A random-looking value in a test file still counts: it
    /// can be a real key. Listed in its own section, never counted.
    pub fn is_test_information(&self) -> bool {
        self.in_test_code()
            && ((self.also_reported_by.is_empty()
                && TEST_FILE_INFORMATION.contains(&self.rule_id.as_str()))
                || (self.rule_id == crate::rule_ids::ASSIGNMENT_RULE
                    && self.secret.as_ref().is_some_and(Secret::reads_as_words)
                    && self
                        .also_reported_by
                        .iter()
                        .all(|r| TEST_FILE_INFORMATION.contains(&r.as_str()))))
    }

    /// Whether the finding is only "worth a look": reported by one of the Semgrep rules in
    /// `WORTH_A_LOOK` and by nothing else, with every other problem on its line the same. One that
    /// another tool also reported, or that shares its line with a problem of another kind, is not.
    pub fn worth_a_look(&self) -> bool {
        (WORTH_A_LOOK.contains(&self.rule_id.as_str()) || self.path_not_from_request())
            && self.also_reported_by.is_empty()
            && self.also_on_this_line.iter().all(Finding::worth_a_look)
    }

    /// Whether this is a file path built from a value that the request-path rule, which reads this file's language,
    /// did not trace to a request's input (backlog 0653, ADR-138). Listed apart as worth a look; it still counts.
    pub fn path_not_from_request(&self) -> bool {
        self.rule_id == PATH_FROM_VALUE
            && std::path::Path::new(&self.location.file)
                .extension()
                .and_then(|e| e.to_str())
                .and_then(|e| sv_scan::ecosystems::language_of(&e.to_ascii_lowercase()))
                .is_some_and(|language| PATH_FROM_REQUEST_LANGUAGES.contains(&language))
    }

    /// Whether the reports list this finding apart from the app's own code: in test or sample code,
    /// in a copy of another project's library kept in the app, or only worth a look. Whichever it
    /// is, it is shown in full and still counts.
    pub fn apart(&self) -> bool {
        self.in_test_code()
            || self.bundled_library.is_some()
            || self.worth_a_look()
            || self.outranked.is_some()
    }
}
