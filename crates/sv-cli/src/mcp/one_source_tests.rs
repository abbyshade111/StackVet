//! Backlog 0664 (ADR-017, Later, 24 October 2026): the coding rules (`data/coding-rules.json`) and the
//! framework lines (`data/stack-guidance.json`) are written once and rendered, never written again, for
//! every place they go. Wiz's rules files show what happens otherwise: generated again for each tool,
//! their copies for one stack disagree. These tests hold every place `sv` gives the rules to the same
//! rules, so a renderer that drops or adds one fails.

use super::tests::{call, scratch_app, text};
use super::*;

#[test]
fn every_place_the_rules_go_holds_the_same_rules_and_lines() {
    let root = scratch_app("one-source", "flask-booking");
    let server = Server::new(&root).unwrap();
    let guidance = call(&server, "stackvet_guidance", json!({ "path": "app" }));
    let found = crate::coding_rules_for(&root.join("app")).unwrap();
    // What `sv rules` writes into AGENTS.md, and prints with `--print`.
    let agents = found.agents_markdown();
    std::fs::remove_dir_all(&root).ok();
    assert_eq!(guidance["isError"], false, "{guidance}");
    let words = text(&guidance);

    // The setup: rules were given, some were left out, and Flask was found, so each check below
    // is about something.
    assert!(
        found.filtered,
        "the example has a stackvet.toml to filter by"
    );
    assert!(
        found.given.len() >= 10,
        "only {} rules given",
        found.given.len()
    );
    assert!(
        found.withheld > 0,
        "no rule was left out, so leaving out is not tested"
    );
    assert_eq!(found.frameworks, ["flask"]);

    let structured: Vec<&str> = guidance["structuredContent"]["rules"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap())
        .collect();
    assert_eq!(structured, found.given, "the structured answer's rules");

    for rule in &found.rules.rules {
        let given = found.given.contains(&rule.id);
        for (place, body) in [("the guidance text", words), ("AGENTS.md", agents.as_str())] {
            assert_eq!(
                body.contains(&rule.rule),
                given,
                "{place} {} {}",
                if given {
                    "lacks"
                } else {
                    "has, though left out,"
                },
                rule.id
            );
        }
    }

    let ast = sv_check::ast::AstRules::load(&crate::ast_rules_path()).unwrap();
    let lines =
        sv_check::stack_guidance::StackGuidance::load(&crate::stack_guidance_path(), &ast).unwrap();
    for framework in &lines.frameworks {
        let flask = framework.id == "flask";
        for line in &framework.lines {
            for (place, body) in [("the guidance text", words), ("AGENTS.md", agents.as_str())] {
                assert_eq!(
                    body.contains(&line.line),
                    flask,
                    "{place}, {}'s line: {}",
                    framework.id,
                    line.line
                );
            }
        }
    }
}

#[test]
fn each_feature_brief_gives_every_rule_on_its_topics_and_no_other() {
    let features: Value =
        serde_json::from_str(&std::fs::read_to_string(crate::feature_briefs_path()).unwrap())
            .unwrap();
    let rules = sv_check::coding_rules::CodingRules::load(&crate::coding_rules_path()).unwrap();
    let root = scratch_app("one-source-briefs", "flask-booking");
    let server = Server::new(&root).unwrap();
    let mut briefs = 0;
    for feature in features["features"].as_array().unwrap() {
        let id = feature["id"].as_str().unwrap();
        let topics: Vec<&str> = feature["guidance"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t.as_str().unwrap())
            .collect();
        let brief = call(
            &server,
            "stackvet_before",
            json!({ "path": "app", "feature": id }),
        );
        assert_eq!(brief["isError"], false, "{id}: {brief}");
        let given: Vec<&str> = brief["structuredContent"]["rules"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["id"].as_str().unwrap())
            .collect();
        let expected: Vec<&str> = rules
            .rules
            .iter()
            .filter(|r| topics.contains(&r.topic.as_str()))
            .map(|r| r.id.as_str())
            .collect();
        assert_eq!(given, expected, "{id}'s brief");
        for r in rules
            .rules
            .iter()
            .filter(|r| expected.contains(&r.id.as_str()))
        {
            assert!(
                text(&brief).contains(&r.rule),
                "{id}'s brief words lack {}",
                r.id
            );
        }
        briefs += 1;
    }
    std::fs::remove_dir_all(&root).ok();
    assert!(briefs >= 10, "only {briefs} briefs were read");
}
