//! Backlog 0555: in the three-tool prompt trial of 14 October 2026, prompts copied from the setup page arrived with
//! their steps renumbered: Cursor pasted prompt 6's as "8." to "13.", VS Code as "3." to "8.". The steps were Markdown
//! list items, and a list pasted into a chat is numbered by the chat. Each step now starts with the words "Step N." on a
//! paragraph of its own, which a copy keeps as written, so "from step 5" still names the step it means.

use std::path::Path;

fn read(relative: &str) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join(relative)).unwrap()
}

/// Every prompt on the setup page, by its marker name, with its quoted lines; and the guide's build prompt.
fn prompts() -> Vec<(String, Vec<String>)> {
    let page = read("docs/prompts/setup.md");
    let mut found: Vec<(String, Vec<String>)> = Vec::new();
    let mut open: Option<(String, Vec<String>)> = None;
    for line in page.lines() {
        let marker = line
            .strip_prefix("<!-- ")
            .and_then(|l| l.strip_suffix(" -->"));
        match (marker.and_then(|m| m.strip_suffix(":start")), &mut open) {
            (Some(name), None) => open = Some((name.to_string(), Vec::new())),
            (Some(name), Some(_)) => panic!("{name} starts inside another prompt"),
            (None, Some((name, lines))) => {
                if marker == Some(&format!("{name}:end")) {
                    found.push(open.take().unwrap());
                } else if line.starts_with('>') {
                    lines.push(line.to_string());
                }
            }
            (None, None) => {}
        }
    }
    assert!(open.is_none(), "a prompt with no end marker");
    let guide = read("docs/GETTING-STARTED.md");
    let start = guide
        .find("## 4. Start the build with this prompt")
        .expect("the guide's step 4");
    let lines = guide[start..]
        .lines()
        .skip_while(|l| !l.starts_with('>'))
        .take_while(|l| l.starts_with('>'))
        .map(str::to_string)
        .collect();
    found.push(("the guide's build prompt".to_string(), lines));
    found
}

/// The number of a line that starts a Markdown list item inside a quote: "> 3. " or "> 3) ".
fn list_item(line: &str) -> Option<&str> {
    let text = line.trim_start_matches('>').trim_start();
    let digits = text.len() - text.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    let after = &text[digits..];
    (digits > 0 && (after.starts_with(". ") || after.starts_with(") "))).then(|| &text[..digits])
}

#[test]
fn every_prompt_numbers_its_steps_in_words_from_1() {
    let prompts = prompts();
    // The setup: the page's seven prompts and the guide's were all found, with their steps.
    assert!(
        prompts.len() >= 8,
        "{:?}",
        prompts.iter().map(|p| &p.0).collect::<Vec<_>>()
    );
    let mut with_steps = 0;
    for (name, lines) in &prompts {
        for line in lines {
            assert!(
                list_item(line).is_none(),
                "{name}: a list item, which a chat renumbers when it is pasted: {line}"
            );
        }
        let steps: Vec<usize> = lines
            .iter()
            .filter_map(|l| l.strip_prefix("> Step "))
            .map(|l| l.split('.').next().unwrap().parse().unwrap())
            .collect();
        if steps.is_empty() {
            continue;
        }
        with_steps += 1;
        assert_eq!(
            steps,
            (1..=steps.len()).collect::<Vec<_>>(),
            "{name}: its steps do not run from 1 in order"
        );
        let words = lines
            .iter()
            .map(|l| l.trim_start_matches('>').trim())
            .collect::<Vec<_>>()
            .join(" ");
        for (at, _) in words.match_indices("step ") {
            let n: String = words[at + 5..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect();
            if let Ok(n) = n.parse::<usize>() {
                assert!(
                    (1..=steps.len()).contains(&n),
                    "{name}: names step {n}, which it does not have"
                );
            }
        }
    }
    assert!(with_steps >= 6, "only {with_steps} prompts had steps");
}
