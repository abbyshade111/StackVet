//! `tools/framework_pages.py`, stackvet.dev's page for each framework (backlog 0619, ADR-129).
//!
//! A page that says StackVet checks more than `docs/COVERAGE.md` does, leaves a requirement out, fetches something
//! from elsewhere, or writes over a folder it did not make, fails here.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use sv_frameworks::paths::Canonical;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonical()
        .unwrap()
}

fn script(args: &[&str]) -> Output {
    Command::new("python3")
        .arg("-I")
        .arg(repo().join("tools/framework_pages.py"))
        .args(args)
        .output()
        .expect("python3 runs")
}

fn said(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-framework-pages-{tag}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    dir
}

/// Every requirement id in one of `data/frameworks/`'s files, under its list's key.
fn ids(file: &str) -> Vec<String> {
    fn walk(v: &serde_json::Value, out: &mut Vec<String>) {
        match v {
            serde_json::Value::Object(m) => {
                for key in ["requirements", "controls"] {
                    for r in m.get(key).and_then(|r| r.as_array()).into_iter().flatten() {
                        out.push(r["id"].as_str().unwrap().to_owned());
                    }
                }
                for (k, v) in m {
                    if k != "requirements" && k != "controls" {
                        walk(v, out);
                    }
                }
            }
            serde_json::Value::Array(a) => a.iter().for_each(|v| walk(v, out)),
            _ => {}
        }
    }
    let data: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(repo().join("data/frameworks").join(file)).unwrap(),
    )
    .unwrap();
    let mut out = Vec::new();
    walk(&data, &mut out);
    out
}

/// The numbers on a page: `data-count="settle"` and the like, in its counts table.
fn count(page: &str, what: &str) -> usize {
    let marker = format!("data-count=\"{what}\">");
    let at = page
        .find(&marker)
        .unwrap_or_else(|| panic!("no count {what}"))
        + marker.len();
    page[at..].split('<').next().unwrap().parse().unwrap()
}

/// `docs/COVERAGE.md`'s summary row for a framework: requirements, can settle, can be credited, supporting only,
/// nothing.
fn summary(name: &str) -> [usize; 5] {
    let doc = std::fs::read_to_string(repo().join("docs/COVERAGE.md")).unwrap();
    let row = doc
        .lines()
        .find(|l| l.starts_with(&format!("| {name} |")))
        .unwrap_or_else(|| panic!("no summary row for {name}"));
    let cells: Vec<usize> = row
        .split('|')
        .skip(2)
        .filter(|c| !c.trim().is_empty())
        .map(|c| c.split_whitespace().next().unwrap().parse().unwrap())
        .collect();
    cells.try_into().unwrap()
}

#[test]
fn its_small_pieces_pass_their_own_checks() {
    let out = script(&["--self-test"]);
    assert!(out.status.success(), "{}", said(&out));
    assert!(
        said(&out).contains("framework_pages self-test: ok"),
        "{}",
        said(&out)
    );
}

