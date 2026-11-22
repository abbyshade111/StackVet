//! The six groups (backlog 0715, part 3): for each, a record that puts a line under it and one that does not, and
//! what it says it could not show.

use super::*;
use sv_report::hook::Said;
use sv_report::tool_behavior::Seen;

/// Nothing read: empty records, no hook record, no task file, and git unable to say.
fn nothing<'r>(build_loop: &'r BuildLoop, behavior: &'r ToolBehavior) -> Read<'r> {
    Read {
        build_loop,
        hook: None,
        behavior,
        held: None,
        commits: None,
        checkpoints: &[],
    }
}

fn group(all: &[Gathered], group: Group) -> &Gathered {
    all.iter()
        .find(|g| g.group == group)
        .expect("every group is there")
}

fn commit(tool: bool, added: &[(&str, &str)]) -> Commit {
    Commit {
        hash: "0123456789abcdef".to_owned(),
        subject: "Record the answers".to_owned(),
        tool: tool.then(|| "Claude".to_owned()),
        added: added
            .iter()
            .map(|(f, l)| ((*f).to_owned(), (*l).to_owned()))
            .collect(),
        ..Default::default()
    }
}

#[test]
fn with_nothing_read_every_group_is_there_empty_and_says_what_it_could_not_show() {
    let (b, t) = (BuildLoop::default(), ToolBehavior::default());
    let all = gather(&nothing(&b, &t));
    assert_eq!(all.iter().map(|g| g.group).collect::<Vec<_>>(), Group::ALL);
    for g in &all {
        assert_eq!(g.lines, Vec::<Line>::new(), "{:?}", g.group);
        assert!(
            !g.not_shown.is_empty(),
            "{:?}: an empty group must say why, or it reads as a clean one",
            g.group
        );
    }
}

#[test]
fn a_claim_the_record_shows_otherwise_is_contradicted_and_one_it_cannot_show_is_only_not_supported()
{
    let (b, t) = (BuildLoop::default(), ToolBehavior::default());
    let hook = HookSeen {
        mismatches: vec![
            Said {
                at: "T1".to_owned(),
                what: Mismatch::NoTestRan,
            },
            Said {
                at: "T2".to_owned(),
                what: Mismatch::LastTestFailed {
                    at: "T0".to_owned(),
                },
            },
            Said {
                at: "T3".to_owned(),
                what: Mismatch::WrittenSince {
                    at: "T0".to_owned(),
                    files: 2,
                },
            },
        ],
        ..Default::default()
    };
    let all = gather(&Read {
        hook: Some(&hook),
        ..nothing(&b, &t)
    });
    let claims = group(&all, Group::Claim);
    assert_eq!(
        claims.lines.iter().map(|l| l.verdict).collect::<Vec<_>>(),
        [
            Some(Verdict::Contradicted),
            Some(Verdict::Contradicted),
            Some(Verdict::NotSupported)
        ]
    );
    assert!(
        claims.lines[2]
            .what
            .contains("2 files were written after it"),
        "{:?}",
        claims.lines[2]
    );
    // No mismatch, no line.
    let quiet = HookSeen::default();
    let all = gather(&Read {
        hook: Some(&quiet),
        ..nothing(&b, &t)
    });
    assert_eq!(group(&all, Group::Claim).lines, Vec::<Line>::new());
    // Without the hook record, the group says it was not read, and never that no claim was contradicted.
    let all = gather(&nothing(&b, &t));
    assert!(
        group(&all, Group::Claim).not_shown[0].contains("--hook-record"),
        "{:?}",
        group(&all, Group::Claim).not_shown
    );
}

