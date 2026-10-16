//! Backlog 0604: ticking the checklist before going live in `sv review`. A line about a requirement `sv` accepts a
//! check by hand for becomes that check, sealed; any other becomes a sealed note the next checklist shows as done; a
//! note not sealed, or changed, is not shown as done; and the checklist still says it credits nothing.

use super::super::tests::Scratch;
use super::*;

const MANIFEST: &str = "manifest-version = 1\n[app]\nname = \"Ticks\"\ndeployment = \"internet\"\n\
                        [stack]\nlanguages = [\"python\"]\n";

/// A scratch app with a report sealed where it was written, and the checklist lines that report holds.
/// The folder the report's own checker reads trusted keys from in this test process.
fn report_keys() -> &'static std::path::Path {
    sv_check::seal::key_folder_for_tests(
        std::env::temp_dir().join(format!("sv-review-ticks-keys-{}", std::process::id())),
    )
}

/// Trusts the scratch app's review key where the report checks seals, as one key folder in a person's home does.
fn trust_review_key(s: &Scratch) {
    let line = std::fs::read_to_string(
        s.0.join("config")
            .join("stackvet")
            .join(sv_check::signed::TRUSTED_FILE),
    )
    .expect("the setup: review made its trust line");
    std::fs::create_dir_all(report_keys()).unwrap();
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(report_keys().join(sv_check::signed::TRUSTED_FILE))
        .unwrap();
    std::io::Write::write_all(&mut file, line.as_bytes()).unwrap();
}

fn with_report(name: &str) -> (Scratch, Vec<Offered>) {
    report_keys();
    let s = Scratch::new(name);
    std::fs::write(s.app().join("app.py"), "def home():\n    return 'hi'\n").unwrap();
    std::fs::write(s.app().join("stackvet.toml"), MANIFEST).unwrap();
    write_report(&s);
    let lines = checklist_lines(&s.app().join(sv_frameworks::names::REPORT_DIR))
        .expect("the setup: the sealed report's checklist is read");
    assert!(!lines.is_empty(), "the setup: the checklist has lines");
    (s, lines)
}

fn write_report(s: &Scratch) {
    let app = s.app();
    let folder = app.join(sv_frameworks::names::REPORT_DIR);
    let mut quiet = |_: &str| {};
    sv_cli::report_folder::write_report_folder(
        &app,
        &folder,
        "a test",
        "elsewhere",
        false,
        || {
            sv_cli::assemble_report(
                &app,
                &sv_cli::ReportOptions::reading_only("a test"),
                &sv_cli::Loaded::load()?,
            )
        },
        &mut quiet,
    )
    .unwrap();
}

fn accepted() -> std::collections::BTreeSet<String> {
    sv_check::human::HumanChecks::load(&sv_frameworks::data::file("human-checks.json"))
        .unwrap()
        .checks
        .into_iter()
        .map(|c| c.id)
        .collect()
}

