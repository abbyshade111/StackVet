//! Backlog 0653, ADR-138: a file path built from a request's input is `ast.file-path-from-request`, a thing to fix; a
//! path built from any other value is `ast.file-path-from-value` alone, listed apart as worth a look where the first
//! rule reads the language, and still a thing to fix where it does not.

use super::*;
use crate::finding::{PATH_FROM_REQUEST, PATH_FROM_REQUEST_LANGUAGES, PATH_FROM_VALUE};
use std::path::PathBuf;

fn rules() -> &'static AstRules {
    static RULES: std::sync::OnceLock<AstRules> = std::sync::OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

fn ids_in(language: &str, file: &str, source: &str) -> Vec<String> {
    scan_file(rules(), language, file, source)
        .into_iter()
        .map(|f| f.rule_id)
        .collect()
}

/// For each language the rule reads: a request's input reaching a file call, and the same call given a value that is
/// not a request's. The second is still the path rule's finding, so the reading never drops what it found before.
const CASES: &[(&str, &str, &str, &str)] = &[
    (
        "python",
        "app.py",
        "def dl():\n    name = request.args['f']\n    path = os.path.join(UPLOADS, name)\n    return open(path).read()\n",
        "def dl(name):\n    return open(os.path.join(UPLOADS, name)).read()\n",
    ),
    (
        "javascript",
        "app.js",
        "app.get('/f', (req, res) => { res.sendFile(req.query.name); });\n",
        "function f(name) { return fs.readFileSync(name); }\n",
    ),
    (
        "typescript",
        "app.ts",
        "app.get('/f', (req: Request, res: Response) => { const p = path.join(dir, req.params.name); fs.unlinkSync(p); });\n",
        "function f(name: string) { return fs.readFileSync(name); }\n",
    ),
    (
        "go",
        "main.go",
        "package main\n\nfunc h(w http.ResponseWriter, r *http.Request) {\n\tname := r.URL.Query().Get(\"f\")\n\tos.ReadFile(name)\n}\n",
        "package main\n\nfunc f(name string) {\n\tos.ReadFile(name)\n}\n",
    ),
    (
        "php",
        "index.php",
        "<?php\nfunction show() {\n    $f = $_GET['f'];\n    echo file_get_contents($f);\n}\n",
        "<?php\nfunction show($p) {\n    return file_get_contents($p);\n}\n",
    ),
    (
        "ruby",
        "app.rb",
        "def show\n  File.read(params[:f])\nend\n",
        "def show(p)\n  File.read(p)\nend\n",
    ),
    (
        "java",
        "A.java",
        "class A { void h(HttpServletRequest req) throws Exception { String n = req.getParameter(\"f\"); new FileInputStream(n); } }\n",
        "class A { void f(String n) throws Exception { new FileInputStream(n); } }\n",
    ),
    (
        "kotlin",
        "A.kt",
        "fun h(call: ApplicationCall) { val f = File(call.parameters[\"f\"]) }\n",
        "fun f(p: String) { val f = File(p) }\n",
    ),
    (
        "csharp",
        "A.cs",
        "class A { void H() { var t = File.ReadAllText(Request.Query[\"f\"]); } }\n",
        "class A { void F(string p) { var t = File.ReadAllText(p); } }\n",
    ),
    (
        "rust",
        "src/main.rs",
        "async fn h(Path(name): Path<String>) -> String { std::fs::read_to_string(&name).unwrap() }\n",
        "fn f(p: &Path) -> String { std::fs::read_to_string(p).unwrap() }\n",
    ),
    (
        "shell",
        "run.sh",
        "cat \"/srv/files/$QUERY_STRING\"\n",
        "cat \"$CONFIG_FILE\"\n",
    ),
];

