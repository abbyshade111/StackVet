//! Four rules credit a whole requirement from a list of calls they look for, and each list has gaps the
//! requirement covers (backlog 0418): V1.3.2 names Spring Expression Language, and Java is taught only
//! `eval`; V1.2.5 is OS command injection, and `asyncio.create_subprocess_shell` is in no Python list;
//! V5.3.2 is every file operation, and `os.remove` is not read; V3.7.2 is every redirect, and a
//! `Location` header set by hand is not read. A clean run of each is *checked in part*, as the deny-list
//! rules' are (0281, 0302). Each case below is one of those gaps, so "clean" is exactly what is wrong
//! with a whole credit.

use super::*;
use std::path::PathBuf;

fn rules() -> &'static AstRules {
    static RULES: OnceLock<AstRules> = OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// The credit `rule` gives for `requirement` on an app of one file, `name`, holding `code`; the rule
/// must have found nothing in it.
fn credit(tag: &str, name: &str, code: &str, rule: &str, requirement: &str) -> crate::Verified {
    let dir = std::env::temp_dir().join(format!("sv-ast-call-lists-{tag}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join(name), code).unwrap();
    let scan = scan_dir(rules(), &dir);
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        scan.findings.iter().all(|f| f.rule_id != rule),
        "the setup: {rule} found something in {name}: {:?}",
        scan.findings
    );
    let credit = scan
        .verified
        .iter()
        .find(|v| v.check_id == rule)
        .unwrap_or_else(|| panic!("the setup: {rule} gave no credit for {name}"))
        .clone();
    assert_eq!(credit.requirement_ids, vec![requirement.to_owned()]);
    credit
}

#[test]
fn a_clean_run_of_the_call_list_rules_credits_their_requirements_in_part() {
    let cases = [
        (
            "spel",
            "App.java",
            "import org.springframework.expression.spel.standard.SpelExpressionParser;\n\
             class App {\n  Object run(String input) {\n    \
             return new SpelExpressionParser().parseExpression(input).getValue();\n  }\n}\n",
            "ast.dynamic-code-execution",
            "V1.3.2",
        ),
        (
            "shell",
            "app.py",
            "import asyncio\nasync def run(cmd):\n    \
             return await asyncio.create_subprocess_shell(cmd)\n",
            "ast.shell-command",
            "V1.2.5",
        ),
        (
            "path",
            "app.py",
            "import os\ndef forget(name):\n    os.remove(name)\n",
            "ast.file-path-from-value",
            "V5.3.2",
        ),
        (
            "redirect",
            "server.js",
            "function go(req, res) {\n  res.setHeader('Location', req.query.next);\n  \
             res.statusCode = 302;\n  res.end();\n}\n",
            "ast.open-redirect",
            "V3.7.2",
        ),
    ];
    for (tag, name, code, rule, requirement) in cases {
        let credit = credit(tag, name, code, rule, requirement);
        assert!(
            credit.in_part,
            "{rule}: not seeing the calls it looks for is part of {requirement}, not all of it: {credit:?}"
        );
    }
}
