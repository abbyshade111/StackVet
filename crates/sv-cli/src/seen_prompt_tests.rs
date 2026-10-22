//! An AI question's prompt reaches `seen.json` beside its answer, through the same cut as the answer (backlog 0623): a
//! key in it would be taken out, though no prompt `sv` writes carries one.

use super::*;

#[test]
fn the_prompt_is_kept_beside_its_answer_and_a_key_in_it_is_cut() {
    let rules = SecretRules::shared().expect("the rules sv ships");
    // A key the secrets scan knows, built from pieces so this file holds none.
    let key = ["sk", "ant", "api03", "Zp8Kd3Wq1Ls6Vn0Rt4Yb9Xm2Qc"].join("-");
    let asked = sv_check::signed_in::Outcome {
        exchanges: vec![
            sv_check::signed_in::recording::Recorded {
                id: "ai-plain-1".to_owned(),
                method: "POST".to_owned(),
                path: "/chat".to_owned(),
                status: Some(200),
                headers: Vec::new(),
                body: "<p>Hi there.</p>".to_owned(),
                prompt: Some(format!("Hello, use {key} SV-PROBE-PLAIN-t1")),
            },
            sv_check::signed_in::recording::Recorded {
                id: "signed-in-account".to_owned(),
                method: "GET".to_owned(),
                path: "/account".to_owned(),
                status: Some(200),
                headers: Vec::new(),
                body: String::new(),
                prompt: None,
            },
        ],
        ..Default::default()
    };
    let mut seen = Seen::default();
    signed_in(rules, Some(&asked), &mut seen);
    assert_eq!(seen.signed_in.len(), 2);
    let prompt = seen.signed_in[0]
        .prompt
        .as_deref()
        .expect("the prompt was kept");
    assert!(
        prompt.starts_with("Hello, use ") && prompt.ends_with(" SV-PROBE-PLAIN-t1"),
        "{prompt}"
    );
    assert!(
        !prompt.contains(&key),
        "the key in the prompt was kept: {prompt}"
    );
    assert_eq!(seen.signed_in[1].prompt, None);
    // And in the file, the prompt is there for the AI question and absent for the other.
    let json = serde_json::to_value(&seen.signed_in).unwrap();
    assert!(json[0]["prompt"].is_string(), "{json}");
    assert!(json[1].get("prompt").is_none(), "{json}");
}
