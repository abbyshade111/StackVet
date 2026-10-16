//! Backlog 0612: `locate.session-lifetime` points the notes question on session timeouts (V7.1.1) at where the code
//! sets how long a session lasts.

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
fn session_lifetime_settings_are_found_and_other_settings_are_not() {
    let python = found(
        "locate.session-lifetime",
        "settings.py",
        "SESSION_COOKIE_AGE = 1209600\n\
         app.config['PERMANENT_SESSION_LIFETIME'] = timedelta(days=7)\n\
         SESSION_SAVE_EVERY_REQUEST = True\n\
         CACHE_TIMEOUT = 600\n",
    );
    assert_eq!(python, ["settings.py:1", "settings.py:2", "settings.py:3"]);
    let javascript = found(
        "locate.session-lifetime",
        "server.js",
        "app.use(session({ cookie: { maxAge: 3600000 } }));\n\
         const t = jwt.sign(p, k, { expiresIn: '1h' });\n\
         const o = { timeout: 5000 };\n",
    );
    assert_eq!(javascript, ["server.js:1", "server.js:2"]);
}
