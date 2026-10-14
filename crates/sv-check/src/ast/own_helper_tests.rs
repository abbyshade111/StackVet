//! A Python app's own redirect helper takes its destination where its own signature says (backlog
//! 0388): the second loop trial's `redirect(req, "/login")`, through the app's `def redirect(req,
//! location)`, was flagged as a destination built from a value, the request object read as the
//! destination.

use super::*;
use std::path::PathBuf;

fn open_redirects(source: &str) -> Vec<usize> {
    let rules = AstRules::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"),
    )
    .expect("rules load");
    scan_file(&rules, "python", "app.py", source)
        .into_iter()
        .filter(|f| f.rule_id == "ast.open-redirect")
        .map(|f| f.location.line)
        .collect()
}

const HELPER: &str = "def redirect(req, location, status=302):\n    \
                      return Response(status, headers={'Location': location})\n\n";

#[test]
fn a_fixed_address_through_the_apps_own_helper_is_not_flagged() {
    let source = format!("{HELPER}def logout(req):\n    return redirect(req, \"/login\")\n");
    assert_eq!(open_redirects(&source), Vec::<usize>::new());
    // By name, too.
    let named =
        format!("{HELPER}def logout(req):\n    return redirect(req, location=\"/login\")\n");
    assert_eq!(open_redirects(&named), Vec::<usize>::new());
}

#[test]
fn a_value_through_the_apps_own_helper_is_still_flagged_at_the_call() {
    let source =
        format!("{HELPER}def login(req):\n    return redirect(req, req.args.get('next'))\n");
    assert_eq!(open_redirects(&source), vec![5]);
    let named = format!(
        "{HELPER}def login(req):\n    return redirect(req, location=req.args.get('next'))\n"
    );
    assert_eq!(open_redirects(&named), vec![5]);
}

#[test]
fn a_framework_redirect_still_reads_its_first_argument() {
    // No helper of that name in the file: the first argument is the destination, as before.
    let fixed =
        "from flask import redirect\n\ndef logout():\n    return redirect(\"/login\", 302)\n";
    assert_eq!(open_redirects(fixed), Vec::<usize>::new());
    let built = "from flask import redirect, request\n\n\
                 def login():\n    return redirect(request.args.get('next'), 302)\n";
    assert_eq!(open_redirects(built), vec![4]);
    // A helper whose parameters name no destination leaves the first argument as it was.
    let unnamed = "def redirect(a, b):\n    return a\n\n\
                   def login(req):\n    return redirect(req, \"/login\")\n";
    assert_eq!(open_redirects(unnamed), vec![5]);
}
