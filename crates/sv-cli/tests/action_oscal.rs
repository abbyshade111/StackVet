//! The Action's `oscal` input (ADR-156, backlog 0678): off unless asked, the file written in the same run as the report
//! (whose seal cannot be shown once the container is gone), into a folder of its own and never the report folder, and
//! kept with the run under a name of its own.

use std::path::PathBuf;

fn action() -> String {
    std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../action.yml"))
        .unwrap()
}

/// The lines that run something, without the comments that say why.
fn code(action: &str) -> String {
    action
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn oscal_is_off_unless_asked_and_written_beside_the_report_folder_not_in_it() {
    let action = action();
    let code = code(&action);
    let input = &action[action
        .find("\n  oscal:")
        .expect("the Action has an oscal input")..];
    // Up to the next input: the next line indented by two spaces exactly.
    let end = input
        .match_indices("\n  ")
        .map(|(i, _)| i)
        .find(|&i| i > 0 && !input[i + 3..].starts_with(' '))
        .unwrap_or(input.len());
    let input = &input[..end];
    assert!(
        input.contains("default: 'false'"),
        "oscal is on by default: {input}"
    );
    assert!(
        code.contains("oscal_dir=\"$out-oscal\""),
        "the OSCAL folder is not one of its own"
    );
    assert!(
        code.contains("args+=(--oscal \"$oscal_dir/stackvet-oscal.json\")"),
        "the file is not written into the OSCAL folder"
    );
    assert!(
        !code.contains("--oscal \"$out/"),
        "the file is written into the report folder"
    );
    // In the same run as the report: one `docker run` carries both the report and the OSCAL folder.
    let run = code
        .lines()
        .find(|l| l.contains("\"${oscal[@]}\""))
        .expect("the Action runs sv with the OSCAL folder");
    assert!(
        run.contains("\"${oscal[@]}\"") && run.contains("\"${args[@]}\""),
        "{run}"
    );
    assert!(
        code.contains("name: ${{ inputs.artifact-name }}-oscal"),
        "the file is not kept with the run"
    );
}

#[test]
fn an_image_that_cannot_write_oscal_is_refused_rather_than_ignored() {
    // An older published image does not know --oscal; asked for the file, the Action says so and fails, rather than
    // passing an option sv would refuse after the whole report had run, or quietly writing none.
    let code = code(&action());
    let asks = code
        .find("grep -q -- '--oscal'")
        .expect("the Action asks the image's help about --oscal");
    let passes = code
        .find("args+=(--oscal")
        .expect("the Action passes --oscal");
    assert!(asks < passes, "the image is asked after --oscal is passed");
}
