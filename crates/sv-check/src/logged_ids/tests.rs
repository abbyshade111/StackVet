//! What the reader of a logger call takes as logging only ids, and what it leaves as a finding (backlog 0583).

use super::logged_ids;

fn read(line: &str) -> Option<Vec<String>> {
    logged_ids(line, 1)
}

#[test]
fn ids_alone_are_read_as_ids() {
    assert_eq!(
        read(r#"    logger.info("Password reset requested for user %s", user_id)"#),
        Some(vec!["user_id".to_owned()])
    );
    assert_eq!(
        read(
            r#"log.warning("token refresh for game %s by %s", game.id, current_user.id, exc_info=True)"#
        ),
        Some(vec!["game.id".to_owned(), "current_user.id".to_owned()])
    );
    assert_eq!(
        read(r#"self.logger.debug('secret rotated for %s', accountId)"#),
        Some(vec!["accountId".to_owned()])
    );
}

#[test]
fn a_call_over_several_lines_is_followed() {
    let text = "logger.error(\n    \"credential check failed for %s (%s)\",\n    user.id,\n    request_id,\n)\n";
    assert_eq!(
        logged_ids(text, 1),
        Some(vec!["user.id".to_owned(), "request_id".to_owned()])
    );
}

#[test]
fn a_value_named_for_a_secret_or_not_an_id_is_left_as_a_finding() {
    for line in [
        // The value itself is the secret.
        r#"logger.info("password for %s is %s", user_id, password)"#,
        r#"logger.info("token %s", token_id)"#,
        r#"logger.info("session %s", session_id)"#,
        r#"logger.info("api key %s", api_key_id)"#,
        // Not an id.
        r#"logger.info("token issued to %s", user.email)"#,
        r#"logger.info("token issued to %s", user)"#,
        r#"logger.info("token issued to %s", get_id(user))"#,
        r#"logger.info("token issued to %s", ids[0])"#,
        // The message is built, so its values cannot be read from the arguments.
        r#"logger.info("token for %s" % user_id)"#,
        r#"logger.info(f"token for %s {token}", user_id)"#,
        r#"logger.info("token for %s".format(token), user_id)"#,
        r#"logger.info("token " + token + " for %s", user_id)"#,
        // A mapping logged beside it may hold anything.
        r#"logger.info("token for %s", user_id, extra=details)"#,
        // Nothing logged, or no call to read.
        r#"logger.info("token for %s")"#,
        r#"print("token for %s", user_id)"#,
    ] {
        assert_eq!(read(line), None, "{line}");
    }
}

#[test]
fn a_call_that_does_not_close_is_left_as_a_finding() {
    let text = "logger.info(\"token for %s\",\n    user_id\n";
    assert_eq!(logged_ids(text, 1), None);
    assert_eq!(logged_ids("logger.info(\"token %s\", user_id)", 2), None);
    assert_eq!(logged_ids("logger.info(\"token %s\", user_id)", 0), None);
}
