//! What the stand-in name server is started with, and how its log is read (ADR-085, backlog 0240).

use super::*;

/// What the script writes, as the name server writes it: a `ready` line, then one line per question.
const LOG: &str = concat!(
    "{\"time\":\"2026-10-10T03:00:00.000Z\",\"ready\":true}\n",
    "{\"time\":\"2026-10-10T03:00:01.000Z\",\"name\":\"api.example.test\",\"type\":1}\n",
    "{\"time\":\"2026-10-10T03:00:01.001Z\",\"name\":\"api.example.test\",\"type\":28}\n",
    "not a question at all\n",
    "{\"time\":\"2026-10-10T03:00:02.000Z\",\"name\":\"no-type.example.test\"}\n",
    "{\"time\":\"2026-10-10T03:00:03.000Z\",\"name\":\"x.example.test\",\"type\":65536}\n",
);

#[test]
fn the_log_is_read_question_by_question_and_repeats_are_kept() {
    // The setup: the log holds two questions for one name, and lines that are not questions.
    assert_eq!(LOG.matches("api.example.test").count(), 2);
    let asked = parse_log(LOG);
    // The `ready` line, the garbage, a question with no type, and a type too big for 16 bits are
    // all skipped; the two questions for the same name are both kept, in order.
    assert_eq!(
        asked,
        vec![
            Lookup {
                name: "api.example.test".to_owned(),
                kind: 1,
                at: "2026-10-10T03:00:01.000Z".to_owned(),
            },
            Lookup {
                name: "api.example.test".to_owned(),
                kind: 28,
                at: "2026-10-10T03:00:01.001Z".to_owned(),
            },
        ]
    );
}

#[test]
fn an_empty_log_reads_as_no_questions() {
    assert!(parse_log("").is_empty());
    assert!(parse_log("{\"time\":\"t\",\"ready\":true}\n").is_empty());
}

#[test]
fn query_types_have_plain_names() {
    assert_eq!(kind_name(1), "A");
    assert_eq!(kind_name(28), "AAAA");
    assert_eq!(kind_name(16), "type 16");
}

#[test]
fn the_name_server_is_started_on_the_fenced_network_and_publishes_nothing() {
    // The setup: the script serves port 53 and writes one JSON line per question.
    assert!(
        SCRIPT.contains("bind(53"),
        "the script does not serve port 53"
    );
    assert!(
        SCRIPT.contains("JSON.stringify"),
        "the script writes no log lines"
    );
    let args = start_args("run-net", "run-dns");
    let at = |flag: &str| args.iter().position(|a| a == flag);
    assert_eq!(
        at("--network").map(|i| args[i + 1].as_str()),
        Some("run-net")
    );
    assert_eq!(at("--name").map(|i| args[i + 1].as_str()), Some("run-dns"));
    assert!(
        args.iter().any(|a| a == PROVIDER_IMAGE),
        "not the pinned Node image"
    );
    // The script goes on the command line; nothing is mounted and nothing is published.
    assert!(args.iter().any(|a| a == SCRIPT));
    assert!(at("-v").is_none() && at("-p").is_none() && at("--publish").is_none());
}
