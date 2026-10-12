//! The phone, two-factor, and recovery changes under `[stack.run.users]`, `not-offered`, and
//! `[account-changes-review]` (ADR-105).

use super::*;

const HEAD: &str = "manifest-version = 1\n[app]\nname = \"R\"\n[stack]\nlanguages = [\"python\"]\n\
                    [stack.run]\nstart = \"python app.py\"\n[stack.run.users]\n";

fn parse(users: &str) -> Result<Manifest> {
    Manifest::parse(&format!("{HEAD}{users}"), Path::new("stackvet.toml"))
}

fn users(m: &Manifest) -> &UsersSection {
    m.stack.run.users.as_ref().expect("a users section")
}

#[test]
fn a_change_with_its_page_reads_and_each_kind_is_found_by_its_name() {
    let m = parse(
        "change-phone = { path = \"/account/phone\", form = { password = \"{password}\", \
         phone = \"{new_value}\" }, shows = \"/account\" }\nnot-offered = [\"recovery\"]\n",
    )
    .unwrap();
    let u = users(&m);
    let phone = u.account_change("phone").expect("read");
    assert_eq!(phone.shows, "/account");
    assert_eq!(phone.request().path, "/account/phone");
    assert_eq!(phone.request().method, "POST");
    assert!(u.account_change("two-factor").is_none());
    assert_eq!(u.not_offered, ["recovery"]);
    let problems: Vec<String> = u
        .problems()
        .into_iter()
        .filter(|p| p.contains("phone") || p.contains("not-offered"))
        .collect();
    assert!(problems.is_empty(), "{problems:?}");
}

#[test]
fn a_change_without_a_page_to_show_it_does_not_read() {
    let err =
        parse("change-phone = { path = \"/account/phone\", form = { phone = \"{new_value}\" } }\n")
            .unwrap_err();
    assert!(format!("{err:#}").contains("shows"), "{err:#}");
}

#[test]
fn each_mistake_in_a_change_or_the_list_is_named() {
    let named = |users: &str, words: &str| {
        let m = parse(users).unwrap();
        let problems = users_problems(&m);
        assert!(
            problems.iter().any(|p| p.contains(words)),
            "{words}: {problems:?}"
        );
    };
    named(
        "change-phone = { path = \"/p\", form = { phone = \"{user}\" }, shows = \"/account\" }\n",
        "`change-phone` (/p) has no field with `{new_value}`",
    );
    named(
        "change-recovery = { path = \"/r\", form = { email = \"{new_value}\" }, shows = \"account\" }\n",
        "`change-recovery` has `shows = \"account\"`, which is not a path on the app",
    );
    named(
        "change-two-factor = { path = \"/t\", form = { n = \"{new_value}\" }, shows = \"/a\" }\n\
         not-offered = [\"two-factor\"]\n",
        "`not-offered` lists `two-factor`, and `change-two-factor` says how it is changed",
    );
    named(
        "not-offered = [\"email\"]\n",
        "`not-offered` lists `email`, which is not one of `phone`, `two-factor`, or `recovery`",
    );
}

fn users_problems(m: &Manifest) -> Vec<String> {
    users(m).problems()
}

#[test]
fn a_confirmed_list_holds_in_any_order_and_not_once_it_changes() {
    let m = parse("not-offered = [\"recovery\", \"phone\"]\n").unwrap();
    let entry = AccountChangesReview {
        not_offered: vec!["phone".into(), "recovery".into()],
        by: "owner".into(),
        on: "2026-10-12".into(),
        seal: None,
    };
    assert!(entry.still_holds_for(&m));
    let changed = parse("not-offered = [\"phone\"]\n").unwrap();
    assert!(!entry.still_holds_for(&changed));
    let none = parse("").unwrap();
    assert!(!entry.still_holds_for(&none));
}

#[test]
fn the_review_reads_back_from_the_file() {
    let m = Manifest::parse(
        &format!(
            "{HEAD}not-offered = [\"phone\"]\n[account-changes-review]\nnot-offered = [\"phone\"]\n\
             by = \"owner\"\non = \"2026-10-12\"\nseal = \"x\"\n"
        ),
        Path::new("stackvet.toml"),
    )
    .unwrap();
    let entry = m.account_changes_review.as_ref().expect("read");
    assert_eq!(entry.not_offered, ["phone"]);
    assert!(entry.still_holds_for(&m));
}
