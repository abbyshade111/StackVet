//! The questions `sv review` asks about its signing key's passphrase (backlog 0591): the owner, told to type `none`
//! for no passphrase, typed it where the passphrase was asked, and the key was made with the word `none`; the next
//! review refused what they typed and said nothing more.

use super::*;

fn folder(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!(
        "sv-review-passphrase-prompts-{name}-{}",
        std::process::id()
    ));
    std::fs::remove_dir_all(&folder).ok();
    folder
}

/// `signing_key` in `folder` with `typed` as what the person types, and what it said.
fn asked(folder: &Path, typed: &str) -> (Option<SigningKey>, String) {
    let mut out = Vec::new();
    let key = signing_key(
        folder,
        &mut std::io::Cursor::new(typed.as_bytes().to_vec()),
        &mut out,
        &mut ask,
    )
    .unwrap();
    (key, String::from_utf8(out).unwrap())
}

/// The key `signing_key` makes when the person types `typed`, as it is kept, and what it said.
fn made_with(name: &str, typed: &str) -> (Stored, String) {
    let folder = folder(name);
    let (key, out) = asked(&folder, typed);
    assert!(key.is_some(), "a key was made: {out}");
    let stored = SigningKey::load_from(&folder).unwrap().expect("a key file");
    std::fs::remove_dir_all(&folder).ok();
    (stored, out)
}

#[test]
fn the_first_question_is_a_plain_yes_or_no_and_offers_no_word_to_type() {
    let (stored, out) = made_with("plain", "\nhorse battery staple\nhorse battery staple\n");
    assert!(matches!(stored, Stored::Locked(_)), "{out}");
    assert!(
        out.contains("Protect the key with a passphrase? (yes/no, Enter for yes)"),
        "{out}"
    );
    assert!(!out.contains("type `none`"), "{out}");
    assert!(
        out.contains("Passphrase (anything but `none` or `no`)"),
        "{out}"
    );
}

#[test]
fn an_answer_that_is_neither_yes_nor_no_is_asked_again() {
    let (stored, out) = made_with("neither", "maybe\nno\n");
    assert!(matches!(stored, Stored::Ready(_)), "{out}");
    assert!(out.contains("Answer yes or no."), "{out}");
}

#[test]
fn none_typed_as_the_passphrase_is_asked_about_and_yes_makes_a_key_without_one() {
    // What the owner did: Enter at the question, then `none` twice. It made a key locked with `none`.
    for (name, word) in [("none", "none"), ("no", "No"), ("nothing", "")] {
        let (stored, out) = made_with(name, &format!("\n{word}\nyes\n"));
        assert!(matches!(stored, Stored::Ready(_)), "{word:?}: {out}");
        assert!(
            out.contains("Did you mean no passphrase? (yes/no)"),
            "{word:?}: {out}"
        );
    }
}

#[test]
fn none_typed_as_the_passphrase_and_then_no_asks_for_a_real_one() {
    let (stored, out) = made_with(
        "none-then-real",
        "\nnone\nno\nhorse battery staple\nhorse battery staple\n",
    );
    assert!(out.contains("Then choose a passphrase"), "{out}");
    let Stored::Locked(locked) = stored else {
        panic!("a key with a passphrase: {out}")
    };
    assert!(
        locked.unlock("none").is_err(),
        "`none` was never kept as the passphrase"
    );
    assert!(locked.unlock("horse battery staple").is_ok());
}

#[test]
fn a_refused_passphrase_brings_what_to_try_and_what_to_do_when_it_is_lost() {
    let folder = folder("unlock");
    let (made, out) = asked(&folder, "\nhorse battery staple\nhorse battery staple\n");
    assert!(made.is_some(), "the setup: a key with a passphrase: {out}");

    // Right the first time: nothing more is said.
    let (key, out) = asked(&folder, "horse battery staple\n");
    assert!(key.is_some(), "{out}");
    assert!(!out.contains("move the key aside"), "{out}");

    // Refused once: what to try, and how to start again without losing what was recorded.
    let (key, out) = asked(&folder, "none\nhorse battery staple\n");
    std::fs::remove_dir_all(&folder).ok();
    assert!(key.is_some(), "{out}");
    assert!(
        out.contains(
            "If you typed `none` or `no` when the key was made, that word is the passphrase"
        ),
        "{out}"
    );
    assert!(out.contains("move the key aside"), "{out}");
    assert!(out.contains(sv_check::signed::SIGNING_KEY_FILE), "{out}");
    assert!(out.contains(sv_check::signed::TRUSTED_FILE), "{out}");
    assert_eq!(
        out.matches("move the key aside").count(),
        1,
        "said once: {out}"
    );
}

#[test]
fn a_new_key_made_after_moving_the_old_aside_leaves_the_old_one_trusted() {
    // What the unlock help promises: the old key's line stays in the trusted list, so its seals keep counting.
    let folder = folder("moved-aside");
    let app_folder = folder.join("app");
    std::fs::create_dir_all(&app_folder).unwrap();
    let keys = folder.join("keys");
    let app = sv_check::seal::App::of(&app_folder).unwrap();
    let (old, _) = asked(&keys, "no\n");
    let old = old.expect("the setup: a first key");
    assert!(sv_check::signed::trust_here(&keys, &old, &app).unwrap());
    let path = keys.join(sv_check::signed::SIGNING_KEY_FILE);
    std::fs::rename(&path, path.with_extension("lost")).unwrap();
    let (new, out) = asked(&keys, "no\n");
    let new = new.expect("a new key is made once the old one is aside");
    assert!(out.contains("Made this computer's signing key"), "{out}");
    assert!(sv_check::signed::trust_here(&keys, &new, &app).unwrap());
    let trusted = std::fs::read_to_string(keys.join(sv_check::signed::TRUSTED_FILE)).unwrap();
    std::fs::remove_dir_all(&folder).ok();
    assert!(
        trusted.contains(&old.trusted_line(&app).unwrap()),
        "{trusted}"
    );
    assert!(
        trusted.contains(&new.trusted_line(&app).unwrap()),
        "{trusted}"
    );
}
