//! Which fields the reader takes as looking like personal data, and the table it puts each in (backlog 0497).

use super::{TokenFields, in_text, looks_personal};

fn names(kind: &str, source: &str) -> Vec<(String, usize)> {
    in_text(kind, "schema", source)
        .into_iter()
        .map(|f| (f.name(), f.line))
        .collect()
}

fn pairs(list: &[(&str, usize)]) -> Vec<(String, usize)> {
    list.iter().map(|(n, l)| ((*n).to_owned(), *l)).collect()
}

#[test]
fn names_that_look_personal_and_names_that_do_not() {
    for name in [
        "email",
        "Email",
        "phone_number",
        "dateOfBirth",
        "dob",
        "address",
        "postcode",
        "ssn",
        "passport_number",
        "card_number",
        "iban",
        "diagnosis",
        "salary",
        "first_name",
        "lastName",
        "ip_address",
        "password_hash",
    ] {
        assert!(looks_personal(name), "{name}");
    }
    for name in [
        "id",
        "user_id",
        "title",
        "body",
        "created_at",
        "status",
        "condition",
        "notes",
        "name",
        "emailed",
        "address_book_id",
    ] {
        assert!(!looks_personal(name), "{name}");
    }
}

#[test]
fn sql_tables_in_a_file_or_a_string() {
    let sql = "\
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  password_hash TEXT,
  created_at TIMESTAMP
);
CREATE TABLE notes (id INTEGER, owner_id INTEGER, body TEXT);
CREATE TABLE \"public\".\"patients\" (\"diagnosis\" text, dob date);
";
    assert_eq!(
        names("sql", sql),
        pairs(&[
            ("users.email", 3),
            ("users.password_hash", 4),
            ("patients.diagnosis", 8),
            ("patients.dob", 8),
        ])
    );
    let python = "\
db.execute(\"\"\"
    CREATE TABLE accounts (
        id INTEGER PRIMARY KEY,
        phone VARCHAR(20)
    )
\"\"\")
email = request.form['email']
";
    assert_eq!(names("python", python), pairs(&[("accounts.phone", 4)]));
}

#[test]
fn python_models_and_prisma_models() {
    let python = "\
class User(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    email = db.Column(db.String(120))
    salary: Mapped[int] = mapped_column()

class Patient(models.Model):
    first_name = models.CharField(max_length=50)

class Helper:
    email = 'not a field'
";
    assert_eq!(
        names("python", python),
        pairs(&[
            ("User.email", 3),
            ("User.salary", 4),
            ("Patient.first_name", 7)
        ])
    );
    let prisma = "\
model User {
  id    Int    @id
  email String @unique
  iban  String?
}
";
    assert_eq!(
        names("prisma", prisma),
        pairs(&[("User.email", 3), ("User.iban", 4)])
    );
}

#[test]
fn javascript_schemas() {
    let script = "\
const userSchema = new mongoose.Schema({
  email: { type: String, required: true },
  phone: String,
  title: String,
});
const Patient = sequelize.define('Patient', {
  diagnosis: DataTypes.TEXT,
});
const config = { email: process.env.EMAIL };
";
    assert_eq!(
        names("script", script),
        pairs(&[
            ("user.email", 2),
            ("user.phone", 3),
            ("Patient.diagnosis", 7)
        ])
    );
}

#[test]
fn the_token_sentence_names_the_fields_and_marks_the_personal_ones() {
    let token = TokenFields {
        carried: "the cookie `session`".to_owned(),
        fields: vec!["sub".to_owned(), "email".to_owned(), "role".to_owned()],
        cookie: false,
        audience: Vec::new(),
    };
    let said = token.said();
    assert!(said.contains("carried in the cookie `session`"), "{said}");
    assert!(
        said.contains("`sub`, `email` (looks like personal data), `role`"),
        "{said}"
    );
    assert!(
        said.contains("Field names only; the values are not shown."),
        "{said}"
    );
}
