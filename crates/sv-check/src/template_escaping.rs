//! V1.1.2, in part: the app's template engine escapes what it writes by itself, and nothing the
//! outside tools read turns that off (backlog 0613, ADR-053, Later, 16 October 2026).
//!
//! V1.1.2 asks that output encoding happen as the last step before the interpreter, or in the
//! interpreter itself. An engine that escapes by default does the second. The rules that read its
//! escaping switched off (`autoescape=False`, `mark_safe`, `|safe`, EJS's `<%-`, Vue's `v-html`,
//! React's `dangerouslySetInnerHTML`) are the outside tools' rules that name V1.1.2 as a finding
//! (`findings_against`, ADR-018, Later, 15 October 2026); they are read from `adapters.json`, so a
//! rule mapped there later joins the condition by itself.
//!
//! Credited in part, never whole: HTML put together by hand outside the engine, and output to other
//! interpreters (SQL, a shell), are not seen by this.

use std::collections::{BTreeMap, BTreeSet};

use crate::adapters::{AdapterRun, Adapters};
use crate::finding::Finding;
use crate::verified::Verified;

pub const CHECK_ID: &str = "config.template-engine-escapes";

const REQUIREMENT: &str = "V1.1.2";

/// Engines that escape by default and whose ways of switching it off the mapped rules read, by the
/// bill of materials' ecosystem and package name, with the name a person knows the engine by.
/// Jinja2 counts under Flask or Django only: used alone it escapes nothing unless told to.
const ESCAPING: &[(&str, &str, &str)] = &[
    ("Python", "django", "Django templates"),
    ("Python", "flask", "Jinja2 under Flask"),
    ("npm", "react", "React"),
    ("npm", "react-dom", "React"),
    ("npm", "vue", "Vue"),
    ("npm", "ejs", "EJS"),
    ("npm", "mustache", "Mustache"),
];

/// Engines that escape by default, but whose ways of switching it off no mapped rule reads: a
/// Handlebars `{{{ }}}` nothing looked for is not "nothing turns it off".
const SWITCH_OFFS_UNREAD: &[(&str, &str)] = &[
    ("npm", "handlebars"),
    ("npm", "express-handlebars"),
    ("npm", "hbs"),
    ("npm", "nunjucks"),
    ("npm", "svelte"),
    ("npm", "pug"),
    ("npm", "@angular/core"),
    ("Python", "chameleon"),
];

/// Engines that do not escape unless told to.
const NOT_ESCAPING: &[(&str, &str)] = &[("Python", "mako"), ("npm", "dot")];

/// The languages whose switch-offs the mapped rules read. Anything else in the app (Go's
/// `template.HTML`, Ruby's `raw`, a JSP) could switch escaping off unread.
const READ_LANGUAGES: &[&str] = &["python", "javascript", "typescript", "html", "notebook"];

/// What `credit` reads, gathered by the caller.
pub struct Seen<'a> {
    /// The tools that could run this time: those the owner turned on (ADR-115).
    pub adapters: &'a Adapters,
    pub run: &'a AdapterRun,
    /// The app's languages, by `sv`'s names.
    pub languages: &'a BTreeSet<String>,
    /// The app's files, relative to its folder: a rule that reads only some files (`*.vue`) and
    /// none of them is here had nothing to read.
    pub files: &'a BTreeSet<String>,
    /// Every package the app's package lists declare, as (ecosystem, name).
    pub packages: &'a [(String, String)],
    /// The ecosystems of package lists `sv` found and could not read.
    pub unread_package_lists: &'a [String],
    /// Languages and templates `sv`'s own code rules could not read (ADR-054).
    pub unread_code: &'a [String],
    /// Findings from `sv`'s own rules, before the outside tools'.
    pub earlier: &'a [Finding],
}

/// The in-part credit for V1.1.2, or why it is withheld. Withheld says nothing in the report:
/// V1.1.2 stays not assessed, as it was.
pub fn credit(seen: &Seen) -> Result<Verified, String> {
    let judged = judge(seen);
    // Withheld without a finding, so seen withholding only through this (ADR-059).
    if judged.is_err() {
        crate::verified::unless_credited(CHECK_ID, &[]);
    }
    judged
}

