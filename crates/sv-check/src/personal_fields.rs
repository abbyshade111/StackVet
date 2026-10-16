//! The columns and fields the app's schema shows whose names look like personal data, for the data-classification
//! questions (backlog 0497; ADR-022, Later).
//!
//! V14.1.1 asks that the data the app handles be identified and classified, and V14.1.2 that each level's protection be
//! written down. A person who is not a programmer cannot read a migration, but can read "users.email, users.phone,
//! appointments.diagnosis" and say which of those their classification names. This lists them from what declares a
//! table: SQL `CREATE TABLE` (in a `.sql` file or a string in the code), a SQLAlchemy or Django model, a Prisma model,
//! and a Mongoose or Sequelize field.
//!
//! # It finds, and does not judge
//!
//! Like a locator (`crate::locate`), this makes no finding and credits nothing. A column named `address` may hold a web
//! address, and personal data can sit in a column named `notes` or in a JSON blob, so the list is where to look, never
//! evidence, and never the classification itself.

use regex::Regex;
use std::sync::LazyLock;

/// The most fields a question shows; the rest are counted.
pub const MOST_FIELDS: usize = 30;

/// What the list is, and what it is not, shown above it.
pub const FIELDS_SAY: &str = "The columns and fields in the app's schema whose names look like personal data, by \
     table. These are where to look, not evidence: a name can mislead, and personal data can sit in a field named \
     something else (`notes`, a JSON column), which is not in this list. Your classification should name each kind of \
     data the app keeps, and say what protection each level gets.";

/// A field name that looks like personal data, matched against the name with its case lowered and `_` and `-` taken
/// out.
static PERSONAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(e?mail(address)?|emailaddr|phone(number)?|mobile|tel|telephone|dob|dateofbirth|birth(date|day)?|address(line\d)?|street|city|postcode|postalcode|zip(code)?|ssn|socialsecurity(number)?|nationalid|passport(number)?|taxid|driverslicen[cs]e|(credit)?card(number)?|cardlast4|cvv|iban|bic|accountnumber|sortcode|routingnumber|diagnos[ie]s|medication|allerg(y|ies)|health\w*|medical\w*|bloodtype|salary|income|firstname|lastname|fullname|surname|givenname|familyname|gender|sex|ethnicity|religion|ipaddress|ip|latitude|longitude|lat|lng|location|geolocation|password(hash)?|passwordhash|hashedpassword)$",
    )
    .expect("the personal field pattern compiles")
});

/// `CREATE TABLE name (`, with or without quotes, `IF NOT EXISTS`, or a schema.
static CREATE_TABLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)create\s+table\s+(?:if\s+not\s+exists\s+)?[`"\[]?(?:\w+[`"\]]?\.[`"\[]?)?(\w+)[`"\]]?\s*\("#)
        .expect("the CREATE TABLE pattern compiles")
});

/// A column inside `CREATE TABLE ( ... )`: a name followed by a type.
static SQL_COLUMN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?i)^\s*[`"\[]?(\w+)[`"\]]?\s+(text|varchar|char|character|integer|int|bigint|smallint|serial|bigserial|real|numeric|decimal|float|double|boolean|bool|date|datetime|timestamp|timestamptz|time|uuid|json|jsonb|blob|bytea|citext|inet)\b"#,
    )
    .expect("the SQL column pattern compiles")
});

/// A Python model class: `class User(Base)`, `class User(db.Model)`, `class Patient(models.Model)`.
static PY_CLASS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*class\s+(\w+)\s*\(([^)]*)\)\s*:").expect("the Python class pattern compiles")
});

/// A Python model field: `email = Column(`, `email = db.Column(`, `email: Mapped[str] = mapped_column(`,
/// `email = models.EmailField(`.
static PY_FIELD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^\s+(\w+)\s*(?::\s*[^=]+)?=\s*(?:\w+\.)?(?:Column|mapped_column|\w+Field|ForeignKey)\s*\(",
    )
    .expect("the Python field pattern compiles")
});

/// A Prisma model: `model User {`.
static PRISMA_MODEL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*model\s+(\w+)\s*\{").expect("the Prisma model pattern compiles")
});

/// A Prisma field: a name and a type.
static PRISMA_FIELD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*(\w+)\s+(String|Int|BigInt|Float|Decimal|Boolean|DateTime|Json|Bytes)\b")
        .expect("the Prisma field pattern compiles")
});

