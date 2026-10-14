//! Backlog 0412 (ADR-115): the PDF's own pieces. The reports' Markdown is read as the renderers write it, a table's
//! header row is drawn again on each page it runs onto, a row taller than a page is split rather than cut off, and
//! the file is one a reader can follow: the read-back refuses a file whose table of objects is wrong.

use super::read::{Block, Look, blocks, spans};
use super::{Paper, back, document};

#[test]
fn the_markdown_the_renderers_write_is_read_as_they_mean_it() {
    let s = spans(
        r"**Where:** `a|b` line 4, \[not a link\], &lt;img>, *see* [the rule](https://example.com/r)",
    );
    let text: String = s.iter().map(|x| x.text.as_str()).collect();
    assert_eq!(text, "Where: a|b line 4, [not a link], <img>, see the rule");
    assert!(s[0].look.bold && s[0].text == "Where:");
    assert!(s.iter().any(|x| x.look
        == Look {
            code: true,
            ..Look::default()
        }
        && x.text == "a|b"));
    assert!(s.iter().any(|x| x.look.italic && x.text == "see"));
    assert!(
        s.iter()
            .any(|x| x.link.as_deref() == Some("https://example.com/r") && x.text == "the rule")
    );

    let md = "# Title\n\nOne\nparagraph.\n\n- an item\n  going on\n1. first\n\n| a | n |\n|---|---:|\n| x \\| y | 3 |\n\n<details>\n<summary><strong>Folded</strong> list</summary>\n\n````text\n> 4  code ```\n````\n";
    let b = blocks(md);
    assert_eq!(
        b[0],
        Block::Heading {
            level: 1,
            text: spans("Title")
        }
    );
    assert_eq!(b[1].text(), "One paragraph.");
    assert!(matches!(&b[2], Block::Item { marker, .. } if marker == "\u{2022}"));
    assert_eq!(b[2].text(), "an item going on");
    assert!(matches!(&b[3], Block::Item { marker, .. } if marker == "1."));
    assert!(
        matches!(&b[4], Block::Table { right, rows, .. } if *right == vec![false, true] && rows.len() == 1)
    );
    assert_eq!(b[4].text(), "a n x | y 3");
    assert_eq!(b[5].text(), "Folded list");
    assert_eq!(b[6], Block::Code(vec!["> 4  code ```".to_owned()]));
    assert_eq!(b.len(), 7, "{b:#?}");
}

/// A table of `rows` rows, its header `left` and `right`.
fn long_table(rows: usize, cell: &str) -> String {
    let mut md = String::from("# A long table\n\n| left | right |\n|---|---|\n");
    for i in 0..rows {
        md.push_str(&format!("| row {i} | {cell} |\n"));
    }
    md
}

#[test]
fn a_tables_header_row_is_drawn_again_on_every_page_it_runs_onto() {
    for paper in [Paper::Letter, Paper::A4] {
        let pdf = document(&long_table(200, "words"), "t", None, paper);
        let read = back::read(&pdf).unwrap();
        // The setup: the table runs over more than one page.
        assert!(read.pages.len() > 2, "{}", read.pages.len());
        for (n, page) in read.pages.iter().enumerate() {
            let texts: Vec<&str> = page.drawn.iter().map(|d| d.text.as_str()).collect();
            // The first page opens with the heading above the table.
            let from = usize::from(n == 0);
            assert_eq!(
                &texts[from..from + 2],
                ["left", "right"],
                "page {}: {texts:?}",
                n + 1
            );
        }
        // And every row is there, once.
        let text = read.text();
        for i in 0..200 {
            assert_eq!(text.matches(&format!("row {i} ")).count(), 1, "row {i}");
        }
    }
}

#[test]
fn a_row_taller_than_a_page_is_split_between_its_lines_and_loses_none() {
    let words: Vec<String> = (0..3000).map(|i| format!("w{i}")).collect();
    let pdf = document(&long_table(1, &words.join(" ")), "t", None, Paper::Letter);
    let read = back::read(&pdf).unwrap();
    assert!(read.pages.len() >= 3, "{}", read.pages.len());
    let text = read.text();
    let said: std::collections::BTreeSet<&str> = text.split_whitespace().collect();
    for w in &words {
        assert!(said.contains(w.as_str()), "{w} was lost");
    }
    for page in &read.pages[1..] {
        assert_eq!(page.drawn[0].text, "left", "the header is not drawn again");
    }
}

#[test]
fn headings_are_bookmarks_and_every_page_says_which_it_is() {
    let pdf = document(
        "# One\n\n## Two\n\ntext\n\n### Three\n\nmore\n",
        "t",
        None,
        Paper::A4,
    );
    let read = back::read(&pdf).unwrap();
    for title in ["One", "Two", "Three"] {
        assert!(
            read.bookmarks.iter().any(|b| b == title),
            "{:?}",
            read.bookmarks
        );
    }
    assert!(read.text().contains("Page 1 of 1"));
    assert_eq!(read.pages[0].size, Paper::A4.size());
}

#[test]
fn a_file_whose_table_of_objects_is_wrong_does_not_read() {
    let pdf = document("# One\n\ntext\n", "t", None, Paper::Letter);
    assert!(back::read(&pdf).is_ok());
    // The table saying the first object starts one byte later than it does.
    assert!(
        pdf.contains("\n0000000009 00000 n \n"),
        "the setup: the first object starts at byte 9"
    );
    let moved = pdf.replacen("\n0000000009 00000 n \n", "\n0000000010 00000 n \n", 1);
    let refused = back::read(&moved).unwrap_err();
    assert!(refused.contains("not where the table says"), "{refused}");
}

#[test]
fn links_are_clickable() {
    let pdf = document(
        "See [the rule](https://example.com/a(b)).\n",
        "t",
        None,
        Paper::Letter,
    );
    let read = back::read(&pdf).unwrap();
    assert_eq!(read.pages[0].links, ["https://example.com/a(b"]);
}
