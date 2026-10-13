//! A copy of `sv` moved out of its build folder, with its data beside it (ADR-036).
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;
use sv_frameworks::paths::Canonical;

/// A scratch folder of this test's own.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-moved-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).unwrap();
        }
    }
}

fn repository_data() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../data")
        .canonical()
        .unwrap()
}

fn data_line(program: &Path, env: Option<&Path>) -> String {
    let mut command = Command::new(program);
    command.arg("--version").env_remove("SV_DATA_DIR");
    if let Some(dir) = env {
        command.env("SV_DATA_DIR", dir);
    }
    let out = command.output().unwrap();
    assert!(out.status.success(), "{out:?}");
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .find_map(|l| l.strip_prefix("data: ").map(str::to_owned))
        .unwrap_or_else(|| panic!("no data line: {out:?}"))
}

#[test]
fn a_copy_of_sv_reads_the_data_beside_it_and_checks_an_app_with_it() {
    // An install: the program in a folder of its own with its data, reached through a link from
    // somewhere else, as `~/.local/bin/sv` reaches `~/.local/share/stackvet/sv`.
    let root = scratch("installed");
    let home = root.join("share/stackvet");
    std::fs::create_dir_all(&home).unwrap();
    std::fs::copy(env!("CARGO_BIN_EXE_sv"), home.join("sv")).unwrap();
    copy_tree(&repository_data(), &home.join("data"));
    std::fs::create_dir_all(root.join("bin")).unwrap();
    std::os::unix::fs::symlink(home.join("sv"), root.join("bin/sv")).unwrap();
    let linked = root.join("bin/sv");

    let said = data_line(&linked, None);
    let app = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
    let checked = Command::new(&linked)
        .args(["check"])
        .arg(&app)
        .env_remove("SV_DATA_DIR")
        .output()
        .unwrap();
    // And `SV_DATA_DIR` still comes first.
    let chosen = data_line(&linked, Some(&repository_data()));
    std::fs::remove_dir_all(&root).ok();

    assert_eq!(
        Path::new(&said),
        home.join("data").canonical().unwrap_or(home.join("data")),
        "the copy read some other data"
    );
    assert!(
        checked.status.code().is_some_and(|c| c < 3),
        "the copy could not check an app: {}",
        String::from_utf8_lossy(&checked.stderr)
    );
    assert_eq!(Path::new(&chosen), repository_data());
}

#[test]
fn a_wrong_sv_data_dir_is_named_and_stops_a_check() {
    let root = scratch("wrong");
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["check"])
        .arg(&root)
        .env("SV_DATA_DIR", root.join("nothing-here"))
        .output()
        .unwrap();
    std::fs::remove_dir_all(&root).ok();
    let said = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(3), "{said}");
    assert!(
        said.contains("SV_DATA_DIR") && said.contains("nothing-here"),
        "{said}"
    );
}

#[test]
fn nothing_but_the_data_finder_reads_from_the_build_folder_at_run_time() {
    // Every `env!("CARGO_MANIFEST_DIR")` outside a test module would be a file read from the build
    // folder whatever `SV_DATA_DIR` or the program's place says, which is the fault this fixed. The
    // test modules and `crates/sv-frameworks/src/data.rs` itself may name it.
    let found = build_folder_reads(&Path::new(env!("CARGO_MANIFEST_DIR")).join(".."));
    assert!(found.len() < 1000, "the walk went somewhere it should not");
    assert!(
        found.is_empty(),
        "read from the build folder at run time: {found:#?}"
    );
}

/// Every place under `crates` (each crate's `src`) that names the build folder outside a test
/// module, as `path:line`.
fn build_folder_reads(crates: &Path) -> Vec<String> {
    scan_src(crates).build_folder
}

