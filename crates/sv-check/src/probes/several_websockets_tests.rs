//! ADR-118 (backlog 0232, part 3): every WebSocket path stackvet.toml names under `[stack.run]` is asked the anonymous
//! question, and the credit is whole only when two or more were named, each upgraded a plain handshake, and every one
//! refused the foreign handshake.

use super::*;

fn answer(id: &str, status: u16) -> ProbeResponse {
    ProbeResponse {
        id: id.to_owned(),
        status,
        headers: if status == 101 {
            vec![("upgrade".to_owned(), "websocket".to_owned())]
        } else {
            Vec::new()
        },
        body: String::new(),
    }
}

fn two() -> Vec<String> {
    vec!["/ws".to_owned(), "/chat".to_owned()]
}

fn credit(verified: &[crate::Verified]) -> Option<&crate::Verified> {
    verified.iter().find(|v| v.check_id == WS_ORIGIN.rule_id)
}

#[test]
fn each_path_named_gets_its_own_two_handshakes() {
    let sent = api_requests(None, &two());
    let asked: Vec<(&str, &str)> = sent
        .iter()
        .map(|r| (r.id.as_str(), r.path.as_str()))
        .collect();
    assert_eq!(
        asked,
        [
            ("ws-no-origin", "/ws"),
            ("ws-foreign-origin", "/ws"),
            ("ws-no-origin-1", "/chat"),
            ("ws-foreign-origin-1", "/chat"),
        ]
    );
}

#[test]
fn two_paths_that_both_refuse_another_site_are_checked_whole() {
    let (f, v, na) = evaluate_api(
        &[
            answer("ws-no-origin", 101),
            answer("ws-foreign-origin", 403),
            answer("ws-no-origin-1", 101),
            answer("ws-foreign-origin-1", 403),
        ],
        None,
        &two(),
    );
    assert!(f.is_empty() && na.is_empty(), "{f:?} {na:?}");
    let credit = credit(&v).expect("credited");
    assert!(!credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("/ws (403)") && credit.scope.contains("/chat (403)"),
        "{}",
        credit.scope
    );
    assert_eq!(
        credit.evidence,
        [
            "ws-no-origin",
            "ws-foreign-origin",
            "ws-no-origin-1",
            "ws-foreign-origin-1"
        ]
    );
}

#[test]
fn a_second_path_open_to_any_site_is_found_naming_it_and_not_credited() {
    let (f, v, _) = evaluate_api(
        &[
            answer("ws-no-origin", 101),
            answer("ws-foreign-origin", 403),
            answer("ws-no-origin-1", 101),
            answer("ws-foreign-origin-1", 101),
        ],
        None,
        &two(),
    );
    let found = f
        .iter()
        .find(|x| x.rule_id == WS_ORIGIN.rule_id)
        .expect("the finding");
    assert!(
        found.description.contains("at /chat"),
        "{}",
        found.description
    );
    assert_eq!(found.evidence, ["ws-no-origin-1", "ws-foreign-origin-1"]);
    assert!(credit(&v).is_none(), "credited beside its own finding");
}

#[test]
fn a_path_that_upgrades_nothing_keeps_the_credit_in_part_and_is_named() {
    let mut paths = two();
    paths.push("/nowhere".to_owned());
    let (_, v, na) = evaluate_api(
        &[
            answer("ws-no-origin", 101),
            answer("ws-foreign-origin", 403),
            answer("ws-no-origin-1", 101),
            answer("ws-foreign-origin-1", 403),
            answer("ws-no-origin-2", 404),
            answer("ws-foreign-origin-2", 404),
        ],
        None,
        &paths,
    );
    let credit = credit(&v).expect("credited for the two that held");
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("not judged: /nowhere"),
        "{}",
        credit.scope
    );
    assert!(
        na.iter()
            .any(|(id, why)| id == "V4.4.2" && why.contains("at /nowhere")),
        "{na:?}"
    );
}

#[test]
fn both_websocket_and_websockets_are_refused_when_the_file_is_read() {
    let text = "manifest-version = 1\n[app]\nname = \"x\"\n[stack.run]\nimage = \"busybox:1.36\"\n\
                start = \"httpd\"\nwebsocket = \"/ws\"\nwebsockets = [\"/ws\", \"/chat\"]\n";
    let refused = sv_manifest::Manifest::parse(text, std::path::Path::new("stackvet.toml"))
        .expect_err("read both");
    assert!(
        refused
            .to_string()
            .contains("both `websocket` and `websockets`"),
        "{refused}"
    );
    let text = text.replace("websocket = \"/ws\"\n", "");
    let read = sv_manifest::Manifest::parse(&text, std::path::Path::new("stackvet.toml"))
        .expect("the list alone is read");
    assert_eq!(read.stack.run.websocket_paths(), two());
}
