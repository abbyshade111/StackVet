//! The Cloudflare client against a stand-in (backlog 0732): what it asks, in what order, and when it stops. Tokens are
//! built from pieces at run time, so nothing here looks like a key to a scanner.

use super::*;
use crate::Answer;
use std::collections::BTreeMap;

/// A stand-in for Cloudflare: an answer for each path under the API's folder, and every address it was asked.
#[derive(Default)]
pub(crate) struct Fake {
    pub answers: BTreeMap<String, (u16, String, Option<String>)>,
    pub asked: Vec<(String, String)>,
}

impl Fake {
    pub fn ok(mut self, path: &str, body: &str) -> Self {
        self.answers
            .insert(path.to_owned(), (200, body.to_owned(), None));
        self
    }
    pub fn answer(mut self, path: &str, status: u16, body: &str) -> Self {
        self.answers
            .insert(path.to_owned(), (status, body.to_owned(), None));
        self
    }
    pub fn redirect(mut self, path: &str, to: &str) -> Self {
        self.answers
            .insert(path.to_owned(), (301, String::new(), Some(to.to_owned())));
        self
    }
}

impl Transport for Fake {
    fn get(&mut self, url: &str, token: &str) -> Result<Answer, String> {
        self.asked.push((url.to_owned(), token.to_owned()));
        let path = url
            .strip_prefix("https://api.cloudflare.com/client/v4")
            .ok_or("not api.cloudflare.com's API")?;
        let (status, body, redirect) = self.answers.get(path).cloned().unwrap_or((
            404,
            r#"{"success":false,"errors":[{"code":7003,"message":"Could not route"}]}"#.to_owned(),
            None,
        ));
        Ok(Answer {
            status,
            headers: Vec::new(),
            body: body.into_bytes(),
            redirect,
        })
    }
}

pub(crate) fn user_token() -> String {
    ["cf", "ut_", &"Q".repeat(40)].concat()
}

const Z: &str = "/zones/z1";