fn judge(seen: &Seen) -> Result<Verified, String> {
    let has = |ecosystem: &str, name: &str| {
        seen.packages
            .iter()
            .any(|(e, n)| e == ecosystem && n.eq_ignore_ascii_case(name))
    };
    if let Some(e) = seen
        .unread_package_lists
        .iter()
        .find(|e| *e == "npm" || *e == "Python")
    {
        return Err(format!("a package list for {e} could not be read"));
    }
    if let Some((e, n)) = NOT_ESCAPING.iter().find(|(e, n)| has(e, n)) {
        return Err(format!("{n} ({e}) does not escape by default"));
    }
    if has("Python", "jinja2") && !has("Python", "flask") && !has("Python", "django") {
        return Err(
            "Jinja2 used without Flask or Django escapes nothing unless told to".to_owned(),
        );
    }
    if let Some((e, n)) = SWITCH_OFFS_UNREAD.iter().find(|(e, n)| has(e, n)) {
        return Err(format!(
            "nothing here reads how {n} ({e}) is told not to escape"
        ));
    }
    let engines: BTreeSet<&str> = ESCAPING
        .iter()
        .filter(|(e, n, _)| has(e, n))
        .map(|(_, _, said)| *said)
        .collect();
    if engines.is_empty() {
        return Err("no template engine that escapes by default is among the packages".to_owned());
    }
    if let Some(l) = seen
        .languages
        .iter()
        .find(|l| !READ_LANGUAGES.contains(&l.as_str()))
    {
        return Err(format!(
            "the app has {l}, whose switch-offs no rule here reads"
        ));
    }
    if let Some(u) = seen.unread_code.first() {
        return Err(format!("{u} could not be read"));
    }
    if let Some(f) = seen
        .earlier
        .iter()
        .chain(&seen.run.findings)
        .find(|f| f.requirement_ids.iter().any(|r| r == REQUIREMENT))
    {
        return Err(format!("{} found escaping turned off", f.rule_id));
    }
    // Every tool with a switch-off rule for this app's languages ran over all of it, with those
    // rules among the ones its report says ran.
    let languages: Vec<String> = seen.languages.iter().cloned().collect();
    let loaded: BTreeMap<&str, &BTreeSet<String>> = seen
        .run
        .loaded
        .iter()
        .map(|(id, rules)| (id.as_str(), rules))
        .collect();
    let mut read_by = Vec::new();
    for adapter in seen.adapters.for_languages(&languages) {
        let rules: Vec<&String> = adapter
            .rules
            .iter()
            .filter(|(_, rule)| rule.findings_against.iter().any(|r| r == REQUIREMENT))
            .filter(|(_, rule)| {
                adapter.language != "*"
                    || rule
                        .languages
                        .iter()
                        .any(|l| l == "*" || seen.languages.contains(l))
            })
            .filter(|(_, rule)| rule.targets.is_empty() || seen.files.iter().any(|f| rule.reads(f)))
            .map(|(id, _)| id)
            .collect();
        if rules.is_empty() {
            continue;
        }
        if !seen.run.ran.contains(&adapter.id) {
            return Err(format!("{} did not run over all of the app", adapter.name));
        }
        if adapter.language == "*" || adapter.credit_loaded_only {
            let ran = loaded.get(adapter.id.as_str());
            if let Some(missing) = rules
                .iter()
                .find(|r| ran.is_none_or(|ran| !ran.contains(r.as_str())))
            {
                return Err(format!(
                    "{}'s report does not list {missing} as run",
                    adapter.name
                ));
            }
        }
        read_by.push(adapter.name.clone());
    }
    if read_by.is_empty() {
        return Err("no tool that reads a switch-off ran".to_owned());
    }
    let engines: Vec<&str> = engines.into_iter().collect();
    Ok(Verified::new(
        CHECK_ID,
        &[REQUIREMENT],
        format!(
            "the template engine escapes by itself ({}), and nothing {} read turns it off; HTML \
             put together by hand, and output to other interpreters, are not seen",
            engines.join(", "),
            read_by.join(" and ")
        ),
    )
    .in_part())
}

#[cfg(test)]
#[path = "template_escaping_tests.rs"]
mod tests;
