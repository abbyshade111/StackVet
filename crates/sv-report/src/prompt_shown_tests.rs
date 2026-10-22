//! "What StackVet saw" shows the prompt `sv` sent for an AI question, under "StackVet sent", as text (backlog 0623):
//! every character a reader cannot see written out, and markup escaped by each format as the app's answers are.

use crate::next_step_tests::empty_report;

fn report_with(prompt: &str) -> (crate::Report, sv_check::Finding) {
    let mut report = empty_report(Vec::new());
    report.seen = Some(crate::seen::Seen {
        signed_in: vec![crate::seen::Exchange {
            id: "ai-hidden-30".into(),
            method: "POST".into(),
            path: "/chat".into(),
            status: 200,
            headers: Vec::new(),
            body: "<p>Done.</p>".into(),
            prompt: Some(prompt.into()),
        }],
        ..Default::default()
    });
    let f = sv_check::Finding {
        evidence: vec!["ai-hidden-30".into()],
        also_reported_by: Vec::new(),
        fingerprint: "f".into(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        rule_id: "probe.ai-hidden-input".into(),
        title: "Hidden characters reach the model".into(),
        severity: sv_check::Severity::Medium,
        confidence: sv_check::Confidence::High,
        location: sv_check::Location::running_app(),
        secret: None,
        requirement_ids: Vec::new(),
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
    };
    report.findings.push(f.clone());
    (report, f)
}

#[test]
fn the_prompt_is_shown_with_its_invisible_characters_written_out() {
    let (report, f) = report_with("Summarize\u{200B} this <img src=x> SV-PROBE-HIDDEN-t30");
    let lines = crate::what_was_seen(&report, &f);
    assert_eq!(
        lines[0],
        "StackVet sent \"Summarize\\u{200B} this <img src=x> SV-PROBE-HIDDEN-t30\" to POST /chat, which answered 200: \"Done.\""
    );
    assert!(
        !lines[0].contains('\u{200B}'),
        "a zero-width character was left in: {:?}",
        lines[0]
    );
    // Each format escapes it as it escapes the app's own answers.
    let html = crate::html::page(&report);
    assert!(
        html.contains("StackVet sent &quot;Summarize\\u{200B} this &lt;img src=x&gt;"),
        "{html}"
    );
    assert!(!html.contains("<img src=x>"), "{html}");
    let md = crate::markdown::security(&report);
    assert!(md.contains("StackVet sent"), "{md}");
    assert!(!md.contains('\u{200B}'), "{md}");
}

#[test]
fn an_exchange_with_no_prompt_reads_as_before() {
    let (mut report, f) = report_with("unused");
    report.seen.as_mut().unwrap().signed_in[0].prompt = None;
    let lines = crate::what_was_seen(&report, &f);
    assert_eq!(lines[0], "POST /chat answered 200: \"Done.\"");
}
