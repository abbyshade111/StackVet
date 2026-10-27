//! The ASVS-to-NIST SP 800-53 crosswalk (`data/asvs-800-53-crosswalk.json`, ADR-156), held as the Secure by Design
//! crosswalk is held in `citations.rs`.
//!
//! Each ASVS 5.0 requirement is paired with the 800-53 Rev. 5 controls and enhancements that ask the same thing, each
//! pair with a phrase naming what the two have in common. The export to OSCAL names those controls beside every result,
//! so a wrong pair puts an ASVS result in front of an assessor under a control it says nothing about. The comparison is
//! the citation guard's (`shares_no_words`): weak on purpose, one word in common with each side, and enough to catch a
//! pair between two different subjects. The control's side is the text NIST wrote, kept true by
//! `tools/nist_controls.py`, never a paraphrase.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use sv_check::suite::shares_no_words;
use sv_frameworks::Frameworks;

fn crosswalk() -> serde_json::Value {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/asvs-800-53-crosswalk.json");
    serde_json::from_str(&std::fs::read_to_string(path).expect("the crosswalk reads"))
        .expect("the crosswalk parses")
}

/// ASVS 5.0's requirements as `sv` itself loads them, by id.
fn asvs() -> BTreeMap<String, String> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/frameworks");
    Frameworks::load(&dir)
        .expect("the frameworks load")
        .requirements
        .into_iter()
        .filter(|(id, _)| id.starts_with('V') && id[1..].starts_with(|c: char| c.is_ascii_digit()))
        .map(|(id, info)| (id, info.description))
        .collect()
}

/// Every (requirement, control, phrase) in the file.
fn pairs(crosswalk: &serde_json::Value) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    for (requirement, controls) in crosswalk["requirements"].as_object().expect("requirements") {
        for (control, because) in controls.as_object().expect("a map") {
            out.push((
                requirement.clone(),
                control.clone(),
                because.as_str().expect("a phrase").to_owned(),
            ));
        }
    }
    out
}

/// A control's own words, as NIST wrote them: its title and statement.
fn control_text(crosswalk: &serde_json::Value, control: &str) -> Option<String> {
    let c = crosswalk["controls"].get(control)?;
    Some(format!(
        "{} {}",
        c["title"].as_str()?,
        c["statement"].as_str()?
    ))
}

/// The pairs whose phrase shares no word with one side or the other, or has no word at all.
fn unbridged(crosswalk: &serde_json::Value) -> Vec<String> {
    let requirements = asvs();
    let mut wrong = Vec::new();
    for (requirement, control, because) in pairs(crosswalk) {
        // A phrase with no substantive word agrees with anything (`citations.rs` says why); compared with an empty text,
        // it is the one that fails to share nothing.
        if !shares_no_words(&because, "") {
            wrong.push(format!(
                "{requirement} ~ {control}: `{because}` has no word the comparison can use"
            ));
            continue;
        }
        let Some(asked) = requirements.get(&requirement) else {
            continue; // reported by the test that every key is a requirement
        };
        if shares_no_words(&because, asked) {
            wrong.push(format!(
                "{requirement} ~ {control}: `{because}` shares nothing with {requirement}: {asked}"
            ));
        }
        let Some(text) = control_text(crosswalk, &control) else {
            continue; // reported by the test that every control has its text
        };
        if shares_no_words(&because, &text) {
            let title = crosswalk["controls"][&control]["title"]
                .as_str()
                .unwrap_or("");
            wrong.push(format!(
                "{requirement} ~ {control}: `{because}` shares nothing with {control} {title}"
            ));
        }
    }
    wrong
}

#[test]
fn every_asvs_requirement_is_listed_and_nothing_else() {
    // A requirement left out would be exported with no related controls, which reads as "nothing in 800-53 asks this"
    // when nobody looked. An empty list is the answer that nobody found one.
    let crosswalk = crosswalk();
    let listed: BTreeSet<&String> = crosswalk["requirements"]
        .as_object()
        .expect("requirements")
        .keys()
        .collect();
    let requirements = asvs();
    assert!(
        requirements.len() >= 345,
        "only {} ASVS requirements loaded",
        requirements.len()
    );
    let missing: Vec<&String> = requirements
        .keys()
        .filter(|id| !listed.contains(id))
        .collect();
    let unknown: Vec<&&String> = listed
        .iter()
        .filter(|id| !requirements.contains_key(**id))
        .collect();
    assert!(missing.is_empty(), "not in the crosswalk: {missing:?}");
    assert!(unknown.is_empty(), "not ASVS 5.0 requirements: {unknown:?}");
}

