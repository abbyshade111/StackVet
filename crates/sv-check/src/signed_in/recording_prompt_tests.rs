//! For an AI question, the recorder keeps the prompt `sv` handed it and nothing else of the request (backlog 0623): not
//! a form field the app's page filled in, which can hold a session's token, and not the request's cookie.

use super::*;
use crate::probes::{ProbeRequest, ProbeResponse};
use crate::signed_in::Http;

struct Canned;

impl Http for Canned {
    fn send(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        Some(ProbeResponse {
            id: request.id.clone(),
            status: 200,
            headers: Vec::new(),
            body: "a reply".to_owned(),
        })
    }
}

fn asked(id: &str) -> ProbeRequest {
    ProbeRequest {
        id: id.to_owned(),
        method: "POST".to_owned(),
        path: "/chat".to_owned(),
        headers: vec![("cookie".to_owned(), "sid=Rk7Pz2Lw9".to_owned())],
        body: Some(b"message=Hello+SV-PROBE-PLAIN-t1&csrf_token=Qx81Hd0sZ".to_vec()),
    }
}

#[test]
fn the_prompt_is_kept_and_the_form_token_and_cookie_are_not() {
    let mut http = Recording::new(Box::new(Canned));
    http.prompt("ai-plain-1", "Hello SV-PROBE-PLAIN-t1");
    http.send(&asked("ai-plain-1"));
    // Asked again under the same id, after a limiter's wait: the prompt goes with it.
    http.send(&asked("ai-plain-1"));
    // A question the AI suite handed no prompt for keeps none.
    http.send(&asked("signed-in-account"));
    let kept = http.finish();
    assert_eq!(kept.len(), 3);
    assert_eq!(kept[0].prompt.as_deref(), Some("Hello SV-PROBE-PLAIN-t1"));
    assert_eq!(
        (kept[1].id.as_str(), kept[1].prompt.as_deref()),
        ("ai-plain-1-2", Some("Hello SV-PROBE-PLAIN-t1"))
    );
    assert_eq!(kept[2].prompt, None);
    let all = format!("{kept:?}");
    assert!(
        !all.contains("Qx81Hd0sZ"),
        "the form's token was kept: {all}"
    );
    assert!(!all.contains("Rk7Pz2Lw9"), "the cookie was kept: {all}");
}
