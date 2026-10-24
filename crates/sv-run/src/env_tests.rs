//! Backlog 0659, ADR-141: `[stack.run] env` sets variables for the copy of the app `sv` runs, and refuses a name that
//! is not a variable, one `sv` sets itself, or one that says it holds a credential.

use super::*;

fn manifest(env: &str) -> Manifest {
    let text = format!(
        "manifest-version = 1\n\n[app]\nname = \"x\"\n\n[stack]\nlanguages = [\"python\"]\n\n[stack.run]\nimage = \"python:3.12-slim\"\nstart = \"python app.py\"\n{env}"
    );
    Manifest::parse(&text, Path::new("stackvet.toml")).expect("the manifest written here")
}

fn plan(env: &str) -> Result<RunPlan, CannotRun> {
    RunPlan::from_manifest(&manifest(env), Path::new("."))
}

#[test]
fn a_data_folder_for_svs_copy_is_carried_to_the_app() {
    let made =
        plan("env = { DATABASE_PATH = \"/tmp/app.db\", APP_ENV = \"sv\" }\n").expect("a plan");
    assert_eq!(
        made.env,
        [
            ("APP_ENV".to_owned(), "sv".to_owned()),
            ("DATABASE_PATH".to_owned(), "/tmp/app.db".to_owned())
        ]
    );
    assert!(
        plan("").expect("a plan").env.is_empty(),
        "none when none is written"
    );
}

#[test]
fn a_name_sv_sets_a_credentials_name_or_not_a_name_is_refused() {
    for (env, why) in [
        ("env = { PORT = \"9000\" }\n", "sets that variable itself"),
        ("env = { PATH = \"/bin\" }\n", "sets that variable itself"),
        (
            "env = { SV_ANYTHING = \"1\" }\n",
            "sets that variable itself",
        ),
        ("env = { SECRET_KEY = \"dev\" }\n", "holds a credential"),
        ("env = { API_TOKEN = \"x\" }\n", "holds a credential"),
        ("env = { DB_PASSWORD = \"x\" }\n", "holds a credential"),
        ("env = { \"1BAD\" = \"x\" }\n", "not a variable name"),
        ("env = { \"A-B\" = \"x\" }\n", "not a variable name"),
        ("env = { NOTE = \"two\\nlines\" }\n", "line break"),
    ] {
        match plan(env) {
            Err(CannotRun::BadEnv { why: said, .. }) => {
                assert!(said.contains(why), "{env}: {said}")
            }
            other => panic!("{env}: refused expected, got {:?}", other.map(|p| p.env)),
        }
    }
    // And the person is told, in plain words, what was refused and what `env` is for.
    let Err(refused) = plan("env = { SECRET_KEY = \"dev\" }\n") else {
        panic!("refused")
    };
    let said = refused.explain();
    assert!(
        said.contains("`SECRET_KEY`") && said.contains("{seed:NAME}"),
        "{said}"
    );
}
