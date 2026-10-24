//! Backlog 0659: the preflight names a path under /tmp in the app's own code, and says how to move it to
//! `[stack.run] env`; test code and an app with none are left alone.

use super::*;

fn source(files: &[(&str, &str)]) -> Source {
    Source {
        files: files
            .iter()
            .map(|(f, t)| ((*f).to_owned(), (*t).to_owned()))
            .collect(),
        unread: Vec::new(),
    }
}

fn manifest(env: &str) -> Manifest {
    let text = format!(
        "manifest-version = 1\n\n[app]\nname = \"x\"\n\n[stack]\nlanguages = [\"python\"]\n\n[stack.run]\nimage = \"python:3.12-slim\"\nstart = \"python app.py\"\n{env}"
    );
    Manifest::parse(&text, std::path::Path::new("stackvet.toml"))
        .expect("the manifest written here")
}

fn words(item: &Item) -> String {
    format!("{item:?}")
}

#[test]
fn a_default_under_tmp_is_named_with_how_to_move_it() {
    let found = item(
        &manifest(""),
        &source(&[(
            "app.py",
            "import os\nDB = os.environ.get(\"DATABASE_PATH\", \"/tmp/notes.db\")\n",
        )]),
    )
    .expect("an item");
    let said = words(&found);
    assert!(said.contains("app.py") && said.contains("line 2"), "{said}");
    assert!(
        said.contains("[stack.run]") && said.contains("env = {"),
        "{said}"
    );
    assert!(said.contains("Look"), "{said}");
}

#[test]
fn test_code_and_an_app_with_no_tmp_path_get_nothing() {
    assert!(
        item(
            &manifest(""),
            &source(&[("app.py", "DB = 'data/notes.db'\n")])
        )
        .is_none()
    );
    assert!(
        item(
            &manifest(""),
            &source(&[("tests/test_app.py", "p = '/tmp/x.db'\n")])
        )
        .is_none()
    );
}

#[test]
fn with_env_set_it_says_to_read_the_folder_from_it() {
    let found = item(
        &manifest("env = { DATABASE_PATH = \"/tmp/app.db\" }\n"),
        &source(&[("app.py", "DB = '/tmp/notes.db'\n")]),
    )
    .expect("an item");
    assert!(
        words(&found).contains("already sets variables"),
        "{}",
        words(&found)
    );
}