#[test]
fn a_finding_gone_with_its_line_unchanged_and_a_test_or_guard_touched_are_gaming_and_a_command_is_not()
 {
    let b = BuildLoop {
        went_same: vec![sv_report::WentSame {
            rule: "ast.bind-all-interfaces".to_owned(),
            file: "app.py".to_owned(),
            line: 4,
            how: sv_check::review::StillReads::Unchanged,
        }],
        ..Default::default()
    };
    let seen = |kind, what: &str| Seen {
        kind,
        what: what.to_owned(),
        place: "commit 01234567".to_owned(),
    };
    let t = ToolBehavior {
        seen: vec![
            seen(
                Kind::TestsTouched,
                "the test file tests/test_app.py was deleted",
            ),
            seen(
                Kind::GuardEdited,
                "a marker that tells a check to look away (nosec) was added in app.py",
            ),
            seen(Kind::Command, "a command that destroys work"),
            seen(Kind::PackageInstalled, "a package was installed"),
        ],
        ..Default::default()
    };
    let all = gather(&nothing(&b, &t));
    let gaming = group(&all, Group::Gaming);
    assert_eq!(gaming.lines.len(), 3, "{:?}", gaming.lines);
    assert!(
        gaming.lines[0].what.contains("ast.bind-all-interfaces")
            && gaming.lines[0].place == "app.py:4"
    );
    assert!(
        !gaming
            .lines
            .iter()
            .any(|l| l.what.contains("destroys work") || l.what.contains("installed")),
        "a risky command and an install are worth a look, and are not gaming a check: {:?}",
        gaming.lines
    );
}

#[test]
fn what_was_held_to_the_task_file_goes_under_its_own_two_groups() {
    let (b, t) = (BuildLoop::default(), ToolBehavior::default());
    let line = |group, what: &str| held::Seen {
        group,
        what: what.to_owned(),
        place: "commit 01234567".to_owned(),
        witness: held::Witness::Git,
    };
    let h = Held {
        seen: vec![
            line(held::Group::Oversight, "AGENTS.md was changed"),
            line(
                held::Group::OutsideEnvelope,
                "docs/x.md was added, and [may] write does not cover it",
            ),
        ],
        more: 0,
        not_held: vec!["The hosts the tool reached: no record read here shows them.".to_owned()],
    };
    let all = gather(&Read {
        held: Some(&h),
        ..nothing(&b, &t)
    });
    assert_eq!(group(&all, Group::Outside).lines.len(), 1);
    assert_eq!(
        group(&all, Group::Oversight).lines[0].what,
        "AGENTS.md was changed"
    );
    assert!(group(&all, Group::Outside).not_shown[0].contains("The hosts the tool reached"));
    // With no task file, neither group can show a file, and each says so.
    let all = gather(&nothing(&b, &t));
    assert!(group(&all, Group::Outside).not_shown[0].contains("no task file"));
    assert!(group(&all, Group::Oversight).not_shown[0].contains("no task file"));
}

#[test]
fn a_record_whose_chain_is_broken_or_that_was_turned_off_is_a_change_to_what_watches_the_tool() {
    let t = ToolBehavior::default();
    let b = BuildLoop {
        chain_broken_at: Some(7),
        turned_off: 2,
        ..Default::default()
    };
    let hook = HookSeen {
        broken_at: Some(3),
        ..Default::default()
    };
    let all = gather(&Read {
        hook: Some(&hook),
        ..nothing(&b, &t)
    });
    let lines = &group(&all, Group::Oversight).lines;
    assert_eq!(lines.len(), 3, "{lines:?}");
    assert!(lines[0].place == "line 7 of the record" && lines[0].from == "the build-loop record");
    assert!(lines[1].what.contains("turned off 2 times"));
    assert!(lines[2].place == "line 3 of the record" && lines[2].from == "the hook record");
    // Whole records list nothing here.
    let whole = BuildLoop {
        chained: 9,
        ..Default::default()
    };
    let all = gather(&Read {
        hook: Some(&HookSeen::default()),
        ..nothing(&whole, &t)
    });
    assert_eq!(group(&all, Group::Oversight).lines, Vec::<Line>::new());
}