#[test]
fn each_tick_is_recorded_as_a_hand_check_or_a_sealed_note_and_shown_done_next_time() {
    let (s, lines) = with_report("ticked");
    let accepted = accepted();
    let hand: Vec<&Offered> = lines
        .iter()
        .filter(|l| l.requirements.iter().any(|id| accepted.contains(id)))
        .collect();
    // Noted done: every line but those whose requirements all accept a check by hand.
    let notes: Vec<&Offered> = lines
        .iter()
        .filter(|l| {
            l.requirements.is_empty() || l.requirements.iter().any(|id| !accepted.contains(id))
        })
        .collect();
    // The setup: the checklist has lines of both kinds.
    assert!(!hand.is_empty() && !notes.is_empty(), "{lines:?}");

    let typed = "owner\n".repeat(lines.len() + 4);
    let (result, said) = s.run(&typed);
    result.unwrap();
    assert!(said.contains("The checklist before going live:"), "{said}");

    let manifest = sv_manifest::Manifest::load(&s.app().join("stackvet.toml")).unwrap();
    let checker = s.checker();
    for line in &hand {
        for id in line.requirements.iter().filter(|id| accepted.contains(*id)) {
            let entry = manifest
                .checked_by_hand
                .get(id)
                .unwrap_or_else(|| panic!("{id} recorded by hand: {said}"));
            assert_eq!(entry.result, "done");
            assert_eq!(entry.by.as_deref(), Some("owner"));
            // Recorded once, from the first line that names it.
            let first = lines
                .iter()
                .find(|l| l.requirements.contains(id))
                .expect("a line names it");
            assert_eq!(
                entry.how.as_deref(),
                Some(format!("Ticked on the checklist before going live: {}", first.what).as_str())
            );
            assert!(
                sv_check::seal::owner_recorded(
                    &checker,
                    entry.seal.as_deref(),
                    &sv_check::seal::hand_check_fields(id, entry)
                )
                .is_ok(),
                "{id}'s seal holds"
            );
        }
    }
    // A tick widens nothing: no requirement outside the hand-check catalog is recorded as checked by hand.
    assert!(
        manifest
            .checked_by_hand
            .keys()
            .all(|id| accepted.contains(id))
    );
    for line in &notes {
        let entry = manifest
            .checklist_done
            .iter()
            .find(|e| e.key == line.key)
            .unwrap_or_else(|| panic!("{} noted: {said}", line.what));
        assert_eq!(entry.by, "owner");
    }

    // The next report shows each noted line as done, with the day, and still says the checklist credits nothing.
    trust_review_key(&s);
    write_report(&s);
    let folder = s.app().join(sv_frameworks::names::REPORT_DIR);
    let again = checklist_lines(&folder).unwrap();
    for line in &notes {
        assert!(
            again.iter().any(|l| l.key == line.key && l.done),
            "{} done",
            line.what
        );
    }
    let markdown = std::fs::read_to_string(folder.join(sv_report::checklist::FILE)).unwrap();
    assert!(markdown.contains("- [x] "), "{markdown}");
    assert!(
        markdown.contains("Ticked in `sv review` by the owner on"),
        "{markdown}"
    );
    assert!(markdown.contains("It credits nothing"), "{markdown}");

    // Nothing is offered again once ticked.
    let (result, said) = s.run("owner\n");
    result.unwrap();
    assert!(!said.contains("The checklist before going live:"), "{said}");
}

#[test]
fn a_note_not_sealed_or_changed_since_is_not_shown_done() {
    let (s, lines) = with_report("unsealed");
    let accepted = accepted();
    let line = lines
        .iter()
        .find(|l| !l.requirements.iter().any(|id| accepted.contains(id)))
        .expect("the setup: a line that is not a hand check");
    // One with no seal, as the AI coding tool could write it, and one sealed by nobody.
    let manifest = format!(
        "{MANIFEST}\n[[checklist-done]]\nkey = \"{}\"\nwhat = \"{}\"\nby = \"owner\"\non = \"2026-10-16\"\n\n\
         [[checklist-done]]\nkey = \"{}\"\nwhat = \"{}\"\nby = \"owner\"\non = \"2026-10-16\"\nseal = \"v3:x:00\"\n",
        line.key,
        line.what.replace('"', "\\\""),
        line.key,
        line.what.replace('"', "\\\"")
    );
    std::fs::write(s.app().join("stackvet.toml"), manifest).unwrap();
    write_report(&s);
    let again = checklist_lines(&s.app().join(sv_frameworks::names::REPORT_DIR)).unwrap();
    assert!(
        again.iter().any(|l| l.key == line.key && !l.done),
        "an entry whose seal does not hold is not done"
    );
}

#[test]
fn with_no_sealed_report_the_checklist_is_not_offered_and_review_says_why() {
    let s = Scratch::new("no-report");
    std::fs::write(s.app().join("app.py"), "x = 1\n").unwrap();
    std::fs::write(s.app().join("stackvet.toml"), MANIFEST).unwrap();
    let (result, said) = s.run("");
    result.unwrap();
    assert!(
        said.contains("The checklist before going live was not offered"),
        "{said}"
    );
}
