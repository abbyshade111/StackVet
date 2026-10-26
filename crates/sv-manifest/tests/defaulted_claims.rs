//! The answers `claims` gives that no line of stackvet.toml gave are named, so the report says they are `sv`'s
//! defaults and not the file's word (backlog 0677, part 1).

use std::path::Path;
use sv_frameworks::Condition;
use sv_manifest::Manifest;

fn parse(text: &str) -> Manifest {
    Manifest::parse(text, Path::new("stackvet.toml")).expect("the manifest parses")
}

#[test]
fn an_answer_left_out_is_a_default_and_one_given_is_not() {
    let silent = parse("");
    // The setup: the defaults still switch each on, so the requirements that turn on them still apply.
    let claims = silent.claims();
    for c in [Condition::Tls, Condition::Internet, Condition::Level2] {
        assert!(claims.contains(&(c, Some(true))), "{}", c.name());
    }
    assert_eq!(
        silent.defaulted_claims(),
        [Condition::Tls, Condition::Internet, Condition::Level2]
    );
    let said = parse(
        "manifest-version = 1\n[app]\naudience = \"customers\"\ndeployment = \"internet\"\n\
         [capabilities]\ntls = \"terminated-upstream\"\n[data]\ncategories = []\n",
    );
    assert_eq!(said.defaulted_claims(), []);
    // Level 2 from the data list left out is a default; from health information listed, an answer.
    let unlisted = parse("manifest-version = 1\n[app]\naudience = \"just-me\"\n");
    assert!(unlisted.defaulted_claims().contains(&Condition::Level2));
    let health = parse(
        "manifest-version = 1\n[app]\naudience = \"just-me\"\n[data]\ncategories = [\"health\"]\n",
    );
    assert!(!health.defaulted_claims().contains(&Condition::Level2));
    // Level 1 is not level 2 at all, defaulted or not.
    let one =
        parse("manifest-version = 1\n[app]\naudience = \"just-me\"\n[data]\ncategories = []\n");
    assert!(!one.defaulted_claims().contains(&Condition::Level2));
}
