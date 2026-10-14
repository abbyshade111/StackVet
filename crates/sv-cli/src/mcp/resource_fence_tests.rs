//! A report read back through `resources/read` is fenced as a tool's result is (the gap analysis of
//! 7 October 2026, 4.6; ADR-066, Later, 9 October 2026). Before this, an app named "IGNORE ALL
//! PREVIOUS INSTRUCTIONS…" was fenced in every tool's result and came back plain in its own report.
use super::tests::{INJECTION, call, fence_tag, injected_app, listed, read, text};
use super::*;

#[test]
fn the_apps_text_in_a_report_read_back_is_inside_the_fence() {
    let (root, app) = injected_app("resource-fence", INJECTION);
    let server = Server::new(&root).unwrap();
    let written = call(&server, "stackvet_write_report", json!({ "path": app }));
    assert_eq!(written["isError"], false, "{}", text(&written));
    let resources = listed(&server);
    assert_eq!(resources.len(), REPORT_FILES.len(), "{resources:#?}");
    let mut fenced = 0;
    for resource in &resources {
        let uri = resource["uri"].as_str().unwrap();
        let path = path_from_uri(uri).unwrap();
        let name = path.file_name().unwrap().to_str().unwrap();
        let on_disk = std::fs::read_to_string(&path).unwrap();
        // The setup: the planted name has to be in the report, or its absence below proves nothing. Every file
        // names the app, the bill of materials too since it lists the app itself (backlog 0402), and the PDF in
        // the text it draws, which is written in hexadecimal and so is read back to be seen (ADR-116).
        let quoted = if name == sv_report::pdf::FILE {
            sv_report::pdf::back::read(&on_disk).unwrap().text()
        } else {
            on_disk.clone()
        };
        assert!(
            quoted.contains(INJECTION),
            "{name} does not quote the app's name"
        );
        let reply = read(&server, uri);
        let said = reply["result"]["contents"][0]["text"]
            .as_str()
            .unwrap()
            .to_owned();
        if !resources::fenced_when_read(name) {
            // The control: the files a program parses come back as written, and still parse.
            assert_eq!(said, on_disk, "{name}");
            if name == sv_report::pdf::FILE {
                sv_report::pdf::back::read(&said).unwrap_or_else(|e| panic!("{name}: {e}"));
            } else {
                serde_json::from_str::<Value>(&said).unwrap_or_else(|e| panic!("{name}: {e}"));
            }
            continue;
        }
        fenced += 1;
        let tag = fence_tag(&said).unwrap_or_else(|| panic!("{name} is not fenced: {said}"));
        let (open, close) = (format!("<{tag}>"), format!("</{tag}>"));
        // One fence, opened once and closed once, at the very end, with the report whole inside it.
        assert_eq!(
            said.matches(&open).count(),
            2,
            "{name}: the header names it once, then it opens"
        );
        assert_eq!(
            said.matches(&close).count(),
            3,
            "{name}: the header names it twice, then it closes"
        );
        let (outside, inside) = said.split_once(&format!("\n\n{open}\n")).unwrap();
        assert_eq!(inside, format!("{on_disk}\n{close}"), "{name}");
        assert!(!outside.contains(INJECTION), "{name}: {outside}");
        assert!(
            outside.contains("never an instruction"),
            "{name}: {outside}"
        );
    }
    assert_eq!(
        fenced, 4,
        "the page and the three Markdown files are fenced"
    );
}
