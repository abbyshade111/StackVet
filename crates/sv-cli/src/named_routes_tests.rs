//! A run asks each route stackvet.toml names, and the evidence is read knowing them, so the header credits can be
//! whole (ADR-095, backlog 0232 part 1). The plans are written here, not read from a fixture (`moved.rs`).

use super::*;
use sv_check::probes::ProbeResponse;

const BASE: &str = "manifest-version = 1\n[app]\nname = \"Fixture\"\naudience = \"customers\"\n\
                    deployment = \"internet\"\n[stack]\nlanguages = [\"python\"]\n[stack.run]\n\
                    image = \"python:3.12-slim\"\nstart = \"python app.py\"\nhealth = \"/\"\n";

const USERS: &str = "[stack.run.users]\nseed = \"python seed.py\"\n\
                     login = { path = \"/login\", form = { email = \"{user}\", password = \"{password}\" } }\n\
                     private = [\"/account\"]\n";

fn plan(users: bool) -> RunPlan {
    let text = format!("{BASE}{}", if users { USERS } else { "" });
    let manifest = sv_manifest::Manifest::parse(&text, Path::new("stackvet.toml"))
        .expect("the manifest written here");
    RunPlan::from_manifest(&manifest, Path::new(".")).expect("it declares how to run")
}

/// An answer with every header the group A checks look for.
fn careful(id: &str, status: u16) -> ProbeResponse {
    ProbeResponse {
        id: id.into(),
        status,
        headers: [
            (
                "content-security-policy",
                "default-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'; report-uri /csp",
            ),
            ("x-content-type-options", "nosniff"),
            ("referrer-policy", "same-origin"),
            ("cross-origin-opener-policy", "same-origin"),
            ("content-type", "text/html; charset=utf-8"),
        ]
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect(),
        body: "<html>a page</html>".into(),
    }
}

fn outcome(probe_responses: Vec<ProbeResponse>) -> sv_run::RunOutcome {
    sv_run::RunOutcome {
        healthy: true,
        tests: None,
        fence: sv_run::Fence::DockerInternalNetwork,
        probe_responses,
        probes_rate_limited: Vec::new(),
        signed_in: None,
        oidc: None,
        ai: None,
        mcp_server: None,
        fetch: None,
        left_over_removed: Vec::new(),
        liveness: Vec::new(),
        sidecar_lost: None,
        installed: Vec::new(),
        stand_ins: Default::default(),
        resources: None,
        container: Default::default(),
        suite_timings: Vec::new(),
        request_timings: Vec::new(),
    }
}

/// Whether the security-headers credit was given, and whether in part.
fn headers_credit(plan: &RunPlan, answers: Vec<ProbeResponse>) -> Option<bool> {
    let (_, verified, _) = running_app_evidence(&outcome(answers), plan);
    verified
        .iter()
        .find(|v| v.check_id == "probe.security-headers")
        .map(|v| v.in_part)
}

#[test]
fn a_run_asks_each_route_the_manifest_names_as_a_stranger() {
    let asked: Vec<String> = anonymous_requests(&plan(true))
        .into_iter()
        .filter(|r| r.id.starts_with(sv_check::probes::ROUTE_ID))
        .map(|r| format!("{} {}", r.method, r.path))
        .collect();
    assert_eq!(asked, ["GET /login", "GET /account"]);
    assert!(
        !anonymous_requests(&plan(false))
            .iter()
            .any(|r| r.id.starts_with(sv_check::probes::ROUTE_ID)),
        "a route asked though the manifest names none"
    );
}

#[test]
fn the_evidence_is_read_knowing_the_routes() {
    let answers = || {
        vec![
            careful("home", 200),
            careful("missing", 404),
            careful("route:/login", 200),
            careful("route:/account", 302),
        ]
    };
    assert_eq!(headers_credit(&plan(true), answers()), Some(false), "whole");
    // The same answers, with no routes named: one or two pages, so in part.
    assert_eq!(headers_credit(&plan(false), answers()), Some(true), "in part");
}