/// A JavaScript schema field: `email: { type: String`, `email: String,`, `email: DataTypes.STRING`.
static JS_FIELD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^\s*['\x22]?(\w+)['\x22]?\s*:\s*(?:\{\s*type\s*:|DataTypes\.|Sequelize\.|Schema\.Types\.|(?:String|Number|Date|Boolean)\s*[,}\]])",
    )
    .expect("the JavaScript field pattern compiles")
});

/// A JavaScript model name: `mongoose.model('User'`, `sequelize.define('User'`, `new Schema` after `const userSchema`.
static JS_MODEL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:\.model|\.define)\s*\(\s*['"](\w+)['"]|(?:const|let|var)\s+(\w+?)(?:Schema)?\s*=\s*new\s+(?:mongoose\.)?Schema\b"#)
        .expect("the JavaScript model pattern compiles")
});

/// One field that looks personal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonalField {
    /// The table or model it is in, when the file says.
    pub table: Option<String>,
    pub field: String,
    pub file: String,
    pub line: usize,
}

impl PersonalField {
    /// `users.email`, or `email` when the table is not known.
    pub fn name(&self) -> String {
        match &self.table {
            Some(table) => format!("{table}.{}", self.field),
            None => self.field.clone(),
        }
    }
}

/// Every field found in an app.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PersonalFields {
    pub fields: Vec<PersonalField>,
    /// Fields past `MOST_FIELDS`, counted and not kept.
    pub more: usize,
    /// What a signed-in user's token carries in plain sight, when the run signed one in and the app's token is a JWT.
    pub token: Option<TokenFields>,
}

/// The names of the fields in the readable part of a signed-in user's JWT (V14.1.1 names "the plaintext payload inside
/// a JWT"). Names only: the values are never kept, so nothing a person or the token holds reaches a report.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TokenFields {
    /// Where the token travels, in words: "the `Authorization` header", "the cookie `session`".
    pub carried: String,
    /// The payload's field names, in order.
    pub fields: Vec<String>,
    /// A session cookie that is JSON in base64, rather than a JWT (backlog 0610).
    pub cookie: bool,
}

impl TokenFields {
    /// The sentence the reports show, with the fields that look like personal data marked.
    pub fn said(&self) -> String {
        let names: Vec<String> = self
            .fields
            .iter()
            .map(|f| {
                if looks_personal(f) {
                    format!("`{f}` (looks like personal data)")
                } else {
                    format!("`{f}`")
                }
            })
            .collect();
        if self.cookie {
            return format!(
                "When StackVet signed in, the app's session cookie, {}, was JSON in base64, which anyone holding the \
                 cookie can read, and held these fields: {}. Field names only; the values are not shown.",
                self.carried,
                names.join(", ")
            );
        }
        format!(
            "When StackVet signed in, the app's sign-in token, carried in {}, held these fields where anyone \
             holding the token can read them: {}. Field names only; the values are not shown.",
            self.carried,
            names.join(", ")
        )
    }
}

/// The field names in a session cookie that is JSON in base64 (backlog 0610): Flask's session cookie
/// (`payload.time.signature`, signed but not encrypted) and Express's cookie-session (`session`, with `session.sig`
/// beside it). The first cookie whose value, or its part before the first `.`, decodes to a JSON object with fields.
/// A compressed Flask cookie (starting with `.`) is not read. Names only, never the values.
pub fn cookie_fields(cookies: &[(String, String)]) -> Option<TokenFields> {
    use crate::encoding::{Alphabet, Padding, unbase64};
    cookies.iter().find_map(|(name, value)| {
        let value = value
            .trim_matches('"')
            .replace("%3D", "=")
            .replace("%3d", "=");
        if value.starts_with('.') || value.len() < 8 {
            return None;
        }
        let part = value.split('.').next()?;
        let bytes = unbase64(part, Alphabet::Web, Padding::Optional)
            .or_else(|| unbase64(part, Alphabet::Standard, Padding::Optional))?;
        let json: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        let object = json.as_object().filter(|o| !o.is_empty())?;
        Some(TokenFields {
            carried: format!("the cookie `{name}`"),
            fields: object.keys().cloned().collect(),
            cookie: true,
        })
    })
}

