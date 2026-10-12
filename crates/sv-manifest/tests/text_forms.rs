//! `browser.text-forms` names several text forms, each with its own `shows`; `text-form` still names one; giving both
//! is said as a problem (ADR-097, backlog 0232 part 6).

use std::path::Path;
use sv_manifest::Manifest;

fn users(browser: &str) -> sv_manifest::UsersSection {
    let text = format!(
        "manifest-version = 1\n[app]\nname = \"t\"\n[stack]\nlanguages = []\n[stack.run]\nimage = \"python:3.12-slim\"\n\
         start = \"python app.py\"\nhealth = \"/\"\n[stack.run.users]\nseed = \"python seed.py\"\n\
         login = {{ path = \"/login\", form = {{ email = \"{{user}}\", password = \"{{password}}\" }} }}\n\
         private = [\"/account\"]\nbrowser = {browser}\n"
    );
    let manifest = Manifest::parse(&text, Path::new("stackvet.toml")).expect("parses");
    manifest.stack.run.users.expect("users")
}

#[test]
fn several_text_forms_are_read_each_with_its_own_page() {
    let u = users(
        r#"{ text-forms = [{ form = "/notes/new", shows = "/notes" }, { form = "/profile" }] }"#,
    );
    let forms = u.browser.as_ref().unwrap().forms();
    assert_eq!(forms.len(), 2);
    assert_eq!(forms[0].form, "/notes/new");
    assert_eq!(forms[0].shows.as_deref(), Some("/notes"));
    assert_eq!(forms[1].shows, None);
    assert!(
        u.problems().iter().all(|p| !p.contains("text-forms")),
        "{:?}",
        u.problems()
    );
}

#[test]
fn the_one_text_form_still_reads_as_one() {
    let u = users(r#"{ text-form = "/notes/new", shows = "/notes" }"#);
    let forms = u.browser.as_ref().unwrap().forms();
    assert_eq!(forms.len(), 1);
    assert_eq!(forms[0].shows.as_deref(), Some("/notes"));
}

#[test]
fn both_the_one_and_the_list_is_said_as_a_problem() {
    let u = users(r#"{ text-form = "/a", text-forms = [{ form = "/b" }] }"#);
    assert!(
        u.problems()
            .iter()
            .any(|p| p.contains("both `text-form` and `text-forms`")),
        "{:?}",
        u.problems()
    );
}
