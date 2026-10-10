//! The password findings name the sign-up and private-page answers of the labels they read (ADR-082,
//! backlog 0229, part 1).

use super::*;

#[test]
fn a_label_names_its_sign_up_and_its_private_page_answer() {
    assert_eq!(
        answers_of(&["short", "long"]),
        [
            "signup-short",
            "private-short",
            "signup-long",
            "private-long"
        ]
    );
    assert!(answers_of(&[]).is_empty());
}
