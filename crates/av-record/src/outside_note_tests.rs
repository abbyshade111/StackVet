//! The note kept outside the app's folder (backlog 0716, part 3): what is noted, when, and what a record rewritten by
//! something that cannot reach the note looks like against it.

use super::*;
use crate::build_loop::{Line, record};

/// An app's folder and a folder this test's notes are kept under. The notes are placed by `XDG_STATE_HOME`, which is
/// set for the test and put back, one test at a time; nothing else in this crate's tests reads it.
struct Scratch {
    app: PathBuf,
    _held: std::sync::MutexGuard<'static, ()>,
    before: Option<std::ffi::OsString>,
}

static ONE_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());

impl Scratch {
    fn new(name: &str) -> Scratch {
        let held = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
        let root = std::env::temp_dir().join(format!("av-note-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("app")).unwrap();
        std::fs::create_dir_all(root.join("state")).unwrap();
        let root = sv_frameworks::paths::canonical(&root).unwrap();
        let before = std::env::var_os("XDG_STATE_HOME");
        // SAFETY: set and put back under a lock every test here takes.
        unsafe { std::env::set_var("XDG_STATE_HOME", root.join("state")) };
        Scratch {
            app: root.join("app"),
            _held: held,
            before,
        }
    }

    /// One call written into the record, and the place noted as `sv mcp` notes it.
    fn call(&self, time: &str) {
        record(&self.app, &line(time)).unwrap();
        write(&self.app, time);
    }

    fn record_file(&self) -> PathBuf {
        crate::checkpoint::record_path(&self.app, crate::checkpoint::Kind::BuildLoop).unwrap()
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

fn line(time: &str) -> Line {
    Line {
        time: time.to_owned(),
        tool: "stackvet_status".to_owned(),
        outcome: Some("ok".to_owned()),
        ..Default::default()
    }
}

#[test]
fn a_record_written_through_sv_passes_through_every_place_noted() {
    let s = Scratch::new("kept");
    assert_eq!(
        standing(&s.app),
        Standing::None,
        "the setup: nothing noted yet"
    );
    s.call("T1");
    s.call("T2");
    s.call("T3");
    assert_eq!(
        read(&s.app).iter().map(|n| n.lines).collect::<Vec<_>>(),
        [1, 2, 3]
    );
    assert_eq!(
        standing(&s.app),
        Standing::Kept {
            noted: 3,
            lines: 3,
            after: 0
        }
    );
    // The notes are outside the app's folder, and hold nothing of what the record holds.
    let path = file_of(&s.app).unwrap();
    assert!(!path.starts_with(&s.app), "{path:?}");
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(!text.contains("stackvet_status"), "{text}");
    // Lines written with no note after them are counted as not noted, never as changed.
    record(&s.app, &line("T4")).unwrap();
    assert_eq!(
        standing(&s.app),
        Standing::Kept {
            noted: 3,
            lines: 3,
            after: 1
        }
    );
}

#[test]
fn a_record_rewritten_whole_inside_the_folder_does_not_pass_through_what_was_noted() {
    let s = Scratch::new("rewritten");
    s.call("T1");
    s.call("T2");
    // What a tool confined to the folder can do: replace every line, with a chain made again from the start.
    std::fs::remove_file(s.record_file()).unwrap();
    record(&s.app, &line("T0")).unwrap();
    record(&s.app, &line("T0b")).unwrap();
    let seen = crate::build_loop::read(&s.app);
    assert_eq!(
        (seen.chained, seen.chain_broken_at),
        (2, None),
        "the setup: its own chain holds"
    );
    assert_eq!(
        standing(&s.app),
        Standing::NotKept {
            at: "T1".to_owned(),
            lines: 1
        }
    );
    // And when sv next writes a line and notes the new record, the old notes still show it.
    s.call("T3");
    assert_eq!(
        standing(&s.app),
        Standing::NotKept {
            at: "T1".to_owned(),
            lines: 1
        }
    );
}

#[test]
fn a_line_changed_or_the_record_cut_or_gone_does_not_pass_either() {
    let s = Scratch::new("changed");
    s.call("T1");
    s.call("T2");
    s.call("T3");
    let whole = std::fs::read_to_string(s.record_file()).unwrap();
    // The second line changed: the first place noted still stands, the second does not.
    let mut lines: Vec<String> = whole.lines().map(str::to_owned).collect();
    lines[1] = lines[1].replacen("stackvet_status", "stackvet_check", 1);
    std::fs::write(s.record_file(), lines.join("\n") + "\n").unwrap();
    assert_eq!(
        standing(&s.app),
        Standing::NotKept {
            at: "T2".to_owned(),
            lines: 2
        }
    );
    // Cut to its first line.
    std::fs::write(
        s.record_file(),
        format!("{}\n", whole.lines().next().unwrap()),
    )
    .unwrap();
    assert_eq!(
        standing(&s.app),
        Standing::NotKept {
            at: "T2".to_owned(),
            lines: 2
        }
    );
    // Gone.
    std::fs::remove_file(s.record_file()).unwrap();
    assert_eq!(
        standing(&s.app),
        Standing::NotKept {
            at: "T1".to_owned(),
            lines: 1
        }
    );
}

#[test]
fn nothing_is_noted_unless_the_program_turns_notes_on_and_a_broken_record_is_never_noted() {
    let s = Scratch::new("off");
    // `record` alone, as every test of the record calls it: no note, so nothing is written under the state folder.
    assert!(
        !ON.load(std::sync::atomic::Ordering::Relaxed),
        "the setup: notes are off in tests"
    );
    record(&s.app, &line("T1")).unwrap();
    assert_eq!(read(&s.app), Vec::<Noted>::new());
    assert!(!file_of(&s.app).unwrap().exists());
    // A record whose chain does not check out is not noted: the note would vouch for the change.
    record(&s.app, &line("T2")).unwrap();
    let whole = std::fs::read_to_string(s.record_file()).unwrap();
    std::fs::write(
        s.record_file(),
        whole.replacen("stackvet_status", "stackvet_check", 1),
    )
    .unwrap();
    write(&s.app, "T3");
    assert_eq!(read(&s.app), Vec::<Noted>::new());
}
