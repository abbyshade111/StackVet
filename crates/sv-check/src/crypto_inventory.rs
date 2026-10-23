//! A cryptographic inventory drafted from what the code shows, for the key-management questions (backlog 0494; ADR-022,
//! Later).
//!
//! V11.1.2 asks for a cryptographic inventory: every key, algorithm, and certificate the app uses, and where each may
//! be used. V11.1.1 asks for a key-management policy: where keys come from, and who else has them. An owner who is not
//! a programmer cannot write either from memory, but can start from what `sv` reads:
//!
//! - the places the code hashes, encrypts, signs, derives a key from a password, or makes random bytes for a secret;
//! - the files whose names say they hold a key or a certificate, named and never read;
//! - the cryptography libraries among the app's packages;
//! - the settings the code reads a secret from (`SECRET_KEY`, `JWT_SECRET`), by name, never the value.
//!
//! # It finds, and does not judge
//!
//! Like a locator (`crate::locate`), this makes no finding and credits nothing. It is a draft for the owner to correct:
//! a call it names may do something else, and a key may be made or kept in a way it does not read.

use regex::Regex;
use std::collections::BTreeSet;
use std::sync::LazyLock;

/// The most places of each kind a question shows; the rest are counted.
pub const MOST_PLACES: usize = 25;

/// What the draft is, and what it is not, shown above it.
pub const INVENTORY_SAYS: &str = "A first draft of a cryptographic inventory, from what StackVet read: where the code \
     hashes, encrypts, signs, or makes keys, the files named as keys or certificates (never opened), the cryptography \
     libraries among the app's packages, and the settings it reads secrets from (names only). These are where to look, \
     not evidence: correct the draft, add what it missed, and say where each key comes from and who else has it.";

/// Kinds of use, each with a pattern for a call that does it. The first that matches a line names it.
static USES: LazyLock<Vec<(&'static str, Regex)>> = LazyLock::new(|| {
    [
        (
            "password hashing",
            r"\b(bcrypt\.(?:hashpw|hashSync|hash|gensalt|genSalt)|argon2\.(?:hash|PasswordHasher)|PasswordHasher\(|scrypt(?:Sync)?\(|pbkdf2(?:Sync|_hmac)?\(|make_password\(|generate_password_hash\(|check_password_hash\()",
        ),
        (
            "key generation",
            r"\b(generateKeyPair(?:Sync)?|generate_private_key|RSA\.generate|KeyPairGenerator\.getInstance|crypto\.subtle\.generateKey|Fernet\.generate_key)\b",
        ),
        (
            "encryption",
            r"\b(createCipheriv|createDecipheriv|Fernet\(|AES\.new|AESGCM\(|ChaCha20Poly1305\(|Cipher\.getInstance|crypto\.subtle\.(?:encrypt|decrypt)|CryptoJS\.AES\.\w+|SecretBox\(|Box\()",
        ),
        (
            "signing or a token",
            r"\b(jwt\.(?:sign|encode|verify|decode)|createSign\(|createVerify\(|crypto\.sign\(|crypto\.verify\(|crypto\.subtle\.(?:sign|verify)|URLSafe(?:Timed)?Serializer\(|Signature\.getInstance)",
        ),
        (
            "a keyed hash (HMAC)",
            r"\b(createHmac|hmac\.new|HMAC\.new|Mac\.getInstance|OpenSSL::HMAC)\b",
        ),
        (
            "hashing",
            r"\b(createHash|hashlib\.(?:sha\d+|md5|sha1|blake2\w|new)|MessageDigest\.getInstance|crypto\.subtle\.digest|Digest::\w+)\b",
        ),
        (
            "random bytes for a secret",
            r"\b(randomBytes|secrets\.token_\w+|os\.urandom|crypto\.getRandomValues|SecureRandom)\b",
        ),
    ]
    .into_iter()
    .map(|(kind, pattern)| (kind, Regex::new(pattern).expect("a use pattern compiles")))
    .collect()
});

