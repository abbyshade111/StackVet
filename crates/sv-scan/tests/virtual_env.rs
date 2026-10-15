//! A Python virtual environment under a name of its own is left out, and named (backlog 0595). In the trial of 15
//! October 2026 the app's `env/` (made by `python3 -m venv env`) was walked as the app's code, and its four
//! `env/bin/python*` links made three check families "partly", with neither the links nor the folder named.

use sv_scan::files::Listing;

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-venv-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[cfg(unix)]
#[test]
fn a_virtual_environment_under_any_name_is_left_out_and_named() {
    let dir = scratch("env");
    std::fs::write(dir.join("app.py"), "print('hi')\n").unwrap();
    let env = dir.join("env");
    std::fs::create_dir_all(env.join("lib/python3.12/site-packages/requests")).unwrap();
    std::fs::create_dir_all(env.join("bin")).unwrap();
    std::fs::write(env.join("pyvenv.cfg"), "home = /usr/bin\n").unwrap();
    std::fs::write(
        env.join("lib/python3.12/site-packages/requests/api.py"),
        "def get(): pass\n",
    )
    .unwrap();
    std::os::unix::fs::symlink("/usr/bin/python3", env.join("bin/python")).unwrap();
    let listing = Listing::of(&dir);
    std::fs::remove_dir_all(&dir).ok();
    let files: Vec<&str> = listing.files.iter().map(|f| f.relative.as_str()).collect();
    assert_eq!(files, vec!["app.py"], "only the app's own file is read");
    assert!(
        listing.links.is_empty(),
        "the environment's links are not the app's: {:?}",
        listing.links
    );
    assert_eq!(listing.skipped.len(), 1, "{:?}", listing.skipped);
    assert_eq!(listing.skipped[0].0, "env");
    assert!(
        listing.skipped[0].1.contains("Python virtual environment"),
        "{}",
        listing.skipped[0].1
    );
}

#[cfg(unix)]
#[test]
fn a_folder_that_only_links_to_a_pyvenv_cfg_or_has_none_is_read() {
    let dir = scratch("not-env");
    // A folder named `env` with no `pyvenv.cfg` holds the app's own code.
    std::fs::create_dir_all(dir.join("env")).unwrap();
    std::fs::write(dir.join("env/settings.py"), "DEBUG = False\n").unwrap();
    // A `pyvenv.cfg` that is a link does not make a folder an environment.
    std::fs::create_dir_all(dir.join("lib")).unwrap();
    std::fs::write(dir.join("lib/util.py"), "X = 1\n").unwrap();
    std::fs::write(dir.join("elsewhere.cfg"), "home = /usr/bin\n").unwrap();
    std::os::unix::fs::symlink(dir.join("elsewhere.cfg"), dir.join("lib/pyvenv.cfg")).unwrap();
    let listing = Listing::of(&dir);
    std::fs::remove_dir_all(&dir).ok();
    let files: Vec<&str> = listing.files.iter().map(|f| f.relative.as_str()).collect();
    assert!(files.contains(&"env/settings.py"), "{files:?}");
    assert!(files.contains(&"lib/util.py"), "{files:?}");
    assert!(listing.skipped.is_empty(), "{:?}", listing.skipped);
}
