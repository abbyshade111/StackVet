//! The four readers of `requirements.txt` read one file the same way (backlog 0291). They all start from
//! `sv_scan::requirements::logical_lines` now; this feeds each the same file, written with what pip allows
//! and what used to split them (a requirement continued over three lines, a comment after a tab, a comment
//! after a continued hash), and holds them to one answer.

use std::collections::BTreeSet;

// No option line such as `--require-hashes`: the install step refuses every one, by its own rule
// (ADR-052), which is what it allows, not how it reads the file.
const FILE: &str = "# pinned with hashes, as pip-compile writes it\n\
flask==3.0.0 \\\n    --hash=sha256:aaaa  # the web framework\n\
requests==2.31.0 --hash=sha256:cccc\t# a tab before this comment\n\
Jinja2 \\\n  ==3.1.4 \\\n  --hash=sha256:bbbb\n";

fn names(list: impl IntoIterator<Item = String>) -> BTreeSet<String> {
    list.into_iter().map(|n| n.to_lowercase()).collect()
}

#[test]
fn the_four_readers_agree_on_one_file() {
    let expected: BTreeSet<String> = ["flask", "jinja2", "requests"].map(String::from).into();

    // 1. Whether it is a hash-pinned lockfile (sv-scan).
    assert!(
        sv_scan::ecosystems::fully_hash_pinned(FILE),
        "every requirement carries a hash"
    );

    // 2. What the manifest asks for (sv-check).
    let declared = sv_check::manifest_lock::declared_names("requirements.txt", FILE)
        .expect("requirements.txt is read");
    assert_eq!(names(declared), expected, "the names asked for");

    // 3. The bill of materials, pinned versions included (sv-check).
    let dir = std::env::temp_dir().join(format!("sv-requirements-readers-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("requirements.txt"), FILE).unwrap();
    let sbom = sv_check::sbom::build(&dir);
    std::fs::remove_dir_all(&dir).ok();
    let listed: Vec<(String, String)> = sbom
        .components
        .iter()
        .map(|c| (c.name.to_lowercase(), c.version.clone()))
        .collect();
    assert_eq!(
        names(listed.iter().map(|(n, _)| n.clone())),
        expected,
        "{listed:?}"
    );
    for (name, version) in [
        ("flask", "3.0.0"),
        ("requests", "2.31.0"),
        ("jinja2", "3.1.4"),
    ] {
        assert!(
            listed.iter().any(|(n, v)| n == name && v == version),
            "{name} {version} not in the bill of materials: {listed:?}"
        );
    }

    // 4. What the install step would refuse as unpinned (sv-run): nothing.
    let unpinned = sv_run::install::unpinned(FILE);
    assert!(unpinned.is_empty(), "read as unpinned: {unpinned:?}");
}
