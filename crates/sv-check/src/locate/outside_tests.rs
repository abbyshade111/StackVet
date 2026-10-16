//! Backlog 0496: `locate.outside-addresses` points the notes question on everything the app talks to (V13.1.1) at the
//! web addresses written into the code. Held to what it must find and to the look-alikes it must leave alone.

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
fn addresses_written_into_the_code_are_found_and_other_text_is_not() {
    let python = found(
        "locate.outside-addresses",
        "app.py",
        "API = 'https://api.stripe.com/v1/charges'\n\
         HOOK = f\"https://hooks.slack.com/services/{key}\"\n\
         LOCAL = 'http://localhost:5000/'\n\
         HOME = 'http://127.0.0.1:8000'\n\
         NOTE = 'see https://example.com for more'\n\
         PATH = '/api/notes'\n",
    );
    assert_eq!(python, ["app.py:1", "app.py:2"]);
    let javascript = found(
        "locate.outside-addresses",
        "client.js",
        "fetch('https://api.openai.com/v1/chat');\n\
         const u = `https://${region}.example.org/x`;\n\
         const v = `https://maps.example.org/${id}`;\n\
         fetch('/api/notes');\n",
    );
    assert_eq!(javascript, ["client.js:1", "client.js:3"]);
}
