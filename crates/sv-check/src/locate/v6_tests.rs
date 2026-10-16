//! Backlog 0609: `locate.password-checks` for the notes question on words kept out of passwords (V6.1.2), and
//! `locate.backup-codes` for the hand check on how backup codes are kept (V6.5.2). Held to what each must find and to
//! the look-alikes each must leave alone.

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
fn password_checks_are_found_and_other_password_lines_are_not() {
    let python = found(
        "locate.password-checks",
        "settings.py",
        "AUTH_PASSWORD_VALIDATORS = [{'NAME': 'CommonPasswordValidator'}]\n\
         validate_password(password, user)\n\
         score = zxcvbn(password)['score']\n\
         hashed = make_password(password)\n\
         PASSWORD_MIN = 8\n",
    );
    assert_eq!(python, ["settings.py:1", "settings.py:2", "settings.py:3"]);
    let javascript = found(
        "locate.password-checks",
        "signup.js",
        "const result = zxcvbn(password);\n\
         if (!validator.isStrongPassword(password)) throw new Error('weak');\n\
         const hash = await bcrypt.hash(password, 12);\n",
    );
    assert_eq!(javascript, ["signup.js:1", "signup.js:2"]);
}

#[test]
fn backup_codes_are_found_and_other_codes_are_not() {
    let python = found(
        "locate.backup-codes",
        "models.py",
        "class User(Base):\n\
         \x20   backup_codes = Column(JSON)\n\
         def generate_backup_codes(n=10):\n\
         \x20   return [secrets.token_hex(5) for _ in range(n)]\n\
         user.recovery_codes = codes\n\
         status_code = 200\n",
    );
    assert_eq!(python, ["models.py:3", "models.py:2", "models.py:5"]);
    let javascript = found(
        "locate.backup-codes",
        "user.js",
        "const schema = new Schema({ recoveryCodes: [String] });\n\
         user.backupCodes = await hashAll(codes);\n\
         res.status(200).json({ code: 200 });\n",
    );
    assert_eq!(javascript, ["user.js:1", "user.js:2"]);
}
