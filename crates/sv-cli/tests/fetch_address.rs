//! `sv report` given a public project's address fetches it through `curl` and checks it (backlog 0670, ADR-154).
//!
//! A `curl` of the test's own, named by `SV_CURL_FOR_TEST` (a debug build's switch), answers as GitHub does: the
//! archive's address sends the download to `codeload.github.com`, which answers with an archive the test made. It
//! writes down what it was asked, so the test sees every request `sv` made, and that none went anywhere it refused.

#![cfg(unix)]

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use sv_check::test_support::scratch;

const SHA: &str = "0123456789abcdef0123456789abcdef01234567";

/// A small project's archive as GitHub makes one: one top folder, and the commit in the archive's comment.
fn archive(to: &Path) {
    let mut zip = zip::ZipWriter::new(std::fs::File::create(to).unwrap());
    let stored =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (name, text) in [
        (
            "project-main/app.py",
            "import os\nprint(os.environ.get('NAME'))\n",
        ),
        ("project-main/requirements.txt", "flask==3.0.0\n"),
    ] {
        zip.start_file(name, stored).unwrap();
        zip.write_all(text.as_bytes()).unwrap();
    }
    zip.set_comment(SHA).unwrap();
    zip.finish().unwrap();
}

/// A `curl` that writes its arguments, one per line and each call ended by `--`, to `asked`, and answers the archive's
/// address on github.com with a redirect to `redirect`, and codeload.github.com with the archive.
fn fake_curl(dir: &Path, redirect: &str) -> (PathBuf, PathBuf) {
    let zip = dir.join("made.zip");
    archive(&zip);
    let asked = dir.join("asked");
    let script = format!(
        "#!/bin/sh\nfor a in \"$@\"; do printf '%s\\n' \"$a\" >> '{asked}'; done\necho -- >> '{asked}'\n\
         out=''; prev=''\n\
         for a in \"$@\"; do [ \"$prev\" = --output ] && out=\"$a\"; prev=\"$a\"; url=\"$a\"; done\n\
         case \"$url\" in\n\
           https://github.com/*) printf '302 %s' '{redirect}';;\n\
           https://codeload.github.com/*) cat '{zip}' > \"$out\"; printf '200 ';;\n\
           *) printf '500 ';;\n\
         esac\n",
        asked = asked.display(),
        zip = zip.display(),
    );
    let curl = dir.join("curl");
    sv_check::test_support::executable(&curl, &script);
    (curl, asked)
}

fn report(dir: &Path, curl: &Path, extra: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .args([
            "report",
            "https://github.com/owner/project",
            "--no-tools",
            "--quiet",
        ])
        .args(extra)
        .current_dir(dir)
        .env("SV_CURL_FOR_TEST", curl)
        .output()
        .expect("sv runs")
}

/// Each call `curl` was asked to make, as its arguments.
fn calls(asked: &Path) -> Vec<Vec<String>> {
    let text = std::fs::read_to_string(asked).unwrap_or_default();
    text.split("--\n")
        .filter(|c| !c.trim().is_empty())
        .map(|c| c.lines().map(str::to_owned).collect())
        .collect()
}

#[test]
fn an_address_is_fetched_once_into_a_folder_of_its_own_and_checked() {
    let dir = scratch("fetched");
    let (curl, asked) = fake_curl(
        &dir,
        "https://codeload.github.com/owner/project/zip/refs/heads/HEAD",
    );
    let out = report(&dir, &curl, &[]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let folder = dir.join("stackvet-fetched/github.com/owner/project-0123456789ab");
    assert!(
        folder.join("app.py").is_file(),
        "the project was not unpacked where it is named: {stderr}"
    );
    assert!(
        folder.join("stackvet-report/report.html").is_file(),
        "no report was written for it: {stderr}"
    );
    assert!(stderr.contains(&format!("commit {SHA}")), "{stderr}");
    assert!(
        stderr.contains("stays for you to read or delete"),
        "{stderr}"
    );
    assert!(stderr.contains("carried no stackvet.toml"), "{stderr}");
    // The report is of somebody else's project, and says so with its address and commit (part 2).
    let html = std::fs::read_to_string(folder.join("stackvet-report/report.html")).unwrap();
    assert!(
        html.contains("somebody else&#39;s project"),
        "report.html does not say whose"
    );
    assert!(html.contains("https://github.com/owner/project"));
    assert!(html.contains(SHA));
    let json = std::fs::read_to_string(folder.join("stackvet-report/report.json")).unwrap();
    assert!(
        json.contains("\"fetched\""),
        "report.json does not record it"
    );

    let calls = calls(&asked);
    assert_eq!(calls.len(), 2, "{calls:?}");
    assert_eq!(
        calls[0].last().map(String::as_str),
        Some("https://github.com/owner/project/archive/HEAD.zip")
    );
    assert_eq!(
        calls[1].last().map(String::as_str),
        Some("https://codeload.github.com/owner/project/zip/refs/heads/HEAD")
    );
    for call in &calls {
        let at = call.iter().position(|a| a == "--proto").expect("--proto");
        assert_eq!(call[at + 1], "=https");
        assert!(!call.iter().any(|a| a == "--location"), "{call:?}");
    }
    // Nothing of sv's own download or unpacking is left beside it.
    let beside: Vec<String> = std::fs::read_dir(folder.parent().unwrap())
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    let mut beside = beside;
    beside.sort();
    assert_eq!(
        beside,
        ["project-0123456789ab", "project-0123456789ab.fetched.json"]
    );
}

#[test]
fn a_redirect_anywhere_else_is_refused_before_it_is_asked() {
    let dir = scratch("away");
    let (curl, asked) = fake_curl(&dir, "https://objects.example.net/project.zip");
    let out = report(&dir, &curl, &[]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(3), "{stderr}");
    assert!(stderr.contains("not followed"), "{stderr}");
    let calls = calls(&asked);
    assert_eq!(calls.len(), 1, "only the archive's own address: {calls:?}");
    let parent = dir.join("stackvet-fetched/github.com/owner");
    assert_eq!(
        std::fs::read_dir(&parent).unwrap().count(),
        0,
        "nothing was left in {}",
        parent.display()
    );
}

#[test]
fn an_address_on_any_other_host_is_refused_before_curl_is_asked() {
    let dir = scratch("other");
    let (curl, asked) = fake_curl(&dir, "https://codeload.github.com/owner/project/zip/main");
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args([
            "report",
            "https://bitbucket.org/owner/project",
            "--no-tools",
        ])
        .current_dir(&dir)
        .env("SV_CURL_FOR_TEST", &curl)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(3), "{stderr}");
    assert!(stderr.contains("GitHub and GitLab only"), "{stderr}");
    assert!(calls(&asked).is_empty());
}
