//! Backlog 0489: the two locators for the validation and business-logic questions. `locate.validation-rules` points
//! the notes questions on validation rules (V2.1.1, V2.1.2), the hand check on validating input (V2.2.1), and the design
//! question on validating at the server (V2.2.2) at the schemas and forms the code writes; `locate.transactions` points
//! the design question on transactions (V2.3.3) at the places changes are grouped. Each is held to what it must find
//! and to the look-alikes it must leave alone.

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
fn each_locator_speaks_to_its_questions() {
    let locators = shipped();
    let questions = |id: &str| {
        let mut q = locators
            .all()
            .find(|l| l.id == id)
            .map(|l| l.questions.clone())
            .unwrap_or_else(|| panic!("{id} is not shipped"));
        q.sort();
        q
    };
    assert_eq!(
        questions("locate.validation-rules"),
        ["V2.1.1", "V2.1.2", "V2.2.1", "V2.2.2"]
    );
    assert_eq!(questions("locate.transactions"), ["V2.3.3"]);
}

#[test]
fn python_schemas_and_forms_are_found_and_other_classes_are_not() {
    let places = found(
        "locate.validation-rules",
        "models.py",
        "class Signup(BaseModel):\n\
         \x20   email: str\n\
         class NoteSchema(ma.Schema):\n\
         \x20   title = fields.Str()\n\
         class NoteSerializer(serializers.ModelSerializer):\n\
         \x20   pass\n\
         class LoginForm(FlaskForm):\n\
         \x20   pass\n\
         class Rules(BaseModel):\n\
         \x20   @field_validator('email')\n\
         \x20   def check(cls, v):\n\
         \x20       return v\n\
         class Note(db.Model):\n\
         \x20   pass\n\
         class Error(Exception):\n\
         \x20   pass\n",
    );
    assert_eq!(
        places,
        [
            "models.py:1",
            "models.py:3",
            "models.py:5",
            "models.py:7",
            "models.py:9",
            "models.py:10"
        ]
    );
}

#[test]
fn javascript_schemas_are_found_and_other_objects_are_not() {
    let places = found(
        "locate.validation-rules",
        "schema.js",
        "const signup = z.object({ email: z.string().email() });\n\
         const note = yup.object({ title: yup.string().max(80) });\n\
         const order = Joi.object({ qty: Joi.number().min(1) });\n\
         app.post('/a', checkSchema({ email: { isEmail: true } }), h);\n\
         const copy = Object.assign({}, a);\n\
         const keys = Object.keys(a);\n",
    );
    assert_eq!(
        places,
        ["schema.js:1", "schema.js:2", "schema.js:3", "schema.js:4"]
    );
}

#[test]
fn typescript_class_validator_rules_are_found() {
    let places = found(
        "locate.validation-rules",
        "dto.ts",
        "class SignupDto {\n\
         \x20 @IsEmail()\n\
         \x20 email: string;\n\
         \x20 @MaxLength(80)\n\
         \x20 name: string;\n\
         \x20 @ApiProperty()\n\
         \x20 note: string;\n\
         }\n",
    );
    assert_eq!(places, ["dto.ts:2", "dto.ts:4"]);
}

#[test]
fn python_transactions_are_found_and_other_calls_are_not() {
    let places = found(
        "locate.transactions",
        "orders.py",
        "@transaction.atomic\n\
         def place(order):\n\
         \x20   pass\n\
         def move(a, b):\n\
         \x20   with transaction.atomic():\n\
         \x20       pass\n\
         \x20   db.session.begin_nested()\n\
         \x20   conn.execute('BEGIN')\n\
         \x20   session.rollback()\n\
         \x20   conn.execute('select 1')\n\
         \x20   log.begin_section('a')\n",
    );
    assert_eq!(
        places,
        [
            "orders.py:1",
            "orders.py:5",
            "orders.py:7",
            "orders.py:8",
            "orders.py:9"
        ]
    );
}

#[test]
fn javascript_transactions_are_found_and_other_calls_are_not() {
    let places = found(
        "locate.transactions",
        "orders.js",
        "await prisma.$transaction([a, b]);\n\
         await sequelize.transaction(async (t) => {});\n\
         await client.query('BEGIN');\n\
         await client.query(`ROLLBACK`);\n\
         const tx = db.transaction(() => {});\n\
         await client.query('select 1');\n\
         const begun = beginning();\n",
    );
    assert_eq!(
        places,
        [
            "orders.js:1",
            "orders.js:2",
            "orders.js:3",
            "orders.js:4",
            "orders.js:5"
        ]
    );
}
