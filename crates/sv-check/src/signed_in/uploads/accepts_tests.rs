//! An app that takes only some kinds of file is judged with the kind `accepts` names (backlog 0584, ADR-046, Later):
//! the game site in the Opus run took single-file HTML pages only, refused the ordinary GIF, and every upload
//! requirement stopped there with no way to say what it takes.

use super::super::fake_app::*;
use super::kind::chosen;
use super::tests::with_upload;
use super::*;

const GATE: &str = "V5.2.1, V5.2.2, V5.3.1, V3.2.1";

fn html_only(more: Flaws) -> Flaws {
    Flaws {
        only_html_pages: true,
        ..more
    }
}

fn accepting(accepts: &[&str]) -> UsersSection {
    let mut u = with_upload(Some("/files/{name}"), Some(UPLOAD_LIMIT as u64));
    u.upload.as_mut().unwrap().accepts = Some(accepts.iter().map(|a| (*a).to_owned()).collect());
    u
}

fn gate(o: &Outcome) -> Option<&String> {
    o.not_assessed
        .iter()
        .find(|(ids, _)| ids == GATE)
        .map(|(_, why)| why)
}

#[test]
fn without_accepts_an_html_only_app_stops_at_the_ordinary_file_and_is_told_how_to_say_so() {
    let o = run_against(
        html_only(Flaws::default()),
        &with_upload(Some("/files/{name}"), Some(UPLOAD_LIMIT as u64)),
    );
    let why = gate(&o).expect("nothing can be judged");
    assert!(why.contains("`sv-probe.gif`"), "{why}");
    assert!(why.contains("`accepts`"), "{why}");
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == CONTENT_MISMATCH.rule_id)
    );
}

#[test]
fn with_accepts_the_checks_send_that_kind_and_credit_what_it_refuses() {
    let o = run_against(html_only(Flaws::default()), &accepting(&["text/html"]));
    assert!(gate(&o).is_none(), "{:?}", o.not_assessed);
    assert!(
        o.steps
            .iter()
            .any(|s| s == "uploaded an ordinary HTML page to /upload"),
        "{:?}",
        o.steps
    );
    for rule in [OVERSIZED_FILE.rule_id, CONTENT_MISMATCH.rule_id] {
        assert!(
            o.verified.iter().any(|v| v.check_id == rule),
            "{rule}: {:?} / {:?}",
            o.verified,
            o.not_assessed
        );
    }
    let mismatch = o
        .verified
        .iter()
        .find(|v| v.check_id == CONTENT_MISMATCH.rule_id)
        .unwrap();
    assert!(
        mismatch
            .scope
            .contains("a file named .html whose contents are not an HTML page"),
        "{}",
        mismatch.scope
    );
}

#[test]
fn with_accepts_each_fault_is_still_found_in_that_kind() {
    let o = run_against(
        html_only(Flaws {
            unchecked_contents_ok: true,
            ..Default::default()
        }),
        &accepting(&["html"]),
    );
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == CONTENT_MISMATCH.rule_id)
        .expect("a .html that is not a page, taken");
    assert!(
        found.description.contains("holding no HTML page at all"),
        "{}",
        found.description
    );

    let o = run_against(
        html_only(Flaws {
            oversized_upload_ok: true,
            ..Default::default()
        }),
        &accepting(&["html"]),
    );
    assert!(
        o.findings
            .iter()
            .any(|f| f.rule_id == OVERSIZED_FILE.rule_id),
        "{:?}",
        o.findings
    );
}

#[test]
fn accepts_naming_nothing_sv_makes_sends_nothing_and_says_so() {
    let o = run_against(Flaws::default(), &accepting(&["docx", "application/zip"]));
    let why = gate(&o).expect("not assessed");
    assert!(why.contains("`docx`, `application/zip`"), "{why}");
    assert!(why.contains("`.html`"), "{why}");
    assert!(
        !o.steps.iter().any(|s| s.starts_with("uploaded")),
        "{:?}",
        o.steps
    );
}

#[test]
fn the_first_kind_sv_makes_is_chosen_in_the_order_given() {
    let list = |l: &[&str]| l.iter().map(|a| (*a).to_owned()).collect::<Vec<_>>();
    let ext = |l: &[&str]| chosen(Some(&list(l))).map(|k| k.ext);
    assert_eq!(chosen(None).map(|k| k.ext), Ok("gif"));
    assert_eq!(ext(&["docx", ".HTM", "png"]), Ok("html"));
    assert_eq!(ext(&["image/png", "text/html"]), Ok("png"));
    assert_eq!(ext(&["image/*"]), Ok("gif"));
    assert_eq!(ext(&[" image/JPEG "]), Ok("jpg"));
    assert!(ext(&[]).unwrap_err().contains("is empty"));
}

#[test]
fn a_name_that_climbs_out_is_sent_as_that_kind_so_a_refusal_of_the_kind_is_not_credited() {
    // Sent as a GIF, the file would be refused for being a GIF, and the refusal read as one of the name.
    let o = run_against(
        html_only(Flaws {
            upload_path_traversal: true,
            ..Default::default()
        }),
        &accepting(&["html"]),
    );
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == UPLOAD_PATH_TRAVERSAL.rule_id),
        "{:?}",
        o.verified
    );
    assert!(
        o.findings
            .iter()
            .any(|f| f.rule_id == UPLOAD_PATH_TRAVERSAL.rule_id),
        "{:?} / {:?}",
        o.findings,
        o.not_assessed
    );
}
