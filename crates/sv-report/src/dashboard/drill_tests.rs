//! The drill-down behind the requirements that moved between two runs (ADR-086, answer 3): each one opens onto
//! its status in every kept run, and the page lists every one that moved, not only the first few.

use super::*;
use crate::Status;

fn run(started: &str, statuses: &[(&str, Status)]) -> Run {
    Run {
        format: 4,
        started: started.to_owned(),
        requirements: statuses
            .iter()
            .map(|(id, status)| KeptStatus {
                id: (*id).to_owned(),
                status: *status,
            })
            .collect(),
        ..Run::default()
    }
}

#[test]
fn a_moved_requirement_opens_onto_its_status_in_every_kept_run() {
    let runs = [
        run("2026-10-01T10:00:00Z", &[("V1.1.1", Status::NotVerified)]),
        run(
            "2026-10-05T10:00:00Z",
            &[("V1.1.1", Status::NotVerified), ("V2.2.2", Status::Checked)],
        ),
        run(
            "2026-10-09T10:00:00Z",
            &[("V1.1.1", Status::Checked), ("V2.2.2", Status::Checked)],
        ),
    ];
    let page = over_time(&runs, 0);
    // The sentence is the summary, and it is what the drill-down opens.
    assert!(
        page.contains("<summary>V1.1.1: not verified then, checked now</summary>"),
        "{page}"
    );
    // Its row shows the status in each of the three runs, oldest first.
    let row = status_across(&runs, "V1.1.1");
    assert_eq!(row.matches("not verified</td>").count(), 2, "{row}");
    assert_eq!(row.matches("<td>checked</td>").count(), 1, "{row}");
    assert!(
        row.find("2026-10-01").unwrap() < row.find("2026-10-09").unwrap(),
        "{row}"
    );
}

#[test]
fn every_requirement_that_moved_is_listed_not_only_the_first_few() {
    // Fifteen requirements, each moving from not verified to checked.
    let fifteen = |status: Status| -> Vec<KeptStatus> {
        (1..=15)
            .map(|n| KeptStatus {
                id: format!("V{n}.1.1"),
                status,
            })
            .collect()
    };
    let runs = [
        Run {
            format: 4,
            started: "2026-10-01T10:00:00Z".to_owned(),
            requirements: fifteen(Status::NotVerified),
            ..Run::default()
        },
        Run {
            format: 4,
            started: "2026-10-09T10:00:00Z".to_owned(),
            requirements: fifteen(Status::Checked),
            ..Run::default()
        },
    ];
    let page = over_time(&runs, 0);
    assert_eq!(page.matches("<details>").count(), 15, "{page}");
}

#[test]
fn a_run_that_kept_no_statuses_or_did_not_finish_is_said_in_its_row() {
    let older = Run {
        format: 1,
        started: "2026-10-01T10:00:00Z".to_owned(),
        ..Run::default()
    };
    let failed = Run {
        format: 4,
        started: "2026-10-05T10:00:00Z".to_owned(),
        outcome: Outcome::Failed,
        ..Run::default()
    };
    let runs = [
        older,
        failed,
        run("2026-10-07T10:00:00Z", &[("V1.1.1", Status::Checked)]),
        run(
            "2026-10-09T10:00:00Z",
            &[("V1.1.1", Status::NeedsAttention)],
        ),
    ];
    let row = status_across(&runs, "V1.1.1");
    assert!(
        row.contains("not kept by the sv that made this run"),
        "{row}"
    );
    assert!(row.contains("the run did not finish"), "{row}");
    assert!(row.contains("<td>needs attention</td>"), "{row}");
}
