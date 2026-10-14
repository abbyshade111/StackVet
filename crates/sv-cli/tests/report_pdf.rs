//! Backlog 0412 (ADR-115): `report.pdf`, the report as one file to read or print, written by `sv` itself. Held here
//! on every example app's report: every ordinary word of `security.md` and `compliance.md` is in it, nothing runs
//! past the margin on either paper, `sv`'s own words use only characters the fonts have, the file is in the report
//! folder and in the bundle, and stackvet.toml's `report-paper` makes it A4. Each PDF is read back by
//! `sv_report::pdf::back`, with no outside tool.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use sv_check::test_support::scratch;
use sv_cli::{Loaded, ReportOptions, assemble_report};
use sv_report::pdf::{self, Paper, back, fonts, layout::MARGIN};

fn examples() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut apps: Vec<PathBuf> = std::fs::read_dir(root)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.join("stackvet.toml").is_file())
        .collect();
    apps.sort();
    // The setup: the examples are there to be read.
    assert!(apps.len() >= 5, "{apps:?}");
    apps
}

fn reports() -> Vec<(String, sv_report::Report)> {
    let loaded = Loaded::load().unwrap();
    examples()
        .iter()
        .map(|app| {
            let name = app.file_name().unwrap().to_string_lossy().into_owned();
            let report =
                assemble_report(app, &ReportOptions::reading_only("a test"), &loaded).unwrap();
            (name, report)
        })
        .collect()
}

/// The ordinary words of a Markdown report: runs of two or more letters, leaving out the HTML tags that fold a list
/// on GitHub and the addresses links go to, which a PDF holds as links rather than as text.
fn words(markdown: &str) -> BTreeSet<String> {
    let mut text = markdown.to_owned();
    for tag in [
        "<details>",
        "</details>",
        "<summary>",
        "</summary>",
        "<strong>",
        "</strong>",
    ] {
        text = text.replace(tag, " ");
    }
    let mut kept = String::new();
    let mut rest = text.as_str();
    while let Some(at) = rest.find("](") {
        kept.push_str(&rest[..at]);
        rest = &rest[at + 2..];
        rest = &rest[rest.find(')').map_or(rest.len(), |e| e + 1)..];
    }
    kept.push_str(rest);
    kept.split(|c: char| !c.is_ascii_alphabetic())
        .filter(|w| w.len() >= 2)
        .map(str::to_owned)
        .collect()
}

#[test]
fn every_ordinary_word_of_the_security_and_compliance_reports_is_in_the_pdf() {
    for (name, report) in reports() {
        // The line in security.md that names the PDF is about the PDF, so it is not in it; and where security.md
        // points to low-findings.md, the PDF points to its own last section.
        let security = sv_report::markdown::security(&report)
            .replace(&sv_report::pdf_line(&report), "")
            .replace(&format!("in `{}`", sv_report::LOW_FINDINGS_FILE), "");
        let wanted: BTreeSet<String> = words(&sv_report::markdown::compliance(&report))
            .union(&words(&security))
            .cloned()
            .collect();
        let read = back::read(&pdf::render(&report)).unwrap();
        let have = words(&read.text());
        let missing: Vec<&String> = wanted.difference(&have).collect();
        assert!(missing.is_empty(), "{name}: not in the PDF: {missing:?}");
        // The setup: a report of some length, with a finding written out in full.
        assert!(read.pages.len() > 10, "{name}: {} pages", read.pages.len());
    }
}

#[test]
fn nothing_runs_past_the_margin_on_either_paper() {
    for (name, report) in reports() {
        for paper in [Paper::Letter, Paper::A4] {
            let pdf = pdf::document(
                &sv_report::markdown::pdf_text(&report),
                "Health tracker",
                None,
                paper,
            );
            let read = back::read(&pdf).unwrap();
            let (w, h) = paper.size();
            let mut seen = 0;
            for (n, page) in read.pages.iter().enumerate() {
                assert_eq!(page.size, (w, h), "{name}: page {}", n + 1);
                for d in &page.drawn {
                    seen += 1;
                    let at = format!(
                        "{name}, {}, page {}: {:?} at {}",
                        paper.name(),
                        n + 1,
                        d.text,
                        d.x
                    );
                    assert!(d.x >= MARGIN - 0.01, "{at} starts in the left margin");
                    assert!(
                        d.x + d.width() <= w - MARGIN + 0.01,
                        "{at} runs into the right margin, {} wide",
                        d.width()
                    );
                    assert!(
                        d.y >= MARGIN - 0.01 && d.y <= h - MARGIN,
                        "{at} is off the page at {}",
                        d.y
                    );
                }
            }
            assert!(seen > 1000, "{name}: only {seen} strings were drawn");
        }
    }
}

