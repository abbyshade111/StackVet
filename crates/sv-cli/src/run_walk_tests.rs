//! The files a run asks for are the ones its caller worked out from its own walk of the folder, and
//! the run does not walk the folder again for them (backlog 0354). Before, `anonymous_requests` and
//! `running_app_evidence` each listed the app folder anew, so `sv report --run` walked it five times.

use super::*;

const MANIFEST: &str = "manifest-version = 1\n[app]\nname = \"Fixture\"\naudience = \"customers\"\n\
                        deployment = \"internet\"\n[stack]\nlanguages = [\"python\"]\n[stack.run]\n\
                        image = \"python:3.12-slim\"\nstart = \"python app.py\"\nhealth = \"/\"\n";

fn asks_for(requests: &[probes::ProbeRequest], path: &str) -> bool {
    requests.iter().any(|r| r.path == path)
}

#[test]
fn the_private_files_asked_for_are_the_ones_given_and_the_folder_is_not_walked_again() {
    let dir = std::env::temp_dir().join(format!("sv-run-walk-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    // A settings file the probes ask the running app for, long enough to be asked about.
    std::fs::write(
        dir.join(".env"),
        "DATABASE_URL=sqlite:///a-local-file-for-this-test.db\n",
    )
    .unwrap();
    let manifest = sv_manifest::Manifest::parse(MANIFEST, Path::new("stackvet.toml"))
        .expect("the manifest written here");
    let plan = RunPlan::from_manifest(&manifest, &dir).expect("it declares how to run");

    // The setup: one walk of the folder finds the file, so it is really there to be asked for.
    let walked = private_files(&sv_scan::files::Listing::of(&dir));
    assert!(walked.iter().any(|f| f.file == ".env"), "{walked:?}");
    let with = anonymous_requests(&plan, &walked);
    // The point: given no files, the run asks for none, though the folder holds one; it would ask
    // for `.env` here if it walked the folder itself.
    let without = anonymous_requests(&plan, &[]);
    std::fs::remove_dir_all(&dir).ok();

    assert!(asks_for(&with, "/.env"), "the walked file is not asked for");
    assert!(
        !asks_for(&without, "/.env"),
        "the run walked the folder again for its private files"
    );
}
