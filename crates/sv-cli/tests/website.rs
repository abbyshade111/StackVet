//! `tools/website.py`, which builds stackvet.dev from the repository (backlog 0376, ADR-132).
//!
//! A site that shows the owner's notes on the words as if they were words, says other numbers than
//! `docs/COVERAGE.md`, fetches something from elsewhere, links to a page it did not write, or writes over a folder it
//! did not make, fails here.

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
        .arg(repo().join("tools/website.py"))
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
    let dir = std::env::temp_dir().join(format!("sv-website-{tag}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    dir
}

/// A level table of `docs/COVERAGE.md`: for each of L1, L2, and L3, its requirements, can settle, and can be
/// credited.
fn levels(heading: &str) -> Vec<[usize; 3]> {
    let doc = std::fs::read_to_string(repo().join("docs/COVERAGE.md")).unwrap();
    let section = doc
        .split(&format!("## {heading}\n"))
        .nth(1)
        .unwrap_or_else(|| panic!("no {heading}"));
    ["L1", "L2", "L3"]
        .iter()
        .map(|l| {
            let row = section
                .lines()
                .find(|r| r.starts_with(&format!("| {l} |")))
                .unwrap_or_else(|| panic!("no {l} in {heading}"));
            let cells: Vec<usize> = row
                .split('|')
                .skip(2)
                .take(3)
                .map(|c| c.trim().parse().unwrap())
                .collect();
            cells.try_into().unwrap()
        })
        .collect()
}

/// Every file under `dir`, as a path relative to it.
fn files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut todo = vec![dir.to_path_buf()];
    while let Some(d) = todo.pop() {
        for entry in std::fs::read_dir(&d).unwrap() {
            let p = entry.unwrap().path();
            if p.is_dir() {
                todo.push(p);
            } else {
                out.push(p.strip_prefix(dir).unwrap().to_path_buf());
            }
        }
    }
    out
}

#[test]
fn its_small_pieces_pass_their_own_checks() {
    let out = script(&["--self-test"]);
    assert!(out.status.success(), "{}", said(&out));
    assert!(
        said(&out).contains("website self-test: ok"),
        "{}",
        said(&out)
    );
}

