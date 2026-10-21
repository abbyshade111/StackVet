//! `[app] level` (backlog 0507, ADR-130): level 3 as the owner's opt-in. It only ever raises the level `audience` and
//! `[data] categories` set, under ADR-015 (the owner's word adds requirements and never removes one): a lower level is
//! refused when the file is read, naming the answers that set the higher one, and a level that is not 1, 2, or 3 too.

use std::path::Path;
use sv_frameworks::Condition;
use sv_manifest::Manifest;

fn parse(audience: &str, data: &str, level: Option<&str>) -> anyhow::Result<Manifest> {
    let level = level.map(|l| format!("level = {l}\n")).unwrap_or_default();
    let text = format!(
        "manifest-version = 1\n[app]\nname = \"t\"\naudience = \"{audience}\"\n{level}{data}"
    );
    Manifest::parse(&text, Path::new("stackvet.toml"))
}

const NOTHING: &str = "[data]\ncategories = []\n";

fn level2(m: &Manifest) -> Option<bool> {
    m.claims()
        .into_iter()
        .find(|(c, _)| *c == Condition::Level2)
        .and_then(|(_, v)| v)
}

#[test]
fn left_out_the_level_is_what_the_two_answers_set() {
    // The setup: the two answers set 1 and 2, so a raised level below is the key's doing.
    assert_eq!(parse("just-me", NOTHING, None).unwrap().target_level(), 1);
    assert_eq!(parse("customers", NOTHING, None).unwrap().target_level(), 2);
}

#[test]
fn three_raises_the_level_and_says_so() {
    for audience in ["just-me", "customers"] {
        let m = parse(audience, NOTHING, Some("3")).unwrap();
        assert_eq!(m.target_level(), 3, "{audience}");
        assert!(
            m.level_because().contains("asks for level 3"),
            "{}",
            m.level_because()
        );
        // A level-3 app is held to every level-2 requirement too.
        assert_eq!(level2(&m), Some(true), "{audience}");
        assert!(m.level_from_unanswered_data().is_none());
    }
    // Two on a level-1 app raises it as well; on a level-2 app it changes nothing, and says nothing.
    assert_eq!(
        parse("just-me", NOTHING, Some("2")).unwrap().target_level(),
        2
    );
    let same = parse("customers", NOTHING, Some("2")).unwrap();
    assert_eq!(same.target_level(), 2);
    assert!(
        !same.level_because().contains("asks for"),
        "{}",
        same.level_because()
    );
}

#[test]
fn a_level_below_the_answers_is_refused_with_the_answers_named() {
    let refused = parse("customers", NOTHING, Some("1"))
        .expect_err("level 1 on a level-2 app is refused")
        .to_string();
    assert!(
        refused.contains("level 1") && refused.contains("level 2"),
        "{refused}"
    );
    assert!(refused.contains("customers use it"), "{refused}");
    assert!(refused.contains("only ever raises"), "{refused}");
    // The same on an unanswered data list, which holds the app to level 2.
    assert!(parse("just-me", "", Some("1")).is_err());
}

#[test]
fn a_level_that_is_not_one_two_or_three_is_refused() {
    for level in ["0", "4"] {
        let refused = parse("just-me", NOTHING, Some(level))
            .err()
            .unwrap_or_else(|| panic!("level {level} is refused"))
            .to_string();
        assert!(refused.contains("levels 1, 2, and 3"), "{refused}");
    }
    assert!(parse("just-me", NOTHING, Some("\"three\"")).is_err());
}
