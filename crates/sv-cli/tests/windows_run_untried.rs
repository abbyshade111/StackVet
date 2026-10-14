//! Backlog 0510: the checks that start the app have never been tried on a Windows computer, and every place a Windows
//! user decides what to expect says so before the fact: `sv doctor` (`doctor::WINDOWS_RUN_UNTRIED`), the setup page's
//! Windows section and its two prompts that reach the full report, and the guide's two Windows passages. The setup page
//! used to say Docker Desktop was "needed later only for the checks that start your app", which read as if that worked.

use std::path::Path;

fn read(relative: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(relative),
    )
    .unwrap_or_else(|e| panic!("{relative}: {e}"))
}

/// The text between `from` and `to`, with its line breaks and quote marks folded into single spaces.
fn between(text: &str, from: &str, to: &str) -> String {
    let start = text.find(from).unwrap_or_else(|| panic!("no {from:?}"));
    let end = start
        + text[start..]
            .find(to)
            .unwrap_or_else(|| panic!("no {to:?} after {from:?}"));
    text[start..end]
        .split_whitespace()
        .filter(|w| *w != ">")
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn the_setup_page_says_what_sv_doctor_says_before_the_windows_prompt() {
    let page = read("docs/prompts/setup.md");
    assert!(
        !page.contains("needed later only"),
        "the setup page still reads as if the checks that start the app work on Windows"
    );
    let intro = between(&page, "### On Windows", "<!-- setup-windows:start -->");
    let first = sv_cli::doctor::WINDOWS_RUN_UNTRIED
        .split_inclusive("running")
        .next()
        .unwrap();
    let first: String = first.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(intro.contains(&first), "{intro}\n\nwants: {first}");
}

#[test]
fn the_windows_prompt_and_the_full_report_prompt_say_it_to_the_ai_tool() {
    let page = read("docs/prompts/setup.md");
    let windows = between(
        &page,
        "<!-- setup-windows:start -->",
        "<!-- setup-windows:end -->",
    );
    assert!(
        windows.contains("have not been tried yet")
            && windows.contains("Don't promise me the full report"),
        "{windows}"
    );
    let full = between(
        &page,
        "<!-- setup-step-6:start -->",
        "<!-- setup-step-6:end -->",
    );
    assert!(
        full.contains(
            "On Windows, tell me first that this step has not been tried on a Windows computer"
        ),
        "{full}"
    );
}

#[test]
fn the_guide_says_it_where_windows_is_set_up_and_where_the_full_report_is() {
    let guide = read("docs/GETTING-STARTED.md");
    let connect = between(
        &guide,
        "**On Windows, the Docker settings do not work yet:**",
        "### Claude",
    );
    assert!(
        connect.contains("the checks that start your app, not at all"),
        "{connect}"
    );
    let full = between(
        &guide,
        "Those checks need `sv report --run`",
        "If your app uses packages",
    );
    assert!(full.contains("untried on Windows"), "{full}");
}
