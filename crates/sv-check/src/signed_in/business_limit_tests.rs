//! A business limit the documentation states (V2.3.2, backlog 0489): one past `at-most` accepted is a finding, refused
//! is credited in part for the one limit named, and anything else (a rate limiter's answer, a refusal before the limit,
//! no limit stated) is said and credits nothing.

use super::fake_app::*;
use super::*;

/// `[policy] limit`: projects per account, at most `n`, through the stand-in app's `/projects`, which no other check uses.
fn projects_at_most(n: u32) -> sv_manifest::PolicySection {
    sv_manifest::PolicySection {
        limit: Some(sv_manifest::BusinessLimit {
            name: "projects per account".to_owned(),
            at_most: n,
            method: "POST".to_owned(),
            path: "/projects".to_owned(),
            form: [("title", "{marker}"), ("csrf_token", "{csrf}")]
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
            json: Default::default(),
        }),
        ..Default::default()
    }
}

/// The scripted app, changed by `adjust`, against `policy`.
fn run_with(policy: &sv_manifest::PolicySection, adjust: impl FnOnce(&mut FakeApp)) -> Outcome {
    let acc = accounts();
    let mut app = FakeApp::new(Flaws::default());
    adjust(&mut app);
    app.users
        .insert(acc.a.user.clone(), (acc.a.password.clone(), false));
    app.users
        .insert(acc.b.user.clone(), (acc.b.password.clone(), false));
    super::run(&mut app, &users(), &acc, true, policy)
}

fn said_of_v2_3_2(o: &Outcome) -> Vec<&str> {
    o.not_assessed
        .iter()
        .filter(|(id, _)| id == "V2.3.2")
        .map(|(_, why)| why.as_str())
        .collect()
}

#[test]
fn one_past_the_limit_accepted_is_a_finding_and_credits_nothing() {
    let o = run_with(&projects_at_most(3), |_| {});
    let f = o
        .findings
        .iter()
        .find(|f| f.rule_id == BUSINESS_LIMIT.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert_eq!(f.requirement_ids, ["V2.3.2"]);
    assert!(
        f.description
            .contains("\"projects per account\" is at most 3"),
        "{}",
        f.description
    );
    assert_eq!(f.evidence, ["limit-1", "limit-4"]);
    assert!(!verified_ids(&o).contains(&BUSINESS_LIMIT.rule_id));
}

#[test]
fn the_limit_held_on_the_server_is_credited_in_part_for_the_one_limit() {
    let o = run_with(&projects_at_most(3), |app| app.most_projects = Some(3));
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == BUSINESS_LIMIT.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("the one limit named"),
        "{}",
        credit.scope
    );
    assert!(
        o.findings
            .iter()
            .all(|f| f.rule_id != BUSINESS_LIMIT.rule_id)
    );
}

#[test]
fn a_refusal_before_the_stated_number_is_said_and_credits_nothing() {
    // The app allows two, and stackvet.toml says three: the third is refused, before the limit was reached.
    let o = run_with(&projects_at_most(3), |app| app.most_projects = Some(2));
    assert!(!verified_ids(&o).contains(&BUSINESS_LIMIT.rule_id));
    assert!(
        o.findings
            .iter()
            .all(|f| f.rule_id != BUSINESS_LIMIT.rule_id)
    );
    assert!(
        said_of_v2_3_2(&o)
            .iter()
            .any(|why| why.contains("before the 3 stackvet.toml states")),
        "{:?}",
        said_of_v2_3_2(&o)
    );
}

#[test]
fn no_limit_stated_or_one_too_high_is_said() {
    let o = run_with(&Default::default(), |_| {});
    assert!(
        said_of_v2_3_2(&o)
            .iter()
            .any(|why| why.contains("name one as `limit`"))
    );
    let o = run_with(&projects_at_most(MOST_TRIED + 1), |_| {});
    assert!(
        said_of_v2_3_2(&o)
            .iter()
            .any(|why| why.contains("cannot hold the app to that number"))
    );
    for o in [
        run_with(&Default::default(), |_| {}),
        run_with(&projects_at_most(MOST_TRIED + 1), |_| {}),
    ] {
        assert!(!verified_ids(&o).contains(&BUSINESS_LIMIT.rule_id));
    }
}

#[test]
fn a_rate_limiter_s_answer_past_the_limit_is_not_the_limit_held() {
    // The same refusal at the same place, but 429: a rate limiter's answer, never read as the business limit (ADR-021).
    let o = run_with(&projects_at_most(3), |app| {
        app.most_projects = Some(3);
        app.projects_limit_status = Some(429);
    });
    assert!(
        !verified_ids(&o).contains(&BUSINESS_LIMIT.rule_id),
        "{:#?}",
        o.verified
    );
    assert!(
        o.findings
            .iter()
            .all(|f| f.rule_id != BUSINESS_LIMIT.rule_id)
    );
    assert!(
        said_of_v2_3_2(&o)
            .iter()
            .any(|why| why.contains("a rate limiter's answer, not the limit's")),
        "{:?}",
        said_of_v2_3_2(&o)
    );
}
