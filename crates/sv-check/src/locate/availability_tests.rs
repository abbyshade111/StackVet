//! Backlog 0498: the two locators for the availability questions. `locate.heavy-work` points the notes question on
//! slow or heavy features (V15.1.3) and the hand check on defending them (V15.2.2) at the work that can be heavy;
//! `locate.availability-brakes` points both at the brakes the code shows. Each is held to what it must find and to the
//! look-alikes it must leave alone.

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
fn both_locators_speak_to_both_questions() {
    let locators = shipped();
    for id in ["locate.heavy-work", "locate.availability-brakes"] {
        let locator = locators
            .all()
            .find(|l| l.id == id)
            .unwrap_or_else(|| panic!("{id} is not shipped"));
        let mut questions = locator.questions.clone();
        questions.sort();
        let expected: &[&str] = if id == "locate.availability-brakes" {
            &["V15.1.3", "V15.2.2", "V2.1.3", "V6.1.1"]
        } else {
            &["V15.1.3", "V15.2.2"]
        };
        assert_eq!(questions, expected, "{id}");
    }
}

#[test]
fn python_heavy_work_is_found_and_light_work_is_not() {
    let places = found(
        "locate.heavy-work",
        "app.py",
        "h = bcrypt.hashpw(pw, bcrypt.gensalt())\n\
         img = Image.open(f)\n\
         upload = request.files['photo']\n\
         client.chat.completions.create(model=m, messages=msgs)\n\
         frame.to_csv(out)\n\
         shutil.make_archive(name, 'zip', folder)\n\
         name = request.form['name']\n\
         total = sum(values)\n\
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
fn javascript_heavy_work_is_found_and_light_work_is_not() {
    let places = found(
        "locate.heavy-work",
        "server.js",
        "const hash = bcrypt.hashSync(pw, 12);\n\
         await sharp(input).resize(200).toFile(out);\n\
         app.post('/up', upload.single('photo'), handler);\n\
         const f = req.files;\n\
         await anthropic.messages.create({ model, messages });\n\
         const name = req.body.name;\n\
         res.json({ ok: true });\n",
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

#[test]
fn python_brakes_are_found_and_other_settings_are_not() {
    let places = found(
        "locate.availability-brakes",
        "app.py",
        "app.config['MAX_CONTENT_LENGTH'] = 2 * 1024 * 1024\n\
         DATA_UPLOAD_MAX_MEMORY_SIZE = 2621440\n\
         r = requests.get(url, timeout=5)\n\
         @limiter.limit('5 per minute')\n\
         def login():\n\
         \x20   pass\n\
         rows = query.limit(50).all()\n\
         send_report.apply_async(args=[uid])\n\
         app.config['SECRET_NAME'] = 'x'\n\
         DEBUG = False\n\
         r = requests.get(url)\n",
    );
    assert_eq!(
        places,
        [
            "app.py:1", "app.py:2", "app.py:3", "app.py:4", "app.py:7", "app.py:8"
        ]
    );
}

#[test]
fn javascript_brakes_are_found_and_other_settings_are_not() {
    let places = found(
        "locate.availability-brakes",
        "server.js",
        "app.use(express.json({ limit: '100kb' }));\n\
         app.use(rateLimit({ windowMs: 60000, max: 10 }));\n\
         const upload = multer({ limits: { fileSize: 1000000 } });\n\
         const r = await axios.get(url, { timeout: 5000 });\n\
         app.use(express.static('public'));\n\
         const r2 = await axios.get(url, { headers: h });\n",
    );
    assert_eq!(
        places,
        ["server.js:1", "server.js:2", "server.js:3", "server.js:4"]
    );
}
