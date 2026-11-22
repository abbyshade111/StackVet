//! The required `test` check holds the suite to a floor on the number of test programs it ran (backlog 0720; the
//! security review of 21 November 2026, A5). Each part counts its `test result:` lines, one a program, and passes the
//! count to `test` beside its credits; `test` adds the four and fails below the floor written in `rust.yml`. Tried on
//! 21 November 2026 with the four counts of a real run (376): it passed at 370 and 376, and failed at 377, with one
//! part's count cut to 3, and with one part's count missing.

use std::path::PathBuf;

fn rust_yml() -> String {
    std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/rust.yml"),
    )
    .unwrap()
}

/// One job of the workflow: from its `  name:` line to the next line naming a job at the same depth.
fn job<'a>(text: &'a str, name: &str) -> &'a str {
    let start = text
        .find(&format!("\n  {name}:\n"))
        .unwrap_or_else(|| panic!("the setup: no job {name}"))
        + 1;
    let rest = &text[start..];
    let is_job = |line: &str| {
        line.len() > 3
            && line.starts_with("  ")
            && !line.starts_with("   ")
            && line.ends_with(':')
            && line[2..line.len() - 1]
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    };
    let mut at = rest.find('\n').map_or(rest.len(), |i| i + 1);
    while at < rest.len() {
        let line_end = rest[at..].find('\n').map_or(rest.len(), |i| at + i);
        if is_job(&rest[at..line_end]) {
            return &rest[..at];
        }
        at = line_end + 1;
    }
    rest
}

#[test]
fn every_part_counts_its_test_programs_and_passes_the_count_on() {
    let text = rust_yml();
    let suite = job(&text, "suite");
    assert!(
        suite.contains("part: [rest, check, cli-1, cli-2]"),
        "the setup: the suite's parts"
    );
    assert!(
        suite.contains(r#"bash "$RUNNER_TEMP/part.sh" 2>&1 | tee "$RUNNER_TEMP/part.log""#)
            && suite.contains("status=${PIPESTATUS[0]}"),
        "a part's output is kept, and its exit status is the tests' own"
    );
    assert!(
        suite.contains(
            r#"grep -c '^test result: ' "$RUNNER_TEMP/part.log" > "$RUNNER_TEMP/programs.txt""#
        ),
        "a part does not count its test programs"
    );
    assert!(
        suite.contains("${{ runner.temp }}/programs.txt"),
        "the count is not passed to `test`"
    );
}

#[test]
fn test_fails_below_the_floor_and_on_a_missing_count() {
    let text = rust_yml();
    let test = job(&text, "test");
    let step = &test[test
        .find("- name: The suite ran no fewer test programs than its floor")
        .expect("the floor's step")..];
    let floor: u32 = step
        .lines()
        .find_map(|l| l.trim().strip_prefix("FLOOR: "))
        .expect("the floor is written in the workflow")
        .parse()
        .expect("the floor is a number");
    assert!(
        floor >= 300,
        "the floor ({floor}) is far under the 376 programs a run made on 21 November 2026"
    );
    for part in ["rest", "check", "cli-1", "cli-2"] {
        assert!(step.contains(part), "the floor does not count {part}");
    }
    assert!(
        step.contains("[ -s \"$count\" ] ||") && step.contains("no count of test programs from")
    );
    assert!(step.contains(r#"if [ "$total" -lt "$FLOOR" ]; then"#) && step.contains("exit 1"));
}
