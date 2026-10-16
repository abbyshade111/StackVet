//! Backlog 0232, part D: each upload route stackvet.toml names is run through the same checks, and a credit stands
//! only when every route gave it.

use super::super::fake_app::*;
use super::*;

fn route(path: &str) -> UploadSection {
    UploadSection {
        path: path.into(),
        field: "file".into(),
        ..Default::default()
    }
}

/// A check that credits `check.x` (V5.2.1) on the routes `credits` names, in part on those `in_part` names.
fn fake_check(
    credits: &'static [&'static str],
    in_part: &'static [&'static str],
) -> impl FnMut(&UploadSection, &mut Outcome) {
    move |r: &UploadSection, out: &mut Outcome| {
        out.steps.push(format!("checked {}", r.path));
        if credits.contains(&r.path.as_str()) {
            let credit =
                crate::Verified::new("check.x", &["V5.2.1"], format!("files on {}", r.path))
                    .with_evidence(vec![format!("seen-{}", r.path)]);
            out.verified.push(if in_part.contains(&r.path.as_str()) {
                credit.in_part()
            } else {
                credit
            });
        }
    }
}

#[test]
fn one_route_runs_as_before() {
    let mut out = Outcome::default();
    on_each_upload(
        &[route("/up")],
        |_| true,
        &mut out,
        fake_check(&["/up"], &[]),
    );
    assert_eq!(out.steps, ["checked /up"], "no route heading for one route");
    assert_eq!(out.verified.len(), 1);
    assert_eq!(out.verified[0].scope, "files on /up");
}

#[test]
fn a_credit_every_route_gave_stands_once_naming_them_all() {
    let mut out = Outcome::default();
    on_each_upload(
        &[route("/a"), route("/b")],
        |_| true,
        &mut out,
        fake_check(&["/a", "/b"], &[]),
    );
    assert_eq!(out.verified.len(), 1, "{:?}", out.verified);
    let credit = &out.verified[0];
    assert!(
        credit
            .scope
            .contains("on each of the 2 upload routes: `/a`, `/b`"),
        "{}",
        credit.scope
    );
    assert_eq!(credit.evidence, ["seen-/a", "seen-/b"]);
    assert!(!credit.in_part);
    assert!(out.steps.contains(&"the upload route `/b`:".to_owned()));
    assert!(out.not_assessed.is_empty());
}

#[test]
fn a_credit_one_route_did_not_give_is_withheld_naming_it() {
    let mut out = Outcome::default();
    on_each_upload(
        &[route("/a"), route("/b")],
        |_| true,
        &mut out,
        fake_check(&["/a"], &[]),
    );
    assert!(out.verified.is_empty(), "{:?}", out.verified);
    let (ids, why) = &out.not_assessed[0];
    assert_eq!(ids, "V5.2.1");
    assert!(why.contains("not credited on `/b`"), "{why}");
}

#[test]
fn in_part_on_any_route_is_in_part() {
    let mut out = Outcome::default();
    on_each_upload(
        &[route("/a"), route("/b")],
        |_| true,
        &mut out,
        fake_check(&["/a", "/b"], &["/b"]),
    );
    assert!(out.verified[0].in_part);
}

#[test]
fn a_second_route_the_app_does_not_have_withholds_the_upload_credits() {
    let mut one = users();
    one.upload = vec![UploadSection {
        path: "/upload".into(),
        field: "file".into(),
        form: [("csrf_token".to_owned(), "{csrf}".to_owned())].into(),
        serves_at: Some("/files/{name}".into()),
        max_bytes: Some(UPLOAD_LIMIT as u64),
        ..Default::default()
    }];
    let alone = run_against(Flaws::default(), &one);
    let credited = |o: &Outcome| {
        o.verified
            .iter()
            .any(|v| v.requirement_ids.iter().any(|r| r == "V5.2.1"))
    };
    assert!(
        credited(&alone),
        "the setup: one route is credited: {:?}",
        alone.not_assessed
    );
    let mut two = one.clone();
    two.upload.push(UploadSection {
        path: "/nowhere".into(),
        ..one.upload[0].clone()
    });
    let both = run_against(Flaws::default(), &two);
    assert!(!credited(&both), "{:?}", both.verified);
    assert!(
        both.not_assessed
            .iter()
            .any(|(_, why)| why.contains("`/nowhere`")),
        "{:?}",
        both.not_assessed
    );
}

#[test]
fn a_route_the_check_does_not_apply_to_holds_no_credit_back() {
    // As for the archive check, on a route that says it unpacks none.
    let mut out = Outcome::default();
    on_each_upload(
        &[route("/a"), route("/b")],
        |r| r.path != "/b",
        &mut out,
        fake_check(&["/a"], &[]),
    );
    assert_eq!(out.verified.len(), 1, "{:?}", out.not_assessed);
    assert!(
        out.verified[0]
            .scope
            .contains("on `/a`, the one upload route it applies to"),
        "{}",
        out.verified[0].scope
    );
    assert!(out.not_assessed.is_empty(), "{:?}", out.not_assessed);
    assert!(
        out.steps.contains(&"checked /b".to_owned()),
        "the route is still run"
    );
}
