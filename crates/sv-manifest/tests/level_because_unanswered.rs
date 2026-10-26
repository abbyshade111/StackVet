//! When stackvet.toml does not say who uses the app, the level's reason names the default as a default, not as the
//! file's answer (backlog 0676, part 1).

use std::path::Path;
use sv_manifest::{Audience, Manifest};

fn parse(text: &str) -> Manifest {
    Manifest::parse(text, Path::new("stackvet.toml")).expect("the manifest parses")
}

#[test]
fn an_unanswered_audience_is_named_as_the_default() {
    for text in [
        "",
        "manifest-version = 1\n[app]\nname = \"t\"\n[data]\ncategories = []\n",
    ] {
        let m = parse(text);
        // The setup: the default is still customers, so the level is still 2; only the words change.
        assert_eq!(m.app.audience, None);
        assert_eq!(m.app.audience(), Audience::Customers);
        assert_eq!(m.target_level(), 2);
        let because = m.level_because();
        assert!(
            because.contains(
                "stackvet.toml does not say who uses the app, so sv takes it that customers do"
            ),
            "{text:?}: {because}"
        );
        assert!(
            !because.contains("says customers use it"),
            "{text:?}: {because}"
        );
    }
    // Said, it is the file's answer, as before.
    let said =
        parse("manifest-version = 1\n[app]\naudience = \"customers\"\n[data]\ncategories = []\n");
    assert_eq!(said.app.audience, Some(Audience::Customers));
    assert!(
        said.level_because()
            .contains("stackvet.toml says customers use it")
    );
}
