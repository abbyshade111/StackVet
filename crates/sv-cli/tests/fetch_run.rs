//! `sv report <address> --run` starts a fetched project only when it says how, and downloads its packages only with
//! `--install` (backlog 0670, part 3; ADR-154). A `curl` of the test's own, named by `SV_CURL_FOR_TEST`, answers as
//! GitHub does, with an archive the test makes; every refusal here comes before a container backend is asked for.

#![cfg(unix)]

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use sv_check::test_support::scratch;

const SHA: &str = "0123456789abcdef0123456789abcdef01234567";

/// A `curl` that answers the archive's address with the archive of `entries`, as GitHub's download host would.
fn fake_curl(dir: &Path, entries: &[(&str, &str)]) -> PathBuf {
    let zip = dir.join("made.zip");
    let mut writer = zip::ZipWriter::new(std::fs::File::create(&zip).unwrap());
    let stored =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (name, text) in entries {
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

fn sv(dir: &Path, curl: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .arg("report")
        .args(args)
        .args(["--no-tools", "--quiet"])
        .current_dir(dir)
        .env("SV_CURL_FOR_TEST", curl)
        .output()
        .expect("sv runs")
}

fn folder(dir: &Path) -> PathBuf {
    dir.join("stackvet-fetched/github.com/owner/project-0123456789ab")
}

/// What the report written for the fetched project says, in compliance.md.
fn compliance(dir: &Path) -> String {
    std::fs::read_to_string(folder(dir).join("stackvet-report/compliance.md"))
        .expect("a report was written")
}

const ADDRESS: &str = "https://github.com/owner/project";
const APP: (&str, &str) = ("app.py", "print('hi')\n");

#[test]
fn a_project_with_no_stackvet_toml_of_its_own_is_read_and_never_started() {
    let dir = scratch("no-manifest");
    let curl = fake_curl(&dir, &[APP]);
    let out = sv(&dir, &curl, &[ADDRESS, "--run"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(2),
        "asked to run, and it did not: {stderr}"
    );
    let text = compliance(&dir);
    assert!(
        text.contains("carries no stackvet.toml of its own, so nothing says how to start it"),
        "{text}"
    );
    assert!(!text.contains("Code nobody vetted was run"));
}

#[test]
fn the_authors_install_needs_the_readers_install() {
    let dir = scratch("install");
    let manifest = "manifest-version = 1\n[app]\nname = \"its own\"\n[stack.run]\n\
                    image = \"python:3.12-slim\"\ninstall = true\nstart = \"python app.py\"\n";
    let curl = fake_curl(&dir, &[APP, ("stackvet.toml", manifest)]);
    let out = sv(&dir, &curl, &[ADDRESS, "--run"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(2), "{stderr}");
    let text = compliance(&dir);
    assert!(
        text.contains("which is its author's word, not yours"),
        "{text}"
    );
    assert!(text.contains("add --install"), "{text}");
}

#[test]
fn install_goes_only_with_run_and_only_for_a_fetched_project() {
    let dir = scratch("install-alone");
    let curl = fake_curl(&dir, &[APP]);
    let out = sv(&dir, &curl, &[ADDRESS, "--install"]);
    assert_eq!(out.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&out.stderr).contains("so it goes with --run"));
    assert!(
        !dir.join("stackvet-fetched").exists(),
        "nothing was fetched"
    );

    let mine = scratch("own-app");
    std::fs::write(mine.join("app.py"), "print('hi')\n").unwrap();
    std::fs::write(
        mine.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"mine\"\n",
    )
    .unwrap();
    let out = sv(&mine, &curl, &[".", "--run", "--install"]);
    assert_eq!(out.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("is for a project fetched from its address"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
