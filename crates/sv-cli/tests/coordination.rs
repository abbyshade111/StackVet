//! The coordination board: one session hands out the work, and every hand-out is a line in git (backlog 0273;
//! `docs/adr/ADR-061.md`, Later, 12 October 2026). `tools/coordination.py` is the only thing that writes
//! `docs/coordination/board.md`; this runs its self-test (the commands, their refusals, the measures on a history made
//! for the test, and the no-context clone) and its check on the real board, so a board that disagrees with the backlog
//! fails here and not in the owner's hands.

use std::process::Command;

fn root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn run(args: &[&str]) -> (bool, String) {
    let out = Command::new("python3")
        .arg("-I")
        .arg(root().join("tools/coordination.py"))
        .args(args)
        .current_dir(root())
        .output()
        .expect("python3 runs");
    (
        out.status.success(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ),
    )
}

#[test]
fn the_board_tool_passes_its_own_test() {
    let (ok, said) = run(&["--self-test"]);
    assert!(ok, "{said}");
    assert!(said.contains("coordination self-test: ok"), "{said}");
}

#[test]
fn the_real_board_agrees_with_the_backlog() {
    // The setup: the board is there to be read.
    assert!(
        root().join("docs/coordination/board.md").is_file(),
        "docs/coordination/board.md is the board"
    );
    let (ok, said) = run(&["check"]);
    assert!(ok, "{said}");
    assert!(said.contains("board: agrees with the backlog"), "{said}");
    let (ok, said) = run(&["board"]);
    assert!(ok, "{said}");
    assert!(
        said.contains("## Sessions") && said.contains("## Queue") && said.contains("## Log"),
        "{said}"
    );
}

#[test]
fn a_hand_out_ends_with_the_footer_whatever_the_item() {
    // 0273 is the trial's own item and is always in the tree (live or done).
    let (ok, said) = run(&["handout", "0273"]);
    assert!(ok, "{said}");
    assert!(
        said.contains("Your next item is backlog 0273")
            && said.contains("Delete nothing, force-push nothing")
            && said.contains("tools/backlog.py done <number>"),
        "{said}"
    );
}
