//! Tests of `docker.rs`: the sidecar's time, reading the containers' clock, how the fence explains itself, and keeping
//! secrets off Docker's command line and out of a failed seed's report. Moved out of `docker.rs` unchanged (backlog
//! 0324).

use super::*;

#[test]
fn a_slow_run_gives_the_sidecar_time_for_an_emailed_codes_ten_minutes() {
    let mut m = sv_manifest::Manifest::default();
    m.stack.run.image = Some("busybox:1.36".to_owned());
    m.stack.run.start = Some("httpd -f".to_owned());
    let mut plan = RunPlan::from_manifest(&m, std::path::Path::new("/tmp/app")).unwrap();
    plan.slow = true;
    let without = sidecar_seconds(&plan);
    plan.users = Some(sv_manifest::UsersSection {
        email_code: Some(sv_manifest::ResetSection {
            request: Default::default(),
            use_code: Default::default(),
            code_pattern: None,
        }),
        ..Default::default()
    });
    assert!(sidecar_seconds(&plan) >= without + 10 * 60);
    plan.slow = false;
    assert_eq!(sidecar_seconds(&plan), SIDECAR_SECONDS);
    // The AI feature's rate check waits a minute, slow or not.
    plan.ai = Some(sv_manifest::AiSection::default());
    plan.policy.ai_requests_per_minute = Some(10);
    assert!(sidecar_seconds(&plan) >= SIDECAR_SECONDS + 60);
}

#[test]
fn availability_is_judged_by_the_daemon_not_the_binary() {
    // `docker --version` prints happily with no daemon behind it. A backend that cannot run
    // anything must not report itself as available, or every app on such a machine is reported
    // as failing rather than as unrun.
    let backend = DockerBackend {
        binary: "definitely-not-a-real-binary-xyz".to_owned(),
        ..DockerBackend::new()
    };
    let err = backend.available().unwrap_err();
    match err {
        CannotRun::NoBackend { checked, .. } => {
            assert!(checked.contains("could not be started"), "{checked}")
        }
        other => panic!("expected NoBackend, got {other:?}"),
    }
}

#[test]
fn the_containers_clock_is_read_against_the_middle_of_the_reading() {
    // The deep review's improvement 5: a Docker machine 47 seconds behind this computer.
    assert_eq!(offset_from(1_000, 953, 1_002), -48);
    assert_eq!(offset_from(1_000, 1_090, 1_000), 90);
    // A second either way is the reading's own uncertainty, not a clock that differs.
    for theirs in [999, 1_000, 1_001] {
        assert_eq!(offset_from(1_000, theirs, 1_001), 0, "{theirs}");
    }
}

#[test]
fn the_fence_explains_itself_without_overstating() {
    let text = Fence::DockerInternalNetwork.explain();
    assert!(text.contains("could not reach the internet"));
    assert!(Fence::None.explain().contains("No network fence"));
}

#[test]
fn no_password_or_secret_stands_on_docker_s_command_line() {
    // The deep review's improvement 5: `-e SV_PASSWORD_A=...` was readable by any user of the
    // computer while `docker exec` ran. Only the names are on the command line now.
    let accounts = crate::new_accounts(true, true);
    let env = seed_env(&accounts);
    let names: Vec<&str> = env.iter().map(|(k, _)| *k).collect();
    let args = seed_args("app", "./seed.sh", &names);
    for (name, value) in &env {
        assert!(args.iter().any(|a| a == name), "{name} is not named");
        // Compared, never printed: the message names the variable only.
        assert!(
            !args.iter().any(|a| a.contains(value.as_str())),
            "{name}'s value is on the command line"
        );
    }
    let secret = crate::random_hex(16);
    let (provider, secrets) = provider_start("net", "idp", &secret);
    assert!(
        !provider.iter().any(|a| a.contains(&secret)),
        "the provider's secret is on the command line"
    );
    let at = provider
        .iter()
        .position(|a| a == "CLIENT_SECRET")
        .expect("named");
    assert_eq!(provider[at - 1], "-e");
    assert_eq!(secrets, vec![("CLIENT_SECRET", secret)]);
}

#[test]
fn seed_is_given_the_admins_secret_only_when_there_is_a_two_factor_step() {
    let accounts = crate::new_accounts(true, true);
    let env = seed_env(&accounts);
    let given = |name: &str| env.iter().find(|(k, _)| *k == name).map(|(_, v)| v.clone());
    // Compared, never printed: the assertion messages name the variable only.
    assert!(
        given("SV_ADMIN_TOTP_SECRET")
            == Some(sv_check::totp::base32(
                accounts.admin_totp_secret.as_ref().unwrap()
            )),
        "SV_ADMIN_TOTP_SECRET is the admin's secret, in base32"
    );
    assert!(
        given("SV_ADMIN_TOTP_SECRET") != given("SV_TOTP_SECRET"),
        "the admin's secret is its own"
    );
    let without = seed_env(&crate::new_accounts(true, false));
    assert!(without.iter().all(|(k, _)| *k != "SV_ADMIN_TOTP_SECRET"));
}

#[test]
fn a_failed_seed_never_carries_a_secret_into_the_report() {
    let accounts = crate::new_accounts(true, true);
    let admin_secret = sv_check::totp::base32(accounts.admin_totp_secret.as_ref().unwrap());
    let user_secret = sv_check::totp::base32(&accounts.totp.as_ref().unwrap().secret);
    let admin_password = accounts.admin.as_ref().unwrap().password.clone();
    // A seed that prints its environment as it fails, in both cases a secret may be printed in.
    let echoed = format!(
        "SV_ADMIN_TOTP_SECRET={admin_secret} SV_TOTP_SECRET={} SV_ADMIN_PASSWORD={admin_password}",
        user_secret.to_lowercase()
    );
    // The setup: the line really carries them, so their absence below is the redaction's doing.
    assert!(first_line(&echoed).contains(&admin_secret));
    let said = seed_failed(1, &echoed, &accounts);
    for secret in [
        &admin_secret,
        &user_secret,
        &user_secret.to_lowercase(),
        &admin_password,
    ] {
        assert!(
            !said.contains(secret.as_str()),
            "a secret reached the report"
        );
    }
    assert!(said.contains("SV_ADMIN_TOTP_SECRET=[a test secret, left out]"));
    assert!(said.contains("failed (exit 1)"));
}
