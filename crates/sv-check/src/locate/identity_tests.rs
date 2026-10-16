//! Backlog 0500, its static part: `locate.upload-limits` for the notes question on what uploads take (V5.1.1),
//! `locate.identity-claims` for the notes question on what the sign-in provider said (V6.8.4), and
//! `locate.backend-connections` for the design questions on how the app's parts sign in to each other (V13.2.1,
//! V13.2.2). Each is held to what it must find and to the look-alikes it must leave alone.

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
fn upload_limits_are_found_and_other_settings_are_not() {
    let python = found(
        "locate.upload-limits",
        "app.py",
        "ALLOWED_EXTENSIONS = {'png', 'jpg'}\n\
         app.config['MAX_CONTENT_LENGTH'] = 2 * 1024 * 1024\n\
         def allowed_file(name):\n\
         \x20   return '.' in name\n\
         DEBUG = False\n\
         app.config['SECRET_NAME'] = 'x'\n",
    );
    assert_eq!(python, ["app.py:3", "app.py:1", "app.py:2"]);
    let javascript = found(
        "locate.upload-limits",
        "upload.js",
        "const upload = multer({ limits: { fileSize: 1000000 }, fileFilter: onlyImages });\n\
         const allowedTypes = ['image/png'];\n\
         const port = 3000;\n",
    );
    assert_eq!(javascript, ["upload.js:1", "upload.js:2"]);
}

#[test]
fn identity_claims_are_found_and_a_regex_substitution_is_not() {
    let python = found(
        "locate.identity-claims",
        "auth.py",
        "token = oauth.google.authorize_access_token()\n\
         info = token['userinfo']\n\
         who = info.get('sub')\n\
         level = claims['acr']\n\
         clean = re.sub('a', 'b', text)\n\
         name = info.get('name')\n",
    );
    assert_eq!(python, ["auth.py:2", "auth.py:3", "auth.py:4"]);
    let javascript = found(
        "locate.identity-claims",
        "auth.js",
        "const who = claims.sub;\n\
         const when = claims['auth_time'];\n\
         const t = tokenSet.id_token;\n\
         const n = profile.displayName;\n",
    );
    assert_eq!(javascript, ["auth.js:1", "auth.js:2", "auth.js:3"]);
}

#[test]
fn backend_connections_are_found_and_web_addresses_are_not() {
    let python = found(
        "locate.backend-connections",
        "settings.py",
        "DB = os.environ['DATABASE_URL']\n\
         CACHE = 'redis://cache:6379/0'\n\
         ENGINE = create_engine('postgresql+psycopg://app@db/notes')\n\
         SITE = 'https://example.com'\n\
         NAME = 'database'\n",
    );
    assert_eq!(python, ["settings.py:1", "settings.py:2", "settings.py:3"]);
    let javascript = found(
        "locate.backend-connections",
        "db.js",
        "const pool = new Pool({ connectionString: process.env['DATABASE_URL'] });\n\
         const mongo = `mongodb+srv://app@cluster0.example.net/notes`;\n\
         const api = 'https://api.example.com';\n",
    );
    assert_eq!(javascript, ["db.js:1", "db.js:2"]);
}
