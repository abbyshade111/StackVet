//! Backlog 0516: the locators held to the example apps, with the places each must find written beside the app, so a
//! change that loses one, or points somewhere new, goes red. And each shape the locators of 0516 look for, alone.
//!
//! The trial of 14 October found the authorization locator pointing at an OpenID provider's address in `oidc-notes`
//! (`config.authorization_endpoint`, a property passed to `new URL`) and at nothing in `notes-with-users`, whose real
//! check is inline: a 403 and a query with `owner = ?`.

use super::*;
use crate::test_support::scratch;

fn shipped() -> Locators {
    Locators::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/locators.json"),
    )
    .expect("the shipped locators load, every query compiled")
}

/// Every place every locator found, as `locator file:line`.
fn all_found(dir: &Path) -> Vec<String> {
    let found = shipped().find(&sv_scan::files::Listing::of(dir));
    found
        .places
        .iter()
        .flat_map(|(id, places)| {
            places
                .iter()
                .map(move |p| format!("{id} {}:{}", p.file, p.line))
        })
        .collect()
}

#[test]
fn each_example_app_is_pointed_where_its_checks_are_and_nowhere_else() {
    let examples = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let expected: &[(&str, &[&str])] = &[
        // 34 lines, no authorization code.
        ("flask-booking", &[]),
        // Inline checks: the admin page refused with a 403 and an `authorization-refused` event (209, 210), each note
        // read only with its owner (241). The other 403s refuse a forged form or a wrong password: where to look, and
        // the person decides. 316 and 336 delete and move a person's own notes by owner.
        (
            "notes-with-users",
            &[
                "locate.owner-checks app.py:241",
                "locate.owner-checks app.py:316",
                "locate.owner-checks app.py:336",
                "locate.refusals app.py:133",
                "locate.refusals app.py:209",
                "locate.refusals app.py:210",
                "locate.refusals app.py:257",
                "locate.refusals app.py:264",
                "locate.refusals app.py:265",
                "locate.refusals app.py:273",
                "locate.refusals app.py:295",
                "locate.refusals app.py:314",
                "locate.refusals app.py:329",
                "locate.refusals app.py:347",
            ],
        ),
        // Notes are kept by account (`notes.get(s.user)`), with no 403 and no owner compared: nothing to point at,
        // and never the provider's `authorization_endpoint` (server.js:69).
        ("oidc-notes", &[]),
        // 25 lines each, no authorization code.
        ("tested-notes", &[]),
        ("partly-passing", &[]),
    ];
    let mut read = 0;
    for (app, places) in expected {
        let dir = examples.join(app);
        assert!(dir.is_dir(), "{} is not there", dir.display());
        // The setup worked: the app's own code was listed, or finding nothing would mean nothing.
        let listed = sv_scan::files::Listing::of(&dir).app_files().count();
        assert!(listed > 0, "{app}: no file of the app's was listed");
        read += 1;
        assert_eq!(all_found(&dir), *places, "{app}");
    }
    assert_eq!(read, 5);
}

fn found_in(name: &str, text: &str) -> Vec<String> {
    let dir = scratch("");
    std::fs::write(dir.join(name), text).unwrap();
    all_found(&dir)
}

#[test]
fn an_openid_address_and_a_header_are_not_authorization_checks() {
    let found = found_in(
        "server.js",
        "const to = new URL(config.authorization_endpoint);\n\
         post(url, { grant_type: 'authorization_code' });\n\
         fetch(url, { headers: { authorization: token } });\n\
         use(req.headers.authorization);\n",
    );
    assert_eq!(found, Vec::<String>::new());
}

#[test]
fn python_refusals_are_found_however_the_403_goes_out() {
    let found = found_in(
        "app.py",
        "def a():\n\
         \x20   abort(403)\n\
         def b():\n\
         \x20   return jsonify(error='no'), 403\n\
         def c():\n\
         \x20   raise PermissionDenied\n\
         def d():\n\
         \x20   raise HTTPException(status_code=403)\n\
         def e():\n\
         \x20   raise exceptions.Forbidden('no')\n\
         def f():\n\
         \x20   return 200\n",
    );
    assert_eq!(
        found,
        vec![
            "locate.refusals app.py:2",
            "locate.refusals app.py:4",
            "locate.refusals app.py:6",
            "locate.refusals app.py:8",
            "locate.refusals app.py:10",
        ]
    );
}

#[test]
fn javascript_refusals_are_found_and_other_statuses_are_not() {
    let found = found_in(
        "server.js",
        "res.status(403).end();\n\
         res.sendStatus(403);\n\
         throw new ForbiddenError('no');\n\
         ctx.status = 403;\n\
         ctx.status = 200;\n\
         const limit = 403;\n\
         job.retries = 403;\n",
    );
    assert_eq!(
        found,
        vec![
            "locate.refusals server.js:1",
            "locate.refusals server.js:2",
            "locate.refusals server.js:3",
            "locate.refusals server.js:4",
        ]
    );
}

#[test]
fn a_records_owner_compared_or_queried_is_found() {
    let python = found_in(
        "views.py",
        "if note.owner != request.user:\n\
         \x20   pass\n\
         rows = db.execute('select * from notes where user_id = ?', (uid,))\n\
         note = Note.query.filter_by(owner_id=current_user.id).first()\n\
         title = 'owner'\n",
    );
    assert_eq!(
        python,
        vec![
            "locate.owner-checks views.py:1",
            "locate.owner-checks views.py:3",
            "locate.owner-checks views.py:4",
        ]
    );
    let javascript = found_in(
        "server.js",
        "if (req.user.id !== note.ownerId) return;\n\
         const rows = await db.query(`select * from notes where owner = $1`, [id]);\n\
         const note = await Note.findOne({ where: { userId: req.user.id } });\n\
         const label = 'owner';\n",
    );
    assert_eq!(
        javascript,
        vec![
            "locate.owner-checks server.js:1",
            "locate.owner-checks server.js:2",
            "locate.owner-checks server.js:3",
        ]
    );
}
