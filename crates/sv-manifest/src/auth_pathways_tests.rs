//! `[stack.auth] pathways`, the owner's list of the ways to sign in (backlog 0491; ADR-123): read as written, left out
//! when absent, and asked commented out in the starter file.

use super::*;

#[test]
fn the_list_is_read_as_written_and_absent_when_left_out() {
    let with = Manifest::parse(
        "manifest-version = 1\n[app]\nname = \"R\"\n[stack]\nlanguages = [\"python\"]\n\
         [stack.auth]\npathways = [\"/login\", \"x-api-key\"]\n",
        Path::new("stackvet.toml"),
    )
    .expect("manifest parses");
    assert_eq!(
        with.stack.auth.and_then(|a| a.pathways),
        Some(vec!["/login".to_owned(), "x-api-key".to_owned()])
    );
    let without = Manifest::parse(
        "manifest-version = 1\n[app]\nname = \"R\"\n[stack]\nlanguages = [\"python\"]\n",
        Path::new("stackvet.toml"),
    )
    .expect("manifest parses");
    assert_eq!(without.stack.auth, None);
}

#[test]
fn the_starter_file_asks_it_commented_out_and_it_parses_once_answered() {
    let spec = crate::spec::STARTER_MANIFEST;
    let line = "# pathways = [\"/login\", \"/auth/google\", \"x-api-key\"]";
    assert!(spec.contains("# [stack.auth]"), "the table is not asked");
    assert!(spec.contains(line), "the list is not asked");
    let answered = spec
        .replacen("# [stack.auth]", "[stack.auth]", 1)
        .replacen(line, &line[2..], 1);
    let parsed: Manifest = toml::from_str(&answered).expect("the answered starter parses");
    assert_eq!(
        parsed.stack.auth.and_then(|a| a.pathways).map(|p| p.len()),
        Some(3)
    );
}
