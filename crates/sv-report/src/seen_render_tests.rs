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
