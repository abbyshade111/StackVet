//! `sv --help`, and the help of the commands a newcomer meets first, say where their path is written down (backlog
//! 0476): `sv --help` ends by naming the guide and `sv doctor`; `report`, `doctor`, and `connect` each name the page and
//! section with the whole flow, as `review`'s does (0254). Each section named is checked to be there.

use std::path::Path;
use std::process::Command;

fn help(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(args)
        .output()
        .unwrap();
    let said = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(out.status.success(), "{args:?}: {said}");
    said
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(relative),
    )
    .unwrap_or_else(|e| panic!("{relative}: {e}"))
}

#[test]
fn sv_help_ends_by_naming_the_guide_and_sv_doctor() {
    let said = help(&["--help"]);
    let tail = &said[said
        .find("NEW TO THIS?")
        .expect("a closing line for a newcomer")..];
    assert!(tail.contains("docs/GETTING-STARTED.md"), "{tail}");
    assert!(tail.contains("`sv doctor`"), "{tail}");
    assert!(
        said.find("EXIT STATUS:") < said.find("NEW TO THIS?"),
        "it comes last"
    );
}

#[test]
fn report_doctor_and_connect_name_the_page_with_the_whole_flow() {
    for (command, says, page, heading) in [
        (
            "report",
            "Open report.html in a browser first",
            "README.md",
            "\n### What a report holds\n",
        ),
        (
            "report",
            "docs/INSTALL.md, \"Installing StackVet on your computer, for `--run`\"",
            "docs/INSTALL.md",
            "\n## Installing StackVet on your computer, for `--run`\n",
        ),
        (
            "doctor",
            "docs/INSTALL.md, \"Installing StackVet",
            "docs/INSTALL.md",
            "\n## Installing StackVet on your computer, for `--run`\n",
        ),
        (
            "connect",
            "docs/GETTING-STARTED.md, section 3, \"Connect StackVet to",
            "docs/GETTING-STARTED.md",
            "\n## 3. Connect StackVet to your AI tool\n",
        ),
    ] {
        // As one line, since the help wraps where it likes.
        let said = help(&[command, "--help"])
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            said.contains(says),
            "`sv {command} --help` lacks {says:?}: {said}"
        );
        assert!(read(page).contains(heading), "{page} has no {heading:?}");
    }
}
