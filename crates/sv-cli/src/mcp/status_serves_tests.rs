//! Backlog 0541: `stackvet_status` opens by naming the folder the server serves and the app its stackvet.toml names,
//! so an AI tool talking to a server started for an earlier app finds out at once; and `sv connect` says that such a
//! server may still be running.

use super::tests::{call, scratch_app, sv_own_words, text};
use super::*;

/// The line naming the folder, which comes before every answer the status gives (after only the fence's notice).
fn serves_line(said: &str) -> &str {
    let at = said
        .find("This StackVet server serves ")
        .unwrap_or_else(|| panic!("no line naming the folder: {said}"));
    let answers = said.find("Is everything ready for").expect("the answers");
    assert!(at < answers, "the folder is named first: {said}");
    said[at..].lines().next().unwrap()
}

#[test]
fn the_status_opens_with_the_folder_and_the_app_named_there() {
    let root = scratch_app("status-serves", "flask-booking");
    let server = Server::new(&root.join("app")).unwrap();
    let result = call(&server, "stackvet_status", json!({}));
    let canonical = sv_frameworks::paths::canonical(&root.join("app")).unwrap();
    std::fs::remove_dir_all(&root).ok();
    assert_eq!(result["isError"], false, "{result}");
    let said = text(&result);
    let first = serves_line(said);
    assert!(first.contains(&canonical.display().to_string()), "{first}");
    assert!(first.contains("Clinic booking"), "{first}");
    assert!(first.contains("`docker stop`"), "{first}");
    // The folder and the name are the app's own text, so they are fenced, and `sv`'s words around them are not.
    let own = sv_own_words(first);
    assert!(!own.contains("Clinic booking"), "{own}");
    assert!(
        own.contains("If that is not the folder of the app you are building"),
        "{own}"
    );
    let structured = &result["structuredContent"];
    assert_eq!(structured["root"], canonical.display().to_string());
    assert_eq!(structured["appName"], "Clinic booking");
}

#[test]
fn a_folder_with_no_stackvet_toml_says_so() {
    let root = std::env::temp_dir().join(format!("sv-mcp-status-bare-{}", std::process::id()));
    std::fs::remove_dir_all(&root).ok();
    std::fs::create_dir_all(&root).unwrap();
    let server = Server::new(&root).unwrap();
    let result = call(&server, "stackvet_status", json!({}));
    std::fs::remove_dir_all(&root).ok();
    let first = serves_line(text(&result)).to_owned();
    assert!(first.contains("has no stackvet.toml yet"), "{first}");
    assert!(result["structuredContent"]["appName"].is_null(), "{result}");
}

#[test]
fn sv_connect_says_an_older_server_may_still_run() {
    for args in [vec![], vec!["claude".to_owned()]] {
        let said = crate::connect::where_it_goes(&args);
        assert!(
            said.contains("an earlier app may still be running"),
            "{said}"
        );
        assert!(
            said.contains("`docker stop`") && said.contains("`stackvet_status`"),
            "{said}"
        );
    }
}
