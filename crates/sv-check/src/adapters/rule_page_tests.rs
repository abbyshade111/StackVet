//! A tool's page about its rule is kept only when it is a plain web address (backlog 0378): the address comes from
//! the tool's report, and is written into `report.html` as a link and into `security.md`.
use super::*;

#[test]
fn the_pages_tools_give_are_kept() {
    for uri in [
        "https://semgrep.dev/r/python.flask.security.injection.tainted-sql-string.tainted-sql-string",
        "https://bandit.readthedocs.io/en/1.9.4/plugins/b105_hardcoded_password_string.html",
        "https://codeql.github.com/codeql-query-help/python/py-sql-injection/#overview",
        "http://localhost:8080/rules?id=x&lang=py",
    ] {
        assert_eq!(kept_help_uri(uri).as_deref(), Some(uri), "{uri}");
    }
}

#[test]
fn an_address_that_could_be_more_than_a_link_is_dropped() {
    let long = format!("https://example.com/{}", "a".repeat(HELP_URI_CHARS));
    for uri in [
        "javascript:alert(1)",
        "JAVASCRIPT:alert(1)",
        "data:text/html,<script>alert(1)</script>",
        "ftp://example.com/rule",
        "//example.com/rule",
        "https://",
        "https:///rule",
        "https://example.com/a b",
        "https://example.com/\"onmouseover=\"alert(1)",
        "https://example.com/'x",
        "https://example.com/a)[click](javascript:alert(1)",
        "https://example.com/<script>",
        "https://example.com/a\nb",
        "https://exa mple.com/",
        "https://user@evil.com\\@example.com/",
        "https://example.com/`x`",
        long.as_str(),
    ] {
        assert_eq!(kept_help_uri(uri), None, "{uri:?} was kept");
    }
}

fn sarif(help_uri: serde_json::Value) -> String {
    serde_json::json!({
        "version": "2.1.0",
        "runs": [{
            "tool": { "driver": { "name": "tool", "rules": [
                { "id": "R1", "shortDescription": { "text": "One" }, "helpUri": help_uri },
                { "id": "R2", "shortDescription": { "text": "Two" } },
            ] } },
            "results": [
                { "ruleId": "R1", "message": { "text": "m" },
                  "locations": [{ "physicalLocation": { "artifactLocation": { "uri": "app.py" }, "region": { "startLine": 3 } } }] },
                { "ruleId": "R2", "message": { "text": "m" },
                  "locations": [{ "physicalLocation": { "artifactLocation": { "uri": "app.py" }, "region": { "startLine": 4 } } }] },
            ],
        }],
    })
    .to_string()
}

fn adapter() -> Adapter {
    serde_json::from_value(serde_json::json!({
        "id": "tool",
        "name": "Tool",
        "language": "python",
        "version": { "command": "tool", "args": ["--version"] },
        "run": { "command": "tool", "args": ["{dir}", "{output}"] },
        "install": "get it",
        "finished_exits": [0],
        "rules": {},
    }))
    .unwrap()
}

#[test]
fn a_finding_carries_its_rules_page_and_only_its_own() {
    let found = parse_sarif(&adapter(), &sarif("https://example.com/rules/R1".into())).unwrap();
    assert_eq!(found.len(), 2, "{found:#?}");
    let page = |rule: &str| {
        found
            .iter()
            .find(|f| f.rule_id == rule)
            .unwrap()
            .help_uri
            .clone()
    };
    assert_eq!(
        page("tool.R1").as_deref(),
        Some("https://example.com/rules/R1")
    );
    assert_eq!(
        page("tool.R2"),
        None,
        "a rule with no page borrowed another's"
    );
    // In report.json only when there is one.
    let json = serde_json::to_string(&found).unwrap();
    assert_eq!(json.matches("help_uri").count(), 1, "{json}");
}

#[test]
fn a_rule_whose_page_is_not_a_plain_address_has_none() {
    for bad in [
        serde_json::json!("javascript:alert(1)"),
        serde_json::json!(7),
        serde_json::json!(null),
    ] {
        let found = parse_sarif(&adapter(), &sarif(bad.clone())).unwrap();
        // The setup: the finding is there, so its missing page is about the page.
        let one = found
            .iter()
            .find(|f| f.rule_id == "tool.R1")
            .expect("R1 is read");
        assert_eq!(one.help_uri, None, "{bad}");
    }
}
