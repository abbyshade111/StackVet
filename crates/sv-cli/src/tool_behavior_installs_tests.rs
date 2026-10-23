//! What the AI coding tool installed that no lockfile holds, and a secret in a file it wrote (backlog 0630, ADR-131,
//! Later): what an install command installs, what is never read as a package, and that a secret is named by its
//! finding, never its value.

use crate::hook_record::Done;
use crate::tool_behavior::{installed, installs_and_secrets};
use sv_check::git::changes::Commit;
use sv_check::sbom::{Component, VersionSource};
use sv_report::tool_behavior::Kind;

fn ran(command: &str) -> Done {
    Done::Ran {
        at: "2026-10-22T18:10:00Z".to_owned(),
        command: command.to_owned(),
    }
}

fn locked(ecosystem: &str, name: &str, source: VersionSource) -> Component {
    Component {
        name: name.to_owned(),
        version: "1.0.0".to_owned(),
        ecosystem: ecosystem.to_owned(),
        source,
    }
}

fn secret_finding(file: &str, line: usize) -> sv_check::Finding {
    sv_check::Finding {
        evidence: Vec::new(),
        also_reported_by: Vec::new(),
        fingerprint: format!("fp-{file}-{line}"),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        rule_id: "secrets.aws-access-key".into(),
        title: "AWS access key ID found in a file".into(),
        severity: sv_check::Severity::Critical,
        confidence: sv_check::Confidence::High,
        location: sv_check::Location {
            file: file.into(),
            line,
        },
        secret: None,
        requirement_ids: vec!["V13.3.1".into()],
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
    }
}

#[test]
fn an_install_command_names_its_packages_without_versions() {
    let names = |c: &str| -> Vec<String> {
        installed(c)
            .into_iter()
            .map(|(e, n)| format!("{e}:{n}"))
            .collect()
    };
    assert_eq!(
        names("pip install flask==3.0 'requests[socks]>=2' -q"),
        ["Python:flask", "Python:requests"]
    );
    assert_eq!(
        names("python3 -m pip install --upgrade httpx"),
        ["Python:httpx"]
    );
    assert_eq!(
        names("cd web && npm install --save-dev @types/node@20 left-pad"),
        ["npm:@types/node", "npm:left-pad"]
    );
    assert_eq!(names("cargo add serde --features derive"), ["Rust:serde"]);
    assert_eq!(
        names("composer require monolog/monolog:^3.0"),
        ["PHP:monolog/monolog"]
    );
    assert_eq!(
        names("go get github.com/gorilla/mux@v1.8.1"),
        ["Go:github.com/gorilla/mux"]
    );
    assert_eq!(names("sudo gem install rails"), ["Ruby:rails"]);
}

#[test]
fn what_is_installed_from_a_file_or_cut_short_is_not_a_package() {
    for command in [
        "pip install -r requirements.txt",
        "pip install -e .",
        "pip install ./dist/app-1.0.whl",
        "npm install",
        "npm ci",
        "pip install git+https://example.test/repo.git",
        "pip install --index-url https://example.test/simple",
        "pytest -q",
        "echo pip install flask",
    ] {
        assert!(
            installed(command).is_empty(),
            "{command}: {:?}",
            installed(command)
        );
    }
    // The hook record cut this command inside its last word: that word is not read as a name.
    let cut = "pip install flask reque… (40 more characters)";
    let names: Vec<String> = installed(cut).into_iter().map(|(_, n)| n).collect();
    assert_eq!(names, ["flask"]);
}

#[test]
fn a_package_no_lockfile_holds_is_a_line_and_a_locked_one_is_not() {
    let done = [
        ran("pip install Flask_Login requests left_over"),
        ran("pip install flask-login"),
    ];
    let components = [
        locked("Python", "flask-login", VersionSource::Locked),
        locked("Python", "requests", VersionSource::Declared),
    ];
    let seen = installs_and_secrets(None, Some(&done), &components, &[]);
    let what: Vec<&str> = seen.iter().map(|s| s.what.as_str()).collect();
    // Flask_Login is flask-login, which a lockfile holds; named once even when installed twice.
    assert_eq!(seen.len(), 2, "{what:#?}");
    assert!(seen.iter().all(|s| s.kind == Kind::PackageInstalled));
    assert!(
        what[0].starts_with("the Python package requests was installed"),
        "{}",
        what[0]
    );
    assert!(
        what[0].ends_with("a manifest names it, but no lockfile in the app pins its version"),
        "{}",
        what[0]
    );
    assert!(
        what[1].contains("left_over")
            && what[1].ends_with("no lockfile or manifest in the app names it")
    );
    assert_eq!(seen[0].place, "the hook record, 2026-10-22T18:10:00Z");
}

#[test]
fn a_secret_is_a_line_only_in_a_file_the_tool_wrote() {
    let done = [Done::Wrote {
        at: "2026-10-22T18:20:00Z".to_owned(),
        file: "config.py".to_owned(),
    }];
    let commit = Commit {
        hash: "0123456789abcdef".to_owned(),
        subject: "Add settings".to_owned(),
        tool: Some("Cursor".to_owned()),
        files: vec![('A', "settings.json".to_owned())],
        added: Vec::new(),
        removed: Vec::new(),
    };
    let findings = [
        secret_finding("config.py", 3),
        secret_finding("settings.json", 7),
        secret_finding("written_by_a_person.py", 1),
    ];
    let seen = installs_and_secrets(Some(&[commit]), Some(&done), &[], &findings);
    assert_eq!(seen.len(), 2, "{seen:#?}");
    assert!(seen.iter().all(|s| s.kind == Kind::SecretWritten));
    assert_eq!(
        seen[0].what,
        "\"AWS access key ID found in a file\" (secrets.aws-access-key) at config.py:3, in a file the tool wrote"
    );
    assert_eq!(seen[0].place, "the hook record, 2026-10-22T18:20:00Z");
    assert!(
        seen[1]
            .place
            .starts_with("commit 01234567 \"Add settings\", by Cursor"),
        "{}",
        seen[1].place
    );
    // A secret found in a file nothing shows the tool writing is the secrets scan's finding, and no line here.
    assert!(
        !seen
            .iter()
            .any(|s| s.what.contains("written_by_a_person.py"))
    );
}
