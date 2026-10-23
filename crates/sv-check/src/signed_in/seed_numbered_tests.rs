//! Backlog 0637, ADR-134: a seed may print numbered values, and each sign-up takes the next one, so an app whose invite
//! codes work once lets every account the run makes in; and the report says why an account was not made in words that
//! fit what the seed printed.

use super::*;

fn values(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn invite_form() -> RequestTemplate {
    RequestTemplate {
        method: "POST".into(),
        path: "/signup".into(),
        form: values(&[("email", "{user}"), ("invite", "{seed:INVITE}")]),
        json: BTreeMap::new(),
    }
}

#[test]
fn each_sign_up_takes_the_next_numbered_value_and_none_past_the_last() {
    let _seed = SeedValues::set(&values(&[
        ("INVITE_1", "first-code"),
        ("INVITE_2", "second-code"),
    ]));
    next_sign_up();
    assert_eq!(fill_seed("{seed:INVITE}"), "first-code");
    next_sign_up();
    assert_eq!(fill_seed("{seed:INVITE}"), "second-code");
    next_sign_up();
    // Past the values printed: left as written, so the app refuses it and the refusal is quoted.
    assert_eq!(fill_seed("{seed:INVITE}"), "{seed:INVITE}");
}

#[test]
fn without_numbered_values_every_sign_up_takes_the_one_value() {
    let _seed = SeedValues::set(&values(&[("INVITE", "the-code")]));
    for _ in 0..3 {
        next_sign_up();
        assert_eq!(fill_seed("invite={seed:INVITE}"), "invite=the-code");
    }
    // Before any sign-up, as a check reading the form fills it: the first value.
    let _again = SeedValues::set(&values(&[("INVITE_1", "first-code")]));
    assert_eq!(fill_seed("{seed:INVITE}"), "first-code");
}

#[test]
fn the_report_says_a_value_may_work_once_and_how_to_print_one_per_account() {
    let _seed = SeedValues::set(&values(&[("INVITE", "the-code")]));
    // The first sign-up: the value was unused, so working once is not why it failed, and the report does not say so.
    next_sign_up();
    let said = sign_up_value_note(&invite_form());
    assert!(said.contains("this was the run's first sign-up"), "{said}");
    assert!(!said.contains("works once"), "{said}");
    next_sign_up();
    let said = sign_up_value_note(&invite_form());
    assert!(
        said.contains("cites `{seed:INVITE}`, one value the seed printed"),
        "{said}"
    );
    assert!(said.contains("`INVITE_1=…`, `INVITE_2=…`"), "{said}");
    assert!(
        !said.contains("the-code"),
        "the value is never said: {said}"
    );
    let _more = SeedValues::set(&values(&[("INVITE_1", "a"), ("INVITE_2", "b")]));
    // Within the values printed: this sign-up had one, and the report does not say it ran short.
    for _ in 0..2 {
        next_sign_up();
    }
    let said = sign_up_value_note(&invite_form());
    assert!(said.contains("This sign-up took `INVITE_2`"), "{said}");
    assert!(!said.contains("went without one"), "{said}");
    next_sign_up();
    let said = sign_up_value_note(&invite_form());
    assert!(
        said.contains("printed 2 numbered `INVITE_` values and this run made 3 sign-ups"),
        "{said}"
    );
    // A value the seed did not print is not said to be one it printed.
    let _none = SeedValues::set(&values(&[("OTHER", "x")]));
    next_sign_up();
    let said = sign_up_value_note(&invite_form());
    assert!(said.contains("which the seed did not print"), "{said}");
    // A form that cites no seed value keeps the words it had.
    let plain = RequestTemplate {
        form: values(&[("email", "{user}")]),
        ..invite_form()
    };
    assert!(sign_up_value_note(&plain).contains("may need a value stackvet.toml does not carry"));
}

/// Every body the run sent, as text, and an answer that refuses nothing.
#[derive(Default)]
struct Sent(Vec<String>);

impl Http for Sent {
    fn send(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        self.0.push(
            String::from_utf8_lossy(request.body.as_deref().unwrap_or_default()).into_owned(),
        );
        Some(ProbeResponse {
            id: request.id.clone(),
            status: 303,
            headers: Vec::new(),
            body: String::new(),
        })
    }
}

#[test]
fn the_sign_ups_a_run_sends_each_carry_their_own_value() {
    // The owner's build: an invite that works once, and three accounts made through the form.
    let _seed = SeedValues::set(&values(&[
        ("INVITE_1", "first-code"),
        ("INVITE_2", "second-code"),
        ("INVITE_3", "third-code"),
    ]));
    let mut http = Sent::default();
    for who in ["a", "b", "c"] {
        let account = Account {
            user: format!("{who}@example.test"),
            password: "Pw-for-the-test-1!".into(),
        };
        super::passwords::sign_up_only(&mut http, &invite_form(), who, &account);
    }
    let invites: Vec<&str> = ["first-code", "second-code", "third-code"]
        .into_iter()
        .filter(|code| http.0.iter().any(|b| b.contains(code)))
        .collect();
    assert_eq!(
        invites.len(),
        3,
        "each sign-up sent a value of its own: {:?}",
        http.0
    );
    assert!(http.0.iter().all(|b| !b.contains("{seed:")), "{:?}", http.0);
}
