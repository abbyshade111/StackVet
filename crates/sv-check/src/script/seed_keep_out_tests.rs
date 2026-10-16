//! Backlog 0536 (ADR-124): a value the seed printed is taken out of everything the run writes down, as a password is: a
//! page that echoes an invite code, a step that quotes the page, a header, and a log line.

use super::*;

fn code() -> String {
    ["inv", "5a9e", "c330"].join("-")
}

#[test]
fn a_value_the_seed_printed_is_taken_out_of_everything_written_down() {
    let values: std::collections::BTreeMap<String, String> = [
        ("INVITE".to_owned(), code()),
        ("SHORT".to_owned(), "ab".to_owned()),
    ]
    .into();
    let mut out = crate::signed_in::Outcome::default();
    out.steps
        .push(format!("the page said: “{} is not valid”", code()));
    out.not_assessed
        .push(("V6.2.1".into(), format!("refused with {}", code())));
    out.exchanges.push(crate::signed_in::recording::Recorded {
        id: "signup-control".into(),
        method: "POST".into(),
        path: format!("/join?code={}", code()),
        status: Some(403),
        headers: vec![("x-invite".into(), code())],
        body: format!("<p>{} was refused</p> about", code()),
    });
    out.log_tail
        .push(format!("signup refused invite={}", code()));
    // The setup: the value is there to be found.
    assert!(out.steps[0].contains(&code()));
    keep_out(&values, &mut out);
    let all = format!(
        "{:?} {:?} {:?} {:?}",
        out.steps, out.not_assessed, out.exchanges, out.log_tail
    );
    assert!(!all.contains(&code()), "a value the seed printed was kept");
    assert!(
        all.contains(SEED_VALUE_SHOWN),
        "nothing says a value was taken out"
    );
    // A value under four characters is not looked for: `ab` in "about" is left alone.
    assert!(out.exchanges[0].body.ends_with("about"));
}
