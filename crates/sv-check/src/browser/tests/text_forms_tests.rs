//! The typed-text check on every text form stackvet.toml names (ADR-097, backlog 0232 part 6): whole only when two or
//! more were named and every one was shown as text. Each form has its own token, none the start of another's.

use super::*;

fn with_forms(forms: &[(&str, Option<&str>)]) -> UsersSection {
    let mut u = users(None, None);
    u.browser = Some(BrowserSection {
        text_forms: forms
            .iter()
            .map(|(form, shows)| sv_manifest::TextForm {
                form: (*form).to_owned(),
                shows: shows.map(str::to_owned),
            })
            .collect(),
        ..Default::default()
    });
    u
}

fn credit(o: &Outcome) -> Option<&crate::Verified> {
    o.verified
        .iter()
        .find(|v| v.check_id == TEXT_AS_MARKUP.rule_id)
}

#[test]
fn every_named_form_shown_as_text_is_whole_and_named() {
    let (o, jobs) = run_on(
        App::default(),
        &with_forms(&[("/notes/new", Some("/notes")), ("/profile", None)]),
    );
    // The setup: both forms were typed into, each with its own token.
    let filled: Vec<&str> = jobs[0]
        .actions
        .iter()
        .filter_map(|a| match a {
            Action::Fill { page, text } => Some((page.as_str(), text.as_str())),
            _ => None,
        })
        .map(|(page, text)| {
            assert!(text.contains(if page == "/profile" {
                "t0kf1z"
            } else {
                "t0kf0z"
            }));
            page
        })
        .collect();
    assert_eq!(filled, ["/notes/new", "/profile"]);
    let credit = credit(&o).expect("credited");
    assert!(!credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("/notes/new, shown on /notes"),
        "{}",
        credit.scope
    );
    assert!(credit.scope.contains("/profile"), "{}", credit.scope);
}

#[test]
fn one_named_form_whose_text_runs_is_found_and_nothing_credited() {
    let app = App {
        runs_for: Some("t0kf1z"),
        ..App::default()
    };
    let (o, _) = run_on(
        app,
        &with_forms(&[("/notes/new", Some("/notes")), ("/profile", None)]),
    );
    assert!(
        o.findings
            .iter()
            .any(|f| f.rule_id == TEXT_AS_MARKUP.rule_id && f.description.contains("/profile")),
        "{:?}",
        o.findings
    );
    assert!(credit(&o).is_none(), "credited beside its own finding");
}

#[test]
fn one_form_named_is_one_sample() {
    let (o, jobs) = run_on(
        App::default(),
        &with_forms(&[("/notes/new", Some("/notes"))]),
    );
    // One form keeps the run's own token, as `text-form` always had.
    assert!(
        jobs[0]
            .actions
            .iter()
            .any(|a| matches!(a, Action::Fill { text, .. } if text.contains("data-sv=\"t0k\"")))
    );
    assert!(credit(&o).expect("credited").in_part);
}

#[test]
fn a_named_form_not_shown_leaves_the_credit_in_part() {
    let app = App {
        drops_for: Some("t0kf1z"),
        ..App::default()
    };
    let (o, _) = run_on(
        app,
        &with_forms(&[("/notes/new", Some("/notes")), ("/profile", None)]),
    );
    let credit = credit(&o).expect("credited for the form that was shown");
    assert!(credit.in_part, "{credit:?}");
    assert!(!credit.scope.contains("/profile"), "{}", credit.scope);
    assert!(
        o.not_assessed
            .iter()
            .any(|(id, why)| id == "V3.2.2" && why.contains("did not appear")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn no_forms_token_is_the_start_of_anothers() {
    for of in [2, 3, 12] {
        for a in 0..of {
            for b in 0..of {
                if a != b {
                    let (x, y) = (form_token("t0k", a, of), form_token("t0k", b, of));
                    assert!(
                        !y.starts_with(&x) && !y.contains(&format!("sv-{x}")),
                        "{x} / {y}"
                    );
                }
            }
        }
    }
}
