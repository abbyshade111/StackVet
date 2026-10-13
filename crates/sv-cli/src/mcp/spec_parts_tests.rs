//! `stackvet_spec` in parts (backlog 0347): the spec was 47,187 characters on 12 October 2026, sent
//! whole on every call, over the 40,000 an answer is kept to and close to the 50,000 past which
//! Claude Code shows the model a file's first 2 KB instead. Each answer now fits, the first holds
//! the start of the file and how to fill it in, and the parts join to the whole spec.

use super::tests::{call, text};
use super::*;
use std::collections::BTreeMap;

fn server() -> Server {
    let examples = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    Server::new(&examples.join("tested-notes")).unwrap()
}

/// Each page an answer shows, by section and page, with its text.
fn pages(answer: &str) -> Vec<((String, usize), String, usize)> {
    let end = answer
        .find(&format!("\n{}\n", crate::parts::PARTS))
        .unwrap_or(answer.len());
    let mut out = Vec::new();
    let mut rest = &answer[..end];
    while let Some(at) = rest.find(crate::parts::MARK) {
        let (head, body) = rest[at + crate::parts::MARK.len()..]
            .split_once("]\n")
            .unwrap();
        let (name, of) = head.split_once(", page ").unwrap();
        let (page, count) = of.split_once(" of ").unwrap();
        let next = body.find(crate::parts::MARK).unwrap_or(body.len());
        out.push((
            (name.to_owned(), page.parse().unwrap()),
            body[..next].to_owned(),
            count.parse().unwrap(),
        ));
        rest = &body[next..];
    }
    out
}

#[test]
fn the_spec_comes_in_parts_each_under_the_budget_and_joined_they_are_the_whole_spec() {
    let server = server();
    let whole = text(&call(&server, "stackvet_spec", json!({ "section": "all" }))).to_owned();
    assert_eq!(whole, catalog::spec_text());
    // The setup: the spec is longer than one answer holds, as it was when this was found.
    assert!(whole.len() > crate::parts::ANSWER_BUDGET, "{}", whole.len());

    let first = call(&server, "stackvet_spec", json!({}));
    assert_eq!(first["isError"], false, "{}", text(&first));
    assert!(first.get("structuredContent").is_none(), "{first:#}");
    let said = text(&first);
    assert!(said.len() <= crate::parts::ANSWER_BUDGET, "{}", said.len());
    // The first answer starts with the start of the file and how to fill it in.
    let shown: Vec<String> = pages(said).into_iter().map(|((s, _), _, _)| s).collect();
    assert_eq!(shown[..2], ["app", "instructions"], "{said}");
    assert!(said.starts_with(&format!(
        "{}app, page 1 of 1]\n# stackvet.toml",
        crate::parts::MARK
    )));
    assert!(said.contains("[app]\nname = \"\""));
    assert!(said.contains("Every capability line starts commented out"));
    assert!(said.contains(crate::parts::PARTS));

    // Every page of every section, each asked for on its own.
    let mut joined: BTreeMap<(String, usize), String> = BTreeMap::new();
    for section in catalog::SPEC_SECTIONS {
        let mut page = 1;
        loop {
            let answer = call(
                &server,
                "stackvet_spec",
                json!({ "section": section, "page": page }),
            );
            assert_eq!(answer["isError"], false, "{}", text(&answer));
            assert!(answer.get("structuredContent").is_none());
            let said = text(&answer);
            assert!(
                said.len() <= crate::parts::ANSWER_BUDGET,
                "{section} {page}"
            );
            let mut count = 0;
            for (at, body, of) in pages(said) {
                if at.0 == *section {
                    count = of;
                }
                if let Some(seen) = joined.get(&at) {
                    assert_eq!(seen, &body, "{at:?} differs between answers");
                }
                joined.insert(at, body);
            }
            assert!(count >= page, "{section} has no page {page}");
            if page == count {
                break;
            }
            page += 1;
        }
    }
    let mut again = String::new();
    for section in catalog::SPEC_SECTIONS {
        for ((s, _), body) in &joined {
            if s == section {
                again.push_str(body);
            }
        }
    }
    assert_eq!(again, whole, "the parts do not join to the whole spec");
}

#[test]
fn a_section_the_spec_does_not_have_is_refused() {
    let answer = call(&server(), "stackvet_spec", json!({ "section": "findings" }));
    assert_eq!(answer["isError"], true, "{}", text(&answer));
    assert!(text(&answer).contains("app, manifest, instructions, prompts"));
}