fn value(v: &str) -> String {
    format!(r#"{{"success":true,"result":{{"id":"x","value":{v}}}}}"#)
}

/// A whole zone, every answer there.
pub(crate) fn whole() -> Fake {
    let mut fake = Fake::default()
        .ok(
            "/zones?name=stackvet.dev",
            r#"{"success":true,"result":[{"id":"z1","name":"stackvet.dev","account":{"id":"a1"},"plan":{"name":"Free Website"}}]}"#,
        )
        .ok("/user/tokens/verify", r#"{"success":true,"result":{"id":"t1","status":"active"}}"#);
    for (id, v) in [
        ("ssl", r#""strict""#),
        ("ssl_automatic_mode", r#""custom""#),
        ("min_tls_version", r#""1.2""#),
        ("tls_1_3", r#""on""#),
        ("always_use_https", r#""on""#),
        (
            "security_header",
            r#"{"strict_transport_security":{"enabled":true,"max_age":31536000,"include_subdomains":true,"preload":false,"nosniff":true}}"#,
        ),
        ("automatic_https_rewrites", r#""on""#),
    ] {
        fake = fake.ok(&format!("{Z}/settings/{id}"), &value(v));
    }
    fake.ok(
        &format!("{Z}/rulesets/phases/http_request_cache_settings/entrypoint"),
        r#"{"success":true,"result":{"rules":[{"description":"Static","expression":"(http.request.uri.path.extension in {\"css\" \"js\"})","enabled":true,"action":"set_cache_settings","action_parameters":{"cache":true,"edge_ttl":{"mode":"respect_origin"}}}]}}"#,
    )
}

fn zone() -> Zone {
    Zone::parse("stackvet.dev").unwrap()
}

#[test]
fn a_whole_reading_asks_the_zone_the_token_each_setting_and_the_cache_rules_in_that_order() {
    let mut fake = whole();
    let token = user_token();
    let reading = read(&zone(), &token, &mut fake);
    assert!(reading.stopped.is_none(), "{:?}", reading.stopped);
    let paths: Vec<String> = fake
        .asked
        .iter()
        .map(|(u, _)| {
            u.trim_start_matches("https://api.cloudflare.com/client/v4")
                .to_owned()
        })
        .collect();
    let mut want = vec![
        "/zones?name=stackvet.dev".to_owned(),
        "/user/tokens/verify".to_owned(),
    ];
    want.extend(SETTINGS.iter().map(|id| format!("{Z}/settings/{id}")));
    want.push(format!(
        "{Z}/rulesets/phases/http_request_cache_settings/entrypoint"
    ));
    assert_eq!(paths, want);
    // Every request went to the one host, with the token given, and each is listed with its status.
    assert!(
        fake.asked
            .iter()
            .all(|(u, t)| u.starts_with("https://api.cloudflare.com/") && *t == token)
    );
    assert_eq!(reading.requests.len(), want.len());
    assert!(
        reading
            .requests
            .iter()
            .all(|r| r.method == "GET" && r.host == HOST && r.status == 200)
    );
    assert!(reading.requests[0].path.starts_with("/client/v4/zones"));
    assert_eq!(reading.token.standing, Standing::ActiveCouldNotConfirm);
    assert_eq!(reading.settings.text("ssl"), Some("strict"));
    assert_eq!(reading.settings.plan.as_deref(), Some("Free Website"));
    assert_eq!(reading.settings.cache_rules.as_ref().map(Vec::len), Some(1));
    assert!(reading.unread.is_empty(), "{:?}", reading.unread);
    // The report never holds the token, only its first characters and its length.
    let json = serde_json::to_string(&reading).unwrap();
    assert!(!json.contains(&token));
}

#[test]
fn a_global_api_key_and_wranglers_sign_in_are_refused_before_anything_is_sent() {
    for token in [
        ["cf", "k_", &"a".repeat(40)].concat(),
        ["cf", "oat_", &"b".repeat(40)].concat(),
    ] {
        let mut fake = whole();
        let reading = read(&zone(), &token, &mut fake);
        assert!(fake.asked.is_empty(), "{token:.6}: something was sent");
        assert!(matches!(reading.token.standing, Standing::Refused { .. }));
        assert!(
            reading
                .stopped
                .as_deref()
                .unwrap()
                .contains("nothing was sent")
        );
    }
}

#[test]
fn a_token_cloudflare_does_not_call_active_stops_the_reading() {
    for (status, body) in [
        (
            200,
            r#"{"success":true,"result":{"id":"t1","status":"disabled"}}"#,
        ),
        (
            401,
            r#"{"success":false,"errors":[{"code":1000,"message":"Invalid API Token"}]}"#,
        ),
    ] {
        let mut fake = whole().answer("/user/tokens/verify", status, body);
        let reading = read(&zone(), &user_token(), &mut fake);
        assert_eq!(
            fake.asked.len(),
            2,
            "the zone and the verify, nothing after"
        );
        assert!(matches!(reading.token.standing, Standing::NotActive { .. }));
        assert!(
            reading
                .stopped
                .as_deref()
                .unwrap()
                .contains("read nothing more")
        );
    }
}

#[test]
fn an_accounts_token_is_verified_under_its_account() {
    let mut fake = whole().ok(
        "/accounts/a1/tokens/verify",
        r#"{"success":true,"result":{"id":"t2","status":"active"}}"#,
    );
    let token = ["cf", "at_", &"R".repeat(40)].concat();
    let reading = read(&zone(), &token, &mut fake);
    assert_eq!(reading.token.kind, TokenKind::Account);
    assert!(fake.asked[1].0.ends_with("/accounts/a1/tokens/verify"));
    assert_eq!(reading.token.standing, Standing::ActiveCouldNotConfirm);
}

#[test]
fn a_zone_the_token_cannot_see_stops_the_reading_and_says_what_it_needs() {
    let mut fake = whole().ok(
        "/zones?name=stackvet.dev",
        r#"{"success":true,"result":[]}"#,
    );
    let reading = read(&zone(), &user_token(), &mut fake);
    assert_eq!(fake.asked.len(), 1);
    assert!(reading.stopped.as_deref().unwrap().contains("Zone Read"));
}

#[test]
fn a_setting_refused_is_unread_with_cloudflares_words_and_the_rest_go_on() {
    let mut fake = whole().answer(
        &format!("{Z}/settings/tls_1_3"),
        403,
        r#"{"success":false,"errors":[{"code":9109,"message":"Unauthorized to access requested resource"}]}"#,
    );
    let reading = read(&zone(), &user_token(), &mut fake);
    assert!(reading.stopped.is_none());
    assert!(!reading.settings.values.contains_key("tls_1_3"));
    assert!(
        reading
            .unread
            .iter()
            .any(|u| u.contains("tls_1_3") && u.contains("403") && u.contains("Unauthorized"))
    );
    assert!(
        reading.settings.values.contains_key("always_use_https"),
        "the settings after it were still read"
    );
}

#[test]
fn no_cache_entrypoint_is_a_zone_with_no_cache_rules() {
    let mut fake = whole().answer(
        &format!("{Z}/rulesets/phases/http_request_cache_settings/entrypoint"),
        404,
        r#"{"success":false,"errors":[{"code":10003,"message":"not found"}]}"#,
    );
    let reading = read(&zone(), &user_token(), &mut fake);
    assert_eq!(reading.settings.cache_rules.as_ref().map(Vec::len), Some(0));
}

#[test]
fn a_redirect_off_the_host_is_not_followed() {
    let mut fake = whole().redirect("/user/tokens/verify", "https://example.com/verify");
    let reading = read(&zone(), &user_token(), &mut fake);
    assert_eq!(fake.asked.len(), 2, "the redirect was not followed");
    assert!(reading.stopped.as_deref().unwrap().contains("not followed"));
}

#[test]
fn a_zones_name_is_a_plain_domain() {
    assert_eq!(Zone::parse("StackVet.dev.").unwrap().0, "stackvet.dev");
    for bad in [
        "stackvet",
        "stackvet.dev/zones",
        "a..b",
        "-a.dev",
        "a.dev&per_page=50",
        "a b.dev",
        "",
    ] {
        assert!(Zone::parse(bad).is_err(), "{bad:?} was taken");
    }
}
