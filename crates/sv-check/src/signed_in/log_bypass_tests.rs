//! Backlog 0499 (V16.3.3): the forgery check brackets its first forged request with two marked requests, and the log
//! reader credits, in part, a refusal the app wrote down while handling it.

use super::Outcome;
use super::fake_app::{FakeApp, Flaws, LogStyle, accounts};

const RULE: &str = "probe.bypass-attempt-logged";

fn run(style: LogStyle, flaws: Flaws) -> (Outcome, String) {
    let mut app = FakeApp::new(flaws);
    app.log_style = Some(style);
    let mut acc = accounts();
    acc.admin = None;
    acc.totp = None;
    let u = super::tests::with_signup();
    let out = super::run(
        &mut app,
        &u,
        &acc,
        false,
        &sv_manifest::PolicySection::default(),
    );
    (out, app.log.join("\n"))
}

#[test]
fn a_refused_forgery_the_app_logged_is_credited_in_part() {
    for style in [LogStyle::Full, LogStyle::Private] {
        let (out, log) = run(style, Flaws::default());
        let (w, status) = out
            .log_markers
            .bypass
            .as_ref()
            .expect("the setup: the forged request was refused and bracketed");
        assert!(*status >= 400, "{style:?}: refused with {status}");
        assert!(
            log.contains(&w.open) && log.contains(&w.close),
            "the setup: both markers reached the log"
        );
        let o = crate::logs::evaluate(&out.log_markers, &log);
        let v = o
            .verified
            .iter()
            .find(|v| v.check_id == RULE)
            .unwrap_or_else(|| panic!("{style:?}: credited"));
        assert!(v.in_part);
        assert_eq!(v.requirement_ids, ["V16.3.3"]);
    }
}

#[test]
fn a_forgery_answered_with_a_200_page_is_not_judged_refused_and_plants_nothing() {
    // The forgery check reads a 200 page as neither taken nor refused (backlog 0560), so there is no refused attempt
    // whose record could be looked for: V16.3.3 is not spoken to at all.
    let flaws = Flaws {
        forgery_refused_on_200: true,
        ..Default::default()
    };
    let (out, log) = run(LogStyle::Full, flaws);
    assert!(out.log_markers.bypass.is_none());
    let o = crate::logs::evaluate(&out.log_markers, &log);
    assert!(!o.verified.iter().any(|v| v.check_id == RULE));
    assert!(!o.not_assessed.iter().any(|(ids, _)| ids == "V16.3.3"));
}

#[test]
fn a_forgery_the_app_took_plants_nothing() {
    let flaws = Flaws {
        no_csrf_check: true,
        ..Default::default()
    };
    let (out, _) = run(LogStyle::Full, flaws);
    assert!(out.log_markers.bypass.is_none());
}
