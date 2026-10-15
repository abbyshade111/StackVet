//! Backlog 0579: the Django-rule gate of 0521 held back on a Flask app whenever any package list in the app could not
//! be read, in any language, though ADR-111 holds it back only for one in the framework's own language. A Flask app
//! with a JavaScript library's `package.json` in `static/` that is not valid JSON kept its Django findings counting.
//! Read here through the scanner, from an app in the shape of the trial's: a `requirements.txt` with comments, a
//! package folder of templates, and a `.venv` beside them that holds Django.

use std::path::{Path, PathBuf};
use sv_check::adapters::{self, Adapters};
use sv_check::finding::Outranked;

const DJANGO_FORM: &str = "python.django.security.django-no-csrf-token.django-no-csrf-token";

fn adapters() -> Adapters {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/adapters.json");
    Adapters::load(&data).expect("the adapter file loads")
}

/// A Flask app in a folder of its own, with `static_package_json` as its JavaScript library's package list.
fn flask_app(name: &str, static_package_json: &str, broken_python_list: bool) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-0579-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let write = |path: &str, text: &[u8]| {
        let path = dir.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    };
    write(
        "requirements.txt",
        b"# The web app\nFlask==3.0.3\nWerkzeug==3.0.3  # WSGI\nJinja2>=3.1\n",
    );
    if broken_python_list {
        // A second Python list, a worker's, that is not valid TOML: it might name Django.
        write("worker/Pipfile", b"[packages\ndjango = \"*\"\n");
    }
    write(
        "vibegames/__init__.py",
        b"from flask import Flask\napp = Flask(__name__)\n",
    );
    write(
        "vibegames/templates/play.html",
        b"<form method=\"post\">\n<input type=\"hidden\" name=\"csrf_token\" value=\"{{ csrf_token() }}\">\n</form>\n",
    );
    write(
        "vibegames/static/vendor/lib/package.json",
        static_package_json.as_bytes(),
    );
    write(".venv/lib/python3.12/site-packages/django/__init__.py", b"");
    dir
}

/// Semgrep's Django form rule on the app's template, set apart or not by what the scanner read of `dir`.
fn django_finding_in(dir: &Path) -> (Option<Outranked>, sv_scan::files::Listing) {
    let adapters = adapters();
    let semgrep = adapters
        .all()
        .iter()
        .find(|a| a.id == "semgrep")
        .expect("semgrep is listed");
    let sarif = serde_json::json!({
        "version": "2.1.0",
        "runs": [{ "tool": { "driver": { "name": "Semgrep OSS", "rules": [] } }, "results": [{
            "ruleId": DJANGO_FORM,
            "level": "warning",
            "message": { "text": "a finding" },
            "locations": [{ "physicalLocation": {
                "artifactLocation": { "uri": "vibegames/templates/play.html" },
                "region": { "startLine": 1 }
            }}]
        }] }]
    })
    .to_string();
    let mut findings = adapters::parse_sarif(semgrep, &sarif).expect("the report reads");
    let listing = sv_scan::files::Listing::of(dir);
    adapters::set_apart_other_frameworks(
        &adapters,
        &mut findings,
        &sv_scan::deps::read_in(&listing),
        &sv_scan::deps::unread_in(&listing),
    );
    (findings[0].outranked.clone(), listing)
}

#[test]
fn an_unread_javascript_package_list_does_not_keep_a_django_finding_on_a_flask_app() {
    let dir = flask_app("static", "{ \"name\": \"lib\", // a comment\n}", false);
    let (outranked, listing) = django_finding_in(&dir);
    // The setup: Flask read from the requirements, Django nowhere the scanner reads, and the library's list unread.
    let declared = sv_scan::deps::read_in(&listing);
    assert!(
        declared
            .iter()
            .any(|d| d.ecosystem == "Python" && d.name == "Flask"),
        "{declared:?}"
    );
    assert!(
        !declared
            .iter()
            .any(|d| d.name.eq_ignore_ascii_case("django")),
        "{declared:?}"
    );
    let unread = sv_scan::deps::unread_in(&listing);
    assert_eq!(
        unread
            .iter()
            .map(|u| (u.ecosystem.as_str(), u.manifest.as_str()))
            .collect::<Vec<_>>(),
        vec![("npm", "vibegames/static/vendor/lib/package.json")]
    );
    assert_eq!(
        outranked,
        Some(Outranked::OtherFramework {
            framework: "Django".to_owned()
        })
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_unread_python_package_list_still_keeps_it() {
    // The worker's Pipfile is not valid TOML: whether the app names Django is not known.
    let dir = flask_app("python", "{ \"name\": \"lib\" }", true);
    let (outranked, listing) = django_finding_in(&dir);
    assert!(
        sv_scan::deps::unread_in(&listing)
            .iter()
            .any(|u| u.ecosystem == "Python"),
        "the setup: the worker's Pipfile is unread"
    );
    let declared = sv_scan::deps::read_in(&listing);
    assert!(
        declared
            .iter()
            .any(|d| d.ecosystem == "Python" && d.name.eq_ignore_ascii_case("flask")),
        "the setup: a Python list was read, so only the unread one holds the gate back: {declared:?}"
    );
    assert_eq!(outranked, None);
    let _ = std::fs::remove_dir_all(&dir);
}
