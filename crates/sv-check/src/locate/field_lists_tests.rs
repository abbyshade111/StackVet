//! Backlog 0608, its last part: `locate.field-lists` points the notes question on field-level access rules (V8.1.2) and
//! the design question on returning only the fields needed (V15.3.1) at the places the code chooses which fields of a
//! record go in or out. Held to what it must find and to the look-alikes it must leave alone.

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
fn python_field_choices_are_found_and_other_assignments_are_not() {
    let places = found(
        "locate.field-lists",
        "serializers.py",
        "class NoteSerializer(serializers.ModelSerializer):\n\
         \x20   class Meta:\n\
         \x20       model = Note\n\
         \x20       fields = ['id', 'title']\n\
         \x20       read_only_fields = ['owner']\n\
         notes = Note.objects.only('id', 'title')\n\
         out = schema.dump(note, only=('id', 'title'))\n\
         @app.get('/notes', response_model=list[NoteOut])\n\
         def list_notes():\n\
         \x20   pass\n\
         total = sum(prices.values())\n\
         fields = parse_qs(body)\n\
         name = 'notes'\n",
    );
    assert_eq!(
        places,
        [
            "serializers.py:4",
            "serializers.py:5",
            "serializers.py:6",
            "serializers.py:7",
            "serializers.py:8"
        ]
    );
}

#[test]
fn javascript_field_choices_are_found_and_other_calls_are_not() {
    let places = found(
        "locate.field-lists",
        "notes.js",
        "const notes = await prisma.note.findMany({ select: { id: true, title: true } });\n\
         const users = await User.findAll({ attributes: ['id', 'name'] });\n\
         const doc = await Note.findById(id).select('title body');\n\
         const safe = _.pick(user, ['id', 'name']);\n\
         class Note { toJSON() { return { id: this.id }; } }\n\
         const el = document.querySelector('#note');\n\
         const all = await Note.find({ owner: id });\n",
    );
    assert_eq!(
        places,
        [
            "notes.js:5",
            "notes.js:1",
            "notes.js:2",
            "notes.js:3",
            "notes.js:4"
        ]
    );
}
