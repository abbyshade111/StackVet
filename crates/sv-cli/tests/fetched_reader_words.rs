//! A report on a fetched project with no stackvet.toml of its own, read page by page, says no answer was given that
//! nobody gave, and none of the words meant for an app's owner (backlog 0677); the same code reported as the person's
//! own app keeps them. A `curl` of the test's own, named by `SV_CURL_FOR_TEST`, answers as GitHub does.

#![cfg(unix)]

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use sv_check::test_support::scratch;

const SHA: &str = "0123456789abcdef0123456789abcdef01234567";

/// A small web app with a sign-in form, so the report has questions only a person can answer.
const FILES: [(&str, &str); 2] = [
    (
        "app.py",
        "from flask import Flask, request\napp = Flask(__name__)\n\n@app.route('/login', methods=['POST'])\n\
         def login():\n    return request.form['password']\n",
    ),
    ("requirements.txt", "flask==3.0.0\n"),
];

/// A `curl` that answers the archive's address with an archive of `FILES`, as GitHub's download host would.
fn fake_curl(dir: &Path) -> PathBuf {
    let zip = dir.join("made.zip");
    let mut writer = zip::ZipWriter::new(std::fs::File::create(&zip).unwrap());
    let stored =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (name, text) in FILES {
        writer
            .start_file(format!("project-main/{name}"), stored)
            .unwrap();
        writer.write_all(text.as_bytes()).unwrap();
    }
    writer.set_comment(SHA).unwrap();
    writer.finish().unwrap();
    let script = format!(
        "#!/bin/sh\nout=''; prev=''\n\
         for a in \"$@\"; do [ \"$prev\" = --output ] && out=\"$a\"; prev=\"$a\"; url=\"$a\"; done\n\
         case \"$url\" in\n\
           https://github.com/*) printf '302 https://codeload.github.com/owner/project/zip/HEAD';;\n\
           https://codeload.github.com/*) cat '{zip}' > \"$out\"; printf '200 ';;\n\
           *) printf '500 ';;\n\
         esac\n",
        zip = zip.display(),
    );
    let curl = dir.join("curl");
    sv_check::test_support::executable(&curl, &script);
    curl
}

/// `sv report` on `target`, run in `dir`, and every page it wrote into `report_dir`, by name.
fn report(dir: &Path, target: &str, report_dir: &Path) -> Vec<(String, String)> {
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["report", target, "--no-tools", "--quiet"])
        .current_dir(dir)
        .env("SV_CURL_FOR_TEST", fake_curl(dir))
        .output()
        .expect("sv runs");
    assert!(
        out.status.code().is_some_and(|c| c <= 1),
        "the report was written: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let mut pages: Vec<(String, String)> = std::fs::read_dir(report_dir)
        .expect("a report folder was written")
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "md" || x == "html"))
        .map(|p| {
            let text = std::fs::read_to_string(&p).unwrap();
            // The HTML page's apostrophes and backticks, as the Markdown writes them.
            let text = text
                .replace("&#39;", "'")
                .replace("&#x27;", "'")
                .replace("<code>", "`")
                .replace("</code>", "`");
            (p.file_name().unwrap().to_string_lossy().into_owned(), text)
        })
        .collect();
    pages.sort();
    assert!(pages.len() >= 9, "every page was written: {pages:?}");
    pages
}

/// Words a report on the person's own app says that are not the reader's to act on, or not true, of somebody else's
/// project.
const THE_OWNER_S: &[&str] = &[
    "not recorded, so read as your AI coding tool's",
    "ask your AI coding tool to write one",
    "given to your AI coding tool as a rule",
    "tell your AI coding tool your answer",
    "or with your AI coding tool, and write the report again",
    "`sv questions` prints them for its chat",
    "**For:** You or your AI coding tool",
    "**For:** Whoever fixes the code",
    "Tick a line in `sv review`",
    "set it aside in `sv review` as a risk you accept",
];

/// What a fetched project's pages say in their place, each on some page: the setup reached every passage.
const THE_READER_S: &[&str] = &[
    "nothing, taken as yes",
    "nobody: sv's default",
    "a test of the project's own could, which its author would write",
    "the project's author's to do, or yours if you put the project on the internet yourself",
    "Only a person can settle these, and for this project that is its author",
    "**For:** You, first: the risks to weigh",
];

#[test]
fn no_page_of_a_fetched_project_s_report_speaks_to_its_owner() {
    let dir = scratch("fetched-reader-words");
    let project = dir.join("stackvet-fetched/github.com/owner/project-0123456789ab");
    let pages = report(
        &dir,
        "https://github.com/owner/project",
        &project.join("stackvet-report"),
    );
    for (name, text) in &pages {
        for owners in THE_OWNER_S {
            assert!(!text.contains(owners), "{name} says {owners:?}");
        }
    }
    let all: String = pages.iter().map(|(_, t)| t.as_str()).collect();
    for readers in THE_READER_S {
        assert!(all.contains(readers), "no page says {readers:?}");
    }
    // The answers nobody gave are named as sv's defaults, not taken on the manifest's word.
    let compliance = &pages.iter().find(|(n, _)| n == "compliance.md").unwrap().1;
    for row in ["| tls |", "| internet |", "| level2 |"] {
        let line = compliance
            .lines()
            .find(|l| l.starts_with(row))
            .unwrap_or_else(|| panic!("compliance.md has a {row} row"));
        assert!(line.contains("nobody: sv's default"), "{line}");
        assert!(!line.contains("manifest's word"), "{line}");
    }

    // The same code, reported as the person's own app, keeps the owner's words: they are there to leave out.
    let mine = dir.join("mine");
    std::fs::create_dir_all(&mine).unwrap();
    for entry in std::fs::read_dir(&project).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            std::fs::copy(&path, mine.join(path.file_name().unwrap())).unwrap();
        }
    }
    let own = report(&dir, "mine", &mine.join("stackvet-report"));
    let all: String = own.iter().map(|(_, t)| t.as_str()).collect();
    assert!(
        !all.contains("somebody else's project"),
        "the setup: an app of the person's own"
    );
    for owners in [
        "ask your AI coding tool to write one",
        "given to your AI coding tool as a rule",
        "tell your AI coding tool your answer",
        "**For:** You or your AI coding tool",
        "Tick a line in `sv review`",
    ] {
        assert!(
            all.contains(owners),
            "the owner's report does not say {owners:?}"
        );
    }
    // Its unanswered tls, internet, and level are still sv's defaults, said as such.
    assert!(all.contains("nobody: sv's default"));
}
