//! What only the app's tests show is not evidence of what the app does (backlog 0538). In the
//! three-tool trial of 14 October 2026 `httpx`, declared for the tests' client, was read as the app
//! calling outside services, and FastAPI's `media_type=` as the app sending pictures to a model; the
//! AI coding tool rewrote the app to make both go away.

use std::path::PathBuf;
use sv_frameworks::Condition;
use sv_scan::{Answer, Evidence, ScanReport, Signatures, scan};

fn data(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../data")
        .join(file)
}

/// A scan of an app made of `files`, with both signature sets as the CLI loads them.
fn scan_files(name: &str, files: &[(&str, &str)]) -> ScanReport {
    let dir = std::env::temp_dir().join(format!("sv-only-in-tests-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    for (path, contents) in files {
        let path = dir.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, contents).unwrap();
    }
    let sigs = Signatures::load_all(&[
        &data("tech-signatures.json"),
        &data("claim-corroborators.json"),
    ])
    .expect("signatures load");
    let report = scan(&dir, &sigs).expect("scan");
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(
        report.files_read,
        files.iter().filter(|(p, _)| p.ends_with(".py")).count(),
        "the setup was read"
    );
    report
}

fn answer(report: &ScanReport, condition: Condition) -> &Answer {
    report
        .answers
        .iter()
        .find(|a| a.condition == condition)
        .expect("condition answered")
}

const MAIN: (&str, &str) = (
    "app/main.py",
    "from fastapi import FastAPI\nfrom fastapi.responses import Response\n\napp = FastAPI()\n\n\
     @app.get('/notes')\ndef notes():\n    return Response('ok', media_type=\"text/plain; charset=utf-8\")\n",
);
const REQUIREMENTS: (&str, &str) = (
    "requirements.txt",
    "fastapi==0.115.0\nhttpx==0.27.0\npytest==8.3.3\n",
);

#[test]
fn a_package_only_the_tests_import_is_neither_yes_nor_no() {
    let report = scan_files(
        "httpx",
        &[
            MAIN,
            REQUIREMENTS,
            (
                "tests/test_main.py",
                "import httpx\nfrom fastapi.testclient import TestClient\nfrom app.main import app\n",
            ),
        ],
    );
    let apis = answer(&report, Condition::ExternalApis);
    assert_eq!(apis.value, None, "{apis:?}");
    match &apis.evidence {
        Evidence::OnlyInTests { shown_by } => assert_eq!(
            shown_by,
            "`httpx`, declared in requirements.txt and imported only by tests/test_main.py"
        ),
        other => panic!("{other:?}"),
    }
}

#[test]
fn the_same_package_the_app_imports_is_a_yes() {
    // The control: the app's own code calls out with it.
    let report = scan_files(
        "httpx-app",
        &[
            MAIN,
            REQUIREMENTS,
            (
                "app/weather.py",
                "import httpx\n\ndef today():\n    return 1\n",
            ),
            ("tests/test_main.py", "import httpx\n"),
        ],
    );
    let apis = answer(&report, Condition::ExternalApis);
    assert_eq!(apis.value, Some(true), "{apis:?}");
    assert!(
        matches!(&apis.evidence, Evidence::Dependency { name, .. } if name == "httpx"),
        "{apis:?}"
    );
}

#[test]
fn a_pattern_only_a_test_file_holds_is_neither_yes_nor_no() {
    let report = scan_files(
        "pattern",
        &[
            MAIN,
            (
                "tests/test_calls.py",
                "def test_x(monkeypatch):\n    requests.post('https://example.test')\n",
            ),
        ],
    );
    let apis = answer(&report, Condition::ExternalApis);
    assert_eq!(apis.value, None, "{apis:?}");
    assert!(
        matches!(&apis.evidence, Evidence::OnlyInTests { shown_by } if shown_by.ends_with("in tests/test_calls.py")),
        "{apis:?}"
    );
    // And the same line in the app's own code is a yes.
    let report = scan_files(
        "pattern-app",
        &[
            MAIN,
            (
                "app/calls.py",
                "def send():\n    requests.post('https://example.test')\n",
            ),
        ],
    );
    assert_eq!(answer(&report, Condition::ExternalApis).value, Some(true));
}

#[test]
fn fastapis_media_type_is_not_a_picture_sent_to_a_model() {
    let report = scan_files("media-type", &[MAIN]);
    let multimodal = answer(&report, Condition::MultimodalAi);
    assert_ne!(multimodal.value, Some(true), "{multimodal:?}");
    // The control: an image block sent to a model still is.
    let report = scan_files(
        "image-block",
        &[(
            "app/ask.py",
            "content = [{\"type\": \"image\", \"source\": {\"type\": \"base64\", \"media_type\": \"image/png\"}}]\n",
        )],
    );
    assert_eq!(answer(&report, Condition::MultimodalAi).value, Some(true));
}
