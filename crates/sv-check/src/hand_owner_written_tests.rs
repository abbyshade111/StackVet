//! A check by hand given as the owner's and never recorded through `sv review` says the AI coding tool can write
//! `by = "owner"`, and the `how` beside it, too (backlog 0593): in the Cursor run of 15 October 2026 the tool wrote three
//! "done" checks with a `how` it composed, given as the owner's.

use super::*;
use crate::human::HumanCheck;

#[test]
fn an_unrecorded_owners_check_says_the_tool_can_write_it_and_its_how() {
    let checks = HumanChecks {
        checks: vec![HumanCheck {
            id: "V12.2.2".into(),
            title: "The certificate on the live site is one browsers trust".into(),
            how: "Open the live site and click the padlock.".into(),
        }],
    };
    let answer = Answer {
        result: DONE.into(),
        on: Some("2026-09-20".into()),
        by: Some("owner".into()),
        how: Some("Padlock shows a trusted certificate.".into()),
        recorded: Err("it was not recorded through `sv review`".into()),
    };
    let out = evaluate(
        &checks,
        &BTreeMap::from([("V12.2.2".to_owned(), answer)]),
        &|_| true,
        Day::parse("2026-09-26").unwrap(),
    );
    assert_eq!(
        out.stated.len(),
        1,
        "the setup: counted as the tool's word: {out:?}"
    );
    let said = &out.stated[0].scope;
    assert!(
        said.contains("Your AI coding tool can write `by = \"owner\"`, and the `how` beside it, as well as you can"),
        "{said}"
    );
    assert!(said.contains("run `sv review`"), "{said}");
}
