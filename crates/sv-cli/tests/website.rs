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
    assert!(home.contains("From a real build in Cursor, 14 October 2026"));
    assert!(home.contains(r#"class="turn asks""#));
    assert!(
        home.contains("I want to do invite codes because I only want my family to have access")
    );
    // The tools the setup prompts were tried in, from docs/prompts/setup.md, in place of the placeholder.
    assert!(!home.contains("[the tools they have really been tried in"));
    assert!(
        home.contains("The setup prompts have been tried in Cursor"),
        "the tools tried are not named"
    );
    // Nothing on the home page is shown as coming: both of its walk-through buttons lead to the build's
    // (backlog 0373, at the owner's word).
    assert!(!home.contains("(coming)"), "a button is shown as coming");
    assert_eq!(
        home.matches(r#"href="how-a-build-goes.html">See how a build goes</a>"#)
            .count(),
        2,
        "the opening's button and Get started's"
    );

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
        "check-a-project.html",
        "standards/index.html",
        "standards/asvs.html",
        "reports/notes-with-users/report.html",
        "reports/notes-with-users/compliance.html",
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

    // How a build goes: the home page's button leads to it, every step leads on to the next, and every screen is
    // one of the walk-through's own, read for anything personal before it was kept (backlog 0373).
    assert!(home.contains(r#"href="how-a-build-goes.html">See how a build goes</a>"#));
    let build = read("how-a-build-goes.html");
    let steps = build.matches(r#"<section class="step""#).count();
    let shots = std::fs::read_dir(repo().join("docs/website/walkthrough"))
        .unwrap()
        .count();
    assert!(steps > 2, "the setup: the page has steps");
    assert_eq!(
        build.matches(r#"src="walkthrough/"#).count(),
        shots,
        "every screenshot kept is shown once"
    );
    assert_eq!(
        build.matches(">Next: ").count() + build.matches(">Start: ").count(),
        steps - 1,
        "each step but the last leads on"
    );
    for note in [
        "has no screenshots, so it is not shown",
        "a fault the owner found here",
    ] {
        assert!(
            !build.contains(note),
            "a note on the page's words is shown: {note}"
        );
    }

    // How StackVet works: its own words, with the real VS Code line, then the paper's description.
    let how = read("how-it-works.html");
    assert!(how.contains("When only you can answer, your AI tool stops and asks."));
    assert!(how.contains("I shouldn’t invent a contact address"));
    assert!(
        how.contains("does not write the app"),
        "the paper's description is not shown"
    );
    assert!(
        !how.contains("make_how_it_works.py"),
        "a note on the page's words is shown"
    );

    // Checking somebody else's project (backlog 0671): its own words, its notes left out, and the home page points to it.
    let project = read("check-a-project.html");
    // The headline is escaped as the site escapes every heading.
    assert!(project.contains("Thinking of using somebody else&#x27;s code? Check it first."));
    assert!(project.contains("That does not make it safe to use"));
    assert!(project.contains("code nobody vetted was run"));
    for note in [
        "backlog 0670",
        "ADR-154",
        "crates/sv-report",
        "Tried on 25 October 2026",
    ] {
        assert!(
            !project.contains(note),
            "a note on the page's words is shown: {note}"
        );
    }
    assert!(
        read("index.html").contains("href=\"check-a-project.html\""),
        "the home page does not point to the page"
    );

    // The folders' own markers are not uploaded with the site (deploy/stackvet.dev/wrangler.jsonc).
    let ignore = read(".assetsignore");
    assert_eq!(
        ignore,
        ".stackvet-site\nstandards/.stackvet-framework-pages\n"
    );
    assert!(
        dir.join(".stackvet-site").is_file()
            && dir.join("standards/.stackvet-framework-pages").is_file()
    );

    // The headers Cloudflare sends: every inline script a page carries is allowed by its hash, and nothing else.
    let headers = read("_headers");
    let policy = headers
        .lines()
        .find_map(|l| l.trim().strip_prefix("Content-Security-Policy: "))
        .expect("a content security policy");
    let script_src = policy
        .split(';')
        .find_map(|d| d.trim().strip_prefix("script-src "))
        .expect("script-src");
    assert!(!script_src.contains("unsafe"), "{script_src}");
    assert!(policy.contains("default-src 'none'") && policy.contains("frame-ancestors 'none'"));
    let mut scripts = 0;
    for page in made
        .iter()
        .filter(|p| p.extension().is_some_and(|x| x == "html"))
    {
        let text = std::fs::read_to_string(dir.join(page)).unwrap();
        for (at, _) in text.match_indices("<script>") {
            let body = &text[at + 8..at + text[at..].find("</script>").unwrap()];
            let hash = std::process::Command::new("python3")
                .args(["-I", "-c", "import base64,hashlib,sys;print(base64.b64encode(hashlib.sha256(sys.stdin.buffer.read()).digest()).decode())"])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .and_then(|mut c| {
                    use std::io::Write;
                    c.stdin.take().unwrap().write_all(body.as_bytes())?;
                    c.wait_with_output()
                })
                .unwrap();
            let hash = String::from_utf8(hash.stdout).unwrap();
            assert!(
                script_src.contains(&format!("'sha256-{}'", hash.trim())),
                "{}'s script is not allowed by the policy",
                page.display()
            );
            scripts += 1;
        }
    }
    assert!(
        scripts > 0,
        "the setup: the framework pages carry their filter's script"
    );

    // StackVet's own report on itself, and the credits, each a page.
    let itself = read("checks-itself.html");
    assert!(itself.contains("Held to ASVS level 3"), "{itself}");
    // A summary only (the owner's choice of 23 October 2026, backlog 0633): the counts and the bill of materials,
    // never the finding-by-finding files.
    assert!(
        itself.contains("things to fix were found, and"),
        "the counts are on the page"
    );
    // The paper's own report stays in the repository; the site neither links nor names it.
    assert!(
        !itself.contains("self-assessment-v2"),
        "the paper's own report is not on the page"
    );
    let published: Vec<&PathBuf> = made
        .iter()
        .filter(|p| p.starts_with("self-assessment"))
        .collect();
    assert_eq!(
        published,
        [&PathBuf::from("self-assessment/sbom.cdx.json")],
        "only the bill of materials of StackVet's report on itself is published"
    );
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
