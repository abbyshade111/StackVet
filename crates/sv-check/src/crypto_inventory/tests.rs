//! What the inventory draft takes as a use of cryptography and as a secret setting (backlog 0494).

use super::in_text;

fn whats(source: &str) -> (Vec<String>, Vec<String>) {
    let (uses, settings) = in_text("app", source);
    (
        uses.into_iter()
            .map(|p| format!("{}@{}", p.what, p.line))
            .collect(),
        settings
            .into_iter()
            .map(|p| format!("{}@{}", p.what, p.line))
            .collect(),
    )
}

#[test]
fn each_kind_of_use_is_named_with_the_call() {
    let source = "\
h = hashlib.sha256(data).hexdigest()
pw = bcrypt.hashpw(password, bcrypt.gensalt())
f = Fernet(key)
token = jwt.encode(claims, key, algorithm='HS256')
mac = hmac.new(key, msg, 'sha256')
nonce = secrets.token_urlsafe(32)
priv = rsa.generate_private_key(public_exponent=65537, key_size=3072)
# hashlib.md5 in a comment is not a use
";
    let (uses, _) = whats(source);
    assert_eq!(
        uses,
        vec![
            "hashing, `hashlib.sha256`@1",
            "password hashing, `bcrypt.hashpw`@2",
            "encryption, `Fernet`@3",
            "signing or a token, `jwt.encode`@4",
            "a keyed hash (HMAC), `hmac.new`@5",
            "random bytes for a secret, `secrets.token_urlsafe`@6",
            "key generation, `generate_private_key`@7",
        ]
    );
}

#[test]
fn settings_named_for_a_secret_are_listed_by_name_and_others_are_not() {
    let source = "\
SECRET_KEY = os.environ['SECRET_KEY']
signing = os.getenv(\"JWT_SIGNING_KEY\")
port = os.environ.get('PORT')
const key = process.env.ENCRYPTION_KEY;
const url = process.env.DATABASE_URL;
const salt = process.env['PASSWORD_SALT'];
";
    let (_, settings) = whats(source);
    assert_eq!(
        settings,
        vec![
            "the setting `SECRET_KEY`@1",
            "the setting `JWT_SIGNING_KEY`@2",
            "the setting `ENCRYPTION_KEY`@4",
            "the setting `PASSWORD_SALT`@6",
        ]
    );
}
