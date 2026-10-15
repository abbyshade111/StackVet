//! Each prompt on the setup page ends with the version of its text (backlog 0588), so a prompt pasted into an AI coding
//! tool carries it, and a trial's record can say which text the tool read. The stamp names the prompt, the day its text
//! last changed, and a fingerprint of its words: the first eight hex characters of the SHA-256 of its quoted lines, the
//! stamp left out, each with its `>` and the spaces around it taken off, joined with single spaces. A commit cannot name
//! itself; `git log -S "fingerprint ..."` finds the one that wrote a stamp. `tools/prompt_stamps.py` writes them.

use std::path::Path;

fn read(relative: &str) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join(relative)).unwrap()
}

const PROMPTS: [(&str, &str); 8] = [
    ("setup-step-1", "prompt 1"),
    ("setup-step-2", "prompt 2"),
    ("setup-prompt", "prompt 3 for a Mac or Linux"),
    ("setup-windows", "prompt 3 for Windows"),
    ("setup-step-4", "prompt 4"),
    ("setup-step-5", "prompt 5"),
    ("setup-step-6", "prompt 6"),
    ("setup-step-7", "prompt 7"),
];

const RUN: &str = "run `python3 tools/prompt_stamps.py` to write the stamps";

fn is_stamp(line: &str) -> bool {
    line.starts_with("> *(StackVet setup ")
}

/// The quoted lines between a prompt's markers. The setup is asserted: a marker renamed would leave nothing to read.
fn quoted(page: &str, marker: &str) -> Vec<String> {
    let start = page
        .find(&format!("<!-- {marker}:start -->"))
        .unwrap_or_else(|| panic!("{marker}: start marker"));
    let end = page
        .find(&format!("<!-- {marker}:end -->"))
        .unwrap_or_else(|| panic!("{marker}: end marker"));
    let lines: Vec<String> = page[start..end]
        .lines()
        .filter(|l| l.starts_with('>'))
        .map(str::to_owned)
        .collect();
    assert!(lines.len() > 3, "{marker}: a prompt between its markers");
    lines
}

fn fingerprint(lines: &[String]) -> String {
    let words = lines
        .iter()
        .filter(|l| !is_stamp(l))
        .map(|l| l.trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ");
    let flat = words.split_whitespace().collect::<Vec<_>>().join(" ");
    sv_cli::bundle::sha256(flat.as_bytes())[..8].to_owned()
}

/// The stamp line a prompt's text should end with, but for its date, which is the stamp's own.
fn expected_prefix_and_suffix(name: &str, fingerprint: &str) -> (String, String) {
    (
        format!("> *(StackVet setup {name}, text of "),
        format!(", fingerprint {fingerprint})*"),
    )
}

#[test]
fn every_prompt_ends_with_the_stamp_of_its_own_text() {
    let page = read("docs/prompts/setup.md");
    for (marker, name) in PROMPTS {
        let lines = quoted(&page, marker);
        let last = lines.last().unwrap();
        let (prefix, suffix) = expected_prefix_and_suffix(name, &fingerprint(&lines));
        assert!(
            last.starts_with(&prefix) && last.ends_with(&suffix),
            "{name} ends with {last:?}, not its stamp ({prefix}<date>{suffix}): {RUN}"
        );
        let date = &last[prefix.len()..last.len() - suffix.len()];
        let parts: Vec<&str> = date.split(' ').collect();
        assert!(
            parts.len() == 3
                && parts[0].parse::<u8>().is_ok_and(|d| (1..=31).contains(&d))
                && parts[2].len() == 4
                && parts[2].parse::<u16>().is_ok(),
            "{name}: the stamp's date reads as a date ({date:?})"
        );
        assert_eq!(
            lines.iter().filter(|l| is_stamp(l)).count(),
            1,
            "{name}: one stamp, at its end"
        );
    }
}

#[test]
fn the_guides_copy_of_prompt_5_carries_the_pages_stamp() {
    let page = read("docs/prompts/setup.md");
    let ours = quoted(&page, "setup-step-5").pop().unwrap();
    let guide = read("docs/GETTING-STARTED.md");
    let at = guide
        .find("## 4. Start the build with this prompt")
        .expect("the guide's step 4");
    let theirs: Vec<&str> = guide[at..]
        .lines()
        .skip_while(|l| !l.starts_with('>'))
        .take_while(|l| l.starts_with('>'))
        .collect();
    assert!(theirs.len() > 3, "the setup: the guide's prompt was found");
    assert_eq!(theirs.last().copied(), Some(ours.as_str()), "{RUN}");
}

#[test]
fn a_word_changed_changes_the_fingerprint() {
    // The setup for the check above: a changed word reaches the fingerprint, and the stamp and the quote marks do not.
    let lines = |text: &str| -> Vec<String> { text.lines().map(str::to_owned).collect() };
    let base = fingerprint(&lines("> Look at my computer\n> and say what is there."));
    assert_eq!(
        base,
        fingerprint(&lines(
            ">   Look at my computer\n> and say what is there.\n>\n> *(StackVet setup prompt 1, text of 1 May 2026, fingerprint 00000000)*"
        ))
    );
    assert_ne!(
        base,
        fingerprint(&lines("> Look at my computer\n> and say what is here."))
    );
}
