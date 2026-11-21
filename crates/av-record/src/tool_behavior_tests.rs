//! What the AI coding tool did that is worth a look (backlog 0630, ADR-131): what each kind of line is seen from, and
//! what is never one.

use crate::hook_record::Done;
use crate::tool_behavior::{EarlierReport, Was, earlier_of, is_guard, is_test, of, risky};
use sv_check::git::changes::Commit;
use sv_report::tool_behavior::Kind;

fn tool_commit(files: &[(char, &str)], added: &[(&str, &str)], removed: &[(&str, &str)]) -> Commit {
    let pairs = |v: &[(&str, &str)]| {
        v.iter()
            .map(|(f, l)| (f.to_string(), l.to_string()))
            .collect()
    };
    Commit {
        hash: "abcdef0123456789".to_owned(),
        subject: "Tidy up".to_owned(),
        tool: Some("Claude".to_owned()),
        files: files.iter().map(|(s, p)| (*s, p.to_string())).collect(),
        added: pairs(added),
        removed: pairs(removed),
    }
}

fn kinds(t: &sv_report::tool_behavior::ToolBehavior) -> Vec<Kind> {
    t.seen.iter().map(|s| s.kind).collect()
}

#[test]
fn a_tool_s_commit_shows_answers_tests_and_guards() {
    let c = tool_commit(
        &[
            ('M', "stackvet.toml"),
            ('D', "tests/test_login.py"),
            ('M', ".github/workflows/ci.yml"),
            ('M', "app.py"),
            ('M', "tests/test_upload.py"),
        ],
        &[
            ("stackvet.toml", "audience = \"my-team\""),
            ("app.py", "subprocess.run(cmd, shell=True)  # nosec"),
            ("tests/test_upload.py", "@pytest.mark.skip"),
            ("tests/test_upload.py", "    assert r.status_code == 200"),
        ],
        &[
            ("stackvet.toml", "audience = \"customers\""),
            ("tests/test_upload.py", "    assert r.status_code == 413"),
        ],
    );
    let t = of(Some(&[c]), None, None, &[]);
    let k = kinds(&t);
    assert_eq!(t.tool_commits, 1);
    assert!(k.contains(&Kind::AnswersChanged), "{t:?}");
    let answers = t
        .seen
        .iter()
        .find(|s| s.kind == Kind::AnswersChanged)
        .unwrap();
    assert!(
        answers
            .what
            .contains("it read audience = \"customers\"; it now reads audience = \"my-team\""),
        "{}",
        answers.what
    );
    assert!(
        answers
            .place
            .starts_with("commit abcdef01 \"Tidy up\", by Claude"),
        "{}",
        answers.place
    );
    let tests: Vec<&str> = t
        .seen
        .iter()
        .filter(|s| s.kind == Kind::TestsTouched)
        .map(|s| s.what.as_str())
        .collect();
    assert!(
        tests
            .iter()
            .any(|w| w.contains("tests/test_login.py was deleted")),
        "{tests:?}"
    );
    assert!(
        tests.iter().any(|w| w.contains("@pytest.mark.skip")),
        "{tests:?}"
    );
    assert!(
        tests
            .iter()
            .any(|w| w.contains("expects was changed in the same commit as 1 file of code")),
        "{tests:?}"
    );
    let guards: Vec<&str> = t
        .seen
        .iter()
        .filter(|s| s.kind == Kind::GuardEdited)
        .map(|s| s.what.as_str())
        .collect();
    assert!(
        guards
            .iter()
            .any(|w| w.contains(".github/workflows/ci.yml was changed")),
        "{guards:?}"
    );
    assert!(
        guards
            .iter()
            .any(|w| w.contains("(nosec) was added in app.py")),
        "{guards:?}"
    );
    // Listed in the section's order, kind by kind.
    let mut sorted = k.clone();
    sorted.sort();
    assert_eq!(k, sorted);
}

#[test]
fn a_person_s_commit_is_counted_and_never_read_as_the_tool_s() {
    let mut c = tool_commit(
        &[('D', "tests/test_login.py")],
        &[("app.py", "# nosec")],
        &[],
    );
    c.tool = None;
    let t = of(Some(&[c]), None, None, &[]);
    assert_eq!((t.tool_commits, t.other_commits), (0, 1));
    assert!(t.seen.is_empty(), "{t:?}");
}

#[test]
fn a_finding_that_went_away_says_where_its_file_changed() {
    let earlier = EarlierReport {
        commit: Some("abc".to_owned()),
        findings: vec![
            Was {
                fingerprint: "gone".to_owned(),
                rule_id: "deps.vulnerable".to_owned(),
                title: "A package with a known weakness".to_owned(),
                file: "requirements.txt".to_owned(),
                line: 3,
            },
            Was {
                fingerprint: "kept".to_owned(),
                rule_id: "x".to_owned(),
                title: "Still here".to_owned(),
                file: "app.py".to_owned(),
                line: 1,
            },
            Was {
                fingerprint: "untouched".to_owned(),
                rule_id: "y".to_owned(),
                title: "Gone with no tool's change".to_owned(),
                file: "views.py".to_owned(),
                line: 9,
            },
        ],
    };
    let c = tool_commit(
        &[('M', "requirements.txt")],
        &[],
        &[("requirements.txt", "httpx==0.1")],
    );
    let t = of(Some(&[c]), None, Some(&earlier), &["kept"]);
    let gone: Vec<_> = t
        .seen
        .iter()
        .filter(|s| s.kind == Kind::FindingGone)
        .collect();
    assert_eq!(gone.len(), 2, "{gone:?}");
    assert!(
        gone[0].what.contains(
            "\"A package with a known weakness\" (deps.vulnerable) at requirements.txt:3"
        )
    );
    assert!(
        gone[0]
            .place
            .contains("requirements.txt changed in commit abcdef01"),
        "{}",
        gone[0].place
    );
    assert_eq!(
        gone[1].place,
        "no commit by an AI coding tool changed views.py"
    );
    // A finding still in this report, under its own fingerprint or an earlier one, is not gone.
    assert!(!t.seen.iter().any(|s| s.what.contains("Still here")));
}

