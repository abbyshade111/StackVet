//! `ast.otp-time-from-request` (backlog 0625, part 12, V6.5.8): a one-time code checked against a time the request
//! sent, so whoever sends it chooses the moment the code is checked for. Only ever a finding.

use super::*;
use std::path::PathBuf;

const RULE: &str = "ast.otp-time-from-request";

fn rules() -> &'static AstRules {
    static RULES: OnceLock<AstRules> = OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// Whether the rule reports `source`, read as `file`. A file the parser could not read fails the test rather than
/// reading as clean.
fn reported(language: &str, file: &str, source: &str) -> bool {
    let read = read_file(rules(), language, file, source);
    assert!(!read.parse_error, "{file} did not parse: {source}");
    assert!(
        read.broken.is_empty(),
        "a query did not compile: {:?}",
        read.broken
    );
    read.findings.iter().any(|f| f.rule_id == RULE)
}

const PYTHON: &[(&str, bool)] = &[
    (
        "import pyotp\ndef check(request, secret):\n    return pyotp.TOTP(secret).verify(request.form[\"code\"], for_time=request.form[\"at\"])\n",
        true,
    ),
    (
        "import pyotp\ndef check(request, totp):\n    at = int(request.json[\"time\"])\n    return totp.verify(request.json[\"code\"], for_time=at)\n",
        true,
    ),
    // The server's own clock: no time given, or one the server made.
    (
        "import pyotp\ndef check(request, totp):\n    return totp.verify(request.form[\"code\"])\n",
        false,
    ),
    (
        "import pyotp, time\ndef check(request, totp):\n    return totp.verify(request.form[\"code\"], for_time=time.time())\n",
        false,
    ),
    // `valid_window` is no time.
    (
        "def check(request, totp):\n    return totp.verify(request.form[\"code\"], valid_window=1)\n",
        false,
    ),
];

const JAVASCRIPT: &[(&str, bool)] = &[
    (
        "const speakeasy = require('speakeasy');\napp.post('/2fa', (req, res) => {\n  const ok = speakeasy.totp.verify({ secret: user.secret, encoding: 'base32', token: req.body.token, time: req.body.time });\n});\n",
        true,
    ),
    (
        "app.post('/2fa', (req, res) => {\n  const t = req.query.t;\n  speakeasy.totp.verifyDelta({ secret, token: req.body.token, epoch: t });\n});\n",
        true,
    ),
    (
        "app.post('/2fa', (req, res) => {\n  speakeasy.totp.verify({ secret, encoding: 'base32', token: req.body.token });\n});\n",
        false,
    ),
    (
        "app.post('/2fa', (req, res) => {\n  speakeasy.totp.verify({ secret, token: req.body.token, time: Math.floor(Date.now() / 1000) });\n});\n",
        false,
    ),
];

const GO: &[(&str, bool)] = &[
    (
        "package main\nfunc check(w http.ResponseWriter, r *http.Request) {\n\tts, _ := strconv.ParseInt(r.FormValue(\"t\"), 10, 64)\n\tok, _ := totp.ValidateCustom(r.FormValue(\"code\"), secret, time.Unix(ts, 0), opts)\n\t_ = ok\n}\n",
        true,
    ),
    (
        "package main\nfunc check(w http.ResponseWriter, r *http.Request) {\n\tok, _ := totp.ValidateCustom(r.FormValue(\"code\"), secret, time.Now().UTC(), opts)\n\t_ = ok\n}\n",
        false,
    ),
];

#[test]
fn a_one_time_code_checked_against_the_requests_time_is_found_in_python() {
    for (source, want) in PYTHON {
        assert_eq!(reported("python", "auth.py", source), *want, "{source}");
    }
}

#[test]
fn a_one_time_code_checked_against_the_requests_time_is_found_in_javascript_and_typescript() {
    for (source, want) in JAVASCRIPT {
        assert_eq!(reported("javascript", "auth.js", source), *want, "{source}");
        assert_eq!(reported("typescript", "auth.ts", source), *want, "{source}");
    }
}

#[test]
fn a_one_time_code_checked_against_the_requests_time_is_found_in_go() {
    for (source, want) in GO {
        assert_eq!(reported("go", "auth.go", source), *want, "{source}");
    }
}

#[test]
fn the_finding_cites_v6_5_8_only_and_credits_nothing() {
    let read = read_file(rules(), "python", "auth.py", PYTHON[0].0);
    let f = read
        .findings
        .iter()
        .find(|f| f.rule_id == RULE)
        .expect("the first case is reported");
    assert_eq!(f.requirement_ids, ["V6.5.8"]);
    assert!(rules().rules().any(|r| r.id == RULE && r.findings_only));
}
