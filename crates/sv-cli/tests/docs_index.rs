//! `docs/README.md` lists every document in `docs/` and `docs/research/`, and says which are live, dated, or generated
//! (backlog 0441). A document added without a row, or a moved one whose old path no longer leads to it, fails here.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The Markdown files directly in `folder`, by name.
fn documents(folder: &str) -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(root().join(folder))
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".md"))
        .collect();
    out.sort();
    out
}

/// A one-line pointer left at a moved document's old path, and the path it names.
fn moved_to(folder: &str, name: &str) -> Option<String> {
    let text = std::fs::read_to_string(root().join(folder).join(name)).unwrap();
    let title = text.lines().next()?;
    let target = title.strip_prefix("# Moved: `")?.strip_suffix('`')?;
    Some(target.to_owned())
}

#[test]
fn every_document_has_a_row_saying_what_kind_it_is() {
    let index = std::fs::read_to_string(root().join("docs/README.md")).unwrap();
    let mut missing = Vec::new();
    for (folder, prefix) in [("docs", ""), ("docs/research", "research/")] {
        let found = documents(folder);
        assert!(!found.is_empty(), "the setup read no documents in {folder}");
        for name in found {
            if name == "README.md" || moved_to(folder, &name).is_some() {
                continue;
            }
            let row = index
                .lines()
                .find(|line| line.starts_with(&format!("| [`{prefix}{name}`]")));
            match row {
                Some(row)
                    if ["| live", "| dated", "| generated"]
                        .iter()
                        .any(|k| row.contains(k)) => {}
                _ => missing.push(format!("{prefix}{name}")),
            }
        }
    }
    assert!(
        missing.is_empty(),
        "docs/README.md has no row, or no kind, for: {missing:?}"
    );
}

#[test]
fn a_moved_document_s_old_path_leads_to_it() {
    let mut pointers = 0;
    for name in documents("docs") {
        let Some(target) = moved_to("docs", &name) else {
            continue;
        };
        pointers += 1;
        assert!(
            root().join(&target).is_file(),
            "docs/{name} points at {target}, which is not there"
        );
        assert!(
            target.ends_with(&format!("/{name}")),
            "docs/{name} points at a file of another name: {target}"
        );
    }
    assert!(
        pointers >= 5,
        "the five dated readings moved in 0441 leave pointers; read {pointers}"
    );
}
