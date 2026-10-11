//! The dashboard links the backlog overview only when it is given a board file (backlog 0227, ADR-086).

use super::*;

#[test]
fn the_page_links_the_board_when_one_is_given_and_not_otherwise() {
    let board = Path::new("/Users/owner/code/stackvet-backlog.html");
    let linked = page(&[], "2026-10-11", Some(board));
    assert!(linked.contains("The backlog overview"), "{linked}");
    assert!(
        linked.contains("href=\"file:///Users/owner/code/stackvet-backlog.html\""),
        "{linked}"
    );
    let unlinked = page(&[], "2026-10-11", None);
    assert!(!unlinked.contains("The backlog overview"), "{unlinked}");
}
