//! The names the app looked up reach `seen.json` with no credential in them, each name once with a
//! count (ADR-085, backlog 0240).

use super::*;
use sv_run::name_server::Lookup;

fn rules() -> SecretRules {
    SecretRules::load(&crate::secret_rules_path()).expect("sv's secret rules load")
}

fn asked(name: &str, kind: u16, at: &str) -> Lookup {
    Lookup {
        name: name.to_owned(),
        kind,
        at: at.to_owned(),
    }
}

#[test]
fn a_key_in_a_looked_up_name_does_not_reach_the_record() {
    let rules = rules();
    // A key the secrets scan knows, built from pieces so this file holds none, put in a subdomain.
    let key = ["sk", "ant", "api03", "Zp8Kd3Wq1Ls6Vn0Rt4Yb9Xm2Qc"].join("-");
    let name = format!("{key}.leak.example.test");
    // The setup: the scan really finds the key in a name, so the check below means something.
    let (after, found) = redact_around_markers(&rules, &name);
    assert!(
        found >= 1 && !after.contains(&key),
        "the scan misses the key in a name"
    );
    let received = sv_run::stand_ins::StandIns {
        names: Some(vec![
            asked(&name, 1, "2026-10-10T03:00:01.000Z"),
            asked(&name, 28, "2026-10-10T03:00:01.001Z"),
        ]),
        ..Default::default()
    };
    let mut seen = Seen::default();
    stand_ins(&rules, &received, &mut seen);
    let kept = serde_json::to_string(&seen).unwrap();
    assert!(!kept.contains(&key), "a key reached the record: {kept}");
    assert!(
        kept.contains("leak.example.test"),
        "the rest of the name is gone: {kept}"
    );
    assert!(seen.credentials_removed >= 1, "{seen:#?}");
}

#[test]
fn a_name_asked_twice_for_one_kind_is_kept_once_with_a_count() {
    let rules = rules();
    let received = sv_run::stand_ins::StandIns {
        names: Some(vec![
            asked("api.example.test", 1, "t1"),
            asked("api.example.test", 1, "t2"),
            asked("api.example.test", 28, "t3"),
            asked("cdn.example.test", 1, "t4"),
        ]),
        ..Default::default()
    };
    let mut seen = Seen::default();
    stand_ins(&rules, &received, &mut seen);
    let names = seen.stand_ins.names.expect("the name server ran");
    // The same name for two kinds of address is two entries; the repeat is counted, not repeated.
    assert_eq!(
        names,
        vec![
            sv_report::seen::NameAsked {
                name: "api.example.test".to_owned(),
                kind: "A".to_owned(),
                asked: 2,
                first: "t1".to_owned(),
            },
            sv_report::seen::NameAsked {
                name: "api.example.test".to_owned(),
                kind: "AAAA".to_owned(),
                asked: 1,
                first: "t3".to_owned(),
            },
            sv_report::seen::NameAsked {
                name: "cdn.example.test".to_owned(),
                kind: "A".to_owned(),
                asked: 1,
                first: "t4".to_owned(),
            },
        ]
    );
}

#[test]
fn the_names_say_whether_the_name_server_ran_and_asked_nothing() {
    let rules = rules();
    // Ran, and was asked nothing: an empty list, present in the record.
    let ran = sv_run::stand_ins::StandIns {
        names: Some(Vec::new()),
        ..Default::default()
    };
    let mut seen = Seen::default();
    stand_ins(&rules, &ran, &mut seen);
    assert_eq!(seen.stand_ins.names, Some(Vec::new()));
    assert!(!seen.stand_ins.is_empty());
    let kept = serde_json::to_string(&seen.stand_ins).unwrap();
    assert!(kept.contains("\"names\":[]"), "{kept}");

    // Did not run: no `names` key at all, so a reader cannot take it for "asked nothing".
    let absent = sv_run::stand_ins::StandIns::default();
    let mut seen = Seen::default();
    stand_ins(&rules, &absent, &mut seen);
    assert_eq!(seen.stand_ins.names, None);
    let kept = serde_json::to_string(&seen.stand_ins).unwrap();
    assert!(!kept.contains("names"), "{kept}");
}

#[test]
fn the_report_says_how_many_names_the_app_looked_up_and_not_what_they_are() {
    assert_eq!(
        super::names_sentence(None),
        None,
        "no name server, no sentence"
    );
    let none = super::names_sentence(Some(&[])).unwrap();
    assert!(none.contains("looked up no name"), "{none}");

    let lookups = [
        asked("api.example.test", 1, "2026-10-11T00:00:00Z"),
        asked("api.example.test", 28, "2026-10-11T00:00:01Z"),
        asked("cdn.example.test", 1, "2026-10-11T00:00:02Z"),
    ];
    let sentence = super::names_sentence(Some(&lookups)).unwrap();
    // Two distinct names, though three questions: the sentence counts names, once each.
    assert!(sentence.contains("2 distinct names"), "{sentence}");
    assert!(sentence.contains("stand_ins.names"), "{sentence}");
    assert!(
        !sentence.contains("api.example.test"),
        "a name was printed: {sentence}"
    );
}
