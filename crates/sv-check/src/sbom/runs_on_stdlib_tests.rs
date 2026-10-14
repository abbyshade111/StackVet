//! The bill of materials when the list of Python's own modules cannot be read (backlog 0445): it says so, and never
//! reads as an app that imports nothing of Python's own library.

use super::*;

use crate::test_support::scratch;

#[test]
fn the_shipped_list_is_read() {
    let modules = python_stdlib(&sv_frameworks::data::file("python-stdlib.json"))
        .expect("the python-stdlib.json sv ships");
    for module in ["json", "os", "sqlite3", "imp"] {
        assert!(modules.contains(module), "{module}");
    }
}

#[test]
fn a_missing_damaged_or_empty_list_is_named_as_not_read() {
    let dir = scratch("bad");
    for (name, text) in [
        ("missing.json", None),
        ("damaged.json", Some("{ \"modules\": [")),
        ("no-modules.json", Some("{ \"python\": \"3.13\" }")),
        ("empty.json", Some("{ \"modules\": [] }")),
    ] {
        let path = dir.join(name);
        if let Some(text) = text {
            std::fs::write(&path, text).unwrap();
        }
        let why = python_stdlib(&path).expect_err(name);
        assert!(
            why.starts_with("not read: python-stdlib.json"),
            "{name}: {why}"
        );
    }
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn the_bill_says_the_modules_were_not_read_not_that_there_are_none() {
    let dir = scratch("app");
    std::fs::write(dir.join("server.py"), "import json\nimport sqlite3\n").unwrap();
    let listing = sv_scan::files::Listing::of(&dir);
    let read = python_with(&listing, None, Ok(["json".to_owned()].into())).unwrap();
    assert!(read.modules_not_read.is_none());
    assert_eq!(
        read.modules.len(),
        1,
        "the setup: with the list read, the app's import is found"
    );

    let python = python_with(
        &listing,
        Some("python:3.12"),
        Err("not read: why".to_owned()),
    )
    .unwrap();
    std::fs::remove_dir_all(&dir).ok();
    assert!(python.modules.is_empty());
    assert_eq!(python.modules_not_read.as_deref(), Some("not read: why"));
    // The rest of what is known stays.
    assert_eq!(python.version.as_deref(), Some("3.12"));

    let runs = RunsOn {
        language: Some(python),
        ..RunsOn::default()
    };
    let doc = serde_json::to_value(to_cyclonedx_with(&Sbom::default(), &runs)).unwrap();
    let platform = doc["components"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["type"] == "platform")
        .expect("python");
    let modules = platform["properties"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == "securevibe:modules")
        .expect("the modules property");
    assert_eq!(modules["value"], "not read: why", "{platform}");
    assert!(!platform.to_string().contains("none"), "{platform}");
}
