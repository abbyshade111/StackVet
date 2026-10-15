//! A made-up session is judged beside the real one (backlog 0560, ADR-119): a sign-in page served with 200 is
//! not the page opening, and a page that opens to somebody not signed in is not asked at all.

use super::*;

/// An app that answers each request id from its script: (status, body), and 403 to the rest.
struct Scripted(Vec<(&'static str, u16, &'static str)>);

impl Http for Scripted {
    fn send(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        let (status, body) = self
            .0
            .iter()
            .find(|(id, _, _)| *id == request.id)
            .map_or((403, ""), |(_, s, b)| (*s, *b));
        Some(ProbeResponse {
            id: request.id.clone(),
            status,
            headers: Vec::new(),
            body: body.to_owned(),
        })
    }
}

fn signed_in() -> SignedIn {
    let mut session = Session::default();
    session
        .cookies
        .push(("sid".into(), "0123456789abcdef0123".into()));
    SignedIn {
        session,
        set_at_login: vec![Cookie {
            name: "sid".into(),
            value: String::new(),
            http_only: true,
            same_site: None,
            secure: false,
            path: None,
        }],
        before_login: Vec::new(),
        landed: String::new(),
        answer: String::new(),
        limited: false,
    }
}

const ACCOUNT: &str = "<h1>Your account</h1><p>alice@example.com</p><form action='/logout'><button>Sign out</button></form>";
const SIGN_IN: &str = "<h1>Please sign in</h1><form action='/login'><input name='email'><button>Continue</button></form>";

fn check(app: &mut Scripted, private: &[&str], served_anonymously: &[&str]) -> Outcome {
    let private: Vec<String> = private.iter().map(|p| (*p).to_owned()).collect();
    let anonymous: Vec<String> = served_anonymously.iter().map(|p| (*p).to_owned()).collect();
    let mut out = Outcome::default();
    invented_session_check(
        app,
        &signed_in(),
        Some(private[0].as_str()),
        &private,
        &anonymous,
        Some(true),
        &mut out,
    );
    out
}

#[test]
fn a_sign_in_page_served_with_200_to_a_made_up_session_is_not_the_page_opening() {
    let mut app = Scripted(vec![
        ("invented-session-control", 200, ACCOUNT),
        ("invented-session", 200, SIGN_IN),
    ]);
    let o = check(&mut app, &["/account"], &[]);
    assert!(
        o.findings.is_empty(),
        "a sign-in page on a 200 was read as the made-up session taken: {:?}",
        o.findings
    );
    assert!(o.verified.is_empty());
    assert!(
        o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("such as a sign-in form served with 200")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn the_page_itself_served_to_a_made_up_session_is_still_found() {
    let mut app = Scripted(vec![
        ("invented-session-control", 200, ACCOUNT),
        ("invented-session", 200, ACCOUNT),
    ]);
    let o = check(&mut app, &["/account"], &[]);
    assert_eq!(o.findings.len(), 1, "{:?}", o.not_assessed);
}

#[test]
fn a_page_open_to_everybody_is_not_asked_with_a_made_up_session() {
    // `/public` opens to anybody; asked with a made-up session, it would open and be blamed on the session.
    let mut app = Scripted(vec![
        ("invented-session-control", 200, ACCOUNT),
        ("invented-session", 403, ""),
        ("invented-session-control-1", 200, ACCOUNT),
        ("invented-session-1", 200, ACCOUNT),
    ]);
    let o = check(&mut app, &["/account", "/public"], &["/public"]);
    assert!(o.findings.is_empty(), "{:?}", o.findings);
    assert!(
        o.steps
            .iter()
            .any(|s| s.contains("did not ask /public with a made-up session")),
        "{:?}",
        o.steps
    );
    // The page refused is credited, in part: one page stood for the list.
    assert!(o.verified.iter().all(|v| v.in_part), "{:?}", o.verified);
    assert_eq!(o.verified.len(), 1);
}
