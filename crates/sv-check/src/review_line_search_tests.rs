//! Backlog 0675: finding the line an entry's fingerprint names reads each line once. On `data/reach.json`, 4,567 lines
//! with hundreds that read the same (`"finding_only": true`), `sv review` sat for about two minutes with nothing on the
//! screen, because every line's fingerprint counted the earlier lines that read the same and worked out again, for each,
//! the lines that set the names it uses.

use super::*;

/// A file shaped like `data/reach.json`: blocks whose lines repeat, some setting a name a later line uses.
fn repeating(blocks: usize) -> Vec<String> {
    let mut lines = vec!["{".to_owned(), " \"rules\": [".to_owned()];
    for i in 0..blocks {
        lines.extend([
            "  {".to_owned(),
            format!("   \"id\": \"rule.number-{i}\","),
            "   \"run\": \"Reads the code\",".to_owned(),
            format!(
                "   \"looks_for\": \"Something the rule finds, number {}\",",
                i % 7
            ),
            "   \"finding_only\": true".to_owned(),
            "  },".to_owned(),
        ]);
        if i % 50 == 0 {
            // A name set, then used, so `reaching` has lines to find.
            lines.push(format!("   value = {i}"));
            lines.push("   \"finding_only\": value".to_owned());
        }
    }
    lines.extend([" ]".to_owned(), "}".to_owned()]);
    lines
}

#[test]
fn every_line_has_the_fingerprint_it_had_one_line_at_a_time() {
    // Small enough that the reference, one line at a time, is quick.
    let lines = repeating(120);
    let all = fingerprints(
        "secrets.default-credential-address",
        "data/reach.json",
        &lines,
    );
    assert_eq!(all.len(), lines.len());
    for (i, fingerprint) in all.iter().enumerate() {
        assert_eq!(
            fingerprint,
            &line_fingerprint(
                "secrets.default-credential-address",
                "data/reach.json",
                &lines,
                i
            ),
            "line {}: {}",
            i + 1,
            lines[i]
        );
    }
    // The setup: lines that read the same are told apart, so the count of earlier ones is in play.
    let repeated = lines
        .iter()
        .filter(|l| l.trim() == "\"finding_only\": true")
        .count();
    assert!(repeated >= 120, "{repeated}");
    let distinct: std::collections::HashSet<&String> = all.iter().collect();
    assert_eq!(distinct.len(), all.len(), "two lines share a fingerprint");
}

#[test]
fn a_line_is_found_in_a_file_the_size_of_reach_json_in_seconds() {
    let dir = crate::test_support::scratch("line-search");
    let lines = repeating(760);
    assert!(
        lines.len() > 4_500,
        "the setup is the size of reach.json: {}",
        lines.len()
    );
    std::fs::create_dir_all(dir.join("data")).unwrap();
    std::fs::write(dir.join("data/reach.json"), lines.join("\n") + "\n").unwrap();
    // Near the end, as line 3616 of 4,567 was.
    let at = lines.len() - 900;
    let fingerprint = line_fingerprint(
        "secrets.default-credential-address",
        "data/reach.json",
        &lines,
        at,
    );
    let started = std::time::Instant::now();
    let found = lines_with_fingerprint(
        &dir,
        "secrets.default-credential-address",
        "data/reach.json",
        &fingerprint,
    );
    let took = started.elapsed();
    assert_eq!(found, vec![(at + 1, lines[at].trim().to_owned())]);
    // 330 seconds before, in a debug build; under a tenth of a second after.
    assert!(took.as_secs_f64() < 2.0, "finding the line took {took:?}");
}
