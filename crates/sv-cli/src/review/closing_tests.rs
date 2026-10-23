//! Backlog 0634: the closing questions of `sv review` say how many are left once, and each where it stands; none says
//! it is the last while others follow. The owner's walk-through build was told "Last, …", then "One more: …" twice.

use super::closing_place;
use super::tests::{Scratch, with_app};

const ALL_FOUR: &str = "manifest-version = 1\n[app]\nname = \"R\"\naudience = \"just-me\"\n[stack]\nlanguages = \
[\"python\"]\n[capabilities]\nauth = false\n[stack.run.users]\nnot-offered = [\"two-factor\"]\n[data]\ncategories = [\"contact\"]\n[policy]\n\
fix-within-days = { critical = 7, high = 30 }\n";

#[test]
fn four_closing_questions_are_counted_and_numbered_in_order() {
    let s = Scratch::new("closing-four");
    with_app(&s, ALL_FOUR);
    let (result, out) = s.run("owner\nowner\nowner\nowner\n");
    result.unwrap();
    assert!(out.contains("\n4 more questions.\n"), "{out}");
    let at = |s: &str| out.find(s).unwrap_or_else(|| panic!("no {s:?} in:\n{out}"));
    let order = [
        at("Question 1 of 4: the answers that set the app's level."),
        at("Question 2 of 4: the account changes your app does not offer."),
        at("Question 3 of 4: how soon a known vulnerability in your app's packages is fixed."),
        at("Question 4 of 4: what stackvet.toml says your app does and does not do."),
    ];
    assert!(order.windows(2).all(|w| w[0] < w[1]), "{out}");
    for said_too_soon in ["Last, ", "Last: ", "One more: "] {
        assert!(!out.contains(said_too_soon), "{said_too_soon:?} in:\n{out}");
    }
}

#[test]
fn a_question_alone_is_one_more_and_many_are_numbered() {
    assert_eq!(closing_place(1, 1), "One more question");
    assert_eq!(closing_place(1, 3), "Question 1 of 3");
    assert_eq!(closing_place(3, 3), "Question 3 of 3");
}
