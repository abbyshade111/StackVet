//! `ast.open-redirect` reads a destination given by name (backlog 0601): `RedirectResponse(url="/")` is as fixed as
//! `RedirectResponse("/")`, and the clean Cursor + Flash run of 15 October 2026 reported thirteen of them. The
//! literal-argument exemption saw the whole `url="/"`, not the `"/"` in it.

use super::*;
use std::path::PathBuf;

const RULE: &str = "ast.open-redirect";

fn rules() -> &'static AstRules {
    static RULES: OnceLock<AstRules> = OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// Whether the rule reports `source`. A file the parser could not read fails the test rather than reading as clean.
fn reported(language: &str, file: &str, source: &str) -> bool {
    let read = read_file(rules(), language, file, source);
    assert!(!read.parse_error, "{file} did not parse: {source}");
    assert!(
        read.broken.is_empty(),
        "a query did not compile: {:?}",
        read.broken
    );
    read.findings.iter().any(|f| f.rule_id == RULE)
}

#[test]
fn a_fixed_destination_given_by_name_is_not_reported_in_python() {
    for (source, expected) in [
        // The trial's own lines (`app/main.py:165` and `:218`).
        (
            "def f():\n    return RedirectResponse(url=\"/\", status_code=303)\n",
            false,
        ),
        (
            "def f():\n    return RedirectResponse(status_code=303, url=\"/login\")\n",
            false,
        ),
        (
            "def f():\n    return redirect(location=url_for(\"index\"))\n",
            false,
        ),
        (
            "def f():\n    return HttpResponseRedirect(redirect_to=\"/done\")\n",
            false,
        ),
        // Given by name, and built from the request: still reported.
        (
            "def f(request):\n    return RedirectResponse(url=request.query_params[\"next\"])\n",
            true,
        ),
        (
            "def f(request):\n    return redirect(location=request.args.get(\"next\"))\n",
            true,
        ),
        // The positional forms, as before.
        ("def f():\n    return RedirectResponse(\"/\")\n", false),
        (
            "def f(request):\n    return RedirectResponse(request.query_params[\"next\"])\n",
            true,
        ),
    ] {
        assert_eq!(
            reported("python", "app/main.py", source),
            expected,
            "{source}"
        );
    }
}

#[test]
fn a_fixed_destination_given_by_name_is_not_reported_in_csharp_and_kotlin() {
    for (language, file, source, expected) in [
        (
            "csharp",
            "C.cs",
            "class C { object F() { return Redirect(url: \"/home\"); } }",
            false,
        ),
        (
            "csharp",
            "C.cs",
            "class C { object F(string next) { return Redirect(url: next); } }",
            true,
        ),
        (
            "kotlin",
            "a.kt",
            "suspend fun f(call: Call) { call.respondRedirect(url = \"/\") }",
            false,
        ),
        (
            "kotlin",
            "a.kt",
            "suspend fun f(call: Call) { call.respondRedirect(url = call.parameters[\"next\"]!!) }",
            true,
        ),
    ] {
        assert_eq!(
            reported(language, file, source),
            expected,
            "{language}: {source}"
        );
    }
}
