//! Backlog 0532: the preflight says when the test command's runner is not among the packages `install = true`
//! installs, as in the three-tool trial's app, and is quiet or content when it is.

use super::*;
use sv_report::fence::Fence;

/// An app folder with these files, and the preflight's answer about its test runner.
fn said(test: &str, install: bool, files: &[(&str, &str)]) -> Option<(Answer, String)> {
    // A folder of its own per call: the tests run at once.
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("sv-test-runner-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (name, text) in files {
        std::fs::write(dir.join(name), text).unwrap();
    }
    let toml = format!(
        "manifest-version = 1\n[app]\nname = \"t\"\n[stack.run]\nimage = \"python:3.12-slim\"\n\
         start = \"python app.py\"\nhealth = \"/\"\ntest = {test:?}\ninstall = {install}\n"
    );
    let manifest = Manifest::parse(&toml, Path::new("stackvet.toml")).expect("the manifest parses");
    // The setup reaches the manifest, or every "nothing said" below passes for nothing.
    assert_eq!(manifest.stack.run.test.as_deref(), Some(test));
    assert_eq!(manifest.stack.run.install, Some(install));
    let found = item(&manifest, &dir).map(|item| {
        assert_eq!(item.topic, "test-runner");
        (item.answer, item.text(&Fence::none()))
    });
    let _ = std::fs::remove_dir_all(&dir);
    found
}

#[test]
fn the_trial_app_is_told_pytest_is_only_in_requirements_dev() {
    let (answer, words) = said(
        "pytest -q",
        true,
        &[
            ("requirements.txt", "flask==3.0.3\n"),
            (
                "requirements-dev.txt",
                "-r requirements.txt\npytest==8.3.3\n",
            ),
        ],
    )
    .expect("the runner is judged");
    assert_eq!(answer, Answer::Look, "{words}");
    assert!(words.contains("runs `pytest`"), "{words}");
    assert!(words.contains("does not name `pytest`"), "{words}");
    assert!(
        words.contains("requirements-dev.txt") && words.contains("`pytest==8.3.3`"),
        "the line to move is quoted from where it is: {words}"
    );
    assert!(words.contains("pytest: not found"), "{words}");
}

#[test]
fn each_way_of_starting_a_runner_is_read() {
    for (test, files) in [
        (
            "python -m pytest",
            vec![("requirements.txt", "flask==3.0.3\n")],
        ),
        (
            "PYTHONPATH=. pytest tests",
            vec![("requirements.txt", "flask==3.0.3\n")],
        ),
        (
            "npx jest --ci",
            vec![("package.json", r#"{"dependencies":{"express":"4.19.2"}}"#)],
        ),
        (
            "npm test",
            vec![(
                "package.json",
                r#"{"scripts":{"test":"vitest run"},"dependencies":{"express":"4.19.2"}}"#,
            )],
        ),
    ] {
        let (answer, words) =
            said(test, true, &files).unwrap_or_else(|| panic!("`{test}` was not judged"));
        assert_eq!(answer, Answer::Look, "`{test}`: {words}");
    }
}

#[test]
fn a_runner_that_is_installed_looks_right() {
    // The control for the tests above: a runner named where the install reads it is not said to be missing.
    for (test, files) in [
        (
            "pytest -q",
            vec![("requirements.txt", "flask==3.0.3\nPyTest==8.3.3  # tests\n")],
        ),
        (
            "npm test",
            vec![(
                "package.json",
                r#"{"scripts":{"test":"jest"},"devDependencies":{"jest":"29.7.0"}}"#,
            )],
        ),
    ] {
        let (answer, words) =
            said(test, true, &files).unwrap_or_else(|| panic!("`{test}` was not judged"));
        assert_eq!(answer, Answer::Looks, "`{test}`: {words}");
    }
}

#[test]
fn nothing_is_said_without_install_or_for_a_runner_it_does_not_know() {
    let requirements = [("requirements.txt", "flask==3.0.3\n")];
    // Without `install = true` the image brings the runner, which the text cannot show.
    assert_eq!(said("pytest -q", false, &requirements), None);
    for test in [
        "python run_tests.py",
        "python -m unittest",
        "go test ./...",
        "./run-tests.sh",
    ] {
        assert_eq!(said(test, true, &requirements), None, "`{test}`");
    }
}

#[test]
fn the_whole_preflight_says_it() {
    // The answer above reaches `sv preflight` and `stackvet_preflight`, which both read `of`.
    let dir = std::env::temp_dir().join(format!("sv-test-runner-of-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"t\"\n[stack.run]\nimage = \"python:3.12-slim\"\n\
         start = \"python app.py\"\nhealth = \"/\"\ntest = \"pytest -q\"\ninstall = true\n",
    )
    .unwrap();
    std::fs::write(dir.join("requirements.txt"), "flask==3.0.3\n").unwrap();
    std::fs::write(dir.join("app.py"), "print('hi')\n").unwrap();
    let (items, _, _, _) = crate::preflight::of(&dir).expect("the preflight reads the folder");
    let _ = std::fs::remove_dir_all(&dir);
    let runner = items
        .iter()
        .find(|item| item.topic == "test-runner")
        .expect("the preflight answers about the test runner");
    assert_eq!(runner.answer, Answer::Look);
}