#[test]
fn a_requests_input_reaching_a_file_call_is_found_in_each_language_and_another_value_is_not() {
    let mut wrong = Vec::new();
    for (language, file, from_request, other) in CASES {
        let found = ids_in(language, file, from_request);
        if !found.iter().any(|id| id == PATH_FROM_REQUEST) {
            wrong.push(format!(
                "{language}: a request's input was missed: {found:?}"
            ));
        }
        let found = ids_in(language, file, other);
        if found.iter().any(|id| id == PATH_FROM_REQUEST) {
            wrong.push(format!(
                "{language}: a value not from a request was read as one"
            ));
        }
        // Shell's path rule already read only a web request's variables, so its other case has no finding at all.
        if *language != "shell" && !found.iter().any(|id| id == PATH_FROM_VALUE) {
            wrong.push(format!(
                "{language}: the path rule no longer finds a value it found before: {found:?}"
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn every_language_the_rule_reads_is_one_its_findings_are_listed_apart_in() {
    let rule = rules()
        .coverage()
        .into_iter()
        .find(|(id, ..)| *id == PATH_FROM_REQUEST)
        .expect("the rule is loaded");
    let mut read: Vec<&str> = rule.1.to_vec();
    read.sort();
    assert_eq!(
        read, PATH_FROM_REQUEST_LANGUAGES,
        "the list in finding.rs is the rule's own"
    );
    let covered: Vec<&str> = CASES.iter().map(|(l, ..)| *l).collect();
    for language in PATH_FROM_REQUEST_LANGUAGES {
        assert!(covered.contains(language), "{language} has no case here");
    }
}

#[test]
fn a_value_followed_through_names_and_rust_bindings_is_still_the_requests() {
    for (language, file, source) in [
        // Three names deep: the request's value, a join, then the call.
        (
            "python",
            "app.py",
            "def dl():\n    a = request.args['f']\n    b = a.strip()\n    c = os.path.join(UPLOADS, b)\n    return open(c).read()\n",
        ),
        // Rust: a `let` from an extractor's field.
        (
            "rust",
            "src/main.rs",
            "async fn h(Query(q): Query<Params>) { let p = dir.join(&q.file); std::fs::read(p).unwrap(); }\n",
        ),
        // An upload's own name, followed back through `while let` to the `Multipart` extractor.
        (
            "rust",
            "src/main.rs",
            "async fn up(mut m: Multipart) { while let Some(field) = m.next_field().await.unwrap() { let name = field.file_name().unwrap().to_owned(); std::fs::write(dir.join(name), b\"\").unwrap(); } }\n",
        ),
        // Actix's extractor, written with its module.
        (
            "rust",
            "src/main.rs",
            "async fn h(path: web::Path<String>) { std::fs::remove_file(path.into_inner()).unwrap(); }\n",
        ),
    ] {
        assert!(
            ids_in(language, file, source)
                .iter()
                .any(|id| id == PATH_FROM_REQUEST),
            "{language}: {source}"
        );
    }
}

#[test]
fn a_request_value_made_safe_is_not_reported() {
    // `secure_filename` reduces a submitted name to a bare one, as the rule's own fix says to.
    let source = "UPLOADS = '/srv/uploads'\n\ndef up():\n    name = secure_filename(request.files['f'].filename)\n    open(os.path.join(UPLOADS, name), 'wb')\n";
    let found = ids_in("python", "app.py", source);
    assert!(!found.iter().any(|id| id == PATH_FROM_REQUEST), "{found:?}");
    assert!(!found.iter().any(|id| id == PATH_FROM_VALUE), "{found:?}");
}

#[test]
fn a_paths_own_file_name_is_not_a_requests_input() {
    // StackVet's own `main.rs` writes its dashboard as `parent.join(out.file_name())`: std's `Path::file_name`, not an
    // upload's. The first build of this rule read `.file_name()` as a request's input and reported it.
    let source = "fn write(out: &Path) { let name = out.file_name().unwrap(); let target = parent.join(name); std::fs::read(&target).unwrap(); }\n";
    let found = ids_in("rust", "src/main.rs", source);
    assert!(!found.iter().any(|id| id == PATH_FROM_REQUEST), "{found:?}");
}