#[test]
fn every_cited_control_has_nists_text_and_no_other_is_kept() {
    // `tools/nist_controls.py` writes exactly the cited controls. One cited and missing would be exported with no text
    // to read it by; one kept and no longer cited is text nothing holds to the catalog's next release.
    let crosswalk = crosswalk();
    let cited: BTreeSet<String> = pairs(&crosswalk).into_iter().map(|(_, c, _)| c).collect();
    let kept: BTreeSet<String> = crosswalk["controls"]
        .as_object()
        .expect("controls")
        .keys()
        .cloned()
        .collect();
    let no_text: Vec<&String> = cited.difference(&kept).collect();
    let uncited: Vec<&String> = kept.difference(&cited).collect();
    assert!(
        no_text.is_empty(),
        "cited with no text (run tools/nist_controls.py): {no_text:?}"
    );
    assert!(
        uncited.is_empty(),
        "kept but cited by nothing (run tools/nist_controls.py): {uncited:?}"
    );
    for (id, control) in crosswalk["controls"].as_object().expect("controls") {
        // OSCAL's own ids, as an assessment-results file names them: `ac-7`, `ia-5.1`.
        let (family, number) = id
            .split_once('-')
            .unwrap_or_else(|| panic!("{id} is not an OSCAL control id"));
        assert!(
            family.len() == 2
                && family.chars().all(|c| c.is_ascii_lowercase())
                && number
                    .split('.')
                    .all(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit())),
            "{id} is not an OSCAL control id"
        );
        for field in ["label", "title", "statement"] {
            assert!(
                control[field]
                    .as_str()
                    .is_some_and(|s| !s.trim().is_empty()),
                "{id} has no {field}"
            );
        }
    }
    let catalog = &crosswalk["catalog"];
    assert!(
        catalog["version"]
            .as_str()
            .is_some_and(|v| v.starts_with("5.")),
        "the catalog is not 800-53 Rev. 5: {catalog}"
    );
    assert!(
        catalog["release"]
            .as_str()
            .is_some_and(|r| r.starts_with('v')),
        "no pinned release: {catalog}"
    );
}

#[test]
fn every_pair_shares_vocabulary_with_both_texts() {
    let crosswalk = crosswalk();
    let count = pairs(&crosswalk).len();
    assert!(
        count >= 100,
        "only {count} pairs read, so this is not reading the file"
    );
    let wrong = unbridged(&crosswalk);
    assert!(
        wrong.is_empty(),
        "{} pair(s) not bridged:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

#[test]
fn a_pair_between_two_different_subjects_is_caught() {
    // The mistake the guard is for: password rules paired with protecting what is sent over a network. Each side's
    // phrase is right for its own text, so only the comparison with the other side can catch it.
    let mut crosswalk = crosswalk();
    let password = asvs()
        .into_iter()
        .find(|(_, text)| text.contains("password") && text.contains("characters"))
        .map(|(id, _)| id)
        .expect("an ASVS password-length requirement");
    assert!(
        crosswalk["controls"].get("sc-8").is_some()
            || crosswalk["controls"].get("sc-8.1").is_some()
    );
    let transmission = if crosswalk["controls"].get("sc-8.1").is_some() {
        "sc-8.1"
    } else {
        "sc-8"
    };
    assert!(
        unbridged(&crosswalk).is_empty(),
        "the file must pass before it is broken"
    );
    crosswalk["requirements"][&password] = serde_json::json!({ transmission: "protecting the confidentiality of transmitted information" });
    let wrong = unbridged(&crosswalk);
    assert_eq!(wrong.len(), 1, "{wrong:?}");
    assert!(
        wrong[0].starts_with(&format!("{password} ~ {transmission}")),
        "{wrong:?}"
    );
    // And a phrase of nothing but short words would agree with any pair, so it is refused too.
    crosswalk["requirements"][&password] = serde_json::json!({ transmission: "the app" });
    assert!(unbridged(&crosswalk)[0].contains("has no word"));
}
