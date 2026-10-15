//! A design answer given as the owner's and never recorded through `sv review` says the AI coding tool can write
//! `by = "owner"` too (backlog 0593): in the Cursor run of 15 October 2026 the tool wrote four such answers itself.

use super::*;

#[test]
fn an_unrecorded_owners_answer_says_the_tool_can_write_it_too() {
    let questions = Questions {
        questions: vec![Question {
            id: "V8.3.1".into(),
            title: "Who may do what is enforced on the server".into(),
            asks: "Are the authorization rules enforced on the server?".into(),
            plain: None,
            where_means: "the file where authorization is enforced".into(),
            how_to_find_out: None,
            why_not_checked: None,
        }],
    };
    let answers = BTreeMap::from([(
        "V8.3.1".to_owned(),
        Answer {
            answer: YES.into(),
            location: Some("auth.py".into()),
            by: Some(OWNER.into()),
            recorded: Err("it was not recorded through `sv review`".into()),
        },
    )]);
    let out = evaluate(&questions, &answers, &|_| true, &|_| true, true);
    assert_eq!(
        out.stated.len(),
        1,
        "the setup: counted as the tool's word: {out:?}"
    );
    let said = &out.stated[0].scope;
    assert!(
        said.contains("Your AI coding tool can write `by = \"owner\"` as well as you can"),
        "{said}"
    );
    assert!(
        said.contains("not yours and not a check of the code"),
        "{said}"
    );
    assert!(said.contains("run `sv review`"), "{said}");

    // Recorded through `sv review`, it is the owner's, and nothing of the kind is said.
    let mut recorded = answers;
    recorded.get_mut("V8.3.1").unwrap().recorded = Ok(crate::seal::Sealed::Here);
    let out = evaluate(&questions, &recorded, &|_| true, &|_| true, true);
    assert_eq!(out.attested.len(), 1, "{out:?}");
    assert!(
        !out.attested[0].scope.contains("can write"),
        "{}",
        out.attested[0].scope
    );
}
