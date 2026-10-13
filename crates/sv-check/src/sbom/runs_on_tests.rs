//! What the app runs on, in its bill of materials (backlog 0402): the trial's app, on Python's own library alone, had a
//! bill listing nothing, which read as "nothing ships".

use super::*;
use std::path::{Path, PathBuf};

fn folder(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-runs-on-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    for (file, text) in files {
        let path = dir.join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    dir
}

fn listing(dir: &Path) -> sv_scan::files::Listing {
    sv_scan::files::Listing::of(dir)
}

#[test]
fn imports_are_read_as_written_and_the_apps_own_left_out() {
    let text = "import os, sys as system\nimport http.server\nfrom sqlite3 import connect\nfrom . import db\nfrom .app import x\nfrom __future__ import annotations\n# import tarfile\nimport json  # parse\n    import hashlib\nx = 'import zipfile'\n";
    let found: Vec<String> = python_imports(text).into_iter().collect();
    assert_eq!(
        found,
        ["hashlib", "http.server", "json", "os", "sqlite3", "sys"]
    );
}

#[test]
fn an_official_python_images_tag_names_the_version_and_nothing_else_does() {
    for (image, version) in [
        ("python:3.12-slim", Some("3.12")),
        ("python:3.12.7-bookworm", Some("3.12.7")),
        ("docker.io/library/python:3.11", Some("3.11")),
        ("python:3.12-slim@sha256:abc", Some("3.12")),
        ("python:latest", None),
        ("python:slim", None),
        ("python", None),
        ("node:22-alpine", None),
        ("ghcr.io/someone/python:3.12", None),
        ("mypython:3.12", None),
    ] {
        assert_eq!(python_image_version(image).as_deref(), version, "{image}");
    }
}

#[test]
fn a_python_app_on_its_own_library_lists_the_app_its_image_python_and_the_modules_it_imports() {
    let dir = folder(
        "stdlib",
        &[
            (
                "server.py",
                "import json\nfrom http.server import HTTPServer\nimport db\n",
            ),
            ("db.py", "import sqlite3\nimport os\n"),
            ("tests/test_app.py", "import unittest\nimport sqlite3\n"),
        ],
    );
    let runs = runs_on(
        &listing(&dir),
        "dog-feed-reminder",
        Some("python:3.12-slim"),
        None,
    );
    let doc = serde_json::to_value(to_cyclonedx_with(&Sbom::default(), &runs)).unwrap();
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(
        doc["metadata"]["component"]["name"], "dog-feed-reminder",
        "{doc}"
    );
    assert_eq!(doc["metadata"]["component"]["type"], "application");
    let components = doc["components"].as_array().unwrap();
    let image = components
        .iter()
        .find(|c| c["type"] == "container")
        .expect("the image");
    assert_eq!(
        (image["name"].as_str(), image["version"].as_str()),
        (Some("python"), Some("3.12-slim"))
    );
    assert!(
        image
            .to_string()
            .contains("not read: the app was not started here"),
        "{image}"
    );
    let python = components
        .iter()
        .find(|c| c["type"] == "platform")
        .expect("python");
    assert_eq!(python["name"], "python");
    assert_eq!(python["version"], "3.12");
    let modules: Vec<&str> = python["components"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["name"].as_str().unwrap())
        .collect();
    // The app's own `db` is not Python's; the test file's imports count, as the tests ship with the code.
    assert_eq!(
        modules,
        ["http.server", "json", "os", "sqlite3", "unittest"],
        "{python}"
    );
    let sqlite = &python["components"][3];
    assert!(
        sqlite.to_string().contains("db.py, tests/test_app.py"),
        "{sqlite}"
    );
    assert!(
        sqlite.to_string().contains("not a separate package"),
        "{sqlite}"
    );
    // None of it is a package: the list of packages stays what it was.
    assert_eq!(
        doc["metadata"]["properties"][0]["name"],
        "securevibe:complete"
    );
}

#[test]
fn the_digest_is_named_when_the_app_was_run_and_a_version_is_never_guessed() {
    let dir = folder("digest", &[("app.py", "import json\n")]);
    let runs = runs_on(
        &listing(&dir),
        "x",
        Some("ghcr.io/me/app:1.0"),
        Some("ghcr.io/me/app@sha256:0123"),
    );
    let doc = serde_json::to_value(to_cyclonedx_with(&Sbom::default(), &runs)).unwrap();
    std::fs::remove_dir_all(&dir).ok();
    let components = doc["components"].as_array().unwrap();
    let image = components
        .iter()
        .find(|c| c["type"] == "container")
        .unwrap();
    assert!(
        image
            .to_string()
            .contains("ghcr.io/me/app@sha256:0123, read from this computer's Docker"),
        "{image}"
    );
    let python = components.iter().find(|c| c["type"] == "platform").unwrap();
    assert!(
        python.get("version").is_none(),
        "a version from an image that is not python's: {python}"
    );
    assert!(
        python.to_string().contains("not known: the image"),
        "{python}"
    );
}

#[test]
fn an_app_with_no_python_lists_no_python_and_with_no_manifest_names_no_subject() {
    let dir = folder("node", &[("index.js", "const fs = require('fs');\n")]);
    let runs = runs_on(&listing(&dir), "", None, None);
    let doc = serde_json::to_value(to_cyclonedx_with(&Sbom::default(), &runs)).unwrap();
    std::fs::remove_dir_all(&dir).ok();
    assert!(doc["metadata"].get("component").is_none(), "{doc}");
    assert_eq!(doc["components"], serde_json::json!([]), "{doc}");
    // And the plain document, with nothing known about what the app runs on, is as it was.
    assert_eq!(
        serde_json::to_value(to_cyclonedx(&Sbom::default())).unwrap(),
        doc,
        "to_cyclonedx is to_cyclonedx_with nothing known"
    );
}

#[test]
fn every_component_type_is_one_cyclonedx_1_5_has() {
    let dir = folder("types", &[("app.py", "import json\n")]);
    let runs = runs_on(&listing(&dir), "x", Some("python:3.12"), None);
    let doc = serde_json::to_value(to_cyclonedx_with(&Sbom::default(), &runs)).unwrap();
    std::fs::remove_dir_all(&dir).ok();
    // CycloneDX 1.5's component types (the schema's `component.type` enum).
    let types = [
        "application",
        "framework",
        "library",
        "container",
        "platform",
        "operating-system",
        "device",
        "device-driver",
        "firmware",
        "file",
        "machine-learning-model",
        "data",
    ];
    let mut seen = vec![doc["metadata"]["component"]["type"].clone()];
    for c in doc["components"].as_array().unwrap() {
        seen.push(c["type"].clone());
        for inner in c["components"].as_array().into_iter().flatten() {
            seen.push(inner["type"].clone());
        }
    }
    assert!(
        seen.len() >= 4,
        "the setup: the app, the image, python, and a module: {seen:?}"
    );
    for kind in seen {
        assert!(types.contains(&kind.as_str().unwrap()), "{kind}");
    }
}