#[test]
fn the_site_says_the_settled_words_and_coverage_md_s_numbers_and_nothing_else() {
    let dir = scratch("built");
    let out = script(&["--out", dir.to_str().unwrap()]);
    assert!(out.status.success(), "{}", said(&out));
    let read = |f: &str| std::fs::read_to_string(dir.join(f)).unwrap();
    let home = read("index.html");

    // The words, and none of the notes on them: each section's last list is what its lines rest on.
    for words in [
        "Vibe-code your app with security in mind from the first prompt.",
        "You don&#x27;t need to be a programmer.",
        "Your code stays on your computer.",
        "Free, open source, and honest about being new.",
    ] {
        assert!(home.contains(words), "the home page lacks {words:?}");
    }
    for note in [
        "says how the app is built, not what it ends up being",
        "are the three tools",
        "Contact is GitHub issues only",
        "The owner chose this column",
        "Left blank until the paper",
        "Chosen by the owner on 12 October 2026",
        "The site does not go live while",
    ] {
        assert!(!home.contains(note), "the home page shows a note: {note:?}");
    }
    // The conversation beside the headline, with the AI tool's question in the question bubble.
    assert!(home.contains("An example, written to show how a build goes"));
    assert!(home.contains(r#"class="turn asks""#));
    // The tools the setup prompts were tried in, from docs/prompts/setup.md, in place of the placeholder.
    assert!(!home.contains("[the tools they have really been tried in"));
    assert!(
        home.contains("The setup prompts have been tried in Cursor"),
        "the tools tried are not named"
    );
    // A button for something not made yet is shown as coming, never as a link.
    assert!(home.contains("Watch the walk-through<span class=\"small\">&nbsp;(coming)"));

    // The numbers are docs/COVERAGE.md's, not the ones written in the words.
    let asvs = levels("ASVS 5.0 by level");
    let aisvs = levels("AISVS 1.0 by level");
    for (label, rows) in [
        ("ASVS 5.0, levels 1 and 2", vec![asvs[0], asvs[1]]),
        ("ASVS 5.0, level 3", vec![asvs[2]]),
        ("AISVS 1.0, levels 1 and 2", vec![aisvs[0], aisvs[1]]),
        ("AISVS 1.0, level 3", vec![aisvs[2]]),
    ] {
        let requirements: usize = rows.iter().map(|r| r[0]).sum();
        let credited: usize = rows.iter().map(|r| r[2]).sum();
        let at = home
            .find(&format!("<td>{label}</td>"))
            .unwrap_or_else(|| panic!("no row {label}"));
        let row = &home[at..at + home[at..].find("</tr>").unwrap()];
        assert!(
            row.contains(&format!("<td>{requirements}</td><td>{credited}</td>")),
            "{label}: {row}"
        );
    }
    let more = |t: &[[usize; 3]]| t.iter().map(|r| r[1] - r[2]).sum::<usize>();
    assert!(
        home.contains(&format!(
            "a check for {} more of the ASVS requirements and {} more of the AISVS ones",
            more(&asvs),
            more(&aisvs)
        )),
        "the sentence on what a check can only show missing"
    );

    // Every page is there, and every link between them leads somewhere.
    let made = files(&dir);
    for page in [
        "index.html",
        "how-it-works.html",
        "reports.html",
        "checks-itself.html",
        "credits.html",
        "standards/index.html",
        "standards/asvs.html",
        "reports/notes-with-users/report.html",
        "reports/notes-with-users/compliance.html",
        "self-assessment/compliance.html",
        "self-assessment/sbom.cdx.json",
    ] {
        assert!(made.contains(&PathBuf::from(page)), "{page} is not made");
    }
    for page in made
        .iter()
        .filter(|p| p.extension().is_some_and(|x| x == "html"))
    {
        let text = std::fs::read_to_string(dir.join(page)).unwrap();
        // Self-contained: nothing is fetched from anywhere.
        for fetches in [
            "<script src",
            "@import",
            "url(",
            "<iframe",
            "src=\"http",
            "<link rel=\"stylesheet",
        ] {
            assert!(
                !text.contains(fetches),
                "{} fetches something: {fetches}",
                page.display()
            );
        }
        if page.starts_with("reports/notes-with-users") && page.ends_with("report.html") {
            continue; // the example report as sv wrote it, a page of its own
        }
        for attr in ["href=\"", "src=\""] {
            for (at, _) in text.match_indices(attr) {
                let link = &text[at + attr.len()..];
                let link = &link[..link.find('"').unwrap()];
                if link.starts_with("http") || link.starts_with('#') || link.starts_with("mailto:")
                {
                    continue;
                }
                let target = page.parent().unwrap().join(link.split('#').next().unwrap());
                assert!(
                    dir.join(&target).exists(),
                    "{} links to {link}, which is not there",
                    page.display()
                );
            }
        }
    }

    // StackVet's own report on itself, and the credits, each a page.
    let itself = read("checks-itself.html");
    assert!(itself.contains("Held to ASVS level 3"), "{itself}");
    assert!(read("credits.html").contains("OWASP Application Security Verification Standard"));
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

    // Inside the repository, only under exports/, which git ignores.
    let out = script(&["--out", repo().join("docs/site").to_str().unwrap()]);
    assert!(!out.status.success(), "{}", said(&out));
    assert!(
        said(&out).contains("inside the repository outside exports/"),
        "{}",
        said(&out)
    );
    assert!(!repo().join("docs/site").exists());

    // A folder of framework pages alone, as tools/framework_pages.py wrote them before the site had a home page,
    // makes way for the site, which holds them in standards/.
    let dir = scratch("old");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join(".stackvet-framework-pages"), "").unwrap();
    std::fs::write(dir.join("asvs.html"), "old").unwrap();
    let out = script(&["--out", dir.to_str().unwrap()]);
    assert!(out.status.success(), "{}", said(&out));
    assert!(!dir.join("asvs.html").exists() && dir.join("standards/asvs.html").exists());

    // Run again over its own folder, it brings the site up to date.
    let out = script(&["--out", dir.to_str().unwrap()]);
    assert!(out.status.success(), "{}", said(&out));
    std::fs::remove_dir_all(&dir).ok();
}