/// What `scan_src` finds outside test modules.
struct Scan {
    /// Each `env!("CARGO_MANIFEST_DIR")`, or `option_env!`, however spaced, as `path:line`.
    build_folder: Vec<String>,
    /// Each file under the repository's `data/` folder that `include_str!` or `include_bytes!` compiles
    /// in, named from `data/`, with where (backlog 0293).
    compiled_data: Vec<(String, String)>,
}

fn scan_src(crates: &Path) -> Scan {
    let crates = crates.to_path_buf();
    let data = crates
        .join("../data")
        .canonicalize()
        .unwrap_or_else(|_| crates.join("../data"));
    let mut found = Vec::new();
    let mut compiled_data = Vec::new();
    let mut stack = vec![crates.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            if path.is_dir() {
                // Only each crate's `src`: `tests/` and `build.rs` run where the source is.
                if dir == crates
                    || name == "src"
                    || dir.components().any(|c| c.as_os_str() == "src")
                {
                    stack.push(path);
                }
                continue;
            }
            if !path.extension().is_some_and(|e| e == "rs")
                || path.ends_with("sv-frameworks/src/data.rs")
            {
                continue;
            }
            // A whole file that is a test module, declared `#[cfg(test)] mod name;` by the file
            // above it (`mcp/tests.rs` since 8 October 2026), may name it as a test module may.
            let stem = path.file_stem().unwrap().to_string_lossy().into_owned();
            let declared_for_tests = |parent: PathBuf| {
                std::fs::read_to_string(parent).is_ok_and(|text| {
                    let lines: Vec<&str> = text.lines().collect();
                    lines.iter().enumerate().any(|(n, line)| {
                        line.starts_with("#[cfg(")
                            && line.contains("test")
                            && lines
                                .get(n + 1)
                                .is_some_and(|next| *next == format!("mod {stem};"))
                    })
                })
            };
            // A file at the top of a crate's `src/` is declared by the crate's `lib.rs` or `main.rs`
            // (backlog 0262: `sv-cli/src/resources_beside_quotas_tests.rs` was flagged on 11 October).
            let at_top_of_src = dir.file_name().is_some_and(|n| n == "src");
            if declared_for_tests(dir.join("mod.rs"))
                || declared_for_tests(dir.with_extension("rs"))
                || (at_top_of_src
                    && (declared_for_tests(dir.join("lib.rs"))
                        || declared_for_tests(dir.join("main.rs"))))
            {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            // Each top-level `#[cfg(test)] mod … {` (or `#[cfg(all(test, …))]`) is skipped to its closing `}` at the margin: one
            // followed by the end of the file or by another item, not by more of a string that
            // happens to hold a `}` line.
            let lines: Vec<&str> = text.lines().collect();
            let item_follows = |n: usize| {
                lines[n + 1..]
                    .iter()
                    .find(|l| !l.trim().is_empty())
                    .is_none_or(|next| {
                        [
                            "#[",
                            "#!",
                            "//",
                            "pub",
                            "fn ",
                            "mod ",
                            "use ",
                            "const ",
                            "static ",
                            "struct ",
                            "enum ",
                            "impl",
                            "type ",
                            "macro_rules!",
                        ]
                        .iter()
                        .any(|start| next.starts_with(start))
                    })
            };
            let mut in_test = false;
            for (n, line) in lines.iter().enumerate() {
                if !in_test
                    && line.starts_with("#[cfg(")
                    && line.contains("test")
                    && lines
                        .get(n + 1)
                        .is_some_and(|next| next.starts_with("mod ") && next.ends_with('{'))
                {
                    in_test = true;
                } else if in_test && *line == "}" && item_follows(n) {
                    in_test = false;
                } else if !in_test {
                    // Spaces inside the macro do not hide it, and `option_env!` names the same folder.
                    let packed: String = line.split_whitespace().collect();
                    if packed.contains("env!(\"CARGO_MANIFEST_DIR\"") {
                        found.push(format!("{}:{}", path.display(), n + 1));
                    }
                    for include in ["include_str!(\"", "include_bytes!(\""] {
                        let Some((_, rest)) = packed.split_once(include) else {
                            continue;
                        };
                        let Some((named, _)) = rest.split_once('"') else {
                            continue;
                        };
                        let file = path.parent().unwrap().join(named);
                        let file = file.canonicalize().unwrap_or(file);
                        if let Ok(inside) = file.strip_prefix(&data) {
                            compiled_data.push((
                                inside.to_string_lossy().replace('\\', "/"),
                                format!("{}:{}", path.display(), n + 1),
                            ));
                        }
                    }
                }
            }
        }
    }
    Scan {
        build_folder: found,
        compiled_data,
    }
}

