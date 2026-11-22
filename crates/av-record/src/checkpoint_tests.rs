//! Where a record's chain has got to, and whether the record still passes through a place signed for (backlog 0716,
//! part 3). The build-loop record is in the app's own folder, so these need nothing of this computer's; the records
//! kept outside it, and the signing, are tried through `av` itself (`crates/av-cli/src/checkpoint_tests.rs`).

use super::*;
use crate::build_loop::{Line, record};

fn app(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("av-checkpoint-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn call(time: &str) -> Line {
    Line {
        time: time.to_owned(),
        tool: "stackvet_status".to_owned(),
        outcome: Some("ok".to_owned()),
        ..Default::default()
    }
}

fn record_file(dir: &Path) -> std::path::PathBuf {
    sv_scan::ecosystems::build_loop_record_in(dir)
}

#[test]
fn a_record_s_head_is_its_last_link_and_a_record_with_none_has_no_head() {
    let dir = app("head");
    assert_eq!(head(&dir, Kind::BuildLoop), Err(NoHead::Nothing));
    record(&dir, &call("2026-11-22T10:00:00Z")).unwrap();
    record(&dir, &call("2026-11-22T10:01:00Z")).unwrap();
    let at = head(&dir, Kind::BuildLoop).expect("two chained lines");
    assert_eq!(at.lines, 2);
    assert_eq!(at.chain.len(), 64);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_record_still_passes_through_a_head_after_more_is_written_and_not_after_a_line_is_changed_or_cut()
 {
    let dir = app("passes");
    record(&dir, &call("2026-11-22T10:00:00Z")).unwrap();
    record(&dir, &call("2026-11-22T10:01:00Z")).unwrap();
    let signed = head(&dir, Kind::BuildLoop).unwrap();
    assert_eq!(still_passes(&dir, Kind::BuildLoop, &signed), Some(true));

    // More lines after it: what was signed for still stands.
    record(&dir, &call("2026-11-22T10:02:00Z")).unwrap();
    assert_eq!(still_passes(&dir, Kind::BuildLoop, &signed), Some(true));
    assert_eq!(head(&dir, Kind::BuildLoop).unwrap().lines, 3);

    // A line before it changed: the tool's name in the first line.
    let path = record_file(&dir);
    let whole = std::fs::read_to_string(&path).unwrap();
    assert!(whole.contains("stackvet_status"), "the setup: {whole}");
    let changed = whole.replacen("stackvet_status", "stackvet_check", 1);
    std::fs::write(&path, &changed).unwrap();
    assert_eq!(still_passes(&dir, Kind::BuildLoop, &signed), Some(false));
    // And a record whose chain is broken has no head to sign: signing would be signing for the change.
    assert!(matches!(
        head(&dir, Kind::BuildLoop),
        Err(NoHead::Broken { .. })
    ));

    // The record put back, then cut to its first line: fewer lines than were signed for.
    let first = whole.lines().next().unwrap();
    std::fs::write(&path, format!("{first}\n")).unwrap();
    assert_eq!(still_passes(&dir, Kind::BuildLoop, &signed), Some(false));

    // The record gone: nothing to read.
    std::fs::remove_file(&path).unwrap();
    assert_eq!(still_passes(&dir, Kind::BuildLoop, &signed), None);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_record_rewritten_whole_with_a_chain_of_its_own_does_not_pass_through_what_was_signed() {
    // What a chain alone cannot show: every line replaced, and the chain made again from the start. The new record
    // checks out by itself, and is not the one that was signed for.
    let dir = app("rewritten");
    record(&dir, &call("2026-11-22T10:00:00Z")).unwrap();
    record(&dir, &call("2026-11-22T10:01:00Z")).unwrap();
    let signed = head(&dir, Kind::BuildLoop).unwrap();
    std::fs::remove_file(record_file(&dir)).unwrap();
    record(&dir, &call("2026-11-22T09:00:00Z")).unwrap();
    record(&dir, &call("2026-11-22T09:01:00Z")).unwrap();
    // The setup: the new record's own chain holds, with as many lines.
    let now = head(&dir, Kind::BuildLoop).expect("the new record checks out by itself");
    assert_eq!(now.lines, 2);
    assert_ne!(now.chain, signed.chain);
    assert_eq!(still_passes(&dir, Kind::BuildLoop, &signed), Some(false));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn what_is_signed_names_the_record_its_length_and_its_last_link() {
    let at = Head {
        lines: 12,
        chain: "ab".repeat(32),
    };
    assert_eq!(
        fields(Kind::Proxy, &at),
        ["av checkpoint", "proxy", "12", &"ab".repeat(32)]
    );
    // A checkpoint of one record is not one of another, or of another length.
    assert_ne!(fields(Kind::Proxy, &at), fields(Kind::Hook, &at));
    assert_ne!(
        fields(Kind::Proxy, &at),
        fields(
            Kind::Proxy,
            &Head {
                lines: 13,
                ..at.clone()
            }
        )
    );
}
