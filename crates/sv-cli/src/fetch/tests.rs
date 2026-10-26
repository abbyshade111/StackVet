//! A project's address read, the download held to its hosts, and its archive unpacked as data only (backlog 0670,
//! ADR-154). Nothing here reaches the network: a stand-in answers for the host, and each archive is made by the test.

use super::*;
use std::io::Write;
use sv_check::test_support::scratch;
use zip::write::SimpleFileOptions;

const SHA: &str = "0123456789abcdef0123456789abcdef01234567";

fn read(text: &str) -> Address {
    read_address(text).unwrap_or_else(|e| panic!("{text}: {e:#}"))
}

fn refused(text: &str) -> String {
    match read_address(text) {
        Ok(a) => panic!("{text} was read, as {a:?}"),
        Err(e) => format!("{e:#}"),
    }
}

#[test]
fn a_github_address_is_read_with_its_branch_or_commit() {
    let plain = read("https://github.com/owner/project");
    assert_eq!(plain.host, Host::GitHub);
    assert_eq!(plain.path, "owner/project");
    assert_eq!(plain.reference, None);
    assert_eq!(
        plain.archive(),
        "https://github.com/owner/project/archive/HEAD.zip"
    );
    for same in [
        "https://github.com/owner/project/",
        "https://github.com/owner/project.git",
        "https://www.github.com/owner/project?tab=readme-ov-file#readme",
    ] {
        assert_eq!(read(same), plain, "{same}");
    }
    let branch = read("https://github.com/owner/project/tree/feature/new-login");
    assert_eq!(branch.reference.as_deref(), Some("feature/new-login"));
    assert_eq!(
        branch.archive(),
        "https://github.com/owner/project/archive/feature/new-login.zip"
    );
    assert_eq!(
        branch.shown(),
        "https://github.com/owner/project/tree/feature/new-login"
    );
    let commit = read(&format!("https://github.com/owner/project/commit/{SHA}"));
    assert_eq!(commit.reference.as_deref(), Some(SHA));
}

#[test]
fn a_gitlab_address_is_read_with_its_groups_and_branch() {
    let nested = read("https://gitlab.com/group/subgroup/project");
    assert_eq!(nested.host, Host::GitLab);
    assert_eq!(nested.path, "group/subgroup/project");
    assert_eq!(
        nested.archive(),
        "https://gitlab.com/api/v4/projects/group%2Fsubgroup%2Fproject/repository/archive.zip"
    );
    let branch = read("https://gitlab.com/group/project/-/tree/release/2.0");
    assert_eq!(branch.path, "group/project");
    assert_eq!(branch.reference.as_deref(), Some("release/2.0"));
    assert_eq!(
        branch.archive(),
        "https://gitlab.com/api/v4/projects/group%2Fproject/repository/archive.zip?sha=release%2F2.0"
    );
    assert_eq!(
        branch.shown(),
        "https://gitlab.com/group/project/-/tree/release/2.0"
    );
}

#[test]
fn any_other_address_is_refused_with_what_was_expected() {
    assert!(refused("http://github.com/owner/project").contains("only an https://"));
    for other in [
        "https://bitbucket.org/owner/project",
        "https://github.com.example.net/owner/project",
        "https://user@github.com/owner/project",
        "https://github.com:8443/owner/project",
    ] {
        assert!(refused(other).contains("GitHub and GitLab only"), "{other}");
    }
    for no_project in [
        "https://github.com/owner",
        "https://gitlab.com/group/-/tree/main",
    ] {
        assert!(
            refused(no_project).contains("names no project"),
            "{no_project}"
        );
    }
    for page in [
        "https://github.com/owner/project/issues",
        "https://github.com/owner/project/blob/main/README.md",
        "https://gitlab.com/group/project/-/issues",
    ] {
        assert!(refused(page).contains("a page within a project"), "{page}");
    }
    for name in [
        "https://github.com/owner/pro%20ject",
        "https://github.com/../project",
        "https://github.com/owner/project/tree/a..b",
        "https://github.com/owner/project/tree/--upload-pack=x",
    ] {
        refused(name);
    }
    assert!(looks_like_address("https://github.com/owner/project"));
    assert!(!looks_like_address("my-app"));
}

