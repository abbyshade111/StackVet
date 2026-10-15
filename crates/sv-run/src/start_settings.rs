//! The settings a start command gives the copy of the app `sv` runs, named in the report (backlog 0386). In the second
//! loop trial, twelve of 90 builds gave that copy settings the shipped app would not have: a sign-in limit raised to 60,
//! 100, or 1,000, and in four, the session cookie's `Secure` flag turned off. The report said nothing of the limits, so
//! the owner read a wrong-password finding about a copy that differs from what they ship. Names only, never a value:
//! a value can be a key.

/// Every environment setting the start command gives, by name, in the order written: a word `NAME=value` whose name is
/// a variable's name (letters, digits, and `_`, not starting with a digit), as before the command or after `env` or
/// `export`. A flag (`--name=value`) is not one: it is the program's own argument, which this does not read.
pub fn named_in(start: &str) -> Vec<String> {
    let unquoted: String = start.chars().filter(|c| !matches!(c, '"' | '\'')).collect();
    let mut out: Vec<String> = Vec::new();
    for word in unquoted.split(|c: char| c.is_whitespace() || ";&|()`".contains(c)) {
        let Some((name, _)) = word.split_once('=') else {
            continue;
        };
        let variable = name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
            && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        if variable && !out.iter().any(|n| n == name) {
            out.push(name.to_owned());
        }
    }
    out
}

/// Of `named_in`, the settings whose names look like a limit on signing in: one part of the name is about signing in
/// (`LOGIN`, `SIGNIN`, `SIGN_IN`, `AUTH`, `PASSWORD`) and another about a limit (`LIMIT`, `ATTEMPTS`, `FAILURES`, `MAX`,
/// `TRIES`, `LOCKOUT`). Every one the trial's builds used is: `CLUB_LOGIN_ATTEMPTS`, `GARDEN_SIGNIN_LIMIT`,
/// `CLUB_LOGIN_MAX_FAILURES`, `AUTH_IP_LIMIT`. A limit on signing up or on resetting a password is not.
pub fn sign_in_limits_in(start: &str) -> Vec<String> {
    const SIGNING_IN: &[&str] = &["login", "logon", "signin", "auth", "password", "passwords"];
    const LIMIT: &[&str] = &[
        "limit", "limits", "attempt", "attempts", "failure", "failures", "failed", "max", "tries",
        "lockout",
    ];
    named_in(start)
        .into_iter()
        .filter(|name| {
            let lower = name.to_ascii_lowercase();
            let mut parts: Vec<&str> = lower.split('_').filter(|p| !p.is_empty()).collect();
            // `SIGN_IN` is one word written as two.
            let joined = lower.replace('_', "");
            let sign_in = joined.contains("signin");
            if sign_in {
                parts.push("signin");
            }
            parts.iter().any(|p| SIGNING_IN.contains(p)) && parts.iter().any(|p| LIMIT.contains(p))
        })
        .collect()
}

/// The settings named for a sentence: "`A`", "`A` and `B`", "`A`, `B`, and `C`".
fn joined(names: &[String]) -> String {
    let quoted: Vec<String> = names.iter().map(|n| format!("`{n}`")).collect();
    match quoted.as_slice() {
        [] => String::new(),
        [one] => one.clone(),
        [a, b] => format!("{a} and {b}"),
        [rest @ .., last] => format!("{}, and {last}", rest.join(", ")),
    }
}

/// The report's sentence about the run: the settings the start command gave the copy `sv` ran, by name. None when it
/// gave none.
pub fn note(start: &str) -> Option<String> {
    let names = named_in(start);
    (!names.is_empty()).then(|| {
        format!(
            "Its start command gave the copy `sv` ran {} {}, which the app as it is shipped may not have, so what \
             was found by running it is about the copy with {}. Only the name{} shown here, since a value can be a key.",
            if names.len() == 1 { "the setting" } else { "the settings" },
            joined(&names),
            if names.len() == 1 { "it" } else { "them" },
            if names.len() == 1 { " is" } else { "s are" },
        )
    })
}

