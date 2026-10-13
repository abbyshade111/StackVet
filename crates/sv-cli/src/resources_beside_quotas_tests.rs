//! The container's own readings go beside C9.1.1's unchecked quotas, and only there, and only when they were taken
//! (ADR-094). The note is matched by its name, `sv_check::ai::UNSEEN_QUOTAS`, so a change of its words cannot quietly
//! stop the readings reaching it.

use super::*;

const OTHER_C9: &str = "Whether the app puts a time limit on its tools: something else.";

/// A plan from a manifest written here, not read from a fixture: a test in `src/` that named the build folder would
/// read from it at run time (`crates/sv-cli/tests/moved.rs`).
fn plan() -> RunPlan {
    let text = "manifest-version = 1\n[app]\nname = \"Fixture\"\naudience = \"just-me\"\n\
                deployment = \"local-only\"\n[stack]\nlanguages = []\n[stack.run]\n\
                image = \"busybox:1.36\"\nstart = \"httpd -f -p $PORT\"\nhealth = \"/\"\n";
    let manifest = sv_manifest::Manifest::parse(text, Path::new("stackvet.toml"))
        .expect("the manifest written here");
    RunPlan::from_manifest(&manifest, Path::new(".")).expect("it declares how to run")
}

fn outcome(resources: Option<sv_run::resources::Taken>) -> sv_run::RunOutcome {
    sv_run::RunOutcome {
        healthy: true,
        tests: None,
        fence: sv_run::Fence::DockerInternalNetwork,
        probe_responses: Vec::new(),
        probes_rate_limited: Vec::new(),
        signed_in: None,
        oidc: None,
        ai: Some(sv_check::signed_in::Outcome {
            not_assessed: vec![
                ("C9.1.1".to_owned(), sv_check::ai::UNSEEN_QUOTAS.to_owned()),
                ("C9.1.1".to_owned(), OTHER_C9.to_owned()),
            ],
            ..Default::default()
        }),
        mcp_server: None,
        fetch: None,
        left_over_removed: Vec::new(),
        liveness: Vec::new(),
        sidecar_lost: None,
        installed: Vec::new(),
        stand_ins: Default::default(),
        resources,
        container: Default::default(),
        suite_timings: Vec::new(),
        request_timings: Vec::new(),
    }
}

fn taken() -> sv_run::resources::Taken {
    sv_run::resources::Taken {
        reading: sv_run::resources::parse_stats("0.00% | 404KiB / 64MiB | 736B / 126B | 0B / 0B")
            .unwrap(),
        limits: sv_run::resources::Limits {
            nano_cpus: 0,
            mem_bytes: 0,
        },
    }
}

fn notes(resources: Option<sv_run::resources::Taken>) -> Vec<(String, String)> {
    let (_, _, not_assessed) = running_app_evidence(&outcome(resources), &plan(), &[]);
    let c9: Vec<_> = not_assessed
        .into_iter()
        .filter(|(r, _)| r == "C9.1.1")
        .collect();
    assert_eq!(
        c9.len(),
        2,
        "both C9.1.1 notes should reach the evidence: {c9:?}"
    );
    c9
}

#[test]
fn readings_taken_are_put_beside_the_unchecked_quotas_only() {
    let c9 = notes(Some(taken()));
    let quotas = &c9
        .iter()
        .find(|(_, w)| w.starts_with(sv_check::ai::UNSEEN_QUOTAS))
        .expect("the quota note")
        .1;
    assert!(
        quotas.ends_with(sv_run::resources::BESIDE_THE_QUOTAS),
        "{quotas}"
    );
    assert!(
        c9.iter().any(|(_, w)| w == OTHER_C9),
        "a C9.1.1 note about something else was changed: {c9:?}"
    );
}

#[test]
fn no_readings_leave_the_quota_note_as_it_was() {
    let c9 = notes(None);
    assert!(
        c9.iter().any(|(_, w)| w == sv_check::ai::UNSEEN_QUOTAS),
        "the note pointed at readings that were never taken: {c9:?}"
    );
}
