//! A folder the owner leaves out of the checks, end to end (ADR-099): left out only once the owner
//! confirmed it through `sv review`, a key found there counted all the same, and the folder and its
//! reason always named in the report.

use std::path::{Path, PathBuf};
use std::process::Command;

/// A key's shape, built here so this file holds none.
fn planted_key() -> String {
    ["sk", "ant", "api03", "Zp8Kd3Wq1Ls6Vn0Rt4Yb9Xm2Qc"].join("-")
}

const WHY: &str =
    "Recorded answers from the payment provider, kept to replay in tests, not code we run.";

/// An app whose `fixtures/` holds a weak hash (a finding) and a key, with `entry` as its
/// `[[left-out]]` entry (none when empty). Its own code is `app.py`.
fn app(root: &Path, entry: &str) -> PathBuf {
    let dir = root.join("app");
    std::fs::create_dir_all(dir.join("fixtures")).unwrap();
    std::fs::write(dir.join("app.py"), "print('hello')\n").unwrap();
    std::fs::write(
        dir.join("fixtures/old.py"),
        "import hashlib\n\ndef digest(b):\n    return hashlib.md5(b).hexdigest()\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("fixtures/settings.py"),
        format!("ANTHROPIC_API_KEY = \"{}\"\n", planted_key()),
    )
    .unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        format!(
            "manifest-version = 1\n[app]\nname = \"Shop\"\n[stack]\nlanguages = [\"python\"]\n\
             [capabilities]\nauth = false\n{entry}"
        ),
    )
    .unwrap();
    dir
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-left-out-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("config")).unwrap();
    dir
}

/// `sv report` on the app, with this scratch folder's keys: the report's words and its findings, as
/// the file each is in and whether it holds a key.
fn report(root: &Path, app: &Path) -> (String, Vec<(String, bool)>) {
    let out = root.join("report");
    let run = Command::new(env!("CARGO_BIN_EXE_sv"))
        .arg("report")
        .arg(app)
        .arg("--out")
        .arg(&out)
        .env("XDG_CONFIG_HOME", root.join("config"))
        .output()
        .expect("sv runs");
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("report.json")).unwrap()).unwrap();
    let findings = json["findings"]
        .as_array()
        .expect("a findings list")
        .iter()
        .map(|f| {
            (
                f["location"]["file"].as_str().unwrap_or("").to_owned(),
                !f["secret"].is_null(),
            )
        })
        .collect();
    (
        std::fs::read_to_string(out.join("security.md")).unwrap(),
        findings,
    )
}

/// The entry, sealed as `sv review` seals it with this scratch folder's key, for the app at `app`.
fn confirmed(root: &Path, app: &Path, folder: &str, why: &str, by: &str) -> String {
    let key = sv_check::seal::Key::load_or_make_in(
        &root.join("config").join(sv_frameworks::names::CONFIG_DIR),
    )
    .expect("a key")
    .0
    .for_app(&sv_check::seal::App::of(app).unwrap());
    let entry = sv_manifest::LeftOut {
        folder: folder.into(),
        why: why.into(),
        by: Some(by.into()),
        on: Some("2026-10-12".into()),
        seal: None,
    };
    let fields = sv_check::seal::left_out_fields(&entry);
    let seal = key.seal(&sv_check::seal::as_strs(&fields));
    format!(
        "[[left-out]]\nfolder = \"{folder}\"\nwhy = \"{why}\"\nby = \"{by}\"\non = \"2026-10-12\"\n\
         seal = \"{seal}\"\n"
    )
}

fn in_fixtures(findings: &[(String, bool)]) -> (usize, usize) {
    let there = findings.iter().filter(|(f, _)| f.starts_with("fixtures/"));
    let keys = there.clone().filter(|(_, key)| *key).count();
    (there.count() - keys, keys)
}