#[test]
fn svs_own_words_use_only_characters_the_fonts_have() {
    // The example apps' own text is plain ASCII, so a character the fonts lack here is in `sv`'s words or its
    // data: found by this test, rather than printed as "?" in someone's report.
    for (name, report) in reports() {
        let text = sv_report::markdown::pdf_text(&report);
        let lacking: BTreeSet<char> = text.chars().filter(|c| !fonts::representable(*c)).collect();
        assert!(lacking.is_empty(), "{name}: the fonts lack {lacking:?}");
    }
    let plain =
        std::fs::read_to_string(sv_frameworks::data::file("plain-requirements.json")).unwrap();
    let lacking: BTreeSet<char> = plain
        .chars()
        .filter(|c| !fonts::representable(*c))
        .collect();
    assert!(
        lacking.is_empty(),
        "plain-requirements.json: the fonts lack {lacking:?}"
    );
}

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            copy(&path, &to.join(entry.file_name()));
        } else {
            std::fs::copy(&path, to.join(entry.file_name())).unwrap();
        }
    }
}

fn sv(args: &[&Path]) -> String {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(args)
        .output()
        .unwrap();
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.code().is_some_and(|c| c < 2), "{said}");
    said
}

#[test]
fn the_pdf_is_in_the_report_folder_and_in_the_bundle_and_report_paper_makes_it_a4() {
    let root = scratch("");
    let app = root.join("app");
    copy(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/partly-passing"),
        &app,
    );
    std::fs::remove_dir_all(app.join(sv_scan::ecosystems::DEFAULT_REPORT_DIR)).ok();

    sv(&[Path::new("report"), &app, Path::new("--no-tools")]);
    let folder = app.join(sv_scan::ecosystems::DEFAULT_REPORT_DIR);
    let pdf_text = std::fs::read_to_string(folder.join(pdf::FILE)).expect("report.pdf is written");
    let read = back::read(&pdf_text).unwrap();
    assert_eq!(
        read.pages[0].size,
        Paper::Letter.size(),
        "US Letter unless asked"
    );
    assert!(
        read.text()
            .contains("report.html, beside it, is the complete version")
    );
    // Named in the reports that list what the folder holds.
    for named_in in ["report.html", "security.md"] {
        let text = std::fs::read_to_string(folder.join(named_in)).unwrap();
        assert!(
            text.contains("report.pdf"),
            "{named_in} does not name the PDF"
        );
    }

    let zip = root.join("bundle.zip");
    sv(&[
        Path::new("bundle"),
        &app,
        Path::new("--no-tools"),
        Path::new("--out"),
        &zip,
    ]);
    let bytes = std::fs::read(&zip).unwrap();
    let name = b"/report/report.pdf";
    assert!(
        bytes.windows(name.len()).any(|w| w == name),
        "the bundle holds no report.pdf"
    );

    let manifest = app.join("stackvet.toml");
    let toml = std::fs::read_to_string(&manifest).unwrap();
    assert!(toml.contains("[app]\n"), "the setup");
    std::fs::write(
        &manifest,
        toml.replacen("[app]\n", "[app]\nreport-paper = \"a4\"\n", 1),
    )
    .unwrap();
    sv(&[Path::new("report"), &app, Path::new("--no-tools")]);
    let read = back::read(&std::fs::read_to_string(folder.join(pdf::FILE)).unwrap()).unwrap();
    assert_eq!(
        read.pages[0].size,
        Paper::A4.size(),
        "report-paper = \"a4\" was not followed"
    );
    std::fs::remove_dir_all(&root).ok();
}
