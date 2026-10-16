//! Backlog 0232, part D, the owner's choice of 16 October 2026: `upload` is a list, written as one
//! `[stack.run.users.upload]` table as before or as several `[[stack.run.users.upload]]` entries.

use super::*;

const HEAD: &str = "manifest-version = 1\n[app]\nname = \"R\"\n[stack]\nlanguages = [\"python\"]\n[stack.run]\nstart = \"python app.py\"\n[stack.run.users]\n";

fn uploads(text: &str) -> Vec<UploadSection> {
    Manifest::parse(
        &format!("{HEAD}{text}"),
        std::path::Path::new("stackvet.toml"),
    )
    .unwrap_or_else(|e| panic!("{e:#}"))
    .stack
    .run
    .users
    .expect("the setup: a users section")
    .upload
}

#[test]
fn one_table_is_a_list_of_one_as_before() {
    let got =
        uploads("[stack.run.users.upload]\npath = \"/up\"\nfield = \"file\"\nmax-bytes = 10\n");
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].path, "/up");
    assert_eq!(got[0].max_bytes, Some(10));
}

#[test]
fn several_entries_are_each_kept_in_order() {
    let got = uploads(
        "[[stack.run.users.upload]]\npath = \"/avatar\"\nfield = \"photo\"\n\n\
         [[stack.run.users.upload]]\npath = \"/documents\"\nfield = \"file\"\nserves-at = \"/d/{name}\"\n",
    );
    let paths: Vec<&str> = got.iter().map(|u| u.path.as_str()).collect();
    assert_eq!(paths, ["/avatar", "/documents"]);
    assert_eq!(got[1].serves_at.as_deref(), Some("/d/{name}"));
}

#[test]
fn none_is_an_empty_list() {
    assert!(uploads("").is_empty());
}

#[test]
fn the_field_list_is_the_structs_own() {
    // serde names every field it takes when it meets one it does not.
    let err = toml::from_str::<UploadSection>("path = \"/up\"\nfield = \"f\"\nnot-a-field = 1\n")
        .expect_err("an unknown field is refused")
        .to_string();
    let listed = err
        .split("expected one of ")
        .nth(1)
        .unwrap_or_else(|| panic!("{err}"));
    let mut said: Vec<&str> = listed
        .split(',')
        .map(|f| f.trim().trim_matches('`'))
        .map(|f| f.split('`').next().unwrap_or(f).trim())
        .filter(|f| !f.is_empty())
        .collect();
    said.sort_unstable();
    let mut ours: Vec<&str> = UPLOAD_FIELDS.to_vec();
    ours.sort_unstable();
    assert_eq!(said, ours, "{err}");
}

#[test]
fn a_list_written_inline_is_read_too() {
    let got = uploads(
        "upload = [{ path = \"/avatar\", field = \"photo\" }, { path = \"/documents\", field = \"file\" }]\n",
    );
    let paths: Vec<&str> = got.iter().map(|u| u.path.as_str()).collect();
    assert_eq!(paths, ["/avatar", "/documents"]);
}
