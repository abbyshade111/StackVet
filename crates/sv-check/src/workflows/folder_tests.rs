//! Each workflow fixture gets a folder of its own (backlog 0679). The tests here run on several threads of one process,
//! and the folder was named only by the name a test passed and the process's id, which every test in the process
//! shares. Two pairs of tests passed the same name (`"env"` and `"inherit"`), so one could delete the other's folder
//! while it was being written: on Windows, on 27 October 2026, a test failed writing into a folder that was gone.

use super::tests::app;

#[test]
fn two_fixtures_given_one_name_get_folders_of_their_own() {
    let workflow = "on: [push]\npermissions: { contents: read }\njobs: {}\n";
    let first = app("same-name", &[("first.yml", workflow)], &[]);
    assert!(
        first.join(".github/workflows/first.yml").is_file(),
        "the first fixture must be written"
    );
    let second = app("same-name", &[("second.yml", workflow)], &[]);
    let first_survived = first.join(".github/workflows/first.yml").is_file();
    let apart = first != second;
    let _ = std::fs::remove_dir_all(&first);
    let _ = std::fs::remove_dir_all(&second);
    assert!(
        apart,
        "two fixtures given one name share the folder {}",
        first.display()
    );
    assert!(
        first_survived,
        "making the second fixture removed the first one's files"
    );
}