/// What the wrong-password finding adds when the start command sets something that looks like a sign-in limit: the
/// finding may be about a limit raised for the copy `sv` ran alone. None when it sets none.
pub fn sign_in_limit_note(start: &str) -> Option<String> {
    let names = sign_in_limits_in(start);
    (!names.is_empty()).then(|| {
        format!(
            "The start command in stackvet.toml sets {}, which looks like a limit on signing in. If it raises the \
             limit for the copy `sv` ran only, this finding is about that copy, and the app as it is shipped may keep \
             the limit it states.",
            joined(&names)
        )
    })
}

/// The rules of `sv`'s run whose credit a setting loosened for the run bears on, by the words of its name (backlog
/// 0564): a credit earned by a copy that differs from the shipped app on the very thing the rule asks about is about
/// that copy. Each table is the rules that ask about the thing the words name; a name that names none of them bears on
/// nothing here, and is only warned about, as before.
const BEARS_ON: &[(&[&str], &[&str])] = &[
    (
        &["cookie", "cookies"],
        &[
            "probe.cookie-attributes",
            "probe.session-cookie-attributes",
            "probe.cookie-without-host-prefix",
        ],
    ),
    (
        &["csrf", "xsrf"],
        &[
            "probe.cross-site-request-accepted",
            "probe.preflight-skipped",
            "probe.csrf-token-shared",
        ],
    ),
    (
        &["hsts", "https"],
        &["probe.no-hsts", "probe.api-redirected-to-https"],
    ),
    (&["csp"], &["probe.security-headers", "probe.csp-no-report"]),
    (
        &["mfa", "2fa", "totp"],
        &["probe.totp-old-code-accepted", "probe.totp-reused"],
    ),
    (
        &["ratelimit"],
        &[
            "probe.create-rate-unlimited",
            "probe.failed-sign-ins-unlimited",
            "probe.email-code-guessing-unlimited",
            "probe.ai-rate-unlimited",
        ],
    ),
    (
        &["auth"],
        &[
            "probe.private-page-anonymous",
            "probe.admin-page-ordinary-user",
            "probe.admin-action-ordinary-user",
            "probe.other-users-data",
            "probe.session-token-unverified",
            "probe.websocket-without-session",
        ],
    ),
];

/// Every rule id the table above names, for a test that each is one of `sv`'s.
pub fn rules_borne_on() -> Vec<&'static str> {
    let mut all: Vec<&'static str> = BEARS_ON
        .iter()
        .flat_map(|(_, rules)| rules.iter().copied())
        .collect();
    all.sort_unstable();
    all.dedup();
    all
}

/// The settings the start command loosens the run's copy with, as `crate::weakening_named_in` shows them (a value only
/// when it is a short word, never one that could be a key), each with the rules it bears on. Only settings that bear on
/// some rule. A setting that only looks like a sign-in limit (`sign_in_limits_in`) is not one: raising a limit on
/// every sign-in for the run while wrong passwords are still stopped at the stated number is what ADR-114 asks of the
/// copy, and a credit for V6.3.1 then shows the copy stopping them; its findings keep `sign_in_limit_note`.
pub fn loosened(start: &str) -> Vec<(String, Vec<&'static str>)> {
    let mut out: Vec<(String, Vec<&'static str>)> = Vec::new();
    for shown in crate::weakening_named_in(start) {
        let name = shown
            .split('=')
            .next()
            .unwrap_or_default()
            .trim_start_matches('-');
        let lower = name.to_ascii_lowercase();
        let mut words: Vec<String> = lower
            .split(['_', '-', '.'])
            .filter(|w| !w.is_empty())
            .map(str::to_owned)
            .collect();
        if lower.replace(['_', '-', '.'], "").contains("ratelimit") {
            words.push("ratelimit".to_owned());
        }
        let mut rules: Vec<&'static str> = BEARS_ON
            .iter()
            .filter(|(names, _)| words.iter().any(|w| names.contains(&w.as_str())))
            .flat_map(|(_, rules)| rules.iter().copied())
            .collect();
        rules.dedup();
        if !rules.is_empty() {
            out.push((shown, rules));
        }
    }
    out
}

#[cfg(test)]
#[path = "start_settings_tests.rs"]
mod tests;
