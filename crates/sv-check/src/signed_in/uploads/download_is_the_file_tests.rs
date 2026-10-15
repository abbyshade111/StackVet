//! A file fetched back for its name is the file, not "under 400" (backlog 0560, ADR-119): a redirect to where it
//! is kept has no file name of the file's to judge.

use super::super::fake_app::*;
use super::tests::with_upload;
use super::*;

#[test]
fn a_redirect_to_storage_is_not_a_file_served_without_a_name() {
    let o = run_against(
        Flaws {
            files_redirect_to_storage: true,
            ..Default::default()
        },
        &with_upload(Some("/files/{name}"), Some(UPLOAD_LIMIT as u64)),
    );
    assert!(
        o.steps
            .iter()
            .any(|s| s == "uploaded an ordinary GIF to /upload"),
        "{:?}",
        o.steps
    );
    assert!(
        !o.findings
            .iter()
            .any(|f| f.rule_id == DOWNLOAD_UNNAMED.rule_id),
        "a redirect was read as the file served without a name: {:?}",
        o.findings
    );
    assert!(
        o.not_assessed.iter().any(|(id, why)| id == "V5.4.1"
            && why.contains("answered 302 with something other than the file")),
        "{:?}",
        o.not_assessed
    );
    // The file with the hostile name, the same: not the file, so its name was not seen served.
    assert!(
        o.not_assessed.iter().any(|(id, why)| id == "V5.4.2"
            && why.contains(
                "could not be fetched back from /files/sv-probe;svinjected=1.gif as the file itself"
            )),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn the_file_served_without_a_name_is_still_found() {
    let o = run_against(
        Flaws {
            download_no_filename: true,
            ..Default::default()
        },
        &with_upload(Some("/files/{name}"), Some(UPLOAD_LIMIT as u64)),
    );
    assert!(
        o.findings
            .iter()
            .any(|f| f.rule_id == DOWNLOAD_UNNAMED.rule_id),
        "{:?}",
        o.not_assessed
    );
}