#[test]
fn a_download_goes_only_to_the_hosts_named() {
    for good in [
        "https://github.com/o/p/archive/HEAD.zip",
        "https://codeload.github.com/o/p/zip/refs/heads/main",
        "https://gitlab.com/api/v4/projects/o%2Fp/repository/archive.zip",
    ] {
        allowed(good).unwrap_or_else(|e| panic!("{good}: {e:#}"));
    }
    for bad in [
        "http://codeload.github.com/o/p/zip/main",
        "https://objects.example.net/o/p.zip",
        "https://github.com@example.net/o/p.zip",
        "https://codeload.github.com:444/o/p.zip",
        "https://github.com.example.net/o/p.zip",
        "file:///etc/passwd",
    ] {
        assert!(allowed(bad).is_err(), "{bad}");
    }
}

/// A host answering each request in turn from `answers`, writing `archive` to the file asked for when it answers with
/// the archive, and keeping every address it was asked.
struct StandIn {
    answers: Vec<Answer>,
    archive: Vec<u8>,
    asked: Vec<String>,
}

impl StandIn {
    fn new(answers: Vec<Answer>, archive: Vec<u8>) -> Self {
        StandIn {
            answers,
            archive,
            asked: Vec::new(),
        }
    }
}

impl Download for StandIn {
    fn get(&mut self, url: &str, to: &Path) -> Result<Answer> {
        self.asked.push(url.to_owned());
        let answer = self.answers.remove(0);
        if answer == Answer::Saved {
            std::fs::write(to, &self.archive)?;
        }
        Ok(answer)
    }
}

fn github() -> Address {
    read("https://github.com/owner/project")
}

#[test]
fn a_redirect_to_the_download_host_is_followed_and_any_other_is_not() {
    let to = scratch("redirect").join("a.zip");
    let codeload = "https://codeload.github.com/owner/project/zip/refs/heads/main";
    let mut host = StandIn::new(
        vec![Answer::Redirect(codeload.to_owned()), Answer::Saved],
        b"zip".to_vec(),
    );
    download(&github(), &mut host, &to).unwrap();
    assert_eq!(
        host.asked,
        [
            "https://github.com/owner/project/archive/HEAD.zip",
            codeload
        ]
    );

    let mut away = StandIn::new(
        vec![
            Answer::Redirect("https://objects.example.net/p.zip".to_owned()),
            Answer::Saved,
        ],
        Vec::new(),
    );
    let e = download(&github(), &mut away, &to).unwrap_err();
    assert!(format!("{e:#}").contains("not followed"), "{e:#}");
    assert_eq!(away.asked.len(), 1, "the address refused was never asked");

    let round = Answer::Redirect("https://github.com/owner/project/archive/HEAD.zip".to_owned());
    let mut circling = StandIn::new(vec![round; MOST_REDIRECTS + 2], Vec::new());
    let e = download(&github(), &mut circling, &to).unwrap_err();
    assert!(format!("{e:#}").contains("more than 3 times"), "{e:#}");
    assert_eq!(circling.asked.len(), MOST_REDIRECTS + 1);
}

#[test]
fn curl_is_held_to_https_follows_nothing_itself_and_has_its_limits() {
    let args = Curl::args(Path::new("out.zip"));
    let after = |flag: &str| {
        let at = args
            .iter()
            .position(|a| a == flag)
            .unwrap_or_else(|| panic!("{flag}: {args:?}"));
        args[at + 1].clone()
    };
    assert_eq!(args[0], "--disable", "first, the only place curl reads it");
    assert_eq!(after("--proto"), "=https");
    assert_eq!(after("--max-redirs"), "0");
    assert_eq!(after("--max-filesize"), MOST_DOWNLOAD_BYTES.to_string());
    assert_eq!(after("--max-time"), MOST_SECONDS.to_string());
    assert_eq!(after("--output"), "out.zip");
    for never in ["--location", "-L", "--insecure", "-k", "--user", "--header"] {
        assert!(!args.iter().any(|a| a == never), "{never}: {args:?}");
    }
}

/// An archive of `entries` (a name and its text, a name ending `/` for a folder), with `comment`.
fn archive(entries: &[(&str, &str)], comment: &str) -> Vec<u8> {
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let stored = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (name, text) in entries {
        if name.ends_with('/') {
            zip.add_directory(*name, stored).unwrap();
        } else {
            zip.start_file(*name, stored.unix_permissions(0o755))
                .unwrap();
            zip.write_all(text.as_bytes()).unwrap();
        }
    }
    zip.set_comment(comment).unwrap();
    zip.finish().unwrap().into_inner()
}

