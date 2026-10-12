//! `[[left-out]]`: the folders the owner leaves out of the checks, each with a reason (ADR-099). An
//! entry is refused as a `not-the-app` entry is, and for giving no reason; every usable folder is
//! read apart from the app.

use std::path::Path;
use sv_manifest::Manifest;

fn manifest(rest: &str) -> Manifest {
    let text = format!(
        "manifest-version = 1\n[app]\nname = \"t\"\n[stack]\nlanguages = [\"python\"]\n\
         [stack.run]\nstart = \"python server/app.py\"\n{rest}"
    );
    Manifest::parse(&text, Path::new("stackvet.toml")).expect("parses")
}

const WHY: &str = "why = \"Recorded answers, kept to replay in tests.\"";

#[test]
fn each_entry_is_used_or_refused_with_why() {
    let m = manifest(&format!(
        "[repository]\nnot-the-app = [\"examples\"]\n\
         [[left-out]]\nfolder = \"./tests/fixtures/\"\n{WHY}\n\
         [[left-out]]\nfolder = \"vendor\\\\old\"\n{WHY}\n\
         [[left-out]]\nfolder = \"legacy\"\nwhy = \"  \"\n\
         [[left-out]]\nfolder = \"*\"\n{WHY}\n\
         [[left-out]]\nfolder = \"../elsewhere\"\n{WHY}\n\
         [[left-out]]\nfolder = \"server\"\n{WHY}\n\
         [[left-out]]\nfolder = \"examples\"\n{WHY}\n"
    ));
    let (used, refused) = m.left_out();
    assert_eq!(
        used,
        [
            (0, "tests/fixtures".to_owned()),
            (1, "vendor/old".to_owned()),
            (6, "examples".to_owned()),
        ]
    );
    let refused = refused.join("\n");
    for part in [
        "`legacy`: it gives no reason in `why`",
        "`*`: it names the whole app, so it is not left out",
        "`../elsewhere`: it is not a plain path inside the app folder",
        "`server`: it holds `server/app.py`, which the start command runs, so it is not left out",
    ] {
        assert!(refused.contains(part), "{part} in:\n{refused}");
    }
    // Every usable folder is read apart from the app, `not-the-app`'s first, each once.
    assert_eq!(m.set_apart(), ["examples", "tests/fixtures", "vendor/old"]);
}

#[test]
fn not_the_app_reads_as_it_did() {
    let m = manifest(
        "[repository]\nnot-the-app = [\"examples/\", \"server\", \".\", \"crates/*/tests\"]\n",
    );
    let (folders, refused) = m.not_the_app();
    assert_eq!(folders, ["examples", "crates/*/tests"]);
    assert_eq!(
        refused,
        [
            "`server`: it holds `server/app.py`, which the start command runs, so it is not used",
            "`.`: it names the whole app, so it is not used",
        ]
    );
    // With no `[[left-out]]`, the folders set apart are `not-the-app`'s alone.
    assert_eq!(m.set_apart(), folders);
}

#[test]
fn an_unknown_key_in_an_entry_is_refused() {
    let text = "manifest-version = 1\n[app]\nname = \"t\"\n[stack]\nlanguages = []\n\
                [[left-out]]\nfolder = \"tests\"\nreason = \"the key is `why`\"\n";
    assert!(Manifest::parse(text, Path::new("stackvet.toml")).is_err());
}
