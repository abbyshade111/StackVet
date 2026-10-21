//! Backlog 0507: three locators for level-3 questions. `locate.oauth-scopes` points V10.2.3 (only the scopes needed) at
//! where the app names them; `locate.hmac` points V11.3.5 (encrypt-then-MAC) at where it computes an HMAC;
//! `locate.shared-state` points V15.4.1 (shared objects reached safely) at top-level objects named as state. Each is held
//! to what it must find and to the look-alikes it must leave alone.

use super::*;
use crate::test_support::scratch;

fn shipped() -> Locators {
    Locators::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/locators.json"),
    )
    .expect("the shipped locators load, every query compiled")
}

/// The places one locator found in a folder holding one file, as `file:line`.
fn found(locator: &str, name: &str, text: &str) -> Vec<String> {
    let dir = scratch("");
    std::fs::write(dir.join(name), text).unwrap();
    let listing = sv_scan::files::Listing::of(&dir);
    // The setup worked: the file was listed as the app's own code, or finding nothing would mean nothing.
    assert_eq!(listing.app_files().count(), 1, "{name} was not listed");
    shipped()
        .find(&listing)
        .places
        .get(locator)
        .map(|places| {
            places
                .iter()
                .map(|p| format!("{}:{}", p.file, p.line))
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn the_scopes_the_app_asks_for_are_found_and_look_alikes_are_not() {
    let python = found(
        "locate.oauth-scopes",
        "app.py",
        "oauth.register('google', client_kwargs={'scope': 'openid email'})\n\
         url = flow.authorization_url(scope='openid profile')\n\
         telescope = 1\n\
         options = {'scoped': True}\n",
    );
    assert_eq!(python, ["app.py:1", "app.py:2"]);
    let javascript = found(
        "locate.oauth-scopes",
        "server.js",
        "const params = { scope: 'openid email', state };\n\
         const auth = { authorization: { params: { scopes: ['read'] } } };\n\
         const other = { scoped: true, scopeless: 1 };\n",
    );
    assert_eq!(javascript, ["server.js:1", "server.js:2"]);
}

#[test]
fn an_hmac_is_found_and_other_hashing_is_not() {
    let python = found(
        "locate.hmac",
        "app.py",
        "import hmac, hashlib\n\
         tag = hmac.new(key, data, hashlib.sha256)\n\
         same = hmac.compare_digest(a, b)\n\
         h = HMAC(key, hashes.SHA256())\n\
         thing = Factory.new()\n\
         digest = hashlib.sha256(data)\n",
    );
    assert_eq!(python, ["app.py:2", "app.py:4"]);
    let javascript = found(
        "locate.hmac",
        "server.js",
        "const tag = crypto.createHmac('sha256', key).update(body).digest('hex');\n\
         const hash = crypto.createHash('sha256').update(body).digest('hex');\n",
    );
    assert_eq!(javascript, ["server.js:1"]);
}

#[test]
fn top_level_state_is_found_and_settings_and_function_locals_are_not() {
    let python = found(
        "locate.shared-state",
        "app.py",
        "_cache = {}\n\
         sessions = dict()\n\
         SETTINGS = {'debug': False}\n\
         def handler():\n    \
             cache = {}\n    \
             return cache\n",
    );
    assert_eq!(python, ["app.py:1", "app.py:2"]);
    let javascript = found(
        "locate.shared-state",
        "server.js",
        "const sessions = new Map();\n\
         let rateCounts = {};\n\
         const config = { port: 3000 };\n\
         function handler() { const cache = new Map(); return cache; }\n",
    );
    assert_eq!(javascript, ["server.js:1", "server.js:2"]);
}
