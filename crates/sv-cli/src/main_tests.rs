use super::*;

#[test]
fn a_bad_body_goes_to_the_routes_the_manifest_names() {
    // ADR-056: the routes that read a body are where the app's code can be made to fail.
    let example = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/notes-with-users/stackvet.toml");
    let manifest = sv_manifest::Manifest::load(&example).expect("the example loads");
    let users = manifest.stack.run.users.as_ref().expect("it has users");
    let routes = body_routes(Some(users));
    let login = users.login.as_ref().expect("it signs in");
    assert!(
        routes.contains(&(login.method.clone(), login.path.clone())),
        "{routes:?}"
    );
    let create = &users.owned.as_ref().expect("it has an owned record").create;
    assert!(
        routes.contains(&(create.method.clone(), create.path.clone())),
        "{routes:?}"
    );
    let ids: Vec<String> = probes::error_requests("/health", &routes)
        .into_iter()
        .map(|r| r.id)
        .collect();
    assert!(
        ids.contains(&format!("bad-body POST {}", login.path)),
        "{ids:?}"
    );
    assert!(body_routes(None).is_empty());
}

#[test]
fn a_decisions_finding_reaches_the_reviews_and_goes_with_its_running_app_finding() {
    // The review of 6 October, item 7: the decision's own finding was made after the reviews
    // were applied, so no review of it could ever count.
    let section = "# Design decisions\n\n## Safe defaults\n\n- Debug mode: off\n";
    let decided = sv_check::decisions::safe_defaults(section).decided;
    assert_eq!(decided.len(), 1, "the setup: the line is read");
    let probe = sv_check::Finding {
        evidence: Vec::new(),
        also_reported_by: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        rule_id: decided[0].switch.rule_id.to_owned(),
        title: "open".to_owned(),
        severity: sv_check::Severity::High,
        confidence: sv_check::Confidence::High,
        location: sv_check::Location {
            file: "(running app)".to_owned(),
            line: 0,
        },
        secret: None,
        requirement_ids: vec!["V13.4.2".to_owned()],
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
    };
    let as_is = |findings: Vec<sv_check::Finding>| sv_check::review::Outcome {
        findings,
        set_aside: Vec::new(),
        not_counted: Vec::new(),
        counted_in_test: Vec::new(),
        left_out: Vec::new(),
    };
    // The review is shown the decision's finding.
    let mut seen = Vec::new();
    decisions_then_reviews(vec![probe.clone()], &decided, |findings| {
        seen = findings.iter().map(|f| f.rule_id.clone()).collect();
        as_is(findings)
    });
    assert!(
        seen.iter().any(|r| r == sv_check::decisions::NOT_HELD_TO),
        "{seen:?}"
    );
    // Kept while the running app's finding is, and gone with it when a person set that aside.
    let kept = decisions_then_reviews(vec![probe.clone()], &decided, as_is);
    assert_eq!(kept.findings.len(), 2);
    let set_aside = decisions_then_reviews(vec![probe], &decided, |mut findings| {
        findings.retain(|f| f.rule_id == sv_check::decisions::NOT_HELD_TO);
        as_is(findings)
    });
    assert!(set_aside.findings.is_empty(), "{:?}", set_aside.findings);
}

#[test]
fn a_line_is_gathered_after_the_reviews_so_a_verdict_sets_aside_one_problem_only() {
    let at = |rule: &str| {
        let mut f = sv_check::Finding {
            evidence: Vec::new(),
            rule_id: rule.to_owned(),
            title: rule.to_owned(),
            severity: sv_check::Severity::High,
            confidence: sv_check::Confidence::Medium,
            location: sv_check::Location {
                file: "app.py".to_owned(),
                line: 5,
            },
            secret: None,
            requirement_ids: Vec::new(),
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
        };
        f.fingerprint = format!("fp-{rule}");
        f
    };
    // A review that sets aside `ast.eval` by its rule, as a counted false alarm does.
    let out = decisions_then_reviews(vec![at("ast.sql"), at("ast.eval")], &[], |mut findings| {
        assert_eq!(findings.len(), 2, "the review sees each problem on its own");
        findings.retain(|f| f.rule_id != "ast.eval");
        sv_check::review::Outcome {
            findings,
            set_aside: Vec::new(),
            not_counted: Vec::new(),
            counted_in_test: Vec::new(),
            left_out: Vec::new(),
        }
    });
    assert_eq!(out.findings.len(), 1);
    assert_eq!(out.findings[0].rule_id, "ast.sql");
    assert!(
        out.findings[0].also_on_this_line.is_empty(),
        "{:?}",
        out.findings
    );
    // The control: with nothing set aside, the line is one finding holding the other.
    let out = decisions_then_reviews(vec![at("ast.sql"), at("ast.eval")], &[], |findings| {
        sv_check::review::Outcome {
            findings,
            set_aside: Vec::new(),
            not_counted: Vec::new(),
            counted_in_test: Vec::new(),
            left_out: Vec::new(),
        }
    });
    assert_eq!(out.findings.len(), 1);
    assert_eq!(out.findings[0].also_on_this_line.len(), 1);
}

