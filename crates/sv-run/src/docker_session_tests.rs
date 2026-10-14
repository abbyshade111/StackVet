//! Single requests through one `docker exec` kept open in the sidecar (backlog 0341): answered as the exec per
//! request answered them, through one exec however many there are, and a sidecar that goes is still told from a
//! silent app. Where there is no container backend it says so and stops; where there is one, a setup that fails is a
//! failure.

use super::*;

fn get(i: usize) -> sv_check::probes::ProbeRequest {
    sv_check::probes::ProbeRequest {
        id: format!("q{i}"),
        method: if i.is_multiple_of(2) { "GET" } else { "POST" }.to_owned(),
        path: format!("/q{i}"),
        headers: Vec::new(),
        body: (i % 2 == 1).then(|| format!("body {i} ").repeat(i * 1000).into_bytes()),
    }
}

#[test]
fn requests_through_one_session_reach_a_real_server_with_one_exec() {
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; this needs one");
        return;
    }
    let network = format!("sv-session-{}-{}", std::process::id(), crate::random_hex(3));
    let server = format!("{network}-app");
    let sidecar = format!("{network}-probe");
    let _ = backend.docker(&["network", "create", "--internal", &network]);
    let started = backend.docker(&[
        "run",
        "-d",
        "--rm",
        "--name",
        &server,
        "--network",
        &network,
        PROVIDER_IMAGE,
        "node",
        "-e",
        "require('http').createServer((q, s) => { let n = 0; q.on('data', d => n += d.length); q.on('end', () => s.end(`${q.method} ${q.url} ${n}`)) }).listen(8080)",
    ]);
    let probe_started = backend.docker(&[
        "run",
        "-d",
        "--rm",
        "--name",
        &sidecar,
        "--network",
        &network,
        "--tmpfs",
        PROBE_TMPFS,
        PROBE_IMAGE,
        "sleep",
        "300",
    ]);
    let outcome =
        (matches!(started, Ok((0, _))) && matches!(probe_started, Ok((0, _)))).then(|| {
            let via = Via::Sidecar(&sidecar);
            // Until the server answers, at most 20 seconds, asked the old way so the session is not started yet.
            let up = (0..20).any(|_| {
                let answered = backend
                    .probe(&Via::FreshContainer(&network), &server, 8080, &get(0))
                    .is_some();
                if !answered {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                answered
            });
            assert!(up, "the test server never answered");
            let answers: Vec<_> = (1..=12)
                .map(|i| backend.probe(&via, &server, 8080, &get(i)))
                .collect();
            let sessions = backend
                .sessions_started
                .load(std::sync::atomic::Ordering::Relaxed);
            let in_session = backend
                .answered_in_session
                .load(std::sync::atomic::Ordering::Relaxed);
            // The sidecar goes, as it does when a run ends early: the next requests have no answer, and why is said.
            let _ = backend.docker(&["rm", "-f", &sidecar]);
            let after: Vec<_> = (13..=14)
                .map(|i| backend.probe(&via, &server, 8080, &get(i)))
                .collect();
            (answers, sessions, in_session, after, backend.sidecar_lost())
        });
    let _ = backend.docker(&["rm", "-f", &server, &sidecar]);
    let _ = backend.docker(&["network", "rm", &network]);
    let (answers, sessions, in_session, after, lost) = outcome.unwrap_or_else(|| {
        panic!("the test containers did not start: {started:?} {probe_started:?}")
    });
    assert_eq!((sessions, in_session), (1, 12), "one exec for all twelve");
    for (i, answer) in (1..=12).zip(&answers) {
        let answer = answer
            .as_ref()
            .unwrap_or_else(|| panic!("no answer to q{i}: {answers:?}"));
        let request = get(i);
        let sent = request.body.as_ref().map_or(0, Vec::len);
        assert_eq!(answer.id, format!("q{i}"));
        assert_eq!(answer.status, 200, "{answer:?}");
        assert_eq!(
            answer.body,
            format!("{} /q{i} {sent}", request.method),
            "each its own answer, whole, in turn"
        );
    }
    assert!(after.iter().all(Option::is_none), "{after:?}");
    assert!(lost.is_some(), "a sidecar that went is said to have gone");
}
