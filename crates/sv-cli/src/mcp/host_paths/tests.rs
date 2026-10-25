//! Backlog 0375 (ADR-153): the MCP server, in a Linux container on Windows, hands back paths in the person's own form
//! and reads the ones it is given from it, and leaves every other string alone.

use super::*;

fn paths() -> HostPaths {
    HostPaths::new("/c/Users/you/app", r"C:\Users\you\app").expect("the two forms agree")
}

#[test]
fn a_path_under_the_folder_is_handed_back_in_windows_form() {
    let p = paths();
    assert_eq!(p.outward("/c/Users/you/app"), r"C:\Users\you\app");
    assert_eq!(
        p.outward("Wrote /c/Users/you/app/stackvet-report/report.html, open it first."),
        r"Wrote C:\Users\you\app\stackvet-report\report.html, open it first."
    );
    // A line number stays after the path; a link keeps forward slashes, as a file address has them.
    assert_eq!(
        p.outward("`/c/Users/you/app/src/a.py:12`"),
        r"`C:\Users\you\app\src\a.py:12`"
    );
    assert_eq!(
        p.outward("file:///c/Users/you/app/stackvet-report/report.html"),
        "file:///C:/Users/you/app/stackvet-report/report.html"
    );
}

#[test]
fn a_windows_path_it_is_given_is_read_as_the_one_inside() {
    let p = paths();
    assert_eq!(
        p.inward(r"C:\Users\you\app\src\a.py"),
        "/c/Users/you/app/src/a.py"
    );
    assert_eq!(p.inward("c:/Users/you/app/src"), "/c/Users/you/app/src");
    assert_eq!(
        p.inward("file:///C:/Users/you/app/a.txt"),
        "file:///c/Users/you/app/a.txt"
    );
}

#[test]
fn what_is_not_under_the_folder_is_left_alone() {
    let p = paths();
    for text in [
        "/c/Users/you/application/x",
        "/c/Users/you/app2",
        "/home/c/Users/you/app/x",
        r"D:\Users\you\app\x",
        "nothing here",
        "",
    ] {
        assert_eq!(p.outward(text), text, "{text}");
        assert_eq!(p.inward(text), text, "{text}");
    }
}

#[test]
fn every_string_of_a_message_changes_and_its_shape_does_not() {
    let p = paths();
    let reply = r#"{"jsonrpc":"2.0","id":7,"result":{"content":[{"type":"text","text":"See /c/Users/you/app/x.md"}],"files":["/c/Users/you/app/a","/tmp/b"],"count":3}}"#;
    let back: serde_json::Value = serde_json::from_str(&p.line(reply, true)).unwrap();
    assert_eq!(back["id"], 7);
    assert_eq!(back["result"]["count"], 3);
    assert_eq!(
        back["result"]["content"][0]["text"],
        r"See C:\Users\you\app\x.md"
    );
    assert_eq!(back["result"]["files"][0], r"C:\Users\you\app\a");
    assert_eq!(back["result"]["files"][1], "/tmp/b");
    // A line that is not JSON goes as it came.
    assert_eq!(
        p.line("not json /c/Users/you/app", true),
        "not json /c/Users/you/app"
    );
}

#[test]
fn a_folder_with_a_space_in_it_and_one_given_with_a_trailing_slash_work() {
    let p = HostPaths::new("/c/Users/Jane Doe/app/", r"C:\Users\Jane Doe\app\")
        .expect("the two forms agree");
    assert_eq!(
        p.outward("/c/Users/Jane Doe/app/r.html"),
        r"C:\Users\Jane Doe\app\r.html"
    );
    assert_eq!(
        p.inward(r"C:\Users\Jane Doe\app\r.html"),
        "/c/Users/Jane Doe/app/r.html"
    );
}

#[test]
fn forms_that_do_not_agree_are_refused() {
    assert!(HostPaths::new("/c/Users/you/app", r"C:\Users\you\other").is_none());
    assert!(
        HostPaths::new("/c/Users/you/app", "/c/Users/you/app").is_none(),
        "not a Windows folder"
    );
}
