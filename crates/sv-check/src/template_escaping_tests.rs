//! Backlog 0613: V1.1.2 credited in part when the template engine escapes by itself and nothing the outside tools read
//! turns it off, and withheld when any one of those does not hold.

use std::collections::BTreeSet;

use super::{Seen, credit};
use crate::adapters::{AdapterRun, Adapters};
use crate::finding::{Confidence, Finding, Location, Severity};

fn adapters() -> Adapters {
    let path = sv_frameworks::data::file("adapters.json");
    Adapters::load(&path)
        .expect("the adapter file loads")
        .opted_in(|_| false)
}

/// Every rule of `id` that names V1.1.2 as a finding.
fn switch_offs(adapters: &Adapters, id: &str) -> BTreeSet<String> {
    let adapter = adapters.all().iter().find(|a| a.id == id).unwrap();
    adapter
        .rules
        .iter()
        .filter(|(_, r)| r.findings_against.iter().any(|x| x == "V1.1.2"))
        .map(|(id, _)| id.clone())
        .collect()
}

/// Bandit and Semgrep ran over all of the app, each listing every switch-off rule as run.
fn clean_run(adapters: &Adapters) -> AdapterRun {
    let mut run = AdapterRun::default();
    for id in ["bandit", "semgrep"] {
        run.ran.push(id.to_owned());
        run.loaded.push((id.to_owned(), switch_offs(adapters, id)));
    }
    run
}

fn switched_off(rule_id: &str) -> Finding {
    crate::finding::found(Finding {
        evidence: Vec::new(),
        rule_id: rule_id.into(),
        title: "escaping turned off".into(),
        severity: Severity::Medium,
        confidence: Confidence::Medium,
        location: Location {
            file: "app.py".into(),
            line: 3,
        },
        secret: None,
        requirement_ids: vec!["V1.1.2".into()],
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
        also_reported_by: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    })
}

struct App {
    languages: BTreeSet<String>,
    files: BTreeSet<String>,
    packages: Vec<(String, String)>,
    unread_package_lists: Vec<String>,
    unread_code: Vec<String>,
    earlier: Vec<Finding>,
}

/// A Flask app with one template.
fn flask() -> App {
    App {
        languages: ["python", "html"].map(String::from).into(),
        files: ["app.py", "templates/index.html"].map(String::from).into(),
        packages: vec![("Python".into(), "Flask".into())],
        unread_package_lists: Vec::new(),
        unread_code: Vec::new(),
        earlier: Vec::new(),
    }
}

fn judge(
    app: &App,
    adapters: &Adapters,
    run: &AdapterRun,
) -> Result<crate::verified::Verified, String> {
    credit(&Seen {
        adapters,
        run,
        languages: &app.languages,
        files: &app.files,
        packages: &app.packages,
        unread_package_lists: &app.unread_package_lists,
        unread_code: &app.unread_code,
        earlier: &app.earlier,
    })
}

#[test]
fn an_escaping_engine_with_every_switch_off_rule_run_clean_is_credited_in_part() {
    let adapters = adapters();
    assert!(
        !switch_offs(&adapters, "semgrep").is_empty()
            && !switch_offs(&adapters, "bandit").is_empty(),
        "the setup has switch-off rules to run"
    );
    let credit = judge(&flask(), &adapters, &clean_run(&adapters)).expect("credited");
    assert_eq!(credit.requirement_ids, ["V1.1.2"]);
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("Jinja2 under Flask"),
        "{}",
        credit.scope
    );
}

#[test]
fn no_escaping_engine_among_the_packages_is_not_credited() {
    let adapters = adapters();
    let mut app = flask();
    app.packages.clear();
    let why = judge(&app, &adapters, &clean_run(&adapters)).unwrap_err();
    assert!(why.contains("no template engine"), "{why}");
}

#[test]
fn an_engine_that_does_not_escape_or_whose_switch_offs_are_unread_is_not_credited() {
    let adapters = adapters();
    for (ecosystem, name) in [
        ("Python", "Mako"),
        ("Python", "jinja2"),
        ("npm", "handlebars"),
    ] {
        let mut app = flask();
        if name == "jinja2" {
            app.packages.clear();
        }
        app.packages.push((ecosystem.into(), name.into()));
        assert!(
            judge(&app, &adapters, &clean_run(&adapters)).is_err(),
            "{name}"
        );
    }
}

#[test]
fn one_switch_off_found_is_not_credited() {
    let adapters = adapters();
    let mut run = clean_run(&adapters);
    run.findings.push(switched_off(
        "semgrep.python.flask.security.xss.audit.template-unescaped-with-safe",
    ));
    let why = judge(&flask(), &adapters, &run).unwrap_err();
    assert!(why.contains("found escaping turned off"), "{why}");
    let mut app = flask();
    app.earlier.push(switched_off("ast.some-own-rule"));
    assert!(judge(&app, &adapters, &clean_run(&adapters)).is_err());
}

#[test]
fn a_language_or_file_nothing_here_reads_is_not_credited() {
    let adapters = adapters();
    let mut app = flask();
    app.languages.insert("go".into());
    let why = judge(&app, &adapters, &clean_run(&adapters)).unwrap_err();
    assert!(why.contains("go"), "{why}");
    let mut app = flask();
    app.unread_code.push("templates/page.erb".into());
    assert!(judge(&app, &adapters, &clean_run(&adapters)).is_err());
    let mut app = flask();
    app.unread_package_lists.push("Python".into());
    assert!(judge(&app, &adapters, &clean_run(&adapters)).is_err());
}

#[test]
fn a_tool_that_did_not_run_or_did_not_list_a_rule_is_not_credited() {
    let adapters = adapters();
    let mut run = clean_run(&adapters);
    run.ran.retain(|id| id != "semgrep");
    let why = judge(&flask(), &adapters, &run).unwrap_err();
    assert!(why.contains("Semgrep"), "{why}");

    let mut run = clean_run(&adapters);
    let flask_rule =
        "python.flask.security.xss.audit.template-unescaped-with-safe.template-unescaped-with-safe";
    run.loaded
        .iter_mut()
        .find(|(id, _)| id == "semgrep")
        .unwrap()
        .1
        .remove(flask_rule);
    let why = judge(&flask(), &adapters, &run).unwrap_err();
    assert!(why.contains(flask_rule), "{why}");
}

#[test]
fn a_rule_for_files_the_app_does_not_have_need_not_have_run() {
    let adapters = adapters();
    let vue = "javascript.vue.security.audit.xss.templates.avoid-v-html.avoid-v-html";
    let mut run = clean_run(&adapters);
    for (_, loaded) in &mut run.loaded {
        loaded.remove(vue);
    }
    assert!(
        judge(&flask(), &adapters, &run).is_ok(),
        "no .vue file, nothing for it to read"
    );
    let mut app = flask();
    app.files.insert("static/Page.vue".into());
    let why = judge(&app, &adapters, &run).unwrap_err();
    assert!(why.contains(vue), "{why}");
}
