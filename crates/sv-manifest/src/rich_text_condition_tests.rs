//! `rich-text` under `[capabilities]` is the owner's answer for V1.3.1 and V1.3.5 (backlog 0505; ADR-117): read as
//! given, and an unanswered one stays unanswered, never a quiet no, whatever the code shows.

use super::*;

fn manifest(rich_text: Option<bool>) -> Manifest {
    let mut text = String::from("manifest-version = 1\n[app]\nname = \"R\"\n[capabilities]\n");
    if let Some(v) = rich_text {
        text.push_str(&format!("rich-text = {v}\n"));
    }
    text.push_str("[stack]\nlanguages = [\"python\"]\n");
    Manifest::parse(&text, Path::new("stackvet.toml")).expect("manifest parses")
}

#[test]
fn rich_text_is_read_as_the_owner_answers_it_and_silence_stays_unanswered() {
    for answer in [Some(true), Some(false), None] {
        let m = manifest(answer);
        assert_eq!(m.capabilities.rich_text, answer);
        let (ctx, resolved) = resolve(&m, &|_| None);
        assert_eq!(ctx.get(Condition::RichText), answer, "{answer:?}");
        let line = resolved
            .iter()
            .find(|r| r.condition == Condition::RichText)
            .expect("rich-text is among the claims");
        let expected = if answer.is_some() {
            ClaimState::Unverifiable
        } else {
            ClaimState::Unanswered
        };
        assert_eq!(line.state, expected, "{answer:?}");
    }
}

#[test]
fn the_starter_file_asks_it_commented_out() {
    assert!(spec::STARTER_MANIFEST.contains("\n# rich-text = ?"));
}