fn unpacked(bytes: &[u8], limits: Limits) -> (PathBuf, Result<Unpacked>) {
    let dir = scratch("unpack");
    let zip = dir.join("a.zip");
    std::fs::write(&zip, bytes).unwrap();
    let into = dir.join("into");
    std::fs::create_dir(&into).unwrap();
    let result = unpack(&zip, &into, limits);
    (into, result)
}

#[test]
fn an_archive_unpacks_without_its_top_folder_and_names_its_commit() {
    let bytes = archive(
        &[
            ("project-main/", ""),
            ("project-main/README.md", "# A project\n"),
            ("project-main/src/app.py", "print('hi')\n"),
        ],
        SHA,
    );
    let (into, result) = unpacked(&bytes, LIMITS);
    let done = result.unwrap();
    assert_eq!(done.commit.as_deref(), Some(SHA));
    assert_eq!(done.files, 2);
    assert!(done.left_out.is_empty(), "{:?}", done.left_out);
    assert_eq!(
        std::fs::read_to_string(into.join("src/app.py")).unwrap(),
        "print('hi')\n"
    );
    assert!(!into.join("project-main").exists());

    // GitLab's archive names the commit in its top folder.
    let gitlab = archive(&[(&format!("project-main-{SHA}/a.txt"), "a")], "");
    let (_, result) = unpacked(&gitlab, LIMITS);
    assert_eq!(result.unwrap().commit.as_deref(), Some(SHA));

    // An archive that says neither leaves it unsaid.
    let neither = archive(&[("a.txt", "a"), ("b.txt", "b")], "made by hand");
    let (into, result) = unpacked(&neither, LIMITS);
    let done = result.unwrap();
    assert_eq!(done.commit, None);
    assert!(
        into.join("a.txt").is_file(),
        "no one top folder, so none dropped"
    );
}

#[test]
fn a_link_and_a_name_outside_the_folder_are_left_out_and_said() {
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let stored = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (name, text) in [
        ("p/ok.txt", "fine"),
        ("p/../escaped.txt", "out"),
        ("/p/absolute.txt", "out"),
        ("p/a\\..\\..\\back.txt", "out"),
        ("p/C:drive.txt", "out"),
    ] {
        zip.start_file(name, stored).unwrap();
        zip.write_all(text.as_bytes()).unwrap();
    }
    zip.add_symlink("p/link", "/etc/passwd", stored).unwrap();
    let bytes = zip.finish().unwrap().into_inner();
    let (into, result) = unpacked(&bytes, LIMITS);
    let done = result.unwrap();
    assert_eq!(done.files, 1, "{:?}", done.left_out);
    assert_eq!(done.left_out.len(), 5, "{:?}", done.left_out);
    assert!(
        done.left_out
            .iter()
            .any(|l| l.starts_with("p/link: a link")),
        "{:?}",
        done.left_out
    );
    assert_eq!(
        done.left_out
            .iter()
            .filter(|l| l.contains("outside the folder"))
            .count(),
        4,
        "{:?}",
        done.left_out
    );
    let parent = into.parent().unwrap();
    assert!(!parent.join("escaped.txt").exists());
    assert!(!into.join("link").exists());
    assert!(std::fs::symlink_metadata(into.join("link")).is_err());
    // Nothing was written anywhere but the one file.
    let written: Vec<_> = std::fs::read_dir(&into).unwrap().collect();
    assert_eq!(written.len(), 1, "{written:?}");
}

#[test]
fn a_second_entry_for_a_file_already_written_is_left_out_not_written_over() {
    let bytes = archive(&[("p/a.txt", "first"), ("p/./a.txt", "second")], SHA);
    let (into, result) = unpacked(&bytes, LIMITS);
    let done = result.unwrap();
    assert_eq!(done.files, 1);
    assert_eq!(
        std::fs::read_to_string(into.join("a.txt")).unwrap(),
        "first"
    );
    assert!(
        done.left_out[0].contains("same name on this computer"),
        "{:?}",
        done.left_out
    );
}

