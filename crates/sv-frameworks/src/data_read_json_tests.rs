//! The one reader of `sv`'s JSON data files (backlog 0460): a file read and parsed, one that cannot be read named with
//! "reading", and one that does not parse given `not_understood`'s words, as every loader said them before.

use super::*;

#[derive(serde::Deserialize, Debug, PartialEq)]
struct Small {
    name: String,
}

#[test]
fn a_file_is_read_and_parsed_and_each_failure_says_which_it_was() {
    let dir = std::env::temp_dir().join(format!("sv-read-json-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    let good = dir.join("good.json");
    std::fs::write(&good, r#"{ "name": "x" }"#).unwrap();
    let bad = dir.join("bad.json");
    std::fs::write(&bad, "{ \"name\": ").unwrap();
    let missing = dir.join("missing.json");

    let read: Small = read_json(&good).unwrap();
    assert_eq!(read, Small { name: "x".into() });
    let unread = format!("{:#}", read_json::<Small>(&missing).unwrap_err());
    assert!(unread.starts_with("reading "), "{unread}");
    let not_parsed = format!("{:#}", read_json::<Small>(&bad).unwrap_err());
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        not_parsed.contains("is one of the data files `sv` ships")
            && not_parsed.contains("Nothing about the app was checked"),
        "{not_parsed}"
    );
}
