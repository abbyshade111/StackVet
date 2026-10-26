//! The checklist before going live (backlog 0589, ADR-121): what a report holds, put in the order of a deployment,
//! crediting nothing.

use crate::checklist::{self, Line, Part};
use crate::next_step_tests::empty_report;
use crate::{Gap, GapReason, Report, RequirementLine, Status};

fn finding(rule: &str, title: &str, severity: sv_check::Severity, file: &str) -> sv_check::Finding {
    sv_check::finding::found(sv_check::Finding {
        evidence: Vec::new(),
        rule_id: rule.into(),
        title: title.into(),
        severity,
        confidence: sv_check::Confidence::High,
        location: sv_check::Location {
            file: file.into(),
            line: 7,
        },
        secret: None,
        requirement_ids: vec!["V13.3.1".into()],
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
        also_reported_by: Vec::new(),
        fingerprint: format!("v2-{rule}-{file}"),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    })
}

fn requirement(id: &str, status: Status) -> RequirementLine {
    RequirementLine {
        id: id.into(),
        description: String::new(),
        chapter: String::new(),
        level: 1,
        status,
        findings: Vec::new(),
        information: Vec::new(),
        checked_by: Vec::new(),
        supported_by: Vec::new(),
        tested_by: Vec::new(),
        documented_by: Vec::new(),
        attested_by: Vec::new(),
        by_hand: Vec::new(),
        withheld_by: Vec::new(),
        whose_word: None,
    }
}

/// A gap as `sv report` says a credit withheld for a loosened run (`loosened_for_the_run` in `sv-cli`).
fn loosened(setting: &str, requirement: &str) -> Gap {
    Gap {
        what: format!("{requirement}, by running the app (probe.x)"),
        why: format!(
            "The copy `sv` ran had `{setting}` set in its start command in stackvet.toml, so what the run saw here \
             is not what the app does as it is shipped, and it is not credited. Take the setting out of the start \
             command, or set it as the shipped app has it, and run again."
        ),
        reason: GapReason::LeftOut,
        requirements: vec![requirement.into()],
    }
}

/// An app on the internet with something for every part.
pub(crate) fn report() -> Report {
    let mut report = empty_report(vec![
        loosened("SESSION_COOKIE_SECURE=0", "V3.3.1"),
        loosened("SESSION_COOKIE_SECURE=0", "V3.3.4"),
        loosened("DEBUG=1", "V13.4.2"),
        Gap {
            what: "The app's tests".into(),
            why: "They were not run.".into(),
            reason: GapReason::NotAsked,
            requirements: Vec::new(),
        },
    ]);
    report.on_the_internet = true;
    report.requirements = vec![
        requirement("V12.2.1", Status::NotVerified),
        requirement("V12.2.2", Status::ByHand),
        requirement("V12.1.1", Status::NotVerified),
    ];
    report.before_going_live = crate::live::before_going_live(&report.requirements);
    report.undecided = vec![crate::UndecidedRequirement {
        id: "V6.2.1".into(),
        description: "Multi-factor sign-in for every account".into(),
        chapter: "V6".into(),
        blocked_on: vec![
            "Does anyone sign in?".into(),
            "Is there an admin page?".into(),
        ],
    }];
    let accepted = finding(
        "ast.sql",
        "A query built by hand",
        sv_check::Severity::Medium,
        "db.py",
    );
    report.findings = vec![
        finding(
            "secrets.openai-key",
            "An OpenAI key in the code",
            sv_check::Severity::Critical,
            "config.py",
        ),
        finding(
            "ast.eval",
            "eval of a value the user gives",
            sv_check::Severity::High,
            "app.py",
        ),
        accepted.clone(),
        finding("ast.a", "A low one", sv_check::Severity::Low, "a.py"),
        finding("ast.b", "Another low one", sv_check::Severity::Low, "b.py"),
    ];
    report.set_aside = vec![sv_check::review::SetAside {
        finding: accepted,
        verdict: sv_check::review::ACCEPTED_RISK.into(),
        why: "Only the owner can reach it.".into(),
        by: "owner".into(),
        on: "2026-10-01".into(),
        sealed: sv_check::seal::Sealed::Here,
    }];
    report.counts.applicable = 40;
    report.counts.not_verified = 25;
    report
}

fn of(lines: &[Line], part: Part) -> Vec<&Line> {
    lines.iter().filter(|l| l.part == part).collect()
}

