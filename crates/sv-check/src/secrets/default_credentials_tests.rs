//! Backlog 0625, part 1: a stock default user name and password, in a connection address or a database image's
//! setting, is a finding against V13.2.3 (no default credentials), and only ever a finding: a clean scan says
//! nothing about the password the real service uses. The defaults are built from pieces at run time, so this file
//! holds none for `sv`'s own scan to find.

use super::*;

fn rules() -> &'static SecretRules {
    SecretRules::shared().expect("the rules sv ships")
}

fn found(file: &str, text: &str) -> Vec<String> {
    scan_text(rules(), file, text)
        .into_iter()
        .filter(|f| f.rule_id.starts_with("secrets.default-credential"))
        .map(|f| f.rule_id)
        .collect()
}

/// A stock default pair, `user:word@`, from its two halves. The second half is named for where it goes, not what
/// it is, and the messages below name each case rather than print its text: the code scanner reads a value given to
/// a name like `password`, or printed, as a real one (backlog 0625, part 1).
fn pair(user: &str, second: &str) -> String {
    format!("{user}:{second}@")
}

/// A long word of the app's own, built from pieces so this file holds none whole.
fn own_word() -> String {
    ["Zt4", "k9qL", "w2Xp"].join("-")
}

#[test]
fn a_stock_default_in_an_address_or_an_images_setting_is_found() {
    let (pg, guest, root) = ("postgres", "guest", "root");
    for (case, (file, text)) in [
        (
            "settings.py",
            format!(
                "DATABASE_URL = \"postgresql://{}db:5432/app\"\n",
                pair(pg, pg)
            ),
        ),
        (
            "app.js",
            format!("amqp.connect('amqp://{}localhost')\n", pair(guest, guest)),
        ),
        (
            "config.rb",
            format!("url: 'mysql://{}127.0.0.1/app'\n", pair(root, root)),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            found(file, &text),
            ["secrets.default-credential-address"],
            "{file}, case {case}"
        );
    }
    for (name, word) in [
        ("POSTGRES_PASSWORD", pg),
        ("MYSQL_ROOT_PASSWORD", root),
        ("RABBITMQ_DEFAULT_PASS", guest),
    ] {
        let compose = format!("services:\n  db:\n    environment:\n      {name}: {word}\n");
        assert_eq!(
            found("docker-compose.yml", &compose),
            ["secrets.default-credential-setting"],
            "{name}, as a key"
        );
        let listed = format!("    environment:\n      - {name}={word}\n");
        assert_eq!(
            found("compose.yaml", &listed),
            ["secrets.default-credential-setting"],
            "{name}, as a list item"
        );
    }
}

#[test]
fn a_password_of_its_own_a_value_from_a_setting_or_a_default_user_alone_is_not() {
    let pg = "postgres";
    for (case, (file, text)) in [
        // A user name that is a default, with a password of its own.
        (
            "settings.py",
            format!(
                "DATABASE_URL = \"postgresql://{}db/app\"\n",
                pair(pg, &own_word())
            ),
        ),
        // The password read from a setting.
        (
            "settings.py",
            format!("DATABASE_URL = f\"postgresql://{pg}:{{os.environ['DB_PASSWORD']}}@db/app\"\n"),
        ),
        (
            "docker-compose.yml",
            "      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}\n".to_owned(),
        ),
        (
            "docker-compose.yml",
            "      POSTGRES_PASSWORD_FILE: /run/secrets/db\n".to_owned(),
        ),
        // A default word as part of a longer password is that longer password.
        (
            "docker-compose.yml",
            format!("      POSTGRES_PASSWORD: {pg}-{}\n", own_word()),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        assert!(found(file, &text).is_empty(), "{file}, case {case}");
    }
}

#[test]
fn the_rules_only_report_so_a_clean_scan_claims_nothing_about_default_credentials() {
    let rules = rules();
    // The setup: the rules are loaded, and are findings-only.
    let ours: Vec<&PatternRule> = rules
        .rules()
        .filter(|r| r.id.starts_with("secrets.default-credential"))
        .collect();
    assert_eq!(ours.len(), 2, "the setup: both rules are loaded");
    assert!(
        ours.iter()
            .all(|r| r.findings_only && r.requirement_ids.iter().any(|id| id == "V13.2.3"))
    );
    assert!(
        !rules.requirement_ids().contains(&"V13.2.3"),
        "a clean scan must not offer itself as evidence of no default credentials: {:?}",
        rules.requirement_ids()
    );
    // And the scan's own claim keeps what it always stood for.
    assert!(rules.requirement_ids().contains(&"V13.3.1"));
}