/// A setting the code reads, by its name: `os.environ["X"]`, `os.environ.get("X")`, `os.getenv("X")`,
/// `process.env.X`, `process.env["X"]`, `ENV["X"]`, `ENV.fetch("X")`.
static SETTING_READ: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:os\.environ(?:\.get)?\s*[\[(]\s*|os\.getenv\s*\(\s*|ENV(?:\.fetch)?\s*[\[(]\s*|process\.env\[\s*)['"]([A-Za-z_][A-Za-z0-9_]*)['"]|process\.env\.([A-Za-z_][A-Za-z0-9_]*)"#)
        .expect("the setting pattern compiles")
});

/// A setting whose name says it holds a secret.
static SECRET_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(secret|private|signing|encryption|_key$|^key$|_keys$|token_key|salt|pepper|cert|passphrase)")
        .expect("the secret name pattern compiles")
});

/// A file whose name says it holds a key or a certificate.
const KEY_FILE_ENDINGS: &[&str] = &[
    ".pem",
    ".key",
    ".p12",
    ".pfx",
    ".jks",
    ".keystore",
    ".crt",
    ".cer",
    ".der",
    ".p8",
];

/// Packages that do cryptography, by the name a package list gives.
const LIBRARIES: &[&str] = &[
    "cryptography",
    "pycryptodome",
    "pycryptodomex",
    "pycrypto",
    "pynacl",
    "bcrypt",
    "argon2-cffi",
    "passlib",
    "pyjwt",
    "python-jose",
    "jwcrypto",
    "itsdangerous",
    "pyopenssl",
    "bcryptjs",
    "argon2",
    "jsonwebtoken",
    "jose",
    "crypto-js",
    "node-forge",
    "tweetnacl",
    "libsodium-wrappers",
    "@noble/hashes",
    "@noble/curves",
    "jwt",
    "rbnacl",
    "golang.org/x/crypto",
    "github.com/golang-jwt/jwt/v5",
    "firebase/php-jwt",
    "defuse/php-encryption",
];

/// One place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Place {
    /// What it is: "hashing, `createHash`", "a key or certificate file", "the setting `JWT_SECRET`".
    pub what: String,
    pub file: String,
    pub line: usize,
}

/// The draft.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CryptoInventory {
    pub uses: Vec<Place>,
    pub key_files: Vec<String>,
    pub libraries: Vec<String>,
    pub settings: Vec<Place>,
    /// Places past `MOST_PLACES`, of any kind, counted and not kept.
    pub more: usize,
}

impl CryptoInventory {
    pub fn is_empty(&self) -> bool {
        self.uses.is_empty()
            && self.key_files.is_empty()
            && self.libraries.is_empty()
            && self.settings.is_empty()
    }
}

/// The draft for an app: its own files, test code aside, and the packages its lists name.
pub fn read(listing: &sv_scan::files::Listing) -> CryptoInventory {
    let mut inventory = CryptoInventory::default();
    for entry in listing.app_files() {
        if listing.is_test_code(&entry.relative) {
            continue;
        }
        let lower = entry.relative.to_lowercase();
        if KEY_FILE_ENDINGS.iter().any(|e| lower.ends_with(e)) {
            inventory.key_files.push(entry.relative.clone());
            continue;
        }
        if entry.language.is_none() {
            continue;
        }
        let Ok(source) = entry.read_text() else {
            continue;
        };
        let (uses, settings) = in_text(&entry.relative, &source);
        inventory.uses.extend(uses);
        inventory.settings.extend(settings);
    }
    let named: BTreeSet<String> = sv_scan::deps::read_in(listing)
        .into_iter()
        .map(|d| d.name.to_lowercase())
        .filter(|name| LIBRARIES.contains(&name.as_str()) || name.starts_with("@noble/"))
        .collect();
    inventory.libraries = named.into_iter().collect();
    for list in [&mut inventory.uses, &mut inventory.settings] {
        inventory.more += list.len().saturating_sub(MOST_PLACES);
        list.truncate(MOST_PLACES);
    }
    inventory.more += inventory.key_files.len().saturating_sub(MOST_PLACES);
    inventory.key_files.truncate(MOST_PLACES);
    inventory
}

/// The uses and the secret settings in one file.
pub fn in_text(file: &str, source: &str) -> (Vec<Place>, Vec<Place>) {
    let mut uses = Vec::new();
    let mut settings = Vec::new();
    for (n, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        if let Some((kind, m)) = USES
            .iter()
            .find_map(|(kind, re)| re.captures(line).map(|m| (kind, m)))
        {
            uses.push(Place {
                what: format!("{kind}, `{}`", m[1].trim_end_matches('(')),
                file: file.to_owned(),
                line: n + 1,
            });
        }
        for m in SETTING_READ.captures_iter(line) {
            let Some(name) = m.get(1).or(m.get(2)) else {
                continue;
            };
            if SECRET_NAME.is_match(name.as_str()) {
                settings.push(Place {
                    what: format!("the setting `{}`", name.as_str()),
                    file: file.to_owned(),
                    line: n + 1,
                });
            }
        }
    }
    (uses, settings)
}

#[cfg(test)]
mod tests;
