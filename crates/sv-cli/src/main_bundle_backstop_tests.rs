use super::*;

#[test]
fn a_report_holding_a_credential_is_not_zipped_and_the_refusal_does_not_quote_it() {
    let rules = SecretRules::load(&secret_rules_path()).unwrap();
    // Built from pieces, so this file holds none.
    let key = ["sk", "ant", "api03", "Zp8Kd3Wq1Ls6Vn0Rt4Yb9Xm2Qc"].join("-");
    let password = ["Qv7r", "Lm2x", "Tz9k"].concat();
    let (redacted, n) = sv_check::secrets::redact_text(
        &rules,
        &format!("Possible hardcoded password: '{password}' and {key}"),
    );
    assert_eq!(n, 2, "{redacted}");
    // What `sv` writes, redacted, goes in.
    let clean = vec![
        ("app/report/security.md".to_owned(), redacted.into_bytes()),
        (
            "app/report/report.json".to_owned(),
            b"{\"title\": \"fine\"}\n".to_vec(),
        ),
    ];
    refuse_a_credential_in_the_report(&rules, &clean).expect("a redacted report is zipped");
    // A value that reached a report whole does not, and the refusal says where, not what.
    for planted in [
        format!("Possible hardcoded password: '{password}'"),
        format!("Authorization: Bearer {key}"),
    ] {
        let mut files = clean.clone();
        files.push((
            "app/report/compliance.md".to_owned(),
            format!("# Report\n\n{planted}\n").into_bytes(),
        ));
        let refused = refuse_a_credential_in_the_report(&rules, &files)
            .expect_err("a report holding a credential is refused")
            .to_string();
        assert!(
            refused.contains("app/report/compliance.md line 3"),
            "{refused}"
        );
        assert!(
            !refused.contains(&password[..5]) && !refused.contains(&key[..8]),
            "the refusal quotes the value: {refused}"
        );
    }
}
