//! Backlog 0487: a locator finds the lines whose names say they check a role or a permission, puts where a check is
//! written before where it is used, and finds nothing by names that only sign a person in.

use super::*;
use crate::test_support::scratch;

fn shipped() -> Locators {
    Locators::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/locators.json"),
    )
    .expect("the shipped locators load, every query compiled")
}

fn found_in(files: &[(&str, &str)]) -> Vec<Located> {
    let dir = scratch("");
    for (name, text) in files {
        let path = dir.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    let found = shipped().find(&sv_scan::files::Listing::of(&dir));
    found
        .places
        .get("locate.authorization-checks")
        .cloned()
        .unwrap_or_default()
}

fn at(file: &str, line: usize, defined: bool) -> Located {
    Located {
        file: file.to_owned(),
        line,
        defined,
    }
}

#[test]
fn python_checks_are_found_where_written_and_where_used() {
    let places = found_in(&[(
        "app.py",
        "from flask import Flask\n\
         app = Flask(__name__)\n\
         \n\
         def require_role(role):\n\
         \x20   pass\n\
         \n\
         @app.route('/admin')\n\
         @require_role('admin')\n\
         def admin():\n\
         \x20   if not current_user.has_permission('edit'):\n\
         \x20       abort(403)\n",
    )]);
    assert_eq!(
        places,
        vec![
            at("app.py", 4, true),
            at("app.py", 8, false),
            at("app.py", 10, false)
        ]
    );
}

#[test]
fn javascript_middleware_passed_to_a_route_is_found() {
    let places = found_in(&[(
        "server.js",
        "function requireAdmin(req, res, next) { next(); }\n\
         const canEditNote = (user, note) => note.owner === user.id;\n\
         app.get('/admin', requireAdmin, handler);\n\
         if (!canEditNote(req.user, note)) res.status(403).end();\n",
    )]);
    assert_eq!(
        places,
        vec![
            at("server.js", 1, true),
            at("server.js", 2, true),
            at("server.js", 3, false),
            at("server.js", 4, false),
        ]
    );
}

#[test]
fn typescript_and_tsx_are_read() {
    let places = found_in(&[
        (
            "guard.ts",
            "export function authorize(user: User): boolean { return user.isAdmin; }\n",
        ),
        (
            "page.tsx",
            "export const Page = () => { if (!hasRole(user, 'admin')) return null; return <div/>; };\n",
        ),
    ]);
    assert_eq!(
        places,
        vec![at("guard.ts", 1, true), at("page.tsx", 1, false)]
    );
}

/// Signing a person in is not deciding what they may do: a locator for V8 that pointed at `login_required` would send
/// the owner to the wrong lines.
#[test]
fn names_that_only_sign_a_person_in_are_not_found() {
    let places = found_in(&[(
        "views.py",
        "@login_required\n\
         def home(request):\n\
         \x20   if not request.user.is_authenticated:\n\
         \x20       return redirect('/login')\n\
         \x20   roles = load(request)\n\
         \x20   return render(request, roles)\n",
    )]);
    assert_eq!(places, Vec::new());
}

#[test]
fn a_name_is_matched_with_its_case_and_underscores_set_aside() {
    assert_eq!(plain("require_Role"), "requirerole");
    assert_eq!(plain("canEditNote"), "caneditnote");
}

#[test]
fn every_question_a_locator_names_is_asked_by_a_catalog() {
    let data = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let notes = crate::notes::Catalog::load(&data.join("security-notes.json")).unwrap();
    let design = crate::design::Questions::load(&data.join("design-questions.json")).unwrap();
    let human = crate::human::HumanChecks::load(&data.join("human-checks.json")).unwrap();
    let asked: BTreeSet<&str> = notes
        .sections
        .iter()
        .map(|s| s.id.as_str())
        .chain(design.questions.iter().map(|q| q.id.as_str()))
        .chain(human.checks.iter().map(|c| c.id.as_str()))
        .collect();
    let locators = shipped();
    assert!(
        locators.all().next().is_some(),
        "the shipped file has a locator"
    );
    for locator in locators.all() {
        for id in &locator.questions {
            assert!(
                asked.contains(id.as_str()),
                "{} points for {id}, which no catalog asks the owner about",
                locator.id
            );
        }
    }
}
