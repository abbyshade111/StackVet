//! Backlog 0625, part 5, ADR-148: V10.3.2, an access token without the scope a route needs, sent to the route
//! stackvet.toml names with it. Against a small app and provider of their own: the route checks the token's scope, or
//! does not.

use super::*;
use crate::probes::ProbeResponse;

#[derive(Default)]
struct Fake {
    /// The route takes any of the provider's tokens, whatever their scope.
    ignores_scope: bool,
    /// The route takes none of the provider's tokens.
    takes_no_bearer: bool,
    /// After refusing one token, refuses every token.
    breaks_after_refusing: bool,
    refused: bool,
    /// Each token minted, by the scope it grants.
    minted: Vec<(String, String)>,
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
        if r.path != "/api/reports" || self.takes_no_bearer {
            return Some(answer(401, ""));
        }
        let token = r
            .headers
            .iter()
            .find(|(k, _)| k == "Authorization")
            .and_then(|(_, v)| v.strip_prefix("Bearer "))
            .unwrap_or("");
        let scope = self
            .minted
            .iter()
            .find(|(t, _)| t == token)
            .map(|(_, s)| s.clone())
            .unwrap_or_default();
        let has = scope.split(' ').any(|s| s == "reports:read");
        if (has || self.ignores_scope) && !(self.breaks_after_refusing && self.refused) {
            Some(answer(200, "[]"))
        } else {
            self.refused = true;
            Some(answer(403, ""))
        }
    }

    fn provider(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        if r.path != idp::ACCESS_TOKEN {
            return Some(answer(404, ""));
        }
        // The scope as the form gave it, `+` for a space.
        let form = r.body_text();
        let scope = form
            .split('&')
            .find_map(|p| p.strip_prefix("scope="))
            .unwrap_or("")
            .replace('+', " ")
            .replace("%3A", ":");
        let token = format!("token-{}", self.minted.len());
        self.minted.push((token.clone(), scope));
        Some(answer(200, &format!("{{\"token\":\"{token}\"}}")))
    }
}

fn section(scoped: bool) -> OidcSection {
    OidcSection {
        start: "/login/oidc".to_owned(),
        private: "/account".to_owned(),
        scoped: scoped.then(|| sv_manifest::ScopedRoute {
            path: "/api/reports".to_owned(),
            scope: "reports:read".to_owned(),
        }),
        ..Default::default()
    }
}

fn run(fake: Fake, scoped: bool) -> Outcome {
    let mut out = Outcome::default();
    access_token_scope(&mut { fake }, &section(scoped), &mut out);
    out
}

fn not_assessed(out: &Outcome) -> Vec<&str> {
    out.not_assessed
        .iter()
        .filter(|(id, _)| id == "V10.3.2")
        .map(|(_, why)| why.as_str())
        .collect()
}

#[test]
fn a_route_that_checks_the_scope_is_credited_in_part() {
    let out = run(Fake::default(), true);
    assert!(out.findings.is_empty(), "{:#?}", out.findings);
    let credit = out
        .verified
        .iter()
        .find(|v| v.check_id == SCOPE.rule_id)
        .expect("credited");
    assert!(credit.in_part);
    assert_eq!(credit.requirement_ids, ["V10.3.2"]);
    assert!(credit.scope.contains("reports:read"), "{}", credit.scope);
}

#[test]
fn a_route_that_takes_a_token_without_the_scope_is_a_finding() {
    let out = run(
        Fake {
            ignores_scope: true,
            ..Default::default()
        },
        true,
    );
    assert_eq!(
        out.findings
            .iter()
            .map(|f| f.rule_id.as_str())
            .collect::<Vec<_>>(),
        [SCOPE.rule_id]
    );
    assert!(out.verified.is_empty());
}

#[test]
fn without_a_named_route_or_the_control_nothing_is_concluded_and_the_report_says_why() {
    let out = run(Fake::default(), false);
    assert!(
        not_assessed(&out)[0].contains("Add `scoped"),
        "{:?}",
        not_assessed(&out)
    );
    let out = run(
        Fake {
            takes_no_bearer: true,
            ..Default::default()
        },
        true,
    );
    assert!(out.findings.is_empty() && out.verified.is_empty());
    assert!(not_assessed(&out)[0].contains("did not take an access token that has the scope"));
    let out = run(
        Fake {
            breaks_after_refusing: true,
            ..Default::default()
        },
        true,
    );
    assert!(out.verified.is_empty());
    assert!(not_assessed(&out)[0].contains("refused the one with it as well"));
}