#[test]
fn every_outside_tool_gets_an_entry_saying_whether_it_looked() {
    let adapters = sv_check::adapters::Adapters::load(&adapters_path()).unwrap();
    let run = sv_check::adapters::AdapterRun {
        ran: vec!["bandit".into()],
        partly: vec![("semgrep".into(), "told to skip tests/".into())],
        not_run: vec![
            (
                "semgrep".into(),
                "ran and found nothing, but was told not to look".into(),
                sv_check::adapters::NotRunCause::LeftOut,
            ),
            (
                "codeql-python".into(),
                "not installed".into(),
                sv_check::adapters::NotRunCause::NotInstalled,
            ),
        ],
        ..Default::default()
    };
    let examined = adapters_examined(&adapters, &["python".to_owned()], &run);
    let state = |rules: &str| {
        examined
            .iter()
            .find(|e| e.rules == rules)
            .unwrap_or_else(|| panic!("no entry for {rules}"))
            .state
    };
    use sv_report::ExaminedState::*;
    assert_eq!(state("bandit."), Ran);
    assert_eq!(
        state("semgrep."),
        Partly,
        "looking away outranks not having found anything"
    );
    assert_eq!(state("codeql-python."), NotRun);
    assert_eq!(state("gosec."), NothingToExamine);
    assert_eq!(state("brakeman."), NothingToExamine);
    assert_eq!(
        examined.len(),
        adapters.all().len(),
        "one entry per tool `sv` knows"
    );
}

#[test]
fn an_entry_names_the_stand_in_that_did_the_looking() {
    let adapters = sv_check::adapters::Adapters::load(&adapters_path()).unwrap();
    let said = "Opengrep ran in place of Semgrep, which is not installed on this computer.";
    let entry = |run: &sv_check::adapters::AdapterRun| {
        adapters_examined(&adapters, &["python".to_owned()], run)
            .into_iter()
            .find(|e| e.rules == "semgrep.")
            .unwrap()
    };
    let mut run = sv_check::adapters::AdapterRun {
        ran: vec!["semgrep".into(), "bandit".into()],
        stood_in: vec![("semgrep".into(), said.into())],
        ..Default::default()
    };
    let ran = entry(&run);
    assert_eq!(ran.state, sv_report::ExaminedState::Ran);
    assert_eq!(ran.stand_in.as_deref(), Some(said));
    // Said on the one tool it is about, and nowhere else.
    let bandit = adapters_examined(&adapters, &["python".to_owned()], &run)
        .into_iter()
        .find(|e| e.rules == "bandit.")
        .unwrap();
    assert_eq!(bandit.stand_in, None);
    // A run that read only part of the app says which program read that part.
    run.ran.clear();
    run.partly = vec![("semgrep".into(), "told to skip tests/".into())];
    let partly = entry(&run);
    assert_eq!(partly.state, sv_report::ExaminedState::Partly);
    assert_eq!(partly.stand_in.as_deref(), Some(said));
    let json = serde_json::to_value(&partly).unwrap();
    assert_eq!(json["stand_in"], said, "{json}");
}

fn untaught() -> Vec<sv_check::ast::Untaught> {
    vec![sv_check::ast::Untaught {
        rule_id: "ast.shell-command".to_owned(),
        title: "A shell command is built from a value".to_owned(),
        languages: vec!["rust".to_owned(), "zig".to_owned()],
    }]
}

#[test]
fn the_terminal_names_each_untaught_rule_and_its_languages() {
    let lines = untaught_lines(&untaught());
    assert!(lines[0].contains("Not looked for"), "{lines:?}");
    assert_eq!(
        lines[1],
        "  A shell command is built from a value (ast.shell-command) — not in rust, zig"
    );
    assert!(untaught_lines(&[]).is_empty(), "no heading over nothing");
}

#[test]
fn the_report_carries_each_untaught_rule_as_a_gap() {
    let gaps = untaught_gaps(&untaught());
    assert_eq!(gaps.len(), 1);
    assert_eq!(
        gaps[0].what,
        "A shell command is built from a value (ast.shell-command), in rust, zig"
    );
    assert!(
        gaps[0]
            .why
            .contains("has not been taught what to look for in rust or zig"),
        "{}",
        gaps[0].why
    );
}

fn data() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

#[test]
fn every_command_sees_the_checklist_levels_grounded_in_asvs() {
    // `scope`, `check` and `report` all load the frameworks through this one function. Without
    // the crosswalk step a checklist control keeps the level `sv` invented for it, and a report
    // files it as above the target on that alone.
    let frameworks = load_frameworks(&data()).expect("the frameworks load");
    let as02 = frameworks
        .get("SBD-AS-02")
        .expect("the checklist is loaded");
    assert_eq!(
        as02.level, 1,
        "nothing in ASVS asks for unified service discovery"
    );
    assert_eq!(
        frameworks
            .get("SBD-DM-01")
            .and_then(|r| r.level_basis.as_deref()),
        Some("level 2, as V14.1.1")
    );
}

#[test]
fn a_level_one_app_is_asked_the_controls_asvs_has_no_level_for() {
    // The same step seen from where it matters: bucketing. At level 1 a control nothing in ASVS
    // matches is applicable, not set aside on a level `sv` made up.
    let frameworks = load_frameworks(&data()).unwrap();
    let config = ApplicabilityConfig::load_v2(&data().join("knowledge"), &overlay_path()).unwrap();
    let buckets = sv_frameworks::applicability::bucket(
        &frameworks,
        &config,
        &sv_frameworks::applicability::ConditionContext::default(),
        1,
    );
    assert!(
        !buckets.out_of_level.iter().any(|id| id == "SBD-MT-02"),
        "SBD-MT-02 (metrics and dashboards) was set aside at level 1"
    );
}
