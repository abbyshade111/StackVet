//! `sv dashboard` writes its page through a staging file beside it (backlog 0314): a link planted at
//! the staging name, in a folder others can write to, is refused rather than written through.

use super::cmd_dashboard;

#[test]
fn a_link_at_the_dashboards_staging_name_is_not_written_through() {
    let dir = std::env::temp_dir().join(format!("sv-dashboard-link-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("app")).unwrap();
    std::fs::create_dir_all(dir.join("out")).unwrap();
    std::fs::write(dir.join("precious.txt"), "keep me\n").unwrap();
    // The name `sv dashboard` stages its page under, in this process.
    let staging = dir
        .join("out")
        .join(format!(".dash.html.sv-{}", std::process::id()));
    std::os::unix::fs::symlink(dir.join("precious.txt"), &staging).unwrap();
    // The setup: the link is there and reaches the file it would write through.
    assert_eq!(std::fs::read_to_string(&staging).unwrap(), "keep me\n");

    let args: Vec<String> = [
        dir.join("app").display().to_string(),
        "--out".to_owned(),
        dir.join("out/dash.html").display().to_string(),
    ]
    .into();
    let result = cmd_dashboard(&args);
    let kept = std::fs::read_to_string(dir.join("precious.txt")).unwrap();
    let target = std::fs::symlink_metadata(dir.join("out/dash.html"));
    std::fs::remove_dir_all(&dir).ok();

    assert!(kept == "keep me\n", "the page was written through the link");
    assert!(
        target.is_err(),
        "the link was moved into the page's place: {target:?}"
    );
    let said = format!(
        "{:#}",
        result.expect_err("a link at the staging name is refused")
    );
    assert!(said.contains("could not be created"), "{said}");
}