#[cfg(unix)]
#[test]
fn a_hook_in_the_archive_is_written_as_data_and_never_runs() {
    use std::os::unix::fs::PermissionsExt;
    let marker = scratch("hook-marker").join("ran");
    let hook = format!("#!/bin/sh\ntouch '{}'\n", marker.display());
    let bytes = archive(
        &[
            ("p/.git/hooks/post-checkout", &hook),
            (
                "p/package.json",
                "{\"scripts\":{\"postinstall\":\"sh .git/hooks/post-checkout\"}}",
            ),
        ],
        SHA,
    );
    let (into, result) = unpacked(&bytes, LIMITS);
    assert_eq!(result.unwrap().files, 2);
    let written = into.join(".git/hooks/post-checkout");
    assert_eq!(std::fs::read_to_string(&written).unwrap(), hook);
    let mode = std::fs::metadata(&written).unwrap().permissions().mode();
    assert_eq!(
        mode & 0o111,
        0,
        "marked 755 in the archive, written {mode:o}"
    );
    assert!(!marker.exists(), "the hook ran");
}

#[test]
fn unpacking_stops_at_its_limits() {
    let many = archive(&[("p/a", "1"), ("p/b", "2"), ("p/c", "3")], SHA);
    let (_, result) = unpacked(
        &many,
        Limits {
            files: 2,
            bytes: 1024,
        },
    );
    let e = format!("{:#}", result.unwrap_err());
    assert!(e.contains("more than 2 files"), "{e}");

    let large = archive(&[("p/a", &"x".repeat(600)), ("p/b", &"y".repeat(600))], SHA);
    let (_, result) = unpacked(
        &large,
        Limits {
            files: 10,
            bytes: 1000,
        },
    );
    let e = format!("{:#}", result.unwrap_err());
    assert!(e.contains("unpacks to more than"), "{e}");

    // At the limits exactly, it unpacks.
    let (_, result) = unpacked(
        &large,
        Limits {
            files: 2,
            bytes: 1200,
        },
    );
    assert_eq!(result.unwrap().files, 2);
}

#[test]
fn a_fetch_is_named_after_its_commit_and_never_over_an_earlier_one() {
    let base = scratch("fetch");
    let bytes = archive(&[("project-main/app.py", "x = 1\n")], SHA);
    let mut host = StandIn::new(vec![Answer::Saved, Answer::Saved], bytes);
    let first = fetch(&github(), &base, &mut host).unwrap();
    let parent = base.join("stackvet-fetched/github.com/owner");
    assert_eq!(first.folder, parent.join("project-0123456789ab"));
    assert!(first.folder.join("app.py").is_file());
    assert!(first.manifest_written);
    assert_eq!(
        std::fs::read_to_string(first.folder.join("stackvet.toml")).unwrap(),
        NOBODY_ANSWERED
    );
    std::fs::write(first.folder.join("app.py"), "changed by the person\n").unwrap();
    let second = fetch(&github(), &base, &mut host).unwrap();
    assert_eq!(second.folder, parent.join("project-0123456789ab-2"));
    assert_eq!(
        std::fs::read_to_string(first.folder.join("app.py")).unwrap(),
        "changed by the person\n"
    );
    let left: Vec<String> = std::fs::read_dir(&parent)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        left.len(),
        2,
        "only the two projects, no download or unpacking left: {left:?}"
    );

    // A project's own stackvet.toml is kept as it is.
    let own = archive(
        &[("project-main/stackvet.toml", "[app]\nname = \"its own\"\n")],
        "1123456789abcdef0123456789abcdef01234567",
    );
    let mut host = StandIn::new(vec![Answer::Saved], own);
    let third = fetch(&github(), &base, &mut host).unwrap();
    assert!(!third.manifest_written);
    assert_eq!(
        std::fs::read_to_string(third.folder.join("stackvet.toml")).unwrap(),
        "[app]\nname = \"its own\"\n"
    );

    // A download that is not an archive leaves nothing behind.
    let mut broken = StandIn::new(vec![Answer::Saved], b"<html>not found</html>".to_vec());
    let e = fetch(&github(), &base, &mut broken).unwrap_err();
    assert!(format!("{e:#}").contains("not a zip archive"), "{e:#}");
    assert_eq!(std::fs::read_dir(&parent).unwrap().count(), 3);
}