#[test]
fn the_data_compiled_in_is_exactly_what_data_readme_says_is() {
    // Backlog 0293: two data files are compiled in, bypassing `sv_frameworks::data` and `SV_DATA_DIR`, by a
    // recorded decision (ADR-036, Later). A third would arrive unrecorded, so each compiled-in file must be
    // marked so in `data/README.md`, and each file marked so must be compiled in.
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let scan = scan_src(&root.join("crates"));
    let compiled: std::collections::BTreeSet<String> =
        scan.compiled_data.iter().map(|(f, _)| f.clone()).collect();
    let readme = std::fs::read_to_string(root.join("data/README.md")).expect("data/README.md");
    let marked: std::collections::BTreeSet<String> = readme
        .lines()
        .filter(|l| l.contains("**compiled in**"))
        .filter_map(|l| l.split('`').nth(1).map(str::to_owned))
        .collect();
    // The setup: both sides were found, so an empty match proves nothing.
    assert!(
        !compiled.is_empty(),
        "no compiled-in data found; the scan is broken"
    );
    assert!(
        !marked.is_empty(),
        "data/README.md marks nothing compiled in"
    );
    assert_eq!(
        compiled, marked,
        "compiled in ({:#?}) and marked **compiled in** in data/README.md differ",
        scan.compiled_data
    );
}

#[test]
fn a_spaced_or_optional_build_folder_and_a_compiled_in_data_file_are_found() {
    // The scan's own fixture: a crate with `option_env!`, a spaced `env!`, and an include of a data file.
    let root = scratch("compiled-in");
    let src = root.join("crates/demo/src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::create_dir_all(root.join("data")).unwrap();
    std::fs::write(root.join("data/table.json"), "{}").unwrap();
    std::fs::write(
        src.join("lib.rs"),
        "const A: Option<&str> = option_env!(\"CARGO_MANIFEST_DIR\");\n\
         const B: &str = env!( \"CARGO_MANIFEST_DIR\" );\n\
         const C: &str = include_str!(\"../../../data/table.json\");\n\
         const D: &str = include_str!(\"lib.rs\");\n",
    )
    .unwrap();
    let scan = scan_src(&root.join("crates"));
    std::fs::remove_dir_all(&root).ok();
    assert_eq!(scan.build_folder.len(), 2, "{:#?}", scan.build_folder);
    let files: Vec<&str> = scan.compiled_data.iter().map(|(f, _)| f.as_str()).collect();
    assert_eq!(files, ["table.json"], "{:#?}", scan.compiled_data);
}

#[test]
fn a_test_file_declared_in_lib_rs_may_name_it_and_any_other_file_may_not() {
    // Backlog 0262: both sides of the guard, on a crate made here. A whole-file test module at the
    // top of `src/`, declared `#[cfg(test)] mod name;` in `lib.rs` (and one in `main.rs`), may name
    // the build folder; a file beside them that is not declared for tests may not.
    let crates = scratch("declared-in-lib");
    let src = crates.join("acrate/src");
    std::fs::create_dir_all(&src).unwrap();
    let names = "let _ = env!(\"CARGO_MANIFEST_DIR\");\n";
    std::fs::write(
        src.join("lib.rs"),
        "#[cfg(test)]\nmod lib_tests;\npub mod plain;\n",
    )
    .unwrap();
    std::fs::write(
        src.join("main.rs"),
        "#[cfg(test)]\nmod main_tests;\nfn main() {}\n",
    )
    .unwrap();
    for file in ["lib_tests.rs", "main_tests.rs", "plain.rs"] {
        std::fs::write(src.join(file), names).unwrap();
    }
    let found = build_folder_reads(&crates);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].ends_with("plain.rs:1"), "{found:#?}");
}

