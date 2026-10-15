//! Backlog 0576 (from sweep 3, 0561): an SVG refused by its `.svg` name is sent again named `.gif`. An app that judges
//! by the name alone takes it and may serve it as the drawing it is, script and all; then the refusal was only of the
//! name, and it is a finding rather than a credit.

use super::super::fake_app::*;
use super::tests::with_upload;
use super::*;

fn run(flaws: Flaws, serves_at: Option<&str>) -> Outcome {
    run_against(flaws, &with_upload(serves_at, None))
}

fn credit(o: &Outcome) -> Option<&crate::Verified> {
    o.verified
        .iter()
        .find(|v| v.check_id == UPLOAD_SVG_SCRIPT.rule_id)
}

fn found(o: &Outcome) -> Option<&crate::Finding> {
    o.findings
        .iter()
        .find(|f| f.rule_id == UPLOAD_SVG_SCRIPT.rule_id)
}

/// The setup every case rests on: the `.svg` was refused, and the same image named `.gif` was sent.
fn refused_by_name_then_sent_as_gif(o: &Outcome) {
    assert!(
        o.steps
            .iter()
            .any(|s| s == "sent an SVG image carrying a script: refused"),
        "the setup: the `.svg` was refused: {:?}",
        o.steps
    );
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("sent the same SVG image named `.gif`")),
        "the setup: the same image was sent named `.gif`: {:?}",
        o.steps
    );
}

/// An app that refuses the `.svg` name, does not look inside a `.gif`, and sets the type from what the file holds.
fn by_name_only() -> Flaws {
    Flaws {
        refuses_svg: true,
        unchecked_contents_ok: true,
        types_by_contents: true,
        ..Default::default()
    }
}

#[test]
fn an_svg_refused_by_name_but_served_as_a_drawing_named_gif_is_found() {
    let o = run(by_name_only(), Some("/files/{name}"));
    refused_by_name_then_sent_as_gif(&o);
    let f = found(&o).unwrap_or_else(|| panic!("not found: {:?}", o.steps));
    assert_eq!(f.severity, Severity::High);
    assert!(
        f.description.contains("`image/svg+xml`"),
        "{}",
        f.description
    );
    assert!(
        f.description.contains("the refusal was of the name"),
        "{}",
        f.description
    );
    assert_eq!(
        f.evidence,
        ["upload-svg-as-gif", "upload-svg-as-gif-fetch"],
        "the finding names the two requests it rests on"
    );
    assert!(credit(&o).is_none(), "credited a refusal of the name only");
}

#[test]
fn served_as_an_attachment_it_is_still_found_but_less_serious() {
    let o = run(
        Flaws {
            svg_as_attachment: true,
            ..by_name_only()
        },
        Some("/files/{name}"),
    );
    refused_by_name_then_sent_as_gif(&o);
    let f = found(&o).unwrap_or_else(|| panic!("not found: {:?}", o.steps));
    assert_eq!(f.severity, Severity::Medium);
}

#[test]
fn an_svg_named_gif_and_refused_too_is_credited_and_says_so() {
    // The app looks inside a `.gif`: an SVG is not one.
    let o = run(
        Flaws {
            refuses_svg: true,
            ..Default::default()
        },
        Some("/files/{name}"),
    );
    refused_by_name_then_sent_as_gif(&o);
    let c = credit(&o).unwrap_or_else(|| panic!("not credited: {:?}", o.not_assessed));
    assert!(
        c.scope.contains("named `.gif` was refused too"),
        "{}",
        c.scope
    );
    assert!(c.in_part);
    assert!(found(&o).is_none());
}

#[test]
fn an_svg_named_gif_and_served_as_a_gif_is_credited_and_says_what_it_was_served_as() {
    // Taken, but served by its name as a GIF, which a browser does not run as a drawing.
    let o = run(
        Flaws {
            refuses_svg: true,
            unchecked_contents_ok: true,
            ..Default::default()
        },
        Some("/files/{name}"),
    );
    refused_by_name_then_sent_as_gif(&o);
    assert!(found(&o).is_none(), "{:?}", o.findings);
    let c = credit(&o).unwrap_or_else(|| panic!("not credited: {:?}", o.not_assessed));
    assert!(c.scope.contains("does not run as a drawing"), "{}", c.scope);
}

#[test]
fn taken_named_gif_with_nowhere_to_fetch_it_is_not_assessed() {
    let o = run(by_name_only(), None);
    refused_by_name_then_sent_as_gif(&o);
    assert!(credit(&o).is_none());
    assert!(found(&o).is_none());
    assert!(
        o.not_assessed.iter().any(|(ids, why)| ids == "V1.3.4"
            && why.contains("taken named `.gif`")
            && why.contains("no `serves-at`")),
        "{:?}",
        o.not_assessed
    );
}
