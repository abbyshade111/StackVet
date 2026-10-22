use super::{Counts, Examined, ExaminedState, Status, lede};

#[test]
fn the_longest_matching_entry_decides_and_no_entry_means_not_looked_for() {
    let entries = vec![
        Examined::ran("config."),
        Examined::not_run("config.secrets-file-committed", "not a git repository"),
        Examined::partly("ast.", "no parser for objective-c"),
    ];
    let state = |rule: &str| Examined::deciding(&entries, rule).map(|e| e.state);
    assert_eq!(state("config.versions-pinned"), Some(ExaminedState::Ran));
    assert_eq!(
        state("config.secrets-file-committed"),
        Some(ExaminedState::NotRun)
    );
    assert_eq!(state("ast.shell-command"), Some(ExaminedState::Partly));
    assert_eq!(state("bandit.B314"), None);
}

#[test]
fn every_status_has_a_row_and_the_rows_add_up() {
    // An exhaustive match: a status added to the enum fails to compile here until it is given
    // a place in `Status::ALL`, which every count table is made from (deep review R5).
    let place = |s: Status| match s {
        Status::NeedsAttention => 0,
        Status::Checked => 1,
        Status::CheckedInPart => 2,
        Status::AppTested => 3,
        Status::Documented => 4,
        Status::ByHand => 5,
        Status::Attested => 6,
        Status::Stated => 7,
        Status::NotVerified => 8,
    };
    for (i, s) in Status::ALL.iter().enumerate() {
        assert_eq!(place(*s), i, "{s:?}");
    }
    let c = Counts {
        applicable: 45,
        needs_attention: 1,
        checked: 2,
        checked_in_part: 9,
        app_tested: 8,
        documented: 3,
        by_hand: 4,
        attested: 5,
        stated: 6,
        not_verified: 7,
        ..Counts::default()
    };
    let rows = c.by_status();
    assert_eq!(rows.iter().map(|(_, n)| n).sum::<usize>(), c.applicable);
    assert_eq!(rows.map(|(_, n)| n), [1, 2, 9, 8, 3, 4, 5, 6, 7]);
    assert_eq!(
        c.looked_at_by_a_check() + c.app_tested + c.on_somebodys_word() + c.not_verified,
        45
    );
    // The app's own tests are never read as a check of `sv`'s.
    assert!(
        Status::AppTested
            .applies_row()
            .contains("not a check of sv's"),
        "{}",
        Status::AppTested.applies_row()
    );
    // The rows that rest on somebody's word say so, and none of them reads as a check.
    for s in [Status::ByHand, Status::Attested, Status::Stated] {
        assert!(
            s.applies_row().contains("rests on your"),
            "{}",
            s.applies_row()
        );
    }
    let lede = lede(&c, "**", "**");
    assert_eq!(
        lede,
        "45 requirements apply to this app. Of those, **12 have been looked at by something**, \
         **8 have been tested only by the app's own tests** (written by your AI coding tool, \
         and not a check of `sv`'s), **18 rest only on somebody's word** (yours, or your AI \
         coding tool's, which nothing here repeated), and **7 have not been looked at at all**."
    );
}

#[test]
fn states_are_spelled_as_report_json_readers_expect() {
    let json = serde_json::to_value(vec![
        Examined::ran("sbom."),
        Examined::nothing_to_examine("gosec.", "this app has no code in go"),
    ])
    .unwrap();
    assert_eq!(json[0]["state"], "ran");
    assert!(
        json[0].get("why").is_none(),
        "a clean run carries no reason"
    );
    assert_eq!(json[1]["state"], "nothing-to-examine");
}
