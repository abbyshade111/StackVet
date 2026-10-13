//! A check asked for again in the same session names the findings it gave before in a line each
//! (backlog 0336): the AI coding tool checks after every edit, and re-read every finding in full
//! each time, about 19,000 characters for `examples/flask-booking`.

use super::tests::{call, conforms, scratch_app, text};
use super::*;

fn check(server: &Server, args: Value) -> Value {
    let result = call(server, "stackvet_check", args);
    assert_eq!(result["isError"], false, "{}", text(&result));
    let declared = tools();
    let schema = &declared
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["name"] == "stackvet_check")
        .unwrap()["outputSchema"];
    if let Err(why) = conforms(&result["structuredContent"], schema, "stackvet_check") {
        panic!("{why}");
    }
    result
}

fn fingerprints(list: &Value, key: &str) -> Vec<String> {
    let mut out: Vec<String> = list
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f[key].as_str().unwrap().to_owned())
        .collect();
    out.sort();
    out
}

/// A small app that keeps only the owner's recipes, on their own computer, with `files` files of
/// code each with three weak hashes: many findings, in a check short enough to come whole.
fn recipes(tag: &str, files: usize) -> PathBuf {
    let root = std::env::temp_dir().join(format!("sv-mcp-{tag}-{}", std::process::id()));
    std::fs::remove_dir_all(&root).ok();
    std::fs::create_dir_all(root.join("app")).unwrap();
    std::fs::write(
        root.join("app/stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"Recipes\"\ndescription = \"The owner's recipes\"\n\
         audience = \"just-me\"\ndeployment = \"local-only\"\n[stack]\nlanguages = [\"python\"]\n\
         [data]\ncategories = []\n[capabilities]\nauth = false\noauth = false\nuploads = false\n\
         email = false\npayments = false\nmcp-server = false\n[capabilities.ai]\nenabled = false\n\
         web-search = false\n",
    )
    .unwrap();
    for n in 0..files {
        let code: String = (0..3)
            .map(|i| format!("def f{i}(x):\n    return hashlib.md5(x).hexdigest()\n"))
            .collect();
        std::fs::write(
            root.join(format!("app/recipes_{n}.py")),
            format!("import hashlib\n{code}"),
        )
        .unwrap();
    }
    root
}

#[test]
fn a_second_check_names_the_findings_it_gave_before_and_gives_new_ones_in_full() {
    let root = recipes("since", 4);
    let server = Server::new(&root).unwrap();
    let app = json!({ "path": "app" });

    // The first check in the session gives every finding in full.
    let first = check(&server, app.clone());
    let data = &first["structuredContent"];
    assert!(data.get("unchanged").is_none() && data.get("since").is_none());
    let all_before = fingerprints(&data["findings"], "fingerprint");
    assert!(all_before.len() >= 12, "the setup: {all_before:?}");
    assert!(
        first["structuredContent"].get("part").is_none(),
        "the setup: it comes whole"
    );

    // Asked again with nothing changed: none in full, every one named, nothing gone.
    let second = check(&server, app.clone());
    let data = &second["structuredContent"];
    assert_eq!(data["findings"], json!([]), "{data:#}");
    assert_eq!(fingerprints(&data["unchanged"], "fingerprint"), all_before);
    assert_eq!(data["gone"], json!([]));
    assert!(data["since"].as_str().is_some_and(|s| !s.is_empty()));
    let said = text(&second);
    // Each finding named in a line saves most of what it took in full (about 555 characters for
    // these on 13 October 2026).
    assert!(
        said.len() + 400 * all_before.len() < text(&first).len(),
        "{} then {}",
        text(&first).len(),
        said.len()
    );
    // What must not be lost: the counts, what was not examined, and what "unchanged" means.
    assert_eq!(data["counts"], first["structuredContent"]["counts"]);
    assert_eq!(
        data["notExamined"],
        first["structuredContent"]["notExamined"]
    );
    assert!(said.contains("NOT EXAMINED"), "{said}");
    assert!(said.contains("UNCHANGED since your check at"), "{said}");
    assert!(
        said.contains("Unchanged means the same fingerprint, not the same code"),
        "{said}"
    );
    for fingerprint in &all_before {
        assert!(said.contains(fingerprint.as_str()), "{fingerprint}: {said}");
    }

    // `all` still gives every finding in full.
    let all = check(&server, json!({ "path": "app", "section": "all" }));
    assert_eq!(
        fingerprints(&all["structuredContent"]["findings"], "fingerprint"),
        all_before
    );
    assert!(all["structuredContent"].get("unchanged").is_none());

    // A file with findings moved: the findings at its old place are gone, those at the new one new.
    let moved = first["structuredContent"]["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["location"]["file"].as_str().unwrap().to_owned())
        .find(|f| f.ends_with(".py") && !f.contains('/'))
        .expect("a finding in a Python file at the app's top");
    std::fs::rename(
        root.join("app").join(&moved),
        root.join("app").join(format!("moved_{moved}")),
    )
    .unwrap();
    let third = check(&server, app);
    std::fs::remove_dir_all(&root).ok();
    let data = &third["structuredContent"];
    let gone = data["gone"].as_array().unwrap();
    assert!(!gone.is_empty(), "{data:#}");
    assert!(gone.iter().all(|g| g["file"] == moved.as_str()), "{data:#}");
    let new = data["findings"].as_array().unwrap();
    assert!(!new.is_empty(), "{data:#}");
    assert!(
        new.iter()
            .all(|f| f["location"]["file"] == format!("moved_{moved}").as_str()),
        "{data:#}"
    );
    let said = text(&third);
    assert!(said.contains("GONE since your check at"), "{said}");
    assert!(said.contains(&format!("moved_{moved}")), "{said}");
}

#[test]
fn with_a_finding_or_two_the_check_comes_in_full_as_before() {
    // Saying what "unchanged" means is longer than one finding: the delta is given only when it is
    // shorter than the whole answer.
    let root = scratch_app("since-one", "flask-booking");
    let server = Server::new(&root).unwrap();
    let first = check(&server, json!({ "path": "app" }));
    let second = check(&server, json!({ "path": "app" }));
    std::fs::remove_dir_all(&root).ok();
    assert!(
        first["structuredContent"]["findings"]
            .as_array()
            .unwrap()
            .len()
            <= 2,
        "the setup"
    );
    assert_eq!(first["structuredContent"], second["structuredContent"]);
}

#[test]
fn a_page_asked_for_is_never_cut_from_the_delta() {
    // Only an answer asked for with no section is shortened; a section asked for is the same
    // whatever was asked before.
    let root = recipes("since-section", 4);
    let server = Server::new(&root).unwrap();
    let findings = json!({ "path": "app", "section": "findings" });
    let before = check(&server, findings.clone());
    check(&server, json!({ "path": "app" }));
    let after = check(&server, findings);
    std::fs::remove_dir_all(&root).ok();
    assert_eq!(before["structuredContent"], after["structuredContent"]);
    assert!(!text(&after).contains("UNCHANGED since"));
}
