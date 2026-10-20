//! Backlog 0489, the owner's choice of 20 October 2026: an `owned` record may say the range a field of its `create`
//! must stay in (`limits`), which the signed-in run holds the app to (V2.2.1).

use super::*;

const HEAD: &str = "manifest-version = 1\n[app]\nname = \"R\"\n[stack]\nlanguages = [\"python\"]\n[stack.run]\nstart = \"python app.py\"\n[stack.run.users]\n";

fn owned(text: &str) -> Result<OwnedSection, String> {
    Manifest::parse(
        &format!("{HEAD}{text}"),
        std::path::Path::new("stackvet.toml"),
    )
    .map_err(|e| format!("{e:#}"))
    .map(|m| {
        m.stack
            .run
            .users
            .expect("the setup: a users section")
            .owned
            .expect("an owned record")
    })
}

const CREATE: &str =
    "owned = { create = { path = \"/orders\", form = { item = \"{marker}\", quantity = \"2\" } }";

#[test]
fn limits_are_read_and_none_is_an_empty_list() {
    let got = owned(&format!(
        "{CREATE}, limits = [{{ field = \"quantity\", min = 1, max = 10 }}] }}\n"
    ))
    .unwrap();
    assert_eq!(
        got.limits,
        [FieldLimit {
            field: "quantity".into(),
            min: 1,
            max: 10
        }]
    );
    assert!(owned(&format!("{CREATE} }}\n")).unwrap().limits.is_empty());
    // A negative least value is a range like any other.
    let got = owned(&format!(
        "{CREATE}, limits = [{{ field = \"quantity\", min = -5, max = 5 }}] }}\n"
    ))
    .unwrap();
    assert_eq!(got.limits[0].min, -5);
}

#[test]
fn a_limit_with_a_key_it_does_not_know_or_without_its_range_is_refused() {
    let unknown = owned(&format!(
        "{CREATE}, limits = [{{ field = \"quantity\", min = 1, max = 10, most = 9 }}] }}\n"
    ));
    assert!(unknown.is_err(), "{unknown:?}");
    let without = owned(&format!(
        "{CREATE}, limits = [{{ field = \"quantity\", min = 1 }}] }}\n"
    ));
    assert!(without.is_err(), "{without:?}");
}
