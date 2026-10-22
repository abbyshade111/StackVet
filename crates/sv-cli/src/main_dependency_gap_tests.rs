use super::*;

fn component(ecosystem: &str, name: &str, source: sbom::VersionSource) -> sbom::Component {
    sbom::Component {
        name: name.to_owned(),
        version: "1.0.0".to_owned(),
        ecosystem: ecosystem.to_owned(),
        source,
    }
}

#[test]
fn a_fully_locked_bill_of_materials_produces_no_gap() {
    // The second witness for the over-reporting guard, at the level the end-to-end test cannot
    // reach: given a document whose every version came from a lockfile, there is nothing to
    // report, and a gap row for nothing reads as a hole where there is none.
    let sbom = sbom::Sbom {
        passed_over: Vec::new(),
        disagreements: Vec::new(),
        lockfiles: Vec::new(),
        components: vec![
            component("npm", "react", sbom::VersionSource::Locked),
            component("npm", "express", sbom::VersionSource::Locked),
            component("Python", "flask", sbom::VersionSource::Locked),
        ],
        unread: Vec::new(),
    };
    assert!(dependency_gaps(&sbom).is_empty());
}

#[test]
fn an_unread_ecosystem_is_reported_as_empty_and_a_declared_one_is_not() {
    // The distinction the whole item is about, held at one place rather than across two apps:
    // these are two different gaps and must not collapse into one sentence again.
    let sbom = sbom::Sbom {
        passed_over: Vec::new(),
        disagreements: Vec::new(),
        lockfiles: Vec::new(),
        components: vec![component("Python", "flask", sbom::VersionSource::Declared)],
        unread: vec![(
            "npm".to_owned(),
            "npm is in use but nothing readable says which versions are installed, so none of \
             its packages are listed"
                .to_owned(),
        )],
    };
    let gaps = dependency_gaps(&sbom);
    assert_eq!(gaps.len(), 2, "{gaps:?}");

    let npm = gaps.iter().find(|g| g.what.contains("npm")).expect("npm");
    assert!(npm.why.contains("empty one"), "{}", npm.why);

    let python = gaps
        .iter()
        .find(|g| g.what.contains("Python"))
        .expect("Python");
    assert!(python.why.contains("what was asked for"), "{}", python.why);
    assert!(
        !python.why.contains("empty one"),
        "a list that was read is not empty: {}",
        python.why
    );
}

#[test]
fn one_ecosystem_being_unreadable_says_nothing_about_another_that_was_read() {
    // An app with both. The npm half is absent and the Python half is real, and the report has
    // to say each of those about the right one — which is exactly what a single sentence for
    // every ecosystem could not do.
    let sbom = sbom::Sbom {
        passed_over: Vec::new(),
        disagreements: Vec::new(),
        lockfiles: Vec::new(),
        components: vec![
            component("Python", "flask", sbom::VersionSource::Declared),
            component("Rust", "serde", sbom::VersionSource::Locked),
        ],
        unread: vec![("npm".to_owned(), "nothing readable".to_owned())],
    };
    let gaps = dependency_gaps(&sbom);
    let named: Vec<&str> = gaps.iter().map(|g| g.what.as_str()).collect();
    assert_eq!(gaps.len(), 2, "{named:?}");
    assert!(
        !named.iter().any(|w| w.contains("Rust")),
        "the locked Rust packages are not a gap: {named:?}"
    );
}