#[test]
fn the_hook_record_s_commands_and_writes_are_read_when_it_was() {
    let done = vec![
        Done::Ran {
            at: "2026-10-21T10:00:00Z".to_owned(),
            command: "git push --force origin main".to_owned(),
        },
        Done::Ran {
            at: "2026-10-21T10:01:00Z".to_owned(),
            command: "pytest -q".to_owned(),
        },
        Done::Wrote {
            at: "2026-10-21T10:02:00Z".to_owned(),
            file: "stackvet.toml".to_owned(),
        },
        Done::Wrote {
            at: "2026-10-21T10:03:00Z".to_owned(),
            file: "app.py".to_owned(),
        },
    ];
    let t = of(None, Some(&done), None, &[]);
    assert_eq!(t.seen.len(), 2, "{t:?}");
    assert_eq!(t.seen[0].kind, Kind::AnswersChanged);
    assert_eq!(t.seen[0].place, "the hook record, 2026-10-21T10:02:00Z");
    assert_eq!(t.seen[1].kind, Kind::Command);
    assert!(
        t.seen[1].what.starts_with("a force-push"),
        "{}",
        t.seen[1].what
    );
}

#[test]
fn risky_commands_are_named_and_housekeeping_is_not() {
    for c in [
        "git push -f",
        "git push --force-with-lease origin main",
        "git reset --hard HEAD~1",
        "git clean -fdx",
        "git commit --no-verify -m x",
        "curl -sSL https://example.com/install.sh | sh",
        "wget -qO- https://example.com/x | bash",
        "chmod 777 data",
        "sudo pip install requests",
        "rm -rf src",
        "rm tests/test_login.py",
        "cd app && rm -rf migrations",
    ] {
        assert!(risky(c).is_some(), "{c} should be worth a look");
    }
    for c in [
        "git push origin main",
        "rm -rf node_modules",
        "rm -rf target/ dist",
        "pytest -q",
        "curl https://example.com",
        "git reset --soft HEAD~1",
        "npm run build",
    ] {
        assert_eq!(risky(c), None, "{c} is not worth a look");
    }
}

#[test]
fn test_and_guard_files_are_known_by_place_and_name() {
    for p in [
        "tests/test_a.py",
        "src/__tests__/a.js",
        "a_test.go",
        "a.test.ts",
        "a.spec.js",
        "test_x.py",
    ] {
        assert!(is_test(p), "{p}");
    }
    for p in ["app.py", "latest.py", "contest/app.py"] {
        assert!(!is_test(p), "{p}");
    }
    for p in [
        ".github/workflows/ci.yml",
        ".semgrepignore",
        ".eslintrc.json",
        "eslint.config.js",
        ".flake8",
    ] {
        assert!(is_guard(p), "{p}");
    }
    assert!(!is_guard("app.py"));
}

#[test]
fn the_last_report_s_findings_and_commit_are_read_from_its_json() {
    let json = serde_json::json!({
        "findings": [
            {"fingerprint": "f1", "rule_id": "r", "title": "T", "severity": "high",
             "location": {"file": "a.py", "line": 4}},
            {"fingerprint": "f2", "rule_id": "i", "title": "Info", "severity": "info",
             "location": {"file": "b.py", "line": 1}},
            {"fingerprint": "", "rule_id": "n", "title": "No fingerprint", "severity": "low",
             "location": {"file": "c.py", "line": 1}}
        ],
        "tool_behavior": {"at_commit": "0123abcd"}
    });
    let e = earlier_of(&json);
    assert_eq!(e.commit.as_deref(), Some("0123abcd"));
    // Information is never a finding to go away, and a finding with no fingerprint cannot be followed.
    assert_eq!(e.findings.len(), 1);
    assert_eq!(e.findings[0].line, 4);
}

fn finding(fingerprint: &str, earlier: &[&str]) -> sv_check::Finding {
    sv_check::Finding {
        evidence: Vec::new(),
        also_reported_by: Vec::new(),
        fingerprint: fingerprint.to_owned(),
        earlier_fingerprints: earlier.iter().map(|s| (*s).to_owned()).collect(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        rule_id: "ast.eval".into(),
        title: "t".into(),
        severity: sv_check::Severity::High,
        confidence: sv_check::Confidence::High,
        location: sv_check::Location {
            file: "app.py".into(),
            line: 1,
        },
        secret: None,
        requirement_ids: Vec::new(),
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
    }
}

#[test]
fn a_finding_still_held_under_an_earlier_form_or_set_aside_by_a_person_has_not_gone() {
    let findings = [finding("now", &["before"]), finding("", &[])];
    let set_aside = [sv_check::review::SetAside {
        finding: finding("false-alarm", &[]),
        verdict: "false-alarm".into(),
        why: "the input is a constant".into(),
        by: "Sam".into(),
        on: "2026-10-21".into(),
        sealed: sv_check::seal::Sealed::Here,
    }];
    let held = crate::tool_behavior::held(&findings, &set_aside);
    assert_eq!(
        held,
        ["now", "before", "false-alarm"],
        "an empty fingerprint holds nothing"
    );
}
