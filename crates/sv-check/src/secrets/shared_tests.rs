//! The credential rules are read once for the whole run, by one loader (backlog 0459). Six places read them for
//! themselves before, one of them again, its thirty patterns compiled anew, for every seed line that failed.

use super::*;

#[test]
fn every_call_hands_back_the_one_set_of_rules() {
    let first = SecretRules::shared().expect("the rules sv ships");
    let second = SecretRules::shared().expect("the rules sv ships");
    assert!(std::ptr::eq(first, second), "read once, not once per call");
    // The setup: they are the rules `sv` ships, which find a key.
    let key = ["sk", "ant", "api03", "Zp8Kd3Wq1Ls6Vn0Rt4Yb9Xm2Qc"].join("-");
    assert_eq!(redact_text(first, &key).1, 1);
}

#[test]
fn the_sentence_for_unread_rules_says_what_was_left_out_and_why() {
    assert!(
        RULES_UNREAD.starts_with("left out, because"),
        "{RULES_UNREAD}"
    );
    assert!(RULES_UNREAD.contains("could not be read"), "{RULES_UNREAD}");
}
