//! The advisory files are parsed on one thread per core, and the records come out in the order the walk found them
//! (backlog 0411): that order is the order the findings are listed in, so a run must not list them differently from
//! the run before it. The files that could not be read are still listed, sorted.

use super::*;

fn record(id: &str) -> String {
    format!(
        r#"{{"id": "{id}", "summary": "test", "affected": [{{"package": {{"ecosystem": "PyPI", "name": "pkg"}},
            "ranges": [{{"type": "ECOSYSTEM", "events": [{{"introduced": "0"}}]}}]}}]}}"#
    )
}

#[test]
fn the_records_come_out_in_the_walk_s_order_and_the_unread_are_listed() {
    let dir = std::env::temp_dir().join(format!("sv-osv-order-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    // More files than any machine has cores, in folders, so the files are split into several runs.
    for folder in ["a", "b/c", "d"] {
        std::fs::create_dir_all(dir.join(folder)).unwrap();
        for i in 0..60 {
            std::fs::write(
                dir.join(folder).join(format!("R-{i}.json")),
                record(&format!("{folder}-{i}")),
            )
            .unwrap();
        }
    }
    std::fs::write(dir.join("b/broken.json"), "{ not json").unwrap();
    std::fs::write(dir.join("d/notes.txt"), "not an advisory").unwrap();

    // The walk's own order, read one file at a time.
    let mut files = Vec::new();
    walk(&dir, &mut files).unwrap();
    let in_order: Vec<String> = files
        .iter()
        .filter_map(|path| read_one(&dir, path).ok())
        .map(|a| a.id)
        .collect();
    // The setup: there is an order to keep, across more files than one run holds.
    assert_eq!(in_order.len(), 180);
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    assert!(files.len() > threads, "the files are split into runs");

    let read = read_database(&dir).unwrap();
    std::fs::remove_dir_all(&dir).ok();
    let ids: Vec<String> = read.records.into_iter().map(|a| a.id).collect();
    assert_eq!(ids, in_order, "the records come out in the walk's order");
    assert_eq!(read.unread.len(), 1, "{:?}", read.unread);
    assert!(
        read.unread[0].0.ends_with("broken.json"),
        "{:?}",
        read.unread
    );
}