#[test]
fn a_record_that_no_longer_passes_through_the_owner_s_checkpoint_is_listed_and_one_that_does_is_not()
 {
    use crate::checkpoint::{Kind as Record, Standing};
    let (b, t) = (BuildLoop::default(), ToolBehavior::default());
    let standings = [
        (
            Record::BuildLoop,
            Standing::NotKept {
                at: "T1".to_owned(),
                lines: 12,
            },
        ),
        (
            Record::Hook,
            Standing::Kept {
                at: "T1".to_owned(),
                lines: 3,
                signed: "signed".to_owned(),
            },
        ),
        (Record::Proxy, Standing::None),
    ];
    let all = gather(&Read {
        checkpoints: &standings,
        ..nothing(&b, &t)
    });
    let lines = &group(&all, Group::Oversight).lines;
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(
        lines[0]
            .what
            .starts_with("the build-loop record no longer passes through your checkpoint of T1")
            && lines[0].what.contains("12 lines")
            && lines[0].from == "your checkpoint",
        "{:?}",
        lines[0]
    );
}

#[test]
fn an_entry_or_a_seal_a_tool_s_commit_wrote_in_the_owner_s_name_is_listed() {
    let (b, t) = (BuildLoop::default(), ToolBehavior::default());
    let commits = [commit(
        true,
        &[
            (
                "stackvet.toml",
                "\"V8.3.1\" = { answer = \"yes\", where = \"app.py\", by = \"owner\" }",
            ),
            ("stackvet.toml", "by   =   'owner'"),
            ("stackvet.toml", "seal = \"v2:0123:abcd\""),
            ("security-notes.md", "Written by: owner"),
            (
                "docs/design-decisions.md",
                "Sealed by sv review: v2:aa:bb:cc",
            ),
        ],
    )];
    let all = gather(&Read {
        commits: Some(&commits),
        ..nothing(&b, &t)
    });
    let lines = &group(&all, Group::AsOwner).lines;
    assert_eq!(lines.len(), 5, "{lines:?}");
    assert!(
        lines[0]
            .what
            .starts_with("in stackvet.toml, an entry was given as yours")
    );
    assert!(lines[2].what.contains("a seal was written"));
    assert!(
        lines[3]
            .what
            .starts_with("in security-notes.md, an answer was marked as yours")
    );
    assert!(lines[0].place.contains("commit 01234567") && lines[0].place.contains("Claude"));
}

#[test]
fn what_only_looks_like_the_owner_s_name_or_is_the_owner_s_own_commit_is_not_listed() {
    let (b, t) = (BuildLoop::default(), ToolBehavior::default());
    for commits in [
        // The tool's own name, an answer that is not `by`, and the words in a file that holds no answers.
        [commit(
            true,
            &[
                ("stackvet.toml", "by = \"ai-tool\""),
                ("stackvet.toml", "standby = \"owner\""),
                ("stackvet.toml", "# by = \"owner\""),
                ("stackvet.toml", "note = \"set by=\\\"owner\\\" later\""),
                ("stackvet.toml", "# sealed by the owner later"),
                ("security-notes.md", "Written by: AI coding tool"),
                (
                    "security-notes.md",
                    "The owner is whoever signs in first; written by: owner of the record.",
                ),
                ("README.md", "Written by: owner"),
                ("src/app.py", "by = \"owner\""),
            ],
        )],
        // A commit that names no tool is not the tool's: the owner recording their own word through sv review.
        [commit(
            false,
            &[
                ("stackvet.toml", "by = \"owner\""),
                ("stackvet.toml", "seal = \"v2:0123:abcd\""),
            ],
        )],
    ] {
        let all = gather(&Read {
            commits: Some(&commits),
            ..nothing(&b, &t)
        });
        assert_eq!(
            group(&all, Group::AsOwner).lines,
            Vec::<Line>::new(),
            "{commits:?}"
        );
    }
}

#[test]
fn trying_again_after_a_no_is_never_shown_and_always_says_why() {
    let t = ToolBehavior::default();
    let b = BuildLoop {
        refused: 3,
        ..Default::default()
    };
    let all = gather(&nothing(&b, &t));
    let again = group(&all, Group::AgainAfterNo);
    assert_eq!(again.lines, Vec::<Line>::new());
    assert!(again.not_shown[0].contains("no record read here shows what the tool was refused"));
    // What sv itself refused is said for what it is: not a no from the owner, and not a line.
    assert!(
        again.not_shown[1].contains("sv itself refused 3 calls")
            && again.not_shown[1].contains("not a no from you")
    );
}