#[test]
fn a_folder_is_left_out_only_once_the_owner_confirmed_it_and_its_keys_still_count() {
    // The control: with no entry, the weak hash and the key in `fixtures/` are both findings. This
    // is what makes the absences below mean something.
    let root = scratch("none");
    let plain = app(&root, "");
    let (_, findings) = report(&root, &plain);
    let (others, keys) = in_fixtures(&findings);
    assert!(others >= 1, "the weak hash is found: {findings:?}");
    assert_eq!(keys, 1, "the key is found: {findings:?}");
    std::fs::remove_dir_all(&root).ok();

    // Written by the AI coding tool and never confirmed: the folder is checked, and says it waits.
    let root = scratch("waiting");
    let waiting = app(
        &root,
        &format!("[[left-out]]\nfolder = \"fixtures\"\nwhy = \"{WHY}\"\nby = \"owner\"\n"),
    );
    let (security, findings) = report(&root, &waiting);
    assert_eq!(in_fixtures(&findings), (others, 1), "{findings:?}");
    assert!(
        security.contains("the folder `fixtures`, left out of the checks"),
        "{security}"
    );
    assert!(
        security.contains("nobody has confirmed it through `sv review`"),
        "{security}"
    );
    assert!(security.contains(WHY), "the reason is printed: {security}");
    std::fs::remove_dir_all(&root).ok();

    // Confirmed: every finding there but the key is set aside, and the report says how many.
    let root = scratch("confirmed");
    let dir = app(&root, "");
    let entry = confirmed(&root, &dir, "fixtures", WHY, "owner");
    std::fs::write(
        dir.join("stackvet.toml"),
        std::fs::read_to_string(dir.join("stackvet.toml")).unwrap() + &entry,
    )
    .unwrap();
    let (security, findings) = report(&root, &dir);
    assert_eq!(in_fixtures(&findings), (0, 1), "{findings:?}");
    assert!(
        security.contains("confirmed by the owner on 2026-10-12"),
        "{security}"
    );
    assert!(security.contains(WHY), "the reason is printed: {security}");
    let set_aside = format!(
        "{others} finding{} there {} not counted",
        if others == 1 { "" } else { "s" },
        if others == 1 { "is" } else { "are" }
    );
    assert!(security.contains(&set_aside), "{set_aside} in {security}");

    // The reason changed after it was confirmed: the seal no longer holds, and the folder is
    // checked again.
    let edited = std::fs::read_to_string(dir.join("stackvet.toml"))
        .unwrap()
        .replace(
            WHY,
            "Old code nobody runs any more, so its findings are noise to us.",
        );
    std::fs::write(dir.join("stackvet.toml"), edited).unwrap();
    let (security, findings) = report(&root, &dir);
    assert_eq!(in_fixtures(&findings), (others, 1), "{findings:?}");
    assert!(
        security.contains("its confirmation does not count here"),
        "{security}"
    );
    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_confirmed_folder_that_holds_the_start_file_is_refused() {
    let root = scratch("start");
    let dir = app(&root, "");
    let entry = confirmed(&root, &dir, "fixtures", WHY, "owner");
    let manifest = std::fs::read_to_string(dir.join("stackvet.toml"))
        .unwrap()
        .replace(
            "[capabilities]",
            "[stack.run]\nstart = \"python fixtures/old.py\"\n[capabilities]",
        );
    std::fs::write(dir.join("stackvet.toml"), manifest + &entry).unwrap();
    let (security, findings) = report(&root, &dir);
    assert!(in_fixtures(&findings).0 >= 1, "still checked: {findings:?}");
    assert!(
        security.contains("which the start command runs, so it is not left out"),
        "{security}"
    );
    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_seal_that_holds_counts_only_when_the_entry_is_the_owner_s() {
    // A seal made with this computer's key over an entry that names someone else: `sv review` never
    // writes one, and a folder is left out on the owner's word alone.
    let root = scratch("not-owner");
    let dir = app(&root, "");
    let entry = confirmed(&root, &dir, "fixtures", WHY, "ai-tool");
    std::fs::write(
        dir.join("stackvet.toml"),
        std::fs::read_to_string(dir.join("stackvet.toml")).unwrap() + &entry,
    )
    .unwrap();
    let (security, findings) = report(&root, &dir);
    assert!(in_fixtures(&findings).0 >= 1, "still checked: {findings:?}");
    assert!(
        security.contains("it is not marked as the owner's"),
        "{security}"
    );
    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_folder_that_holds_all_the_app_s_code_is_not_left_out() {
    // The guard ADR-031 gives `not-the-app`, for `[[left-out]]` too: with the app's only code in
    // the folder, leaving it out would leave nothing to grade.
    let root = scratch("all-code");
    let dir = app(&root, "");
    std::fs::remove_file(dir.join("app.py")).unwrap();
    let entry = confirmed(&root, &dir, "fixtures", WHY, "owner");
    std::fs::write(
        dir.join("stackvet.toml"),
        std::fs::read_to_string(dir.join("stackvet.toml")).unwrap() + &entry,
    )
    .unwrap();
    let (security, findings) = report(&root, &dir);
    assert!(in_fixtures(&findings).0 >= 1, "still checked: {findings:?}");
    assert!(
        security.contains("would leave none of the app's code, so every folder is read as the app"),
        "{security}"
    );
    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn what_a_folder_left_out_uses_cannot_overrule_the_manifest() {
    // As for `not-the-app`: a package in the folder is not one the app uses, even before the owner
    // confirms the entry. The control first: unnamed, the folder's package overrules the manifest.
    const OVERRULED: &str = "stackvet.toml says auth is not used, but `flask-login`";
    let scope = |entry: &str, name: &str| {
        let root = scratch(name);
        let dir = app(&root, entry);
        std::fs::write(
            dir.join("fixtures/requirements.txt"),
            "flask-login==0.6.3\n",
        )
        .unwrap();
        let out = Command::new(env!("CARGO_BIN_EXE_sv"))
            .arg("scope")
            .arg(&dir)
            .env("XDG_CONFIG_HOME", root.join("config"))
            .output()
            .expect("sv runs");
        std::fs::remove_dir_all(&root).ok();
        String::from_utf8_lossy(&out.stdout).into_owned()
    };
    assert!(scope("", "evidence-control").contains(OVERRULED));
    let waiting = format!("[[left-out]]\nfolder = \"fixtures\"\nwhy = \"{WHY}\"\n");
    let said = scope(&waiting, "evidence");
    assert!(!said.contains(OVERRULED), "{said}");
}
