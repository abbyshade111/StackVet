//! `docs/REQUIREMENTS.md` tells a requirement the owner answers from one nothing in `sv` speaks to (backlog 0624):
//! until 20 October 2026 both read *No check*, though V1.1.1, for one, is a design question with a locator that lists
//! every decoding call in the app. This reads the three catalogs `sv review` records the owner's answers from, and
//! `data/locators.json`, without `tools/coverage.py`, and holds each row that no check speaks to to them.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn data(file: &str) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(root().join("data").join(file)).unwrap()).unwrap()
}

fn ids(file: &str, list: &str) -> BTreeSet<String> {
    data(file)[list]
        .as_array()
        .unwrap_or_else(|| panic!("{file} has no `{list}`"))
        .iter()
        .map(|e| e["id"].as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn a_requirement_no_check_reaches_reads_the_owner_answers_exactly_when_the_owner_is_asked() {
    let kinds = [
        (
            "a section in the security notes",
            ids("security-notes.json", "sections"),
        ),
        (
            "a design question",
            ids("design-questions.json", "questions"),
        ),
        ("a check by hand", ids("human-checks.json", "checks")),
    ];
    let mut located: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let locators = data("locators.json");
    for l in ["locators", "packages"]
        .iter()
        .flat_map(|k| locators[k].as_array().unwrap())
    {
        for q in l["questions"].as_array().into_iter().flatten() {
            located
                .entry(q.as_str().unwrap().to_owned())
                .or_default()
                .push(l["id"].as_str().unwrap().to_owned());
        }
    }

    let list = std::fs::read_to_string(root().join("docs/REQUIREMENTS.md")).unwrap();
    let (mut owner, mut none) = (0, 0);
    for line in list.lines().filter(|l| l.starts_with("| **")) {
        let id = line[4..].split("**").next().unwrap();
        let cells: Vec<&str> = line.rsplitn(4, " | ").collect();
        let (status, checks) = (cells[1], cells[0].trim_end_matches(" |"));
        let asked: Vec<&str> = kinds
            .iter()
            .filter(|(_, ids)| ids.contains(id))
            .map(|(how, _)| *how)
            .collect();
        match status {
            "The owner answers" => {
                owner += 1;
                assert!(
                    !asked.is_empty(),
                    "{id} reads the owner answers, and no catalog asks it"
                );
                for how in &asked {
                    assert!(
                        checks.contains(how),
                        "{id} does not say it is {how}: {checks}"
                    );
                }
                for l in located.get(id).into_iter().flatten() {
                    assert!(
                        checks.contains(&format!("`{l}`")),
                        "{id} does not name `{l}`, which shows the owner where to look: {checks}"
                    );
                }
            }
            "No check yet" => {
                none += 1;
                assert!(
                    asked.is_empty(),
                    "{id} reads no check yet, and is asked as {asked:?}"
                );
            }
            _ => {}
        }
    }
    // The setup: both kinds are in the list, and the requirement that started this reads as it should.
    assert!(
        owner > 0 && none > 0,
        "{owner} the owner answers, {none} no check yet"
    );
    let v111 = list
        .lines()
        .find(|l| l.starts_with("| **V1.1.1**"))
        .expect("V1.1.1 is listed");
    assert!(
        v111.contains("| The owner answers |")
            && v111.contains("a design question")
            && v111.contains("`locate.decoding`"),
        "{v111}"
    );
}
