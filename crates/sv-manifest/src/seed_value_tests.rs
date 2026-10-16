//! `{seed:NAME}` in the sign-up form (backlog 0536, ADR-124): a value only the seed prints, so a manifest that cites one
//! with no `seed` is refused, by name.

use super::*;

fn users(seed: bool) -> UsersSection {
    let seed_line = if seed {
        "seed = \"python seed.py\"\n"
    } else {
        ""
    };
    toml::from_str(&format!(
        "{seed_line}private = [\"/account\"]\n\
         login = {{ path = \"/login\", form = {{ email = \"{{user}}\", password = \"{{password}}\" }} }}\n\
         signup = {{ path = \"/signup\", form = {{ email = \"{{user}}\", password = \"{{password}}\", invite_code = \"{{seed:INVITE}}\" }} }}\n"
    ))
    .expect("the section reads")
}

#[test]
fn a_seed_value_with_no_seed_is_refused_and_with_one_is_not() {
    let without = users(false).problems();
    assert!(
        without
            .iter()
            .any(|p| p.contains("`signup` cites a `{seed:NAME}` value")
                && p.contains("`seed` is not set")),
        "{without:?}"
    );
    let with = users(true).problems();
    assert!(!with.iter().any(|p| p.contains("{seed:NAME}")), "{with:?}");
}
