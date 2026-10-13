//! Lines cut from the end of the build-loop record leave a chain that checks out (backlog 0310): each
//! line is chained only to the one before it, so nothing in the record says how long it was. What
//! the report says of a chain that holds must leave room for that.

use super::*;
use std::path::PathBuf;

#[test]
fn lines_cut_from_the_end_leave_a_chain_that_holds() {
    let dir = std::env::temp_dir().join(format!("sv-build-loop-tail-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"t\"\n",
    )
    .unwrap();
    for (time, tool) in [("1", "a"), ("2", "b"), ("3", "c")] {
        let line = Line {
            time: time.to_owned(),
            tool: tool.to_owned(),
            ..Line::default()
        };
        record(&dir, &line).unwrap();
    }
    let file: PathBuf = sv_scan::ecosystems::default_report_dir_in(&dir)
        .join(sv_scan::ecosystems::BUILD_LOOP_RECORD);
    let whole = read(&dir);
    // The setup: three chained lines, and the chain holds.
    assert_eq!(
        (whole.chained, whole.chain_broken_at),
        (3, None),
        "{whole:?}"
    );
    let text = std::fs::read_to_string(&file).unwrap();
    let kept: Vec<&str> = text.lines().take(2).collect();
    std::fs::write(&file, kept.join("\n") + "\n").unwrap();
    let cut = read(&dir);
    std::fs::remove_dir_all(&dir).ok();
    // The last line is gone, and nothing in the record shows it: the chain holds over two.
    assert_eq!((cut.chained, cut.chain_broken_at), (2, None), "{cut:?}");
    assert_ne!(
        cut.chain_head, whole.chain_head,
        "the head moved back a line"
    );
}
