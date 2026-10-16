//! `ast.request-parameter-source-merged` (backlog 0498, V15.3.7): a request value read through a call
//! that merges the address, the form, and sometimes cookies, so the code does not say where it came from. Only ever a
//! finding: a clean result does not show a defense against a name sent twice.

use super::*;
use std::path::PathBuf;

const RULE: &str = "ast.request-parameter-source-merged";

fn rules() -> &'static AstRules {
    static RULES: OnceLock<AstRules> = OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// Whether the rule reports `source`, read as `file`. A file the parser could not read, or a query that did not
/// compile, fails the test rather than reading as clean.
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

fn check(language: &str, file: &str, wrap: impl Fn(&str) -> String, cases: &[(&str, bool)]) {
    for (code, expected) in cases {
        assert_eq!(
            reported(language, file, &wrap(code)),
            *expected,
            "{language}: `{code}` should{} be reported",
            if *expected { "" } else { " not" }
        );
    }
}

#[test]
fn the_rule_is_only_ever_a_finding_against_v15_3_7() {
    let rule = rules()
        .rules()
        .find(|r| r.id == RULE)
        .expect("the rule is shipped");
    assert_eq!(rule.requirement_ids, ["V15.3.7"]);
    assert!(rule.findings_only, "a clean result must credit nothing");
}

#[test]
fn python_merged_reads_are_found_and_one_source_reads_are_not() {
    check(
        "python",
        "app.py",
        |code| format!("from flask import request\n\ndef view():\n    return {code}\n"),
        &[
            ("request.values.get('role')", true),
            ("request.values['role']", true),
            ("request.REQUEST['role']", true),
            ("request.args.get('role')", false),
            ("request.form['role']", false),
            ("settings.values", false),
        ],
    );
}

#[test]
fn javascript_and_typescript_merged_reads_are_found_and_one_source_reads_are_not() {
    for (language, file) in [("javascript", "server.js"), ("typescript", "server.ts")] {
        check(
            language,
            file,
            |code| {
                format!(
                    "app.get('/a', (req, res) => {{\n  const v = {code};\n  res.end(v);\n}});\n"
                )
            },
            &[
                ("req.param('role')", true),
                ("request.param('role')", true),
                ("req.query.role", false),
                ("req.body.role", false),
                ("req.params.role", false),
                ("url.param('role')", false),
            ],
        );
    }
}

#[test]
fn php_go_java_and_csharp_merged_reads_are_found_and_one_source_reads_are_not() {
    check(
        "php",
        "index.php",
        |code| format!("<?php\n$v = {code};\necho $v;\n"),
        &[
            ("$_REQUEST['role']", true),
            ("$_GET['role']", false),
            ("$_POST['role']", false),
        ],
    );
    check(
        "go",
        "main.go",
        |code| {
            format!(
                "package main\n\nimport \"net/http\"\n\nfunc h(w http.ResponseWriter, r *http.Request) {{\n\tv := {code}\n\t_ = v\n}}\n"
            )
        },
        &[
            ("r.FormValue(\"role\")", true),
            ("r.PostFormValue(\"role\")", false),
            ("r.URL.Query().Get(\"role\")", false),
        ],
    );
    check(
        "java",
        "Handler.java",
        |code| {
            format!(
                "class Handler {{\n  void doGet(HttpServletRequest request) {{\n    String v = {code};\n  }}\n}}\n"
            )
        },
        &[
            ("request.getParameter(\"role\")", true),
            ("request.getHeader(\"role\")", false),
            ("config.getParameter(\"role\")", false),
        ],
    );
    check(
        "csharp",
        "Handler.cs",
        |code| format!("class Handler {{\n  void Run() {{\n    var v = {code};\n  }}\n}}\n"),
        &[
            ("Request[\"role\"]", true),
            ("HttpContext.Current.Request[\"role\"]", true),
            ("Request.QueryString[\"role\"]", false),
            ("Request.Form[\"role\"]", false),
        ],
    );
}
