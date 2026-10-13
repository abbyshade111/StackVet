//! Backlog 0351: `note_callers` parses only the Python files whose text holds a pending function's name. A file
//! that names the function without calling it still keeps the finding from saying every caller is safe.

use super::*;
use std::path::PathBuf;

const SAYS: &str = "passes the app's own route or a path on this site";

fn scan_app(name: &str, files: &[(&str, &str)]) -> Finding {
    let rules = AstRules::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"),
    )
    .expect("rules load");
    let dir = std::env::temp_dir().join(format!("sv-ast-0351-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    for (file, text) in files {
        std::fs::write(dir.join(file), text).unwrap();
    }
    let scan = scan_dir(&rules, &dir);
    std::fs::remove_dir_all(&dir).ok();
    let mut found: Vec<Finding> = scan
        .findings
        .into_iter()
        .filter(|f| f.rule_id == "ast.open-redirect")
        .collect();
    assert_eq!(found.len(), 1, "{name}: {found:?}");
    found.remove(0)
}

#[test]
fn a_file_that_names_the_function_without_calling_it_is_still_read() {
    let signin = "from flask import redirect, session\n\n\
                  def finish_sign_in(user, destination):\n    \
                  session['uid'] = user.id\n    \
                  return redirect(destination)\n";
    let views = "from flask import url_for\nfrom signin import finish_sign_in\n\n\
                 def login(user):\n    return finish_sign_in(user, url_for('home.index'))\n";
    // A file that never names it, which is no longer parsed for this.
    let other = "def unrelated(x):\n    return x + 1\n";
    let own = scan_app(
        "own",
        &[
            ("signin.py", signin),
            ("views.py", views),
            ("other.py", other),
        ],
    );
    assert!(own.description.contains(SAYS), "{}", own.description);
    // The function handed on as a value: what reaches the parameter is not written where it is called.
    let handed = "from signin import finish_sign_in\n\nHANDLERS = {'finish': finish_sign_in}\n";
    let unknown = scan_app(
        "handed",
        &[
            ("signin.py", signin),
            ("views.py", views),
            ("other.py", other),
            ("handlers.py", handed),
        ],
    );
    assert!(
        !unknown.description.contains(SAYS),
        "{}",
        unknown.description
    );
}
