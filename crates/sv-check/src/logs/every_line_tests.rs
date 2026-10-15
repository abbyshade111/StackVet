//! Backlog 0232 part 5 (ADR-053, Later): the line's metadata (V16.2.1), its timestamp's zone (V16.2.2), and its format
//! (V16.2.4) are read from every security event's line found, not the refused sign-in's alone. Each is whole only when
//! another line was read too and every one passes; a time with no zone on any of them is the finding.

use super::*;

const NOBODY: &str = "sv-log-nobody-4a91@example.test";
const OK: &str = "sv-log-ok-4a91@example.test";

fn markers() -> Markers {
    Markers {
        failed_sign_in: Some(NOBODY.into()),
        successful_sign_in: Some(OK.into()),
        wrong_password: Some(OK.into()),
        ..Default::default()
    }
}

/// A logfmt line about `who`, with a zoned time unless `zone` is false, and a source address.
fn line(event: &str, who: &str, zone: bool) -> String {
    format!(
        "time=2026-09-26T10:00:01{} level=warn event={event} account={who} from=10.0.0.7",
        if zone { "Z" } else { "" }
    )
}

fn credit<'a>(o: &'a LogOutcome, id: &str) -> Option<&'a crate::Verified> {
    o.verified.iter().find(|v| v.check_id == id)
}

#[test]
fn every_line_passing_makes_all_three_whole() {
    let log = [
        line("sign-in-failed", NOBODY, true),
        line("sign-in-ok", OK, true),
        line("sign-in-failed-wrong-password", OK, true),
    ]
    .join("\n");
    let o = evaluate(&markers(), &log);
    for id in [
        "probe.log-line-metadata",
        "probe.log-timestamp-zoned",
        "probe.log-common-format",
    ] {
        let v = credit(&o, id).unwrap_or_else(|| panic!("{id} not credited: {:?}", o.not_assessed));
        assert!(!v.in_part, "{id}: {}", v.scope);
        assert!(v.scope.contains("2 other"), "{id}: {}", v.scope);
    }
    assert!(o.findings.is_empty(), "{:?}", o.findings);
}

#[test]
fn another_line_with_no_zone_is_the_finding_about_that_line() {
    let log = [
        line("sign-in-failed", NOBODY, true),
        line("sign-in-ok", OK, true),
        line("sign-in-failed-wrong-password", OK, false),
    ]
    .join("\n");
    let o = evaluate(&markers(), &log);
    assert!(credit(&o, "probe.log-timestamp-zoned").is_none());
    let f = o
        .findings
        .iter()
        .find(|f| f.rule_id == "probe.log-timestamp-zoned")
        .expect("the finding");
    assert!(
        f.description.contains(
            "The line recording the wrong password for an account that exists carried the time"
        ),
        "{}",
        f.description
    );
}

#[test]
fn another_line_in_free_text_keeps_the_format_in_part() {
    let log = [
        line("sign-in-failed", NOBODY, true),
        format!("2026-09-26T10:00:02Z auth: sign-in ok for {OK} from 10.0.0.7"),
    ]
    .join("\n");
    let o = evaluate(&markers(), &log);
    let v = credit(&o, "probe.log-common-format").expect("credited");
    assert!(v.in_part, "{}", v.scope);
}

#[test]
fn a_sign_in_line_found_only_by_when_it_was_written_keeps_the_metadata_in_part() {
    // The accepted sign-in is found between its marked requests, not by the account's name: nothing shows who.
    let mut m = markers();
    m.successful_sign_in = None;
    m.wrong_password = None;
    m.successful_window = Some(Window {
        open: "sv-log-before-ok-t".into(),
        close: "sv-log-after-ok-t".into(),
        login_path: "/login".into(),
    });
    let log = [
        line("sign-in-failed", NOBODY, true),
        "time=2026-09-26T10:00:02Z path=/sv-log-before-ok-t status=404".to_owned(),
        "time=2026-09-26T10:00:02Z level=info event=login user_id=7 from=10.0.0.7".to_owned(),
        "time=2026-09-26T10:00:02Z path=/sv-log-after-ok-t status=404".to_owned(),
    ]
    .join("\n");
    let o = evaluate(&m, &log);
    let v = credit(&o, "probe.log-line-metadata").expect("credited");
    assert!(v.in_part, "{}", v.scope);
    // The zone and the format do not depend on who, and every line read passes both.
    assert!(
        !credit(&o, "probe.log-timestamp-zoned")
            .expect("zone")
            .in_part
    );
}
