//! `av checkpoint` (backlog 0716, part 3): what it signs, with which key, and what a later reading of the
//! checkpoints says when a record is left alone, added to, changed, or rewritten whole.

use super::*;
use av_record::checkpoint::Standing;
use sv_check::seal::{App, Checker};
use sv_frameworks::paths::Canonical;

/// A scratch folder for an app, one for keys, and one this test's records are kept under: `av`'s records are placed
/// by `XDG_STATE_HOME`, which is set for the test and put back, one test at a time.
struct Scratch {
    app: std::path::PathBuf,
    keys: std::path::PathBuf,
    _held: std::sync::MutexGuard<'static, ()>,
    before: Option<std::ffi::OsString>,
}

static ONE_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());

impl Scratch {
    fn new(name: &str) -> Scratch {
        let held = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
        let root =
            std::env::temp_dir().join(format!("av-checkpoint-cmd-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for part in ["app", "keys", "state"] {
            std::fs::create_dir_all(root.join(part)).unwrap();
        }
        let root = root.canonical().unwrap();
        let before = std::env::var_os("XDG_STATE_HOME");
        // SAFETY: set and put back under a lock every test here takes; nothing else in this test binary reads it.
        unsafe { std::env::set_var("XDG_STATE_HOME", root.join("state")) };
        Scratch {
            app: root.join("app"),
            keys: root.join("keys"),
            _held: held,
            before,
        }
    }

    /// Runs `av checkpoint` for the app, with `typed` as what the person types.
    fn checkpoint(&self, typed: &str) -> (Result<bool, String>, String) {
        let mut out = Vec::new();
        let done = run(&self.app, &self.keys, &mut typed.as_bytes(), &mut out);
        (done, String::from_utf8(out).unwrap())
    }

    /// How a record stands, checked as a report on this computer checks it.
    fn standing(&self, kind: Kind) -> Standing {
        checkpoint::standing(
            &self.app,
            kind,
            &Checker::in_folder(Some(&self.keys), &self.app),
        )
    }

    fn call(&self, time: &str) {
        av_record::build_loop::record(
            &self.app,
            &av_record::build_loop::Line {
                time: time.to_owned(),
                tool: "stackvet_status".to_owned(),
                outcome: Some("ok".to_owned()),
                ..Default::default()
            },
        )
        .unwrap();
    }

    fn proxied(&self, tool: &str) {
        let path = av_record::passthrough::record_of(&self.app).unwrap();
        av_record::passthrough::append(
            &path,
            serde_json::json!({ "at": "T", "tool": tool, "answer": "ok" }),
        )
        .unwrap();
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        // SAFETY: as in `new`.
        match self.before.take() {
            Some(v) => unsafe { std::env::set_var("XDG_STATE_HOME", v) },
            None => unsafe { std::env::remove_var("XDG_STATE_HOME") },
        }
    }
}

#[test]
fn each_record_with_lines_is_signed_and_stands_until_a_line_signed_for_is_changed() {
    let s = Scratch::new("signs");
    SigningKey::make_in(&s.keys, None).unwrap();
    s.call("2026-11-22T10:00:00Z");
    s.call("2026-11-22T10:01:00Z");
    s.proxied("read_file");
    assert_eq!(
        s.standing(Kind::BuildLoop),
        Standing::None,
        "the setup: nothing signed yet"
    );

    let (done, out) = s.checkpoint("");
    assert_eq!(done, Ok(true), "{out}");
    for said in [
        "Signed: the build-loop record, as it stands at 2 lines.",
        "Signed: the pass-through's record, as it stands at 1 line.",
        "Nothing to sign for the hook record",
        "Your signing key has no passphrase, so anything that runs as you could have made this checkpoint",
        "It says nothing of lines written after it",
    ] {
        assert!(out.contains(said), "missing {said:?}: {out}");
    }
    assert_eq!(checkpoint::read(&s.app).len(), 2);

    // Left alone, and then added to: what was signed for still stands.
    assert!(matches!(
        s.standing(Kind::BuildLoop),
        Standing::Kept { lines: 2, .. }
    ));
    s.call("2026-11-22T10:02:00Z");
    s.proxied("write_file");
    let Standing::Kept { lines, signed, .. } = s.standing(Kind::BuildLoop) else {
        panic!("more lines after a checkpoint do not undo it");
    };
    assert_eq!(lines, 2);
    assert!(
        signed.contains("SHA256:")
            && signed.contains("trusts for this app (a key on this computer")
            // As the owner's second trial read it: a stray comma and two spaces (backlog 0721, part 2).
            && !signed.contains(",  ("),
        "{signed}"
    );
    assert!(matches!(
        s.standing(Kind::Proxy),
        Standing::Kept { lines: 1, .. }
    ));
    assert_eq!(s.standing(Kind::Hook), Standing::None);

    // The pass-through's record with its first line changed.
    let path = av_record::passthrough::record_of(&s.app).unwrap();
    let whole = std::fs::read_to_string(&path).unwrap();
    assert!(whole.contains("read_file"), "the setup: {whole}");
    std::fs::write(&path, whole.replacen("read_file", "list_files", 1)).unwrap();
    assert!(matches!(
        s.standing(Kind::Proxy),
        Standing::NotKept { lines: 1, .. }
    ));
}

#[test]
fn a_record_rewritten_whole_after_it_was_signed_does_not_stand() {
    let s = Scratch::new("rewritten");
    SigningKey::make_in(&s.keys, None).unwrap();
    s.call("2026-11-22T10:00:00Z");
    s.call("2026-11-22T10:01:00Z");
    assert_eq!(s.checkpoint("").0, Ok(true));
    // Every line replaced, with a chain made again from the start: the record checks out by itself.
    std::fs::remove_file(checkpoint::record_path(&s.app, Kind::BuildLoop).unwrap()).unwrap();
    s.call("2026-11-22T09:00:00Z");
    s.call("2026-11-22T09:01:00Z");
    s.call("2026-11-22T09:02:00Z");
    let seen = av_record::build_loop::read(&s.app);
    assert_eq!(
        (seen.chained, seen.chain_broken_at),
        (3, None),
        "the setup: its own chain holds"
    );
    assert!(matches!(
        s.standing(Kind::BuildLoop),
        Standing::NotKept { lines: 2, .. }
    ));
}

#[test]
fn a_key_with_a_passphrase_is_asked_for_it_and_a_wrong_one_signs_nothing() {
    let s = Scratch::new("passphrase");
    SigningKey::make_in(&s.keys, Some("correct horse battery")).unwrap();
    s.call("2026-11-22T10:00:00Z");

    // Wrong every time: nothing is signed, and nothing is written.
    let (done, out) = s.checkpoint("wrong\nalso wrong\nstill wrong\n");
    assert_eq!(done, Ok(false), "{out}");
    assert!(
        out.contains("The signing key was not unlocked, so nothing was signed."),
        "{out}"
    );
    assert_eq!(
        out.matches("That is not the key's passphrase. Try again.")
            .count(),
        2,
        "{out}"
    );
    assert_eq!(checkpoint::read(&s.app).len(), 0);
    assert!(
        !out.contains("wrong"),
        "what was typed is never shown: {out}"
    );

    // Wrong once, then right.
    let (done, out) = s.checkpoint("wrong\ncorrect horse battery\n");
    assert_eq!(done, Ok(true), "{out}");
    assert!(out.contains("Signed: the build-loop record"), "{out}");
    assert!(!out.contains("has no passphrase"), "{out}");
    assert!(
        !out.contains("correct horse"),
        "the passphrase is never shown: {out}"
    );
    let Standing::Kept { signed, .. } = s.standing(Kind::BuildLoop) else {
        panic!("signed with the unlocked key");
    };
    assert!(
        signed.contains("passphrase"),
        "the report says the key has one: {signed}"
    );
}

#[test]
fn with_no_signing_key_nothing_is_signed_and_av_makes_none() {
    let s = Scratch::new("no-key");
    s.call("2026-11-22T10:00:00Z");
    let (done, out) = s.checkpoint("");
    assert_eq!(done, Ok(false), "{out}");
    assert!(
        out.contains("`sv review` makes one") && out.contains("av never makes a key"),
        "{out}"
    );
    assert_eq!(
        std::fs::read_dir(&s.keys).unwrap().count(),
        0,
        "no key was made"
    );
    assert_eq!(checkpoint::read(&s.app).len(), 0);
}

#[test]
fn a_record_whose_chain_is_broken_is_not_signed() {
    let s = Scratch::new("broken");
    SigningKey::make_in(&s.keys, None).unwrap();
    s.call("2026-11-22T10:00:00Z");
    s.call("2026-11-22T10:01:00Z");
    let path = checkpoint::record_path(&s.app, Kind::BuildLoop).unwrap();
    let whole = std::fs::read_to_string(&path).unwrap();
    std::fs::write(
        &path,
        whole.replacen("stackvet_status", "stackvet_check", 1),
    )
    .unwrap();
    let (done, out) = s.checkpoint("");
    assert_eq!(done, Ok(false), "{out}");
    assert!(
        out.contains("Not signed: the chain of the build-loop record does not check out at line")
            && out.contains("Signing it would be signing for that change."),
        "{out}"
    );
    assert_eq!(checkpoint::read(&s.app).len(), 0);
}

#[test]
fn a_checkpoint_signed_by_a_key_this_computer_does_not_trust_is_not_taken_as_the_owner_s() {
    let s = Scratch::new("untrusted");
    SigningKey::make_in(&s.keys, None).unwrap();
    s.call("2026-11-22T10:00:00Z");
    assert_eq!(s.checkpoint("").0, Ok(true));
    assert!(
        matches!(s.standing(Kind::BuildLoop), Standing::Kept { .. }),
        "the setup: trusted here"
    );
    // Read with another computer's keys folder, which names no key for this app.
    let elsewhere = s.keys.with_file_name("other-keys");
    std::fs::create_dir_all(&elsewhere).unwrap();
    let standing = checkpoint::standing(
        &s.app,
        Kind::BuildLoop,
        &Checker::in_folder(Some(&elsewhere), &s.app),
    );
    assert!(
        matches!(standing, Standing::Unchecked { .. }),
        "{standing:?}"
    );
    // And a checkpoint for this app is not one for another.
    let other_app = s.app.with_file_name("other-app");
    std::fs::create_dir_all(&other_app).unwrap();
    assert_ne!(
        App::of(&s.app).unwrap().id(),
        App::of(&other_app).unwrap().id()
    );
}