#[test]
fn each_part_holds_what_the_report_found_for_it_in_the_order_of_a_deployment() {
    let lines = checklist::lines(&report());
    let parts: Vec<Part> = lines.iter().map(|l| l.part).collect();
    let mut sorted = parts.clone();
    sorted.sort_by_key(|p| Part::ALL.iter().position(|q| q == p));
    assert_eq!(parts, sorted, "the parts come in a deployment's order");

    let host = of(&lines, Part::AtTheHost);
    // HTTPS first, naming only the requirements nothing settled (V12.2.2 was checked by hand).
    assert!(host[0].what.starts_with("Turn on HTTPS"), "{host:?}");
    assert_eq!(host[0].requirements, ["V12.2.1", "V12.1.1"]);
    // One line per loosened setting, with every requirement its withheld credits named.
    let cookie: Vec<_> = host
        .iter()
        .filter(|l| l.what.contains("SESSION_COOKIE_SECURE=0"))
        .collect();
    assert_eq!(cookie.len(), 1, "{host:?}");
    assert_eq!(cookie[0].requirements, ["V3.3.1", "V3.3.4"]);
    assert!(
        host.iter()
            .any(|l| l.what.contains("`DEBUG=1`") && l.requirements == ["V13.4.2"])
    );
    // The key in the code, at its place.
    let key = host
        .iter()
        .find(|l| l.what.contains("Take the key out"))
        .expect("the key");
    assert_eq!(key.place.as_deref(), Some("config.py:7"));

    let after = of(&lines, Part::AfterTheDeploy);
    assert!(after.iter().any(
        |l| l.requirements == ["V12.2.1"] && l.command.as_deref() == Some(crate::live::COMMAND)
    ));
    assert!(
        after.iter().all(|l| l.requirements != ["V12.2.2"]),
        "a requirement checked by hand is not asked again"
    );

    let decide = of(&lines, Part::Decide);
    assert_eq!(decide.len(), 1);
    assert!(
        decide[0]
            .what
            .contains("Does anyone sign in? Or: Is there an admin page?"),
        "{decide:?}"
    );
    assert_eq!(decide[0].requirements, ["V6.2.1"]);

    let sign_off = of(&lines, Part::SignOff);
    // The key is in the first part, not here as well.
    assert!(
        sign_off.iter().all(|l| !l.what.contains("OpenAI")),
        "{sign_off:?}"
    );
    assert!(
        sign_off
            .iter()
            .any(|l| l.what.starts_with("eval of a value the user gives")
                && l.what.contains("Fix it, or accept it as a risk"))
    );
    assert!(
        sign_off
            .iter()
            .any(|l| l.what.starts_with("A query built by hand")
                && l.what.contains("Accepted as a risk by owner on 2026-10-01"))
    );
    // The low ones counted, not listed.
    assert!(sign_off.iter().all(|l| !l.what.contains("A low one")));
    assert!(
        sign_off
            .iter()
            .any(|l| l.what.starts_with("2 low-severity findings,"))
    );
}

#[test]
fn an_app_not_for_the_internet_has_no_steps_at_a_live_address_and_says_why() {
    let mut report = report();
    report.on_the_internet = false;
    report.before_going_live.clear();
    let lines = checklist::lines(&report);
    assert!(of(&lines, Part::AfterTheDeploy).is_empty());
    assert!(
        of(&lines, Part::AtTheHost)
            .iter()
            .all(|l| !l.what.starts_with("Turn on HTTPS"))
    );
    let md = checklist::render(&report);
    assert!(md.contains("set `deployment = \"internet\"` there"), "{md}");
}

#[test]
fn the_file_credits_nothing_and_ends_with_what_was_not_verified() {
    let md = checklist::render(&report());
    assert!(md.starts_with("# Before going live: test\n"), "{md}");
    assert!(md.contains(crate::folder::markdown_line(&report(), checklist::FILE).trim_end()));
    assert!(md.contains("It credits nothing"), "{md}");
    assert!(
        !md.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .any(|w| matches!(w, "pass" | "passed" | "passes")),
        "nothing on it reads as a pass: {md}"
    );
    let last = md
        .split("## 5. What this report could not verify")
        .nth(1)
        .expect("the last part");
    assert!(
        last.contains("25 of the 40 requirements that apply"),
        "{last}"
    );
    // The three withheld credits and the tests not run.
    assert!(last.contains("4 limits say"), "{last}");
    assert!(
        !last.contains("- [ ]"),
        "what was not verified has no box to tick"
    );
    // Every line to do has its box, and every part its heading.
    for part in Part::ALL {
        assert!(md.contains(&format!("## {}\n", part.heading())), "{part:?}");
    }
    assert_eq!(
        md.matches("- [ ] ").count(),
        checklist::lines(&report()).len()
    );
}

#[test]
fn the_app_s_own_text_is_made_inert() {
    let mut report = report();
    report.app_name = "Shop [click](https://x.test)".into();
    report.findings[0].location.file = "a`b.py".into();
    let md = checklist::render(&report);
    assert!(
        md.contains("# Before going live: Shop \\[click\\](https://x.test)"),
        "{md}"
    );
    assert!(md.contains("At ``a`b.py:7``."), "{md}");
    let html = checklist::html(&report);
    assert!(!html.contains("href=\"https://x.test"), "{html}");
    assert!(html.contains("<code>a`b.py:7</code>"), "{html}");
}

#[test]
fn the_page_counts_the_things_found_rather_than_listing_them_twice() {
    let report = report();
    let page = crate::html::page(&report);
    let section = page
        .split("<h2>The checklist before going live</h2>")
        .nth(1)
        .expect("the page has the checklist");
    let section = section.split("<h2>").next().unwrap();
    assert!(section.contains(&format!("<a href=\"{0}\">{0}</a>", checklist::FILE)));
    assert!(
        !section.contains("eval of a value the user gives"),
        "{section}"
    );
    // eval, the accepted query, and the two low ones: the key is in the first part.
    assert!(
        section.contains("4 things found are still there"),
        "{section}"
    );
    assert!(
        section.contains("<code>SESSION_COOKIE_SECURE=0</code>"),
        "{section}"
    );
}
