//! One private page listed is said, with why (backlog 0232, part 8): the checks that ask each listed private page can
//! then credit only in part. Two or more, or none, say nothing; admin pages are not counted.

use super::*;

fn items(private: &str, admin: &str) -> Vec<Item> {
    let text = format!(
        "manifest-version = 1\n[app]\nname = \"t\"\n[stack.run]\nimage = \"python:3.12-slim\"\n\
         start = \"python app.py\"\nhealth = \"/health\"\n[stack.run.users]\nseed = \"python seed.py\"\n\
         login = {{ path = \"/login\", form = {{ email = \"{{user}}\", password = \"{{password}}\" }} }}\n\
         private = {private}\nadmin = {admin}\n"
    );
    let manifest =
        Manifest::parse(&text, Path::new("stackvet.toml")).expect("the test's manifest parses");
    let source = Source {
        files: vec![(
            "app.py".to_owned(),
            "routes = {'/health': h, '/login': login, '/account': a, '/notes': n, '/admin': ad}"
                .to_owned(),
        )],
        unread: Vec::new(),
    };
    preflight(&manifest, &source)
}

fn pages(items: &[Item]) -> Vec<&Item> {
    items.iter().filter(|i| i.topic == "pages").collect()
}

#[test]
fn one_private_page_listed_is_said_with_why() {
    let found = items("[\"/account\"]", "[\"/admin\"]");
    let said = pages(&found);
    assert_eq!(said.len(), 1, "{found:#?}");
    assert_eq!(said[0].answer, Answer::Look);
    let text = said[0].text(&Fence::none());
    assert!(text.contains("Only one private page is listed"), "{text}");
    assert!(text.contains("credit only in part"), "{text}");
    assert!(text.contains("/account"), "{text}");
}

#[test]
fn two_private_pages_or_one_with_a_placeholder_beside_it_say_what_they_should() {
    assert!(pages(&items("[\"/account\", \"/notes\"]", "[\"/admin\"]")).is_empty());
    // A page with a placeholder is not asked by those checks, so it does not count as a second page.
    assert_eq!(
        pages(&items("[\"/account\", \"/notes/{id}\"]", "[]")).len(),
        1
    );
}