#[cfg(unix)]
#[test]
fn the_install_script_builds_with_the_versions_in_cargo_lock() {
    // Gap analysis 5.3: without `--locked`, a build chooses today's newest versions of what `sv` is
    // made from, which no test of `sv` ever ran with. A stand-in `cargo` writes down what it was
    // asked, and fails, so nothing is built or installed.
    use std::os::unix::fs::PermissionsExt;
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/install.sh");
    let root = scratch("locked");
    let bin = root.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    let asked = root.join("asked");
    let cargo = bin.join("cargo");
    std::fs::write(
        &cargo,
        format!("#!/bin/sh\necho \"$@\" > '{}'\nexit 1\n", asked.display()),
    )
    .unwrap();
    std::fs::set_permissions(&cargo, std::fs::Permissions::from_mode(0o755)).unwrap();
    let ran = Command::new("sh")
        .arg(&script)
        .env("SV_PREFIX", root.join("prefix"))
        .env_remove("SV_BINARY")
        .env(
            "PATH",
            format!(
                "{}:{}",
                bin.display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        )
        .output()
        .unwrap();
    let args = std::fs::read_to_string(&asked).unwrap_or_default();
    let installed = root.join("prefix/bin/sv").exists();
    std::fs::remove_dir_all(&root).ok();

    assert!(
        args.contains("build"),
        "the setup: the stand-in cargo was asked to build: {args:?}, {}",
        String::from_utf8_lossy(&ran.stderr)
    );
    assert!(args.split_whitespace().any(|a| a == "--locked"), "{args}");
    assert!(
        !ran.status.success() && !installed,
        "a failed build installed something"
    );
}

#[test]
fn the_install_script_puts_sv_and_its_data_in_a_folder_of_their_own() {
    // `tools/install.sh`, with the program already built (`SV_BINARY`) and a folder of the test's own
    // in place of `~/.local` (`SV_PREFIX`).
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/install.sh");
    let root = scratch("script");
    let install = || {
        Command::new("sh")
            .arg(&script)
            .env("SV_PREFIX", &root)
            .env("SV_BINARY", env!("CARGO_BIN_EXE_sv"))
            .env_remove("SV_DATA_DIR")
            .output()
            .unwrap()
    };
    let first = install();
    let home = root.join("share/stackvet");
    let said = data_line(&root.join("bin/sv"), None);
    // Again, as after pulling a newer sv: the same link, and the data replaced whole.
    std::fs::write(home.join("data/left-over.json"), "{}").unwrap();
    let again = install();
    let left_over = home.join("data/left-over.json").exists();
    // A file at bin/sv that the installer did not put there is left alone.
    std::fs::remove_file(root.join("bin/sv")).unwrap();
    std::fs::write(root.join("bin/sv"), "the owner's own").unwrap();
    let refused = install();
    let kept = std::fs::read_to_string(root.join("bin/sv")).unwrap_or_default();
    std::fs::remove_dir_all(&root).ok();

    assert!(first.status.success(), "{first:?}");
    assert_eq!(
        Path::new(&said),
        home.join("data").canonical().unwrap_or(home.join("data"))
    );
    assert!(again.status.success(), "{again:?}");
    assert!(!left_over, "an older install's data was kept");
    assert!(!refused.status.success(), "{refused:?}");
    assert!(
        String::from_utf8_lossy(&refused.stderr).contains("is not this installer's link"),
        "{refused:?}"
    );
    assert_eq!(kept, "the owner's own");
}
