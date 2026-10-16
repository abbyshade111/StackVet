//! Backlog 0488: the two locators for the encoding questions. `locate.decoding` points the decode-once question
//! (V1.1.1) at the calls that turn coded text back into plain text; `locate.dangerous-places` points the hand check on
//! cleaning input (V1.3.3) at the places a value is handed to a shell, a database, a file sent back, an email, or a
//! spreadsheet. Each is held to what it must find and to the look-alikes it must leave alone.

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
fn both_locators_are_shipped_for_their_questions() {
    let locators = shipped();
    let questions = |id: &str| {
        locators
            .all()
            .find(|l| l.id == id)
            .map(|l| l.questions.clone())
            .unwrap_or_else(|| panic!("{id} is not shipped"))
    };
    assert_eq!(questions("locate.decoding"), ["V1.1.1"]);
    assert_eq!(questions("locate.dangerous-places"), ["V1.3.3"]);
}

#[test]
fn python_decoding_is_found_and_reading_bytes_as_text_is_not() {
    let places = found(
        "locate.decoding",
        "app.py",
        "from urllib.parse import unquote\n\
         a = unquote(raw)\n\
         b = urllib.parse.unquote_plus(raw)\n\
         c = html.unescape(raw)\n\
         d = base64.b64decode(raw)\n\
         e = base64.urlsafe_b64decode(raw)\n\
         f = raw.decode('utf-8')\n\
         g = jwt.decode(token, key)\n\
         h = json.loads(raw)\n",
    );
    assert_eq!(
        places,
        ["app.py:2", "app.py:3", "app.py:4", "app.py:5", "app.py:6"]
    );
}

#[test]
fn javascript_decoding_is_found_and_reading_bytes_as_text_is_not() {
    let places = found(
        "locate.decoding",
        "server.js",
        "const a = decodeURIComponent(req.query.q);\n\
         const b = decodeURI(url);\n\
         const c = atob(s);\n\
         const d = querystring.unescape(s);\n\
         const e = new TextDecoder().decode(bytes);\n\
         const f = JSON.parse(s);\n",
    );
    assert_eq!(
        places,
        ["server.js:1", "server.js:2", "server.js:3", "server.js:4"]
    );
}

#[test]
fn python_dangerous_places_are_found_and_ordinary_calls_are_not() {
    let places = found(
        "locate.dangerous-places",
        "app.py",
        "subprocess.run(cmd, shell=True)\n\
         os.system(cmd)\n\
         cursor.execute(sql)\n\
         send_from_directory(folder, name)\n\
         writer.writerow(row)\n\
         send_mail(subject, body, sender, [to])\n\
         app.run(debug=False)\n\
         requests.get(url)\n\
         print(name)\n",
    );
    assert_eq!(
        places,
        [
            "app.py:1", "app.py:2", "app.py:3", "app.py:4", "app.py:5", "app.py:6"
        ]
    );
}

#[test]
fn javascript_dangerous_places_are_found_and_ordinary_calls_are_not() {
    let places = found(
        "locate.dangerous-places",
        "server.js",
        "exec(cmd);\n\
         child_process.execSync(cmd);\n\
         db.query(sql);\n\
         res.sendFile(path);\n\
         res.download(path);\n\
         /a/.exec(text);\n\
         document.querySelector('a');\n\
         app.listen(3000);\n",
    );
    assert_eq!(
        places,
        [
            "server.js:1",
            "server.js:2",
            "server.js:3",
            "server.js:4",
            "server.js:5"
        ]
    );
}
