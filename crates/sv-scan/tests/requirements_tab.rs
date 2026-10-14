//! A tab before `--hash` in `requirements.txt` (backlog 0448): pip reads `flask==3.0.0<TAB>--hash=...` as the
//! requirement `flask==3.0.0` with a hash, and so does `fully_hash_pinned`. After backlog 0291 it looked for a space
//! before the option only, read the version as `3.0.0--hash=...`, and took the whole file as unpinned.

use sv_scan::ecosystems::fully_hash_pinned;

/// A hash built at run time, so nothing here looks like a value copied from somewhere.
fn hash() -> String {
    format!("sha256:{}", "a".repeat(64))
}

#[test]
fn a_tab_before_the_hash_is_read_as_pip_reads_it() {
    let h = hash();
    for (name, text) in [
        ("a tab", format!("flask==3.0.0\t--hash={h}\n")),
        ("tabs", format!("flask==3.0.0\t\t--hash={h}\n")),
        (
            "a continued line starting with a tab",
            format!("flask==3.0.0 \\\n\t--hash={h}\n"),
        ),
        (
            "a continued line with no space before the backslash",
            format!("flask==3.0.0\\\n\t--hash={h}\n"),
        ),
        ("a space, as before", format!("flask==3.0.0 --hash={h}\n")),
        (
            "spaces inside the requirement, as 0291 allowed",
            format!("Jinja2 ==3.1.4\t--hash={h}\n"),
        ),
    ] {
        assert!(fully_hash_pinned(&text), "{name}: {text:?}");
    }
}

#[test]
fn a_tab_does_not_make_a_hash_out_of_nothing() {
    // The setup: a file the reader takes, so the refusals below are about what changed in each.
    let h = hash();
    assert!(fully_hash_pinned(&format!("flask==3.0.0\t--hash={h}\n")));
    for (name, text) in [
        ("no hash", "flask==3.0.0\t\n".to_owned()),
        ("another option", "flask==3.0.0\t--no-deps\n".to_owned()),
        ("a range", format!("flask>=3.0\t--hash={h}\n")),
        ("a wildcard", format!("flask==3.*\t--hash={h}\n")),
    ] {
        assert!(!fully_hash_pinned(&text), "{name}: {text:?}");
    }
}
