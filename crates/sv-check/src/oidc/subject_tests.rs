//! Backlog 0625, part 5, ADR-148: V10.3.3, an access token for a different person who has the first person's email
//! address, sent as a bearer token where the first person's own shows what they saved. Against a small app and provider
//! of their own: the app knows a bearer token's person by its subject, or by its email.

use super::*;
use crate::probes::ProbeResponse;

#[derive(Default)]
struct Fake {
    /// The app knows a bearer token's person by its email address, not its subject.
    keys_on_email: bool,
    /// The page that shows what was saved takes no bearer tokens.
    shows_no_bearer: bool,
    /// The first person saved nothing that shows.
    nothing_saved: bool,
    /// After the other person's token, the page stops showing anything.
    breaks_after_other: bool,
    other_sent: bool,
}

fn answer(status: u16, body: &str) -> ProbeResponse {
    ProbeResponse {
        id: String::new(),
        status,
        headers: Vec::new(),
        body: body.to_owned(),
    }
}

impl Http for Fake {
    fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        if r.path != "/notes" {
            return Some(answer(404, ""));
        }
        let token = r
            .headers
            .iter()
            .find(|(k, _)| k == "Authorization")
            .and_then(|(_, v)| v.strip_prefix("Bearer "));
        let Some(token) = token.filter(|_| !self.shows_no_bearer) else {
            return Some(answer(401, ""));
        };
        // Each token says who it is for: its subject, and the email both people share.
        let first = match token {
            "token-first" => true,
            "token-other" => {
                self.other_sent = true;
                self.keys_on_email
            }
            _ => return Some(answer(401, "")),
        };
        if self.breaks_after_other && self.other_sent && token == "token-first" {
            return Some(answer(200, "<ul></ul>"));
        }
        Some(if first && !self.nothing_saved {
            answer(200, &format!("<ul><li>{MARKER}</li></ul>"))
        } else {
            answer(200, "<ul></ul>")
        })
    }

    fn provider(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        if r.path != idp::ACCESS_TOKEN {
            return Some(answer(404, ""));
        }
        let form = r.body_text();
        let token = if form.contains(&format!("who={}", idp::OTHER_SUBJECT)) {
            "token-other"
        } else {
            "token-first"
        };
        Some(answer(200, &format!("{{\"token\":\"{token}\"}}")))
    }
}

fn section(create: bool) -> OidcSection {
    OidcSection {
        start: "/login/oidc".to_owned(),
        private: "/account".to_owned(),
        shows: Some("/notes".to_owned()),
        create: create.then(|| sv_manifest::RequestTemplate {
            method: "POST".to_owned(),
            path: "/notes".to_owned(),
            form: [("text".to_owned(), "{marker}".to_owned())].into(),
            json: Default::default(),
        }),
        ..Default::default()
    }
}

fn run(fake: Fake, create: bool) -> Outcome {
    let mut out = Outcome::default();
    access_token_subject(&mut { fake }, &section(create), &mut out);
    out
}

fn not_assessed(out: &Outcome) -> Vec<&str> {
    out.not_assessed
        .iter()
        .filter(|(id, _)| id == "V10.3.3")
        .map(|(_, why)| why.as_str())
        .collect()
}

#[test]
fn an_api_that_knows_people_by_subject_is_credited_in_part() {
    let out = run(Fake::default(), true);
    assert!(out.findings.is_empty(), "{:#?}", out.findings);
    let credit = out
        .verified
        .iter()
        .find(|v| v.check_id == SUBJECT.rule_id)
        .expect("credited");
    assert!(credit.in_part);
    assert_eq!(credit.requirement_ids, ["V10.3.3"]);
}

#[test]
fn an_api_that_knows_people_by_email_is_a_finding() {
    let out = run(
        Fake {
            keys_on_email: true,
            ..Default::default()
        },
        true,
    );
    assert_eq!(
        out.findings
            .iter()
            .map(|f| f.rule_id.as_str())
            .collect::<Vec<_>>(),
        [SUBJECT.rule_id]
    );
    assert!(out.verified.is_empty());
}

#[test]
fn without_the_control_nothing_is_concluded_and_the_report_says_why() {
    // No `create`, so nothing was saved to look for.
    let out = run(Fake::default(), false);
    assert!(
        not_assessed(&out)[0].contains("Add `create`"),
        "{:?}",
        not_assessed(&out)
    );
    // The page takes no bearer tokens, or shows nothing for the first person: the absence would show nothing.
    for fake in [
        Fake {
            shows_no_bearer: true,
            ..Default::default()
        },
        Fake {
            nothing_saved: true,
            keys_on_email: true,
            ..Default::default()
        },
    ] {
        let out = run(fake, true);
        assert!(out.findings.is_empty() && out.verified.is_empty());
        assert!(
            not_assessed(&out)[0].contains("did not show what they saved"),
            "{:?}",
            not_assessed(&out)
        );
    }
    // The first person's own token stops working after the other's: the refusal is not credited.
    let out = run(
        Fake {
            breaks_after_other: true,
            ..Default::default()
        },
        true,
    );
    assert!(out.verified.is_empty());
    assert!(
        not_assessed(&out)[0].contains("stopped showing"),
        "{:?}",
        not_assessed(&out)
    );
}
