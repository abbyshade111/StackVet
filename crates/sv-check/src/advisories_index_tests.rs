//! The advisories are looked up by package rather than every component compared with every advisory
//! (backlog 0342). These hold the lookup to what the comparison found: among hundreds of records, the
//! one naming a package under any PEP 503 spelling is found, once; a withdrawn one and one in another
//! ecosystem are not; and a record naming two packages is found for each.

use super::*;
use crate::sbom::VersionSource;

fn component(name: &str, eco: &str) -> Component {
    Component {
        name: name.into(),
        version: "1.0.0".into(),
        ecosystem: eco.into(),
        source: VersionSource::Locked,
    }
}

fn sbom_of(components: Vec<Component>) -> Sbom {
    Sbom {
        passed_over: Vec::new(),
        disagreements: Vec::new(),
        lockfiles: Vec::new(),
        components,
        unread: Vec::new(),
    }
}

/// A record about `name` in `ecosystem`, affecting every version below 9.
fn record(id: &str, affected: &[(&str, &str)], withdrawn: bool) -> Advisory {
    let affected: Vec<String> = affected
        .iter()
        .map(|(ecosystem, name)| {
            format!(
                r#"{{"package": {{"ecosystem": "{ecosystem}", "name": "{name}"}},
                    "ranges": [{{"type": "ECOSYSTEM", "events": [{{"introduced": "0"}}, {{"fixed": "9.0.0"}}]}}]}}"#
            )
        })
        .collect();
    let withdrawn = if withdrawn {
        r#""withdrawn": "2026-01-01T00:00:00Z","#
    } else {
        ""
    };
    serde_json::from_str(&format!(
        r#"{{"id": "{id}", {withdrawn} "summary": "test", "affected": [{}]}}"#,
        affected.join(",")
    ))
    .expect("the record written here parses")
}

#[test]
fn the_lookup_finds_what_comparing_every_record_found() {
    // Hundreds of records about other packages, so a lookup that found nothing, or everything,
    // would show.
    let mut database: Vec<Advisory> = (0..400)
        .map(|i| {
            record(
                &format!("OTHER-{i}"),
                &[("PyPI", &format!("other-{i}"))],
                false,
            )
        })
        .collect();
    database.push(record("WANTED", &[("PyPI", "jupyter-server")], false));
    database.push(record("WITHDRAWN", &[("PyPI", "jupyter_server")], true));
    database.push(record("NPM", &[("npm", "jupyter-server")], false));
    database.push(record(
        "TWO",
        &[("PyPI", "Jupyter.Server"), ("npm", "left-pad")],
        false,
    ));

    for spelled in ["Jupyter_Server", "jupyter-server", "JUPYTER..server"] {
        let result = audit(&sbom_of(vec![component(spelled, "Python")]), &database);
        let found: Vec<String> = result.findings.iter().map(|f| format!("{f:?}")).collect();
        // Two records name it: WANTED and TWO; the withdrawn one and the npm one do not count.
        assert_eq!(result.findings.len(), 2, "{spelled}: {found:#?}");
        assert!(
            found.iter().any(|f| f.contains("WANTED")) && found.iter().any(|f| f.contains("TWO")),
            "{spelled}: {found:#?}"
        );
        assert!(
            found
                .iter()
                .all(|f| !f.contains("WITHDRAWN") && !f.contains("\"NPM\"")),
            "{spelled}: {found:#?}"
        );
    }
    // The record naming two packages is found for the other one too, in its own ecosystem.
    let npm = audit(&sbom_of(vec![component("left-pad", "npm")]), &database);
    assert_eq!(npm.findings.len(), 1, "{:#?}", npm.findings);
    // And a name no record carries finds nothing, among all of them.
    let none = audit(&sbom_of(vec![component("jupyter", "Python")]), &database);
    assert!(none.findings.is_empty(), "{:#?}", none.findings);
}