/// Whether a field's name looks like personal data.
pub fn looks_personal(name: &str) -> bool {
    let plain: String = name
        .chars()
        .filter(|c| *c != '_' && *c != '-')
        .flat_map(char::to_lowercase)
        .collect();
    PERSONAL.is_match(&plain)
}

/// The fields in the app's own files that look personal, not counting test code.
pub fn read(listing: &sv_scan::files::Listing) -> PersonalFields {
    let mut fields = Vec::new();
    for entry in listing.app_files() {
        let relative = entry.relative.as_str();
        let lower = relative.to_lowercase();
        let kind = if lower.ends_with(".sql") {
            "sql"
        } else if lower.ends_with(".prisma") {
            "prisma"
        } else {
            match entry.language {
                Some("python") => "python",
                Some("javascript" | "typescript") => "script",
                _ => continue,
            }
        };
        if sv_scan::files::is_test_path(relative) {
            continue;
        }
        let Ok(source) = entry.read_text() else {
            continue;
        };
        fields.extend(in_text(kind, relative, &source));
    }
    let more = fields.len().saturating_sub(MOST_FIELDS);
    fields.truncate(MOST_FIELDS);
    PersonalFields {
        fields,
        more,
        token: None,
    }
}

/// The fields that look personal in one file, read as `kind`: `sql`, `prisma`, `python`, or `script`. SQL in a string
/// is read in Python and script files too.
pub fn in_text(kind: &str, file: &str, source: &str) -> Vec<PersonalField> {
    let mut out = Vec::new();
    // The table a line is in: one opened by `CREATE TABLE`, a model class, or a model's name.
    let mut sql_table: Option<String> = None;
    let mut sql_depth = 0i32;
    let mut model: Option<String> = None;
    for (n, line) in source.lines().enumerate() {
        let mut push = |table: Option<String>, field: &str| {
            if looks_personal(field) {
                out.push(PersonalField {
                    table,
                    field: field.to_owned(),
                    file: file.to_owned(),
                    line: n + 1,
                });
            }
        };
        // SQL, in a .sql file or inside a string.
        if let Some(m) = CREATE_TABLE.captures(line) {
            sql_table = Some(m[1].to_owned());
            sql_depth = 0;
            let after = &line[m.get(0).map_or(0, |g| g.end())..];
            sql_depth += 1 + depth_change(after);
            for part in after.split(',') {
                if let Some(c) = SQL_COLUMN.captures(part) {
                    push(sql_table.clone(), &c[1]);
                }
            }
            if sql_depth <= 0 {
                sql_table = None;
            }
            continue;
        }
        if sql_table.is_some() {
            for part in line.split(',') {
                if let Some(c) = SQL_COLUMN.captures(part) {
                    push(sql_table.clone(), &c[1]);
                }
            }
            sql_depth += depth_change(line);
            if sql_depth <= 0 {
                sql_table = None;
            }
            continue;
        }
        match kind {
            "prisma" => {
                if let Some(m) = PRISMA_MODEL.captures(line) {
                    model = Some(m[1].to_owned());
                } else if line.trim_start().starts_with('}') {
                    model = None;
                } else if let Some(m) = PRISMA_FIELD.captures(line) {
                    push(model.clone(), &m[1]);
                }
            }
            "python" => {
                if let Some(m) = PY_CLASS.captures(line) {
                    let bases = &m[2];
                    model = (bases.contains("Model")
                        || bases.contains("Base")
                        || bases.contains("SQLModel"))
                    .then(|| m[1].to_owned());
                } else if let Some(m) = PY_FIELD.captures(line) {
                    push(model.clone(), &m[1]);
                }
            }
            "script" => {
                if let Some(m) = JS_MODEL.captures(line) {
                    model = m.get(1).or(m.get(2)).map(|g| g.as_str().to_owned());
                }
                if let Some(m) = JS_FIELD.captures(line) {
                    push(model.clone(), &m[1]);
                }
            }
            _ => {}
        }
    }
    out
}

/// How much the brackets on a line open (positive) or close (negative), outside quotes.
fn depth_change(line: &str) -> i32 {
    let mut depth = 0;
    let mut quote: Option<char> = None;
    for c in line.chars() {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), _) => {}
            (None, '\'') => quote = Some('\''),
            (None, '(') => depth += 1,
            (None, ')') => depth -= 1,
            _ => {}
        }
    }
    depth
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod cookie_tests;
