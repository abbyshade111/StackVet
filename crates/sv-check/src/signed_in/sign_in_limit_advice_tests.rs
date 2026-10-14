//! What the report says when a sign-in limit refused `sv`'s own sign-in (backlog 0383): keep the wrong-password limit,
//! raise only a limit that counts every sign-in, as the spec now says.

use super::*;

#[test]
fn a_refused_sign_in_asks_to_keep_the_wrong_password_limit() {
    let said = sign_in_limit_says(&["login-a".to_owned()]);
    assert!(said.contains("(login-a)"), "{said}");
    assert!(
        said.contains("stopping wrong passwords at the number `failed-sign-ins` declares"),
        "{said}"
    );
    assert!(said.contains("a limit that counts every sign-in"), "{said}");
}
