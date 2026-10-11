//! Every section of a kept record reaches the file it is written to (backlog 229 part 1). A section
//! left out of `kept_value` is written nowhere, which no test of the record itself would notice.

use super::*;

#[test]
fn each_container_reading_reaches_the_file_with_its_id() {
    let seen = Seen {
        liveness: vec![Reading {
            id: "liveness-1".to_owned(),
            after: "the questions asked as somebody not signed in".to_owned(),
            status: "running".to_owned(),
            restarts: 0,
            exit_code: 0,
            out_of_memory: false,
            answered: true,
        }],
        ..Seen::default()
    };
    let value = kept_value("an app", &seen);
    assert_eq!(value["liveness"][0]["id"], "liveness-1", "{value}");
    assert_eq!(value["liveness"][0]["answered"], true, "{value}");
}

#[test]
fn the_about_text_names_every_section_the_file_holds() {
    // The setup: a record that fills each section, so each one is written and can be checked.
    let seen = Seen {
        liveness: vec![Reading {
            id: "liveness-0".to_owned(),
            after: "the questions asked as somebody not signed in".to_owned(),
            status: "running".to_owned(),
            restarts: 0,
            exit_code: 0,
            out_of_memory: false,
            answered: true,
        }],
        stand_ins: StandIns {
            names: Some(vec![NameAsked {
                name: "api.example.test".to_owned(),
                kind: "A".to_owned(),
                asked: 2,
                first: "2026-10-10T03:00:01Z".to_owned(),
            }]),
            ..StandIns::default()
        },
        ..Seen::default()
    };
    let value = kept_value("an app", &seen);
    let sections = [
        "stand_ins",
        "app_log",
        "signed_in",
        "signed_in_unanswered",
        "liveness",
        "tool_output",
    ];
    for section in sections {
        assert!(
            value.get(section).is_some(),
            "{section} is not written: {value}"
        );
        assert!(
            ABOUT.contains(&format!("under {section}")),
            "ABOUT does not name {section}"
        );
    }
    assert!(value["stand_ins"]["names"].is_array(), "{value}");
    assert!(
        ABOUT.contains("under stand_ins.names"),
        "ABOUT does not name stand_ins.names"
    );
}