#[test]
fn every_requirement_is_on_its_page_and_the_counts_are_coverage_md_s() {
    let dir = scratch("built");
    let out = script(&["--out", dir.to_str().unwrap()]);
    assert!(out.status.success(), "{}", said(&out));
    let read = |f: &str| std::fs::read_to_string(dir.join(f)).unwrap();

    for (page, files) in [
        ("asvs.html", vec!["asvs-5.0.0.json"]),
        (
            "aisvs.html",
            vec!["aisvs-1.0.json", "aisvs-1.0-appendix-c.json"],
        ),
        ("secure-by-design.html", vec!["sbd-checklist-0.5.0.json"]),
    ] {
        let text = read(page);
        let wanted: Vec<String> = files
            .iter()
            .flat_map(|f| ids(f))
            .map(|id| {
                if page.starts_with("secure") {
                    format!("SBD-{id}")
                } else {
                    id
                }
            })
            .collect();
        // The setup: the standards were read, or the loop below proves nothing.
        assert!(wanted.len() > 30, "{page}: only {} ids read", wanted.len());
        for id in &wanted {
            assert!(
                text.contains(&format!("<article class=\"req\" id=\"{id}\"")),
                "{page} has no entry for {id}"
            );
        }
        assert_eq!(
            text.matches("<article class=\"req\"").count(),
            wanted.len(),
            "{page}"
        );
        assert_eq!(count(&text, "all"), wanted.len(), "{page}");
    }

    // What StackVet does, held to docs/COVERAGE.md's summary: "can be credited" is confirm or fail, "can settle" less
    // that is only shows it missing.
    let asvs = read("asvs.html");
    let [n, settle, credited, support, none] = summary("OWASP ASVS 5.0");
    assert_eq!(
        [
            count(&asvs, "all"),
            count(&asvs, "settle"),
            count(&asvs, "found"),
            count(&asvs, "support"),
            count(&asvs, "none")
        ],
        [n, credited, settle - credited, support, none]
    );
    let aisvs = read("aisvs.html");
    let core = summary("OWASP AISVS 1.0");
    let appendix = summary("AISVS Appendix C");
    let both: Vec<usize> = (0..5).map(|i| core[i] + appendix[i]).collect();
    assert_eq!(
        [
            count(&aisvs, "all"),
            count(&aisvs, "settle"),
            count(&aisvs, "found"),
            count(&aisvs, "support"),
            count(&aisvs, "none")
        ],
        [both[0], both[2], both[1] - both[2], both[3], both[4]]
    );
    let sbd = read("secure-by-design.html");
    let [n, settle, credited, support, none] = summary("Secure by Design checklist 0.5.0");
    assert_eq!(
        (settle, credited),
        (0, 0),
        "a checklist control is never settled by a check"
    );
    assert_eq!(
        [
            count(&sbd, "all"),
            count(&sbd, "support"),
            count(&sbd, "none")
        ],
        [n, support, none]
    );

    // Self-contained: nothing is fetched, and the only links out are to the standards' sources.
    for page in [
        "index.html",
        "asvs.html",
        "aisvs.html",
        "secure-by-design.html",
    ] {
        let text = read(page);
        for fetches in ["<script src", "<link", "<img", "@import", "url(", "<iframe"] {
            assert!(
                !text.contains(fetches),
                "{page} fetches something: {fetches}"
            );
        }
        for (at, _) in text.match_indices("href=\"http") {
            let link = &text[at + 6..at + 6 + text[at + 6..].find('"').unwrap()];
            assert!(
                link.starts_with("https://github.com/OWASP/"),
                "{page} links out to {link}"
            );
        }
        // What a result is not, said on every page.
        assert!(
            text.contains("A clean result is never proof that a requirement is met"),
            "{page}"
        );
    }
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn it_writes_only_into_a_folder_of_its_own() {
    // A folder with something in it that the tool did not make is left as it is.
    let dir = scratch("theirs");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("notes.txt"), "mine").unwrap();
    let out = script(&["--out", dir.to_str().unwrap()]);
    assert!(!out.status.success(), "{}", said(&out));
    assert!(
        said(&out).contains("was not made by this tool"),
        "{}",
        said(&out)
    );
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
    std::fs::remove_dir_all(&dir).ok();

    // Inside the repository, only under exports/, which git ignores: nothing else could be committed by mistake.
    let out = script(&["--out", repo().join("docs/site").to_str().unwrap()]);
    assert!(!out.status.success(), "{}", said(&out));
    assert!(
        said(&out).contains("inside the repository outside exports/"),
        "{}",
        said(&out)
    );
    assert!(!repo().join("docs/site").exists());

    // Run again over its own folder, it brings the pages up to date.
    let dir = scratch("again");
    for _ in 0..2 {
        let out = script(&["--out", dir.to_str().unwrap()]);
        assert!(out.status.success(), "{}", said(&out));
    }
    std::fs::remove_dir_all(&dir).ok();
}
