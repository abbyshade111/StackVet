//! Backlog 0625, part 6, ADR-147: the code and refresh-token lifetimes django-oauth-toolkit and node-oidc-provider are
//! set to. A code over ten minutes, and a refresh token that never expires, are findings; a code of a minute or less,
//! and a refresh token with an end, are credited in part; what cannot be told is left unsaid.

use super::*;
use crate::test_support::scratch;

fn report_of(files: &[(&str, &str)]) -> ConfigReport {
    let dir = scratch("");
    for (name, text) in files {
        let path = dir.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    let listing = Listing::of(&dir);
    let sbom = crate::sbom::build_in(&listing);
    let mut report = ConfigReport::default();
    check(&listing, &sbom, &mut report);
    std::fs::remove_dir_all(&dir).ok();
    report
}

fn found(report: &ConfigReport) -> Vec<&str> {
    report.findings.iter().map(|f| f.rule_id.as_str()).collect()
}

fn credited(report: &ConfigReport) -> Vec<&str> {
    report
        .passed
        .iter()
        .inspect(|v| assert!(v.in_part, "{v:?}"))
        .map(|v| v.check_id.as_str())
        .collect()
}

fn django(settings: &str) -> Vec<(&'static str, String)> {
    vec![
        (
            "requirements.txt",
            "django==5.1.2\ndjango-oauth-toolkit==3.0.1\n".to_owned(),
        ),
        (
            "app/settings.py",
            format!("INSTALLED_APPS = ['oauth2_provider']\nOAUTH2_PROVIDER = {{\n{settings}}}\n"),
        ),
    ]
}

fn owned<'a>(files: &'a [(&'static str, String)]) -> Vec<(&'static str, &'a str)> {
    files.iter().map(|(n, t)| (*n, t.as_str())).collect()
}

#[test]
fn django_with_a_short_code_and_refresh_tokens_that_end_is_credited_in_part() {
    let files = django(
        "    \"AUTHORIZATION_CODE_EXPIRE_SECONDS\": 60,\n    \"REFRESH_TOKEN_EXPIRE_SECONDS\": 30 * 24 * 3600,\n    \
         \"ROTATE_REFRESH_TOKEN\": False,\n",
    );
    let report = report_of(&owned(&files));
    assert!(report.findings.is_empty(), "{:#?}", report.findings);
    assert_eq!(credited(&report), [CODE_LIFETIME, REFRESH_EXPIRY]);
    assert!(
        report.passed[1].scope.contains("2592000 seconds"),
        "{}",
        report.passed[1].scope
    );
}

#[test]
fn django_left_at_its_defaults_has_short_codes_and_refresh_tokens_that_never_expire() {
    // The setup: the package is installed and nothing about lifetimes is written.
    let files = vec![
        (
            "requirements.txt",
            "django-oauth-toolkit==3.0.1\n".to_owned(),
        ),
        (
            "app/settings.py",
            "INSTALLED_APPS = ['oauth2_provider']\n".to_owned(),
        ),
    ];
    let report = report_of(&owned(&files));
    assert_eq!(found(&report), [REFRESH_EXPIRY]);
    assert_eq!(report.findings[0].location.file, "app/settings.py");
    assert!(report.findings[0].description.contains("never expire"));
    assert_eq!(credited(&report), [CODE_LIFETIME]);
}

#[test]
fn a_code_past_ten_minutes_is_a_finding_and_between_one_and_ten_nothing_is_said() {
    let long = report_of(&owned(&django(
        "    \"AUTHORIZATION_CODE_EXPIRE_SECONDS\": 3600,\n",
    )));
    assert_eq!(found(&long), [CODE_LIFETIME, REFRESH_EXPIRY]);
    let between = report_of(&owned(&django(
        "    \"AUTHORIZATION_CODE_EXPIRE_SECONDS\": 300,\n",
    )));
    assert_eq!(found(&between), [REFRESH_EXPIRY]);
    assert!(
        credited(&between).is_empty(),
        "five minutes passes at level 2 and fails at level 3"
    );
}

#[test]
fn a_refresh_limit_with_rotation_on_or_a_value_from_elsewhere_is_not_judged() {
    for settings in [
        "    \"REFRESH_TOKEN_EXPIRE_SECONDS\": 86400,\n",
        "    \"REFRESH_TOKEN_EXPIRE_SECONDS\": int(os.environ[\"REFRESH_SECONDS\"]),\n    \"ROTATE_REFRESH_TOKEN\": False,\n",
    ] {
        let report = report_of(&owned(&django(settings)));
        assert!(
            found(&report).is_empty(),
            "{settings}: {:#?}",
            report.findings
        );
        assert_eq!(credited(&report), [CODE_LIFETIME], "{settings}");
    }
    let never = report_of(&owned(&django(
        "    \"REFRESH_TOKEN_EXPIRE_SECONDS\": None,\n",
    )));
    assert_eq!(found(&never), [REFRESH_EXPIRY]);
}

#[test]
fn node_oidc_provider_at_its_defaults_is_credited_and_endless_rotation_is_a_finding() {
    let defaults = "import Provider from 'oidc-provider';\nconst provider = new Provider(issuer, { clients });\n";
    let report = report_of(&[("server.js", defaults)]);
    assert!(report.findings.is_empty(), "{:#?}", report.findings);
    assert_eq!(credited(&report), [CODE_LIFETIME, REFRESH_EXPIRY]);
    let endless = "import Provider from 'oidc-provider';\nconst provider = new Provider(issuer, {\n  rotateRefreshToken: true,\n  \
                   ttl: { AuthorizationCode: 15 * 60 },\n});\n";
    let report = report_of(&[("server.js", endless)]);
    assert_eq!(found(&report), [CODE_LIFETIME, REFRESH_EXPIRY]);
    assert_eq!(report.findings[1].location.line, 3);
    assert!(credited(&report).is_empty());
}

#[test]
fn an_app_with_neither_library_is_told_nothing() {
    let report = report_of(&[(
        "app.py",
        "SECRET = 1\nREFRESH_TOKEN_EXPIRE_SECONDS = None\n",
    )]);
    assert!(
        report.findings.is_empty() && report.passed.is_empty(),
        "{:#?}",
        report.findings
    );
}
