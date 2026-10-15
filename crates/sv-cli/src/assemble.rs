//! `sv report`'s one pass over an app, in the stages it names (`REPORT_STAGES`): the app's files
//! read once (`static_scan`), then the report put together from them and from what else was asked
//! for. Each stage below is one function; what they hand each other is in `Scene` (what every stage
//! reads) and in what each returns. The order of the findings, the credits, the gaps, and the
//! `examined` list is the order the stages run in, which the verdict snapshots
//! (`crates/sv-cli/tests/verdicts.rs`) hold.
//!
//! Until 8 October 2026 this was one function of 1,590 lines in `main.rs` (the review of 8 October
//! 2026, item 7).

use super::*;

/// The stages of a report that `assemble_report_saying` names as it starts each, in order. The MCP
/// server passes them on, so a long check does not look stuck.
///
/// Every stage is announced on every run, in this order, so a count of them means the same each
/// time; the three that do not always run say when in their names (the outside tools run unless
/// --no-tools, ADR-111). They were one stage, "Putting
/// the report together", until `sv report --run --tools` sat in it for minutes without a word
/// (backlog 226, part 2, item 15).
pub const REPORT_STAGES: [&str; 10] = [
    "Reading the app's files",
    "Recognizing its languages and frameworks",
    "Listing the packages it uses",
    "Looking for keys and passwords",
    "Reading its configuration",
    "Reading its code",
    "Comparing its packages with known vulnerabilities, when there is a database",
    "Running the outside tools, unless --no-tools",
    "Running the app, when asked with --run",
    "Putting the report together",
];

/// One line of progress for a person watching `sv report`: which stage of how many, and what it is.
pub fn stage_line(n: usize, name: &str) -> String {
    format!("sv report: {} of {}, {name}", n + 1, REPORT_STAGES.len())
}

/// One line of progress within a stage: an outside tool, or a suite of questions to the running
/// app, as it begins (backlog 226, part 2, item 15). Before, the two longest stages printed one line
/// each and were then silent for minutes.
pub fn step_line(what: &str) -> String {
    format!("  now: {what}")
}

/// `step_line`, on stderr, where the stage lines go and the report on stdout is left alone. Silent with `--quiet`.
pub(crate) fn say_step(what: &str) {
    if !quiet() {
        eprintln!("{}", step_line(what));
    }
}

#[cfg(test)]
#[path = "assemble_progress_tests.rs"]
mod progress_tests;

/// Set by `sv report --quiet` (backlog 0247): the progress and timing lines are left out. Counts, warnings and errors
/// are never silenced by it.
pub static QUIET: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Whether `--quiet` was given.
pub fn quiet() -> bool {
    QUIET.load(std::sync::atomic::Ordering::Relaxed)
}

/// How long a stage takes before its time is said when it ends (backlog 0247).
pub const SLOW_STAGE_SECONDS: f64 = 5.0;

/// The line said when a stage ends that took `SLOW_STAGE_SECONDS` or more; `None` for a quick one.
pub fn took_line(n: usize, seconds: f64) -> Option<String> {
    (seconds >= SLOW_STAGE_SECONDS).then(|| {
        format!(
            "sv report: {} of {} took {:.0} s",
            n + 1,
            REPORT_STAGES.len(),
            seconds
        )
    })
}

/// Says how long the stage that just ended took, when it was slow, and not with `--quiet`.
pub fn stage_finished(ended: Option<(usize, std::time::Instant)>) {
    if let Some(line) = ended
        .and_then(|(n, since)| took_line(n, since.elapsed().as_secs_f64()))
        .filter(|_| !quiet())
    {
        eprintln!("{line}");
    }
}

/// Runs one check of the report, and if it panics, catches it: the panic is recorded where it happened, so the
/// check costs itself and not the report (backlog 0234). Returns the place and the message of the panic. Under a
/// debug build, `SV_PANIC_IN_STAGE` set to the check's name makes it panic inside the guard, so the path can be
/// tested (as `SV_PANIC_FOR_TEST` does for the whole run).
pub(crate) fn guarded<T>(name: &str, run: impl FnOnce() -> T) -> std::result::Result<T, String> {
    #[cfg(debug_assertions)]
    let inject = std::env::var("SV_PANIC_IN_STAGE").ok().as_deref() == Some(name);
    // A release build has no injected panic, so it never reads the name (backlog 0393).
    #[cfg(not(debug_assertions))]
    let _ = name;
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        #[cfg(debug_assertions)]
        if inject {
            panic!("a panic asked for by SV_PANIC_IN_STAGE");
        }
        run()
    }))
    .map_err(|_| {
        let (what, place) = crash::take()
            .unwrap_or_else(|| ("no message".to_owned(), "a place not recorded".to_owned()));
        format!("{what}, at {place}")
    })
}

/// The gap for a check that crashed: what it was, and where the panic was (backlog 0234).
pub(crate) fn crash_gap(check: &str, place: &str) -> sv_report::Gap {
    sv_report::Gap {
        what: format!("{check}: the check crashed"),
        why: format!(
            "the check crashed while it ran ({place}), so what it would have said is not in this report; the \
             rest of the report is written"
        ),
        reason: sv_report::GapReason::Crashed,
        requirements: Vec::new(),
    }
}

/// `assemble_report`, calling `starting` with each stage's number (from 0) and name as it begins.
/// What the design answers given as `planned` come to, when they are not a finding: each credits
/// nothing, and the report says which are plans, which are due an answer, and which `sv` cannot
/// follow (`sv_check::design`, "`planned`").
fn planned_gaps(planned: &[sv_check::design::Planned]) -> Vec<sv_report::Gap> {
    use sv_check::design::PlannedState;
    let of = |state: PlannedState| -> Vec<String> {
        planned
            .iter()
            .filter(|p| p.state == state)
            .map(|p| match &p.location {
                Some(path) => format!("{} (in `{path}`)", p.id),
                None => p.id.clone(),
            })
            .collect()
    };
    let decisions = |n: usize| format!("{n} design decision{}", if n == 1 { "" } else { "s" });
    let mut gaps = Vec::new();
    let not_yet = of(PlannedState::NoCodeYet);
    if !not_yet.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!("{} planned, not built yet", decisions(not_yet.len())),
            why: format!(
                "stackvet.toml answers these as planned, and the app has no code yet, so there is \
                 nothing to check: {}. A plan counts for nothing until it is built; once the code \
                 exists, a planned file that is not there is reported as decided, never built.",
                not_yet.join(", ")
            ),
            reason: sv_report::GapReason::Planned,
            requirements: Vec::new(),
        });
    }
    let due = of(PlannedState::FileIsThere);
    if !due.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!("{} planned, and the file is there now", decisions(due.len())),
            why: format!(
                "stackvet.toml still answers these as planned, and the file each names is in the \
                 app now: {}. Look at it, then change the answer to yes if it does what was decided, \
                 or to no if it does not. Until then it counts for nothing.",
                due.join(", ")
            ),
            reason: sv_report::GapReason::Planned,
            requirements: Vec::new(),
        });
    }
    let blind = of(PlannedState::NothingToLookFor);
    if !blind.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!("{} planned, with no file named", decisions(blind.len())),
            why: format!(
                "stackvet.toml answers these as planned without a `where`, and the app has code \
                 now, so `sv` cannot tell whether they were built: {}. Change each answer to yes, \
                 naming the file that does it, or to no.",
                blind.join(", ")
            ),
            reason: sv_report::GapReason::Planned,
            requirements: Vec::new(),
        });
    }
    gaps
}

/// What every stage reads: the app, the options, `sv`'s data, the manifest, the one reading of the
/// app's files, what the manifest's claims came to, and what this computer can check seals with.
#[derive(Clone, Copy)]
struct Scene<'a> {
    app_dir: &'a Path,
    options: &'a ReportOptions,
    loaded: &'a Loaded,
    manifest: &'a Manifest,
    /// The manifest's file name as read: the new name, or the old one (ADR-062).
    manifest_file: &'a str,
    static_scan: &'a static_scan::StaticScan,
    ctx: &'a sv_frameworks::applicability::ConditionContext,
    resolved: &'a [sv_manifest::ResolvedClaim],
    buckets: &'a sv_frameworks::applicability::Buckets,
    seals: &'a sv_check::seal::Checker,
}

/// What the advisory comparison came to.
pub struct Advisories {
    pub findings: Vec<sv_check::Finding>,
    pub verified: Vec<sv_check::Verified>,
    pub gaps: Vec<sv_report::Gap>,
}

/// What the outside tools came to.
pub struct Tools {
    pub verified: Vec<sv_check::Verified>,
    pub gaps: Vec<sv_report::Gap>,
}

/// What running the app came to, or why it was not run.
pub struct RunningApp {
    pub status: sv_report::RunStatus,
    pub note: Option<String>,
    pub steps: Vec<String>,
    pub test_output: Option<sv_check::suite::FailingOutput>,
    pub tests_examined: sv_report::Examined,
    pub probe_verified: Vec<sv_check::Verified>,
    pub test_verified: Vec<sv_check::Verified>,
    /// What the app answered, kept beside the report as `seen.json` (ADR-082). `None` when it
    /// was not asked anything.
    pub seen: Option<sv_report::seen::Seen>,
    /// The field names in the first account's sign-in token, when it is a JWT (backlog 0497). Names only.
    pub token_fields: Option<sv_check::personal_fields::TokenFields>,
    /// How long each suite of questions took, and the app's own tests (backlog 226, part 2, item
    /// 13). Empty when the app was not run.
    pub timings: Vec<(&'static str, u64)>,
    /// How long each request to the running app took (backlog 226, part 2, item 13). Empty when the
    /// app was not run.
    pub request_timings: Vec<(String, u64)>,
    /// How long each check inside the suites took, and their waiting for the app's limit on requests (backlog 0526).
    /// Empty when the app was not run.
    pub check_timings: Vec<(String, u64)>,
}

/// The owner's word, from the notes, the decisions file, and the manifest's design and hand-check
/// answers: what it credits, and the catalogs the report prints the questions from.
pub struct PersonsWord {
    pub verified: Vec<sv_check::Verified>,
    pub notes_catalog: sv_check::notes::Catalog,
    pub design_questions: sv_check::design::Questions,
    pub human_checks: sv_check::human::HumanChecks,
    pub decisions_text: Option<String>,
    /// The SHA-256 of the security notes as read, `None` when there are none (ADR-083).
    pub notes_sha256: Option<String>,
}

/// `assemble_report`, calling `starting` with each stage's number (from 0) and name as it begins.
pub fn assemble_report_saying(
    app_dir: &Path,
    options: &ReportOptions,
    loaded: &Loaded,
    starting: &dyn Fn(usize, &'static str),
) -> Result<sv_report::Report> {
    // When each stage began, so the report can say how long each took (backlog 226, part 2, item
    // 13): a stage lasts until the next begins, the last until the report is put together.
    let began: std::cell::RefCell<Vec<(&'static str, std::time::Instant)>> = Default::default();
    let stage = |n: usize| {
        began
            .borrow_mut()
            .push((REPORT_STAGES[n], std::time::Instant::now()));
        // The names of the stages only, for the opt-in SV_LOG file (backlog 0237).
        crate::own_log::line(&format!("stage {n}: {}", REPORT_STAGES[n]));
        starting(n, REPORT_STAGES[n])
    };
    let started = std::time::SystemTime::now();
    // The new name, or the old one while only it exists (ADR-062); the report says which.
    let located = sv_manifest::locate_or_bail(app_dir)?;
    let manifest_path = located.path.clone();
    let manifest_file = manifest_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| sv_frameworks::names::MANIFEST.to_owned());
    // Read once, so the hash recorded is of the very bytes parsed.
    let manifest_text = std::fs::read_to_string(&manifest_path)
        .with_context(|| format!("reading {}", manifest_path.display()))?;
    let manifest = Manifest::parse(&manifest_text, &manifest_path)?;
    let run_record = report_lock::run_record(started, manifest_text.as_bytes());
    // The same reading of the app `sv check` makes (`static_scan`): one walk of the folder, shared
    // by every check in this report (DESIGN, "One walk of the app"), the languages and packages,
    // the keys and passwords, the configuration, and the code. The bill of materials knows what
    // actually came out of each ecosystem, which is the difference between "this list is
    // approximate" and "this list is empty"; built once and handed to the lockfile check and the
    // findings below.
    let static_scan =
        static_scan::StaticScan::read(app_dir, &manifest.set_apart(), loaded, &stage)?;
    let (ctx, resolved) =
        sv_manifest::resolve(&manifest, &static_scan.scan_report.as_corroborator());
    let buckets = bucket(
        &loaded.frameworks,
        &loaded.config_rules,
        &ctx,
        manifest.target_level(),
    );
    // What this computer can check `sv review`'s seals with (`sv_check::seal`): the owner's own
    // answers count as theirs only when `sv review` recorded them for this app.
    let seals = sv_check::seal::Checker::for_app(app_dir);
    let scene = Scene {
        app_dir,
        options,
        loaded,
        manifest: &manifest,
        manifest_file: &manifest_file,
        static_scan: &static_scan,
        ctx: &ctx,
        resolved: &resolved,
        buckets: &buckets,
        seals: &seals,
    };
    stage(6);
    // What was examined, per family of findings, for a program reading report.json: the same
    // limits as the gaps, decided in the same places (DESIGN, "What was examined, for a program").
    // A bill of materials whose reader crashed is not read: `examined` says so (backlog 0234), and its gap is below.
    let mut examined: Vec<sv_report::Examined> =
        vec![if static_scan.crashed("bill_of_materials") {
            sv_report::Examined::not_run(
                "sbom.",
                static_scan
                    .crashed_why("bill_of_materials")
                    .unwrap_or_default(),
            )
        } else {
            sv_report::Examined::ran("sbom.")
        }];
    // The known-vulnerability check crashing costs only itself (backlog 0234): what it had written to `examined` is
    // taken back, and in its place the entry says it did not run, and the gap says where it crashed.
    let before = examined.len();
    let advisories = match guarded("advisories", || advisories(&scene, &mut examined)) {
        Ok(advisories) => advisories?,
        Err(place) => {
            examined.truncate(before);
            examined.push(sv_report::Examined::not_run(
                "advisory.",
                "the known-vulnerability check crashed, so nothing was compared",
            ));
            Advisories {
                findings: Vec::new(),
                verified: Vec::new(),
                gaps: vec![crash_gap("the known-vulnerability check", &place)],
            }
        }
    };

    let mut findings = Vec::new();
    findings.extend(advisories.findings);
    findings.extend(static_scan.secrets.findings.iter().cloned());
    findings.extend(static_scan.config.findings.iter().cloned());
    // The bill of materials speaks for itself here as it does in `sv check`: incomplete is a finding
    // against V15.1.2, complete is evidence for it, and exactly one of the two says anything. The
    // report used to read only its gaps, so a lockfile it could take nothing from was left to the
    // lockfile check, which saw a lockfile and passed it.
    findings.extend(sbom::incompleteness_finding(&static_scan.bill_of_materials));
    findings.extend(static_scan.code.findings.iter().cloned());
    // A way into the app the code shows that the owner's list of sign-in pathways does not name (backlog 0491; ADR-123).
    // With no list, nothing is compared.
    if let Some(listed) = manifest
        .stack
        .auth
        .as_ref()
        .and_then(|a| a.pathways.as_ref())
    {
        findings.extend(sv_check::sign_in_paths::not_listed_findings(
            &static_scan.listing,
            listed,
        ));
    }

    stage(7);
    // A check that crashes costs itself, not the report (backlog 0234): its gap says where, and the rest is written.
    let tools = match guarded("outside_tools", || {
        outside_tools(&scene, &mut findings, &mut examined)
    }) {
        Ok(tools) => tools?,
        Err(place) => Tools {
            verified: Vec::new(),
            gaps: vec![crash_gap("the outside tools", &place)],
        },
    };

    // Every limit `sv` knows about, said out loud. This list existing is the difference between a
    // report about an app and a report about the part of an app somebody happened to look at.
    let mut gaps = Vec::new();
    // Said first: a file read under its old name is the one thing here about `sv` and not the app.
    if let Some(note) = located.note() {
        gaps.push(sv_report::Gap {
            what: format!("the manifest, named {manifest_file}"),
            why: note,
            reason: sv_report::GapReason::Outdated,
            requirements: Vec::new(),
        });
    }
    stage(8);
    // The same for the app's own checks (backlog 0234): if they crash, the app is said not to have been asked, and
    // the gap says why.
    let run = match guarded("running_app", || {
        running_app(&scene, &mut findings, &mut gaps)
    }) {
        Ok(run) => run,
        Err(place) => {
            gaps.push(crash_gap("the app's own checks", &place));
            RunningApp {
                status: sv_report::RunStatus::NotAsked {
                    why: "its checks crashed before they finished; the gap says where".to_owned(),
                },
                note: None,
                steps: Vec::new(),
                test_output: None,
                tests_examined: sv_report::Examined::not_run(
                    "the app's own tests",
                    "its checks crashed before they finished",
                ),
                probe_verified: Vec::new(),
                test_verified: Vec::new(),
                seen: None,
                token_fields: None,
                timings: Vec::new(),
                request_timings: Vec::new(),
                check_timings: Vec::new(),
            }
        }
    };
    let suite_timings = run.timings.clone();
    let request_timings = run.request_timings.clone();
    let check_timings = run.check_timings.clone();
    stage(9);
    gaps.extend(tools.gaps);
    let manual_only = what_was_not_read(&scene, &mut gaps, &mut examined);
    gaps.extend(advisories.gaps);

    // Everything that ran, looked at what it needed to, and found nothing wrong. Each of these
    // fails closed on its own coverage, so the list is short on an app `sv` could not read fully —
    // which is the honest shape for it to have.
    let mut verified = static_scan.config.passed.clone();
    verified.extend(sbom::completeness_verified(&static_scan.bill_of_materials));
    verified.extend(static_scan.secrets.verified.iter().cloned());
    verified.extend(static_scan.code.verified.iter().cloned());
    verified.extend(run.probe_verified.iter().cloned());
    verified.extend(tools.verified);
    verified.extend(advisories.verified);
    verified.extend(run.test_verified.iter().cloned());

    let (named_in_tests, not_for_tests) = requirements_for_tests(&scene);
    let word = the_owners_word(&scene, &mut findings, &mut gaps)?;
    // A person's word goes to the report in the one list with the checks, each credit saying
    // which tier it rests on (`sv_check::Tier`, set where it was made); the report reads the tier,
    // not the list (8 October 2026).
    verified.extend(word.verified.iter().cloned());
    account_changes_whole(scene.manifest, scene.seals, &mut verified, &mut gaps);
    let coding_rules_cited = coding_rules_cited(&buckets)?;
    let mut report = put_together(
        &scene,
        Gathered {
            findings,
            verified,
            gaps,
            examined,
            manual_only,
            named_in_tests,
            not_for_tests,
            coding_rules_cited,
            run,
            word,
            run_record,
        },
    )?;
    report.timings = timings(
        &began.borrow(),
        std::time::Instant::now(),
        &report.examined,
        &suite_timings,
        &request_timings,
        &check_timings,
    );
    Ok(report)
}

/// How long each stage took, from when each began to when the next did (the last to `ended`), then
/// each outside tool, from the record of its run, then each suite of questions to the running app.
fn timings(
    began: &[(&'static str, std::time::Instant)],
    ended: std::time::Instant,
    examined: &[sv_report::Examined],
    suites: &[(&'static str, u64)],
    requests: &[(String, u64)],
    checks: &[(String, u64)],
) -> Vec<sv_report::Timing> {
    let ms = |d: std::time::Duration| u64::try_from(d.as_millis()).unwrap_or(u64::MAX);
    let mut timings: Vec<sv_report::Timing> = began
        .iter()
        .enumerate()
        .map(|(i, (what, at))| {
            let until = began.get(i + 1).map_or(ended, |(_, next)| *next);
            sv_report::Timing {
                what: (*what).to_owned(),
                took_ms: ms(until.saturating_duration_since(*at)),
            }
        })
        .collect();
    timings.extend(examined.iter().filter_map(|e| {
        let tool = e.tool.as_ref()?;
        Some(sv_report::Timing {
            what: format!("{}{}", sv_report::TOOL_TIMING, tool.program),
            took_ms: tool.took_ms,
        })
    }));
    timings.extend(suites.iter().map(|(suite, took_ms)| sv_report::Timing {
        what: format!("{}{suite}", sv_report::SUITE_TIMING),
        took_ms: *took_ms,
    }));
    timings.extend(requests.iter().map(|(request, took_ms)| sv_report::Timing {
        what: format!("{}{request}", sv_report::REQUEST_TIMING),
        took_ms: *took_ms,
    }));
    timings.extend(checks.iter().map(|(check, took_ms)| sv_report::Timing {
        what: format!("{}{check}", sv_report::CHECK_TIMING),
        took_ms: *took_ms,
    }));
    timings
}

/// Known vulnerabilities, when the owner has pointed at a local advisory database, held to the
/// time frames in stackvet.toml exactly as `sv audit` holds them. Without a database this says
/// so: a report silent about known vulnerabilities reads as a report that found none.
fn advisories(scene: &Scene, examined: &mut Vec<sv_report::Examined>) -> Result<Advisories> {
    let Scene {
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        static_scan,
        ctx,
        resolved,
        buckets,
        seals,
    } = *scene;
    let static_scan::StaticScan {
        crashed: _,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
    } = static_scan;
    let Loaded {
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
        ..
    } = loaded;
    let _ = (
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        ctx,
        resolved,
        buckets,
        seals,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
    );

    let mut findings_from_advisories = Vec::new();

    // Known vulnerabilities, when the owner has pointed at a local advisory database, held to the
    // time frames in stackvet.toml exactly as `sv audit` holds them. Without a database this says
    // so: a report silent about known vulnerabilities reads as a report that found none.
    let mut advisory_verified = Vec::new();
    let mut advisory_gaps = Vec::new();
    match &options.advisories {
        None => {
            examined.push(sv_report::Examined::not_run(
                "advisory.",
                "no advisory database was given (--advisories)",
            ));
            advisory_gaps.push(sv_report::Gap {
                what: "known vulnerabilities in the packages this app ships".to_owned(),
                why: format!(
                    "{} `sv` does not fetch anything, because the list of packages an app depends \
                     on is yours: download an OSV export for this app's ecosystems, unpack it, and \
                     pass its folder with --advisories.{}",
                    options.why_no_advisories,
                    {
                        let mut names: Vec<&str> = bill_of_materials
                            .components
                            .iter()
                            .map(|c| c.ecosystem.as_str())
                            .collect();
                        names.sort_unstable();
                        names.dedup();
                        names
                            .iter()
                            .filter_map(|n| {
                                advisories::osv_download(n).map(|url| format!(" {n}: {url}."))
                            })
                            .collect::<String>()
                    }
                ),
                reason: sv_report::GapReason::NotAsked,
                requirements: Vec::new(),
            })
        }
        Some(dir) => {
            let advisories::Database {
                records: database,
                unread,
            } = advisories::read_database(dir)
                .with_context(|| format!("reading the advisory database at {}", dir.display()))?;
            if database.is_empty() {
                examined.push(sv_report::Examined::not_run(
                    "advisory.",
                    "the advisory database holds no records `sv` could read",
                ));
                advisory_gaps.push(sv_report::Gap {
                    what: "known vulnerabilities in the packages this app ships".to_owned(),
                    why: format!(
                        "{} holds no advisory records `sv` could read, so nothing was compared. \
                         An empty database and a healthy app look the same from here.",
                        dir.display()
                    ),
                    reason: sv_report::GapReason::CouldNotRead,
                    requirements: Vec::new(),
                });
            } else {
                let mut result = advisories::audit_against(
                    bill_of_materials,
                    &database,
                    manifest.policy.fix_within_days.as_ref(),
                    advisories::Day::today(),
                );
                // Whole only as `sv audit` counts it: every ecosystem covered, every version
                // comparable, and the list of packages itself complete.
                let mut short = Vec::new();
                if !unread.is_empty() {
                    short.push(format!(
                        "{} file{} in the advisory database could not be read ({})",
                        unread.len(),
                        if unread.len() == 1 { "" } else { "s" },
                        unread
                            .iter()
                            .take(3)
                            .map(|(name, _)| format!("`{name}`"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
                if !result.uncovered.is_empty() {
                    let names: Vec<String> = result.uncovered.iter().cloned().collect();
                    short.push(format!(
                        "the database holds nothing about {}",
                        names.join(", ")
                    ));
                }
                if !result.uncomparable.is_empty() {
                    short.push(format!(
                        "{} package version(s) could not be compared",
                        result.uncomparable.len()
                    ));
                }
                if !bill_of_materials.is_complete() {
                    short.push("the list of packages is incomplete".to_owned());
                }
                // A finding that stops appearing because a different lockfile was read is not a
                // fixed finding, so a second lockfile nothing compared keeps this from `ran`.
                for passed in &bill_of_materials.passed_over {
                    short.push(format!(
                        "{} not read beside `{}`",
                        passed.not_read_list(),
                        passed.read
                    ));
                }
                // Nor is one that stops appearing because the manifest moved on and the lock did not.
                for disagreement in bill_of_materials
                    .disagreements
                    .iter()
                    .filter(|d| d.differs())
                {
                    short.push(format!(
                        "`{}` asks for other versions than `{}` has",
                        disagreement.manifest, disagreement.lockfile
                    ));
                }
                // Which database, how big, and how recent (backlog 226, part 2, item 20).
                let compared_with = sv_report::AdvisoryDatabase {
                    folder: dir.display().to_string(),
                    records: database.len(),
                    newest: database
                        .iter()
                        .filter_map(|a| a.published.as_deref())
                        .map(|day| day.chars().take(10).collect::<String>())
                        .max(),
                };
                examined.push(
                    if short.is_empty() {
                        sv_report::Examined::ran("advisory.")
                    } else {
                        sv_report::Examined::partly("advisory.", short.join("; "))
                    }
                    .with_advisories(compared_with),
                );
                // Records in a file nobody read were not compared, so nothing is credited on the
                // comparison, as `sv audit` credits nothing (the deep review's improvement 4).
                if !unread.is_empty() {
                    result.verified.clear();
                    advisory_gaps.push(sv_report::Gap {
                        what: "advisory files that could not be read".to_owned(),
                        why: format!(
                            "{}{}. The records in them were not compared with this app's packages.",
                            unread
                                .iter()
                                .take(10)
                                .map(|(name, why)| format!("`{name}`: {why}"))
                                .collect::<Vec<_>>()
                                .join("; "),
                            if unread.len() > 10 {
                                format!("; and {} more", unread.len() - 10)
                            } else {
                                String::new()
                            }
                        ),
                        reason: sv_report::GapReason::CouldNotRead,
                        requirements: Vec::new(),
                    });
                }
                findings_from_advisories = result.findings;
                advisory_verified = result.verified;
                if !result.uncovered.is_empty() {
                    advisory_gaps.push(sv_report::Gap {
                        what: format!(
                            "known vulnerabilities in this app's {} packages",
                            result
                                .uncovered
                                .iter()
                                .cloned()
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                        why: "the advisory database holds nothing about them, so they were not \
                              compared. That is not the same as their being clean."
                            .to_owned(),
                        reason: sv_report::GapReason::Partial,
                        requirements: Vec::new(),
                    });
                }
                if !result.uncomparable.is_empty() {
                    let n = result.uncomparable.len();
                    let shown: Vec<String> = result
                        .uncomparable
                        .iter()
                        .take(5)
                        .map(|(name, version)| format!("{name} {version}"))
                        .collect();
                    advisory_gaps.push(sv_report::Gap {
                        what: format!(
                            "whether {n} package version{} {} a known vulnerability ({}{})",
                            if n == 1 { "" } else { "s" },
                            if n == 1 { "has" } else { "have" },
                            shown.join(", "),
                            if n > 5 { ", …" } else { "" }
                        ),
                        why: "these versions could not be compared with any affected range, so \
                              nothing is claimed about them either way"
                            .to_owned(),
                        reason: sv_report::GapReason::CouldNotRead,
                        requirements: Vec::new(),
                    });
                }
            }
        }
    }
    Ok(Advisories {
        findings: findings_from_advisories,
        verified: advisory_verified,
        gaps: advisory_gaps,
    })
}

/// The language's own tool, where there is one and it is here. A tool that is not installed is
/// recorded as not run, with how to install it, never as having found nothing.
fn outside_tools(
    scene: &Scene,
    findings: &mut Vec<sv_check::Finding>,
    examined: &mut Vec<sv_report::Examined>,
) -> Result<Tools> {
    let Scene {
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        static_scan,
        ctx,
        resolved,
        buckets,
        seals,
    } = *scene;
    let static_scan::StaticScan {
        crashed: _,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
    } = static_scan;
    let Loaded {
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
        ..
    } = loaded;
    let _ = (
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        ctx,
        resolved,
        buckets,
        seals,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
    );

    let mut tool_verified = Vec::new();
    // The language's own tool, where there is one and it is here. A tool that is not installed is
    // recorded as not run, with how to install it — never as having found nothing.
    let mut tool_gaps = Vec::new();
    // A tool the owner has not turned on (CodeQL: `[tools] codeql = true`) is neither run nor named,
    // as if `sv` did not know it (ADR-115). Reading a results file from the owner's own CI, below,
    // runs nothing and keeps the full list.
    let opted_in = adapters
        .as_ref()
        .map(|a| a.opted_in(|name| manifest.tools.turned_on(name)));
    let adapters = &opted_in;
    if options.run_tools {
        let adapters = adapters.as_ref().map_err(|e| anyhow::anyhow!("{e}"))?;
        let languages: Vec<String> = scan_report.languages.iter().cloned().collect();
        let not_holding = adapters.not_holding(|condition| ctx.get(condition));
        // Ctrl-C while a tool runs stops the tool and everything it started, and the run ends
        // here, with the tools' private folder removed and the report folder let go of. Before
        // this, nothing caught Ctrl-C on this path: it ended `sv` alone, and the tool, in a process
        // group of its own, ran on with no limit (the review of 8 October 2026, item 1).
        sv_run::catch_interrupts();
        sv_check::adapters::stop_when(sv_run::interrupted);
        let outcome = sv_check::adapters::run_all_in(
            adapters,
            listing,
            &languages,
            &not_holding,
            &sv_check::adapters::scratch_dir(),
            secret_rules,
            &say_step,
        );
        // What the tools got to before then is not a report of the app, so nothing is written.
        if sv_run::interrupted() {
            eprintln!(
                "Stopped with Ctrl-C. The outside tools were stopped and their reports removed; \
                 nothing was written."
            );
            report_lock::let_go_of_all();
            exit::exit_with(exit::INTERRUPTED);
        }
        examined.extend(adapters_examined(adapters, &languages, &outcome));
        let mut found = outcome.findings;
        sv_check::adapters::set_apart_other_frameworks(
            adapters,
            &mut found,
            &scan_report.declared,
            &scan_report.unread_manifests,
        );
        sv_check::logged_ids::set_apart_logged_ids(app_dir, &mut found);
        findings.extend(found);
        tool_verified = outcome.verified;
        for (id, why, cause) in outcome.not_run {
            use sv_check::adapters::NotRunCause;
            tool_gaps.push(sv_report::Gap {
                what: format!("what `{id}` would have found"),
                why,
                reason: if outcome.partly.iter().any(|(partly, _)| *partly == id) {
                    sv_report::GapReason::Partial
                } else {
                    match cause {
                        NotRunCause::NotInstalled => sv_report::GapReason::NotInstalled,
                        NotRunCause::Stopped => sv_report::GapReason::Stopped,
                        NotRunCause::CouldNotRead => sv_report::GapReason::CouldNotRead,
                        NotRunCause::LeftOut => sv_report::GapReason::LeftOut,
                        NotRunCause::NothingToRead => sv_report::GapReason::NoReader,
                    }
                },
                requirements: Vec::new(),
            });
        }
    } else {
        match adapters {
            Ok(adapters) => {
                for adapter in adapters.all() {
                    examined.push(
                        sv_report::Examined::not_run(
                            format!("{}.", adapter.id),
                            "outside tools left out with --no-tools",
                        )
                        .as_outside_tool(&adapter.name),
                    );
                }
                tool_gaps.push(sv_report::Gap {
                    what: "the security tool this language already has".to_owned(),
                    why: format!(
                        "{} {} each know their languages far better than the handful of rules \
                         built in here.",
                        options.why_no_tools,
                        tool_names(adapters)
                    ),
                    reason: sv_report::GapReason::NotAsked,
                    requirements: Vec::new(),
                });
            }
            // Which tools there are is not known, so none is named, and the report says why
            // rather than listing no outside tools as though there were none.
            Err(why) => tool_gaps.push(sv_report::Gap {
                what: "the security tool this language already has".to_owned(),
                why: format!(
                    "{} Which outside tools sv can run is not known either: {} could not be read \
                     ({why}).",
                    options.why_no_tools,
                    adapters_path().display()
                ),
                reason: sv_report::GapReason::NotAsked,
                requirements: Vec::new(),
            }),
        }
    }
    // A CodeQL results file from the owner's own CI, read whether or not the tools ran: it is a file,
    // not a program, and credited only for the commit checked out (ADR-113).
    if let Some(path) = &options.codeql_results {
        let named = path.display().to_string();
        let gap = |reason: sv_report::GapReason, why: String| sv_report::Gap {
            what: "what your CI's CodeQL results would have said".to_owned(),
            why,
            reason,
            requirements: Vec::new(),
        };
        match (std::fs::read_to_string(path), &loaded.adapters) {
            (Err(e), _) => tool_gaps.push(gap(
                sv_report::GapReason::CouldNotRead,
                format!("{named} could not be read ({e})."),
            )),
            (Ok(_), Err(why)) => tool_gaps.push(gap(
                sv_report::GapReason::CouldNotRead,
                format!(
                "{named} was not read: which CodeQL rules `sv` maps is not known, since {} could \
                 not be read ({why}).",
                    adapters_path().display()
                ),
            )),
            (Ok(text), Ok(adapters)) => {
                let languages: Vec<String> = scan_report.languages.iter().cloned().collect();
                let read = sv_check::ci_codeql::read(
                    adapters,
                    &text,
                    &named,
                    &sv_check::ci_codeql::AppCommit::of(app_dir),
                    app_dir,
                    &languages,
                );
                findings.extend(read.findings);
                tool_verified.extend(read.verified);
                tool_gaps.extend(read.not_used.into_iter().map(|(kind, why)| {
                    use sv_check::ci_codeql::Unused;
                    let reason = match kind {
                        Unused::Unreadable => sv_report::GapReason::CouldNotRead,
                        Unused::NoMap => sv_report::GapReason::NoReader,
                        Unused::OtherCode => sv_report::GapReason::Outdated,
                        Unused::LookedAway => sv_report::GapReason::Partial,
                    };
                    gap(reason, why)
                }));
            }
        }
    }
    Ok(Tools {
        verified: tool_verified,
        gaps: tool_gaps,
    })
}

/// The app started behind the fence and asked questions, with its own tests run, when `--run` was
/// given; otherwise why it was not, as a gap.
fn running_app(
    scene: &Scene,
    findings: &mut Vec<sv_check::Finding>,
    gaps: &mut Vec<sv_report::Gap>,
) -> RunningApp {
    let Scene {
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        static_scan,
        ctx,
        resolved,
        buckets,
        seals,
    } = *scene;
    let static_scan::StaticScan {
        crashed: _,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
    } = static_scan;
    let Loaded {
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
        ..
    } = loaded;
    let _ = (
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        ctx,
        resolved,
        buckets,
        seals,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
    );

    let mut probe_verified = Vec::new();
    let mut test_verified = Vec::new();
    let mut test_output = None;
    // Whether the app's own tests were read for what they name (`tests.`), for `examined` and for
    // telling a review whose finding is gone from one nobody looked for (deep review R3).
    let mut tests_examined =
        sv_report::Examined::not_run("tests.", "the app's own tests run only with --run");
    let mut run_note = None;
    let mut seen = None;
    // What the first account's sign-in token carries in plain sight, for the data-classification questions (backlog 0497).
    let mut token_fields = None;
    let mut run_steps: Vec<String> = Vec::new();
    let mut suite_timings = Vec::new();
    let mut request_timings = Vec::new();
    let mut check_timings = Vec::new();
    let run_status;

    if options.run_the_app {
        // Worked out once, from the static scan's walk of the folder (backlog 0354).
        let private_files = crate::private_files(listing);
        match probe_the_running_app(manifest, app_dir, options.slow, &private_files) {
            Ok((outcome, plan)) => {
                suite_timings.clone_from(&outcome.suite_timings);
                request_timings.clone_from(&outcome.request_timings);
                // Every suite's checks and waits, as `asked` lists the suites (backlog 0526).
                check_timings = outcome
                    .asked()
                    .into_iter()
                    .flat_map(|(_, asked)| asked.timings.iter().cloned())
                    .collect();
                run_status = sv_report::RunStatus::Started {
                    image: plan.image.clone(),
                    asked: anonymous_requests(&plan, &private_files).len(),
                    answered: outcome.probe_responses.len(),
                    signed_in: outcome.signed_in.is_some(),
                    tests: match &outcome.tests {
                        Some(t) if t.stopped_after.is_some() => "stopped",
                        other => {
                            sv_report::RunStatus::tests_state(other.as_ref().map(|t| t.exit_code))
                        }
                    }
                    .to_owned(),
                };
                let (mut running_findings, mut running_verified, signed_in_not_assessed) =
                    running_app_evidence(&outcome, &plan, &private_files);
                limits_set_for_the_run(&mut running_findings, &plan.start);
                let (loosened, settings_files) = loosened_for_the_copy(
                    &mut running_findings,
                    &mut running_verified,
                    &plan.app_dir,
                    &plan.start,
                );
                gaps.extend(loosened);
                gaps.extend(short_answers_gap(
                    secret_rules,
                    &outcome.short_answers,
                    outcome.short_answers_log.as_deref(),
                ));
                let mut record = crate::seen::record(
                    secret_rules,
                    &anonymous_requests(&plan, &private_files),
                    &outcome.probe_responses,
                    &outcome.probes_rate_limited,
                );
                crate::seen::stand_ins(secret_rules, &outcome.stand_ins, &mut record);
                crate::seen::app_log(
                    secret_rules,
                    outcome.signed_in.as_ref(),
                    outcome.ai.as_ref(),
                    &mut record,
                );
                crate::seen::signed_in(secret_rules, outcome.signed_in.as_ref(), &mut record);
                token_fields = outcome
                    .signed_in
                    .as_ref()
                    .and_then(|s| s.token_fields.clone());
                // The AI suite's questions, with ids `ai-<kind>-<n>`, kept beside the signed-in suite's (backlog 0381).
                crate::seen::signed_in(secret_rules, outcome.ai.as_ref(), &mut record);
                crate::seen::liveness(secret_rules, &outcome.liveness, &mut record);
                seen = Some(record);
                findings.extend(running_findings);
                probe_verified = running_verified;
                // The summary, and the steps kept apart from it. Joining them made one
                // 381-word paragraph at the top of the report — the first thing the reader met,
                // and unreadable. The renderers lay the steps out as a list.
                let signed_in_steps: Vec<String> = outcome
                    .signed_in
                    .as_ref()
                    .map(|s| s.steps.clone())
                    .unwrap_or_default();
                let oidc_steps: Vec<String> = outcome
                    .oidc
                    .as_ref()
                    .map(|s| s.steps.clone())
                    .unwrap_or_default();
                let ai_steps: Vec<String> = outcome
                    .ai
                    .as_ref()
                    .map(|s| s.steps.clone())
                    .unwrap_or_default();
                // Every suite's steps, from the one list `sv run` prints from too.
                run_steps = outcome
                    .asked()
                    .into_iter()
                    .flat_map(|(_, asked)| asked.steps.iter().cloned())
                    .collect();
                let signed_in_note = if signed_in_steps.is_empty() {
                    String::new()
                } else {
                    format!(
                        " It was then asked {} more question{} as two signed-in test users.",
                        signed_in_steps.len(),
                        if signed_in_steps.len() == 1 { "" } else { "s" }
                    )
                };
                let oidc_note = if oidc_steps.is_empty() {
                    ""
                } else {
                    " Its sign-in through another service was asked about with a test provider of \
                     `sv`'s own, pointed at it for the run."
                };
                let ai_note = if ai_steps.is_empty() {
                    ""
                } else {
                    " Its AI feature was asked with a test model of `sv`'s own in place of the \
                     real one, so nothing was sent to an AI service and nothing was spent."
                };
                run_note = Some(format!(
                    "This app was started with {} and asked {} question{} while it ran, as \
                     somebody who had not signed in. It answered on {}.{signed_in_note}{oidc_note}{ai_note} {}",
                    plan.image,
                    outcome.probe_responses.len(),
                    if outcome.probe_responses.len() == 1 {
                        ""
                    } else {
                        "s"
                    },
                    plan.health_path,
                    outcome.fence.explain()
                ));
                if let Some(note) = run_note.as_mut() {
                    note.push_str(&format!(
                        " What it answered is kept beside this report in {}, with the \
                         credentials sv recognized cut out: it is the app's own text.",
                        sv_report::seen::FILE
                    ));
                    note.push_str(&format!(
                        " {}",
                        crate::seen::lookups_sentence(outcome.stand_ins.names.as_deref())
                    ));
                }
                if let (Some(note), Some(installed)) = (
                    run_note.as_mut(),
                    sv_run::install::sentence(&outcome.installed),
                ) {
                    note.push_str(&format!(" {installed}"));
                }
                if let (Some(note), Some(removed)) = (
                    run_note.as_mut(),
                    sv_run::cleanup::removed_sentence(&outcome.left_over_removed),
                ) {
                    note.push_str(&format!(" {removed}"));
                }
                if let (Some(note), Some(container)) =
                    (run_note.as_mut(), outcome.container.sentences())
                {
                    note.push_str(&format!(" {container}"));
                }
                if let Some(note) = run_note.as_mut() {
                    note.push_str(&format!(
                        " {}",
                        sv_run::resources::sentence(outcome.resources.as_ref())
                    ));
                }
                // The settings the start command gave the copy run, by name (backlog 0386), then any that look like they
                // weaken it.
                if let (Some(note), Some(settings)) =
                    (run_note.as_mut(), sv_run::start_settings::note(&plan.start))
                {
                    note.push_str(&format!(" {settings}"));
                }
                if let (Some(note), Some(weaker)) =
                    (run_note.as_mut(), sv_run::weakening_note(&plan.start))
                {
                    note.push_str(&format!(" {weaker}"));
                }
                // Which settings files were read for that, and that no other is (backlog 0605).
                if let Some(note) = run_note.as_mut() {
                    note.push_str(&format!(" {settings_files}"));
                }
                for (requirements, why) in signed_in_not_assessed {
                    // AISVS ids are the AI feature's, asked through the test model, which may not
                    // have involved signing in at all.
                    let how = if requirements.starts_with('C') {
                        "by asking the running app's AI feature through a test model"
                    } else {
                        "by asking the running app as a signed-in user"
                    };
                    gaps.push(sv_report::Gap {
                        what: format!("{requirements}, {how}"),
                        why,
                        reason: sv_report::GapReason::Stopped,
                        requirements: requirement_ids(&requirements),
                    });
                }
                gaps.extend(rate_limited_gap(&outcome.probes_rate_limited));
                gaps.extend(sidecar_lost_gap(outcome.sidecar_lost.as_deref()));
                // What asking it could not reach. These replace the "it was never started" gap
                // rather than removing it: the app running answers some questions and not others,
                // and the ones it cannot answer are the ones behind a login.
                for (requirements, why) in probes::running_app_gaps(
                    outcome.signed_in.is_some(),
                    &outcome.probe_responses,
                    &bill_of_materials.components,
                ) {
                    gaps.push(sv_report::Gap {
                        what: format!("{requirements}, by asking the running app"),
                        why,
                        reason: sv_report::GapReason::Partial,
                        requirements: requirement_ids(requirements),
                    });
                }
                match &outcome.tests {
                    Some(result) if result.stopped_after.is_some() => {
                        let after = result.stopped_after.unwrap_or(sv_run::TEST_LIMIT);
                        tests_examined = sv_report::Examined::not_run(
                            "tests.",
                            "the app's own tests were stopped before they finished",
                        );
                        gaps.push(sv_report::Gap {
                            what: "anything the app's own tests would have shown".to_owned(),
                            why: format!(
                                "they had not finished after {}, the most a test run may take, \
                                 and were stopped. A suite cut short credits nothing, whatever it \
                                 printed before it was stopped.",
                                sv_run::minutes(after)
                            ),
                            reason: sv_report::GapReason::Stopped,
                            requirements: Vec::new(),
                        });
                        test_output = sv_check::suite::failing_output(
                            result.exit_code,
                            &result.output,
                            secret_rules,
                        )
                        .map(|t| sv_check::suite::FailingOutput {
                            stopped_after: Some(sv_run::minutes(after)),
                            ..t
                        });
                    }
                    Some(result) => {
                        // Only tests that name a requirement count, and only when something says
                        // they passed. Matching a test to a requirement by what it is called would
                        // credit one on the strength of a name somebody chose for other reasons.
                        let known: std::collections::BTreeSet<&str> =
                            frameworks.requirements.keys().map(String::as_str).collect();
                        let named = sv_check::suite::tests_naming_requirements_in(listing, &known);
                        let describe = |id: &str| {
                            frameworks
                                .requirements
                                .get(id)
                                .map(|r| r.description.clone())
                        };

                        // A suite that passed outright needs no report. One that did not is worth
                        // whatever its runner's own report says still passed — and nothing more,
                        // which is why an unreadable report falls back to crediting nothing rather
                        // than to crediting what it managed to understand.
                        let reported_cases = if result.exit_code == 0 {
                            None
                        } else {
                            match sv_check::suite::reported_cases(result.report.as_deref()) {
                                Some(Ok(cases)) => Some(cases),
                                Some(Err(unreadable)) => {
                                    gaps.push(sv_report::Gap {
                                        what: "which of the app's own tests passed".to_owned(),
                                        why: format!(
                                            "the suite failed (exit {}) and its report could not \
                                             be read: {}. Nothing is credited from it.",
                                            result.exit_code, unreadable.why
                                        ),
                                        reason: sv_report::GapReason::CouldNotRead,
                                        requirements: Vec::new(),
                                    });
                                    None
                                }
                                None => None,
                            }
                        };
                        tests_examined = match (result.exit_code, &reported_cases) {
                            (0, _) => sv_report::Examined::ran("tests."),
                            (_, Some(_)) => sv_report::Examined::partly(
                                "tests.",
                                "the suite failed, and only the tests its runner reported as \
                                 passing were read",
                            ),
                            (_, None) => sv_report::Examined::not_run(
                                "tests.",
                                "the suite failed and nothing says which of its tests passed",
                            ),
                        };
                        let suite_outcome = if result.exit_code == 0 {
                            sv_check::suite::SuiteOutcome::Passed
                        } else {
                            sv_check::suite::SuiteOutcome::Failed {
                                cases: reported_cases.as_deref(),
                            }
                        };
                        let (credited, mismatches) =
                            sv_check::suite::credit(&named, suite_outcome, &describe);

                        test_output = sv_check::suite::failing_output(
                            result.exit_code,
                            &result.output,
                            secret_rules,
                        );
                        if result.exit_code != 0 {
                            gaps.push(sv_report::Gap {
                                what: "anything the failing tests would have shown".to_owned(),
                                why: match (&reported_cases, credited.len()) {
                                    (Some(_), 0) => format!(
                                        "the suite failed (exit {}) and no test its runner \
                                         reported as passing names a requirement",
                                        result.exit_code
                                    ),
                                    (Some(_), n) => format!(
                                        "the suite failed (exit {}); {n} requirement {} from \
                                         tests its runner reported as passing, and the rest of \
                                         the suite says nothing either way",
                                        result.exit_code,
                                        if n == 1 { "claim comes" } else { "claims come" }
                                    ),
                                    (None, _) => format!(
                                        "they failed (exit {}) and nothing says which of them did, \
                                         so nothing can be concluded from them either way. {}",
                                        result.exit_code,
                                        result.report_note.as_deref().unwrap_or(
                                            "Declare test-report in stackvet.toml to have the \
                                             tests that did pass still count."
                                        )
                                    ),
                                },
                                reason: if reported_cases.is_some() {
                                    sv_report::GapReason::Partial
                                } else {
                                    sv_report::GapReason::Stopped
                                },
                                requirements: Vec::new(),
                            });
                        } else if credited.is_empty() {
                            gaps.push(sv_report::Gap {
                                what: "what the app's own tests cover".to_owned(),
                                why: "the suite passed, and no test names the requirement it is \
                                      for, so nothing here can say which requirements they are \
                                      evidence about. `sv init` explains how to name them."
                                    .to_owned(),
                                reason: sv_report::GapReason::NotAsked,
                                requirements: Vec::new(),
                            });
                        }
                        findings.extend(mismatches);
                        test_verified = credited;
                    }
                    None => {
                        tests_examined = sv_report::Examined::not_run(
                            "tests.",
                            "stackvet.toml declares no test command",
                        );
                        gaps.push(sv_report::Gap {
                            what: "the app's own tests".to_owned(),
                            why: "stackvet.toml declares no test command".to_owned(),
                            reason: sv_report::GapReason::NotAsked,
                            requirements: Vec::new(),
                        })
                    }
                }
            }
            Err((reason, kind)) => {
                run_status = sv_report::RunStatus::CouldNotStart {
                    why: reason.clone(),
                };
                tests_examined = sv_report::Examined::not_run(
                    "tests.",
                    "--run was given and the app could not be run",
                );
                gaps.push(sv_report::Gap {
                    what: "the running app".to_owned(),
                    why: format!("--run was given and the app could not be run. {reason}"),
                    reason: kind,
                    requirements: Vec::new(),
                })
            }
        }
    } else {
        run_status = sv_report::RunStatus::NotAsked {
            why: options.why_not_run.clone(),
        };
        gaps.push(sv_report::Gap {
            what: "the running app".to_owned(),
            why: format!(
                "{} Without it, nothing here has asked the app anything — what it sends to a \
                 browser, what it says when something goes wrong, which sites it accepts.",
                options.why_not_run
            ),
            reason: sv_report::GapReason::NotAsked,
            requirements: Vec::new(),
        });
    }
    RunningApp {
        status: run_status,
        note: run_note,
        steps: run_steps,
        test_output,
        tests_examined,
        probe_verified,
        test_verified,
        seen,
        token_fields,
        timings: suite_timings,
        request_timings,
        check_timings,
    }
}

/// What no check read, each said as a gap: links, folders left out, files that did not parse,
/// languages without a parser, the checks that could not run, and the requirements that are design
/// review rather than scanning. Returns those last, for the report.
fn what_was_not_read(
    scene: &Scene,
    gaps: &mut Vec<sv_report::Gap>,
    examined: &mut Vec<sv_report::Examined>,
) -> std::collections::BTreeSet<String> {
    let Scene {
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        static_scan,
        ctx,
        resolved,
        buckets,
        seals,
    } = *scene;
    let static_scan::StaticScan {
        crashed: _,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
    } = static_scan;
    let Loaded {
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
        ..
    } = loaded;
    let _ = (
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        ctx,
        resolved,
        buckets,
        seals,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
    );

    if !listing.links.is_empty() {
        let shown: Vec<&str> = listing.links.iter().take(5).map(String::as_str).collect();
        gaps.push(sv_report::Gap {
            what: format!(
                "{} symbolic link{} in the app, not followed",
                listing.links.len(),
                if listing.links.len() == 1 { "" } else { "s" }
            ),
            why: format!(
                "a link can lead outside the app, or back into it in a loop, so nothing here \
                 followed {}: {}{}. What it points at was not read by any check.",
                if listing.links.len() == 1 {
                    "it"
                } else {
                    "them"
                },
                shown.join(", "),
                if listing.links.len() > 5 {
                    format!(", and {} more", listing.links.len() - 5)
                } else {
                    String::new()
                }
            ),
            reason: sv_report::GapReason::LeftOut,
            requirements: Vec::new(),
        });
    }
    if !listing.skipped.is_empty() {
        let shown: Vec<String> = listing
            .skipped
            .iter()
            .take(5)
            .map(|(dir, why)| format!("`{dir}/` ({why})"))
            .collect();
        gaps.push(sv_report::Gap {
            what: format!(
                "{} folder{} left out as installed or built code",
                listing.skipped.len(),
                if listing.skipped.len() == 1 { "" } else { "s" }
            ),
            why: format!(
                "a folder named like an ecosystem's output, beside the file that makes it so, or a Python \
                 virtual environment under any name, holds code the app installed or built rather than \
                 wrote, and no check read it, which is normal: {}{}. If one holds the app's own code, \
                 move it or rename the folder.",
                shown.join(", "),
                if listing.skipped.len() > 5 {
                    format!(", and {} more", listing.skipped.len() - 5)
                } else {
                    String::new()
                }
            ),
            reason: sv_report::GapReason::LeftOut,
            requirements: Vec::new(),
        });
    }
    if !listing.special.is_empty() {
        let shown: Vec<&str> = listing.special.iter().take(5).map(String::as_str).collect();
        gaps.push(sv_report::Gap {
            what: format!(
                "{} {} in the app that {} not an ordinary file",
                listing.special.len(),
                if listing.special.len() == 1 {
                    "entry"
                } else {
                    "entries"
                },
                if listing.special.len() == 1 {
                    "is"
                } else {
                    "are"
                }
            ),
            why: format!(
                "a named pipe, a socket, or a device is not opened, since opening a pipe waits for \
                 something to write into it: {}{}. No check read {}.",
                shown.join(", "),
                if listing.special.len() > 5 {
                    format!(", and {} more", listing.special.len() - 5)
                } else {
                    String::new()
                },
                if listing.special.len() == 1 {
                    "it"
                } else {
                    "them"
                }
            ),
            reason: sv_report::GapReason::NoReader,
            requirements: Vec::new(),
        });
    }
    if !code.unread_files.is_empty() {
        let shown: Vec<String> = code
            .unread_files
            .iter()
            .take(5)
            .map(|(file, why)| format!("{file} ({why})"))
            .collect();
        gaps.push(sv_report::Gap {
            what: format!(
                "{} file{} in a language the rules read, not opened",
                code.unread_files.len(),
                if code.unread_files.len() == 1 {
                    ""
                } else {
                    "s"
                }
            ),
            why: format!(
                "{}{}. While part of the app went unread, no rule that reads these files' language \
                 can say it found nothing wrong.",
                shown.join("; "),
                if code.unread_files.len() > 5 {
                    format!("; and {} more", code.unread_files.len() - 5)
                } else {
                    String::new()
                }
            ),
            reason: sv_report::GapReason::CouldNotRead,
            requirements: Vec::new(),
        });
    }
    if !code.broken_queries.is_empty() {
        let shown: Vec<String> = code
            .broken_queries
            .iter()
            .map(|b| format!("{} for {} ({})", b.rule_id, b.language, b.why))
            .collect();
        gaps.push(sv_report::Gap {
            what: format!(
                "{} code rule{} that could not run",
                code.broken_queries.len(),
                if code.broken_queries.len() == 1 {
                    ""
                } else {
                    "s"
                }
            ),
            why: format!(
                "{}. The rule's query for that language would not compile, so it read none of \
                 those files; while a rule did not run, no rule that reads code can say it found \
                 nothing wrong. This is a fault in sv's rule file, not in the app.",
                shown.join("; ")
            ),
            reason: sv_report::GapReason::Stopped,
            requirements: Vec::new(),
        });
    }
    gaps.extend(not_the_app_gaps(manifest, scan_report));
    for (id, why) in &config.not_assessed {
        gaps.push(sv_report::Gap {
            what: format!("the check `{id}`"),
            why: why.clone(),
            reason: sv_report::GapReason::CouldNotRead,
            requirements: Vec::new(),
        });
    }
    if !secrets.coverage.skipped.is_empty() {
        // Each named, with why, so nobody has to go looking for which file it was.
        const SHOWN: usize = 5;
        let skipped = &secrets.coverage.skipped;
        let mut named: Vec<String> = skipped
            .iter()
            .take(SHOWN)
            .map(|(file, why)| format!("`{file}` ({why})"))
            .collect();
        if skipped.len() > SHOWN {
            named.push(format!(
                "and {} more, listed by `sv check`",
                skipped.len() - SHOWN
            ));
        }
        gaps.push(sv_report::Gap {
            what: format!(
                "{} file{} not read while looking for credentials",
                skipped.len(),
                if skipped.len() == 1 { "" } else { "s" }
            ),
            why: format!(
                "{}. A credential in a file nothing read is a credential nothing found.",
                named.join(", ")
            ),
            reason: sv_report::GapReason::CouldNotRead,
            requirements: Vec::new(),
        });
    }
    if !code.unread_languages.is_empty() {
        let mut names: Vec<&str> = code.unread_languages.iter().map(String::as_str).collect();
        names.sort_unstable();
        let mut why =
            "`sv` has no parser for these, so the rules that read code did not run on them"
                .to_owned();
        if !code.unread_templates.is_empty() {
            why.push_str(&format!(
                "; the templates are {}",
                shown_files(&code.unread_templates)
            ));
        }
        gaps.push(sv_report::Gap {
            what: format!("code written in {}", names.join(", ")),
            why,
            reason: sv_report::GapReason::NoReader,
            requirements: Vec::new(),
        });
    }
    if !code.sql_files.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!("the SQL in {}", shown_files(&code.sql_files)),
            why: "no rule reads a file of SQL. Nothing is held back for it: the rules against \
                  queries built by hand look at how the app's code builds a query"
                .to_owned(),
            reason: sv_report::GapReason::NoReader,
            requirements: Vec::new(),
        });
    }
    if !code.unparsed_files.is_empty() {
        let n = code.unparsed_files.len();
        let mut shown: Vec<&str> = code
            .unparsed_files
            .iter()
            .map(String::as_str)
            .take(5)
            .collect();
        if n > 5 {
            shown.push("…");
        }
        gaps.push(sv_report::Gap {
            what: format!(
                "part of {n} file{} that did not parse cleanly ({})",
                if n == 1 { "" } else { "s" },
                shown.join(", ")
            ),
            why: "whatever sat where the parser gave up was not read, so a rule whose call is named \
                  anywhere in these files cannot say it found nothing wrong anywhere in this app; a \
                  rule whose call is named nowhere in them could not have found it there. What was \
                  found stands"
                .to_owned(),
            reason: sv_report::GapReason::Partial,
            requirements: Vec::new(),
        });
    }
    gaps.extend(untaught_gaps(&code.untaught));
    gaps.extend(static_scan.package_gaps());
    gaps.extend(static_scan.crash_gaps());
    examined.extend(static_scan.examined());
    // A package list found and not read: what it names is unknown, so a technology known only by its
    // package is answered as incomplete, not as not used (backlog 0226, part 1, item 11; 0236).
    for unread in &scan_report.unread_manifests {
        gaps.push(sv_report::Gap {
            what: format!("the package list {}", unread.manifest),
            why: format!(
                "{}, so the packages it names were not read: a technology `sv` knows only by its \
                 package is not answered as not used, but as incomplete. Fix the file and check again",
                unread.why
            ),
            reason: sv_report::GapReason::CouldNotRead,
            requirements: Vec::new(),
        });
    }
    if !scan_report.unread_extensions.is_empty() {
        let mut exts: Vec<&str> = scan_report
            .unread_extensions
            .iter()
            .map(String::as_str)
            .collect();
        exts.sort_unstable();
        gaps.push(sv_report::Gap {
            what: format!("files ending {}", exts.join(", ")),
            why:
                "the technology scan did not look in these, so no technology can be called absent \
                  on their account"
                    .to_owned(),
            reason: sv_report::GapReason::NoReader,
            requirements: Vec::new(),
        });
    }
    // The Secure by Design checklist is design review, not scanning. Its controls are applicable
    // and every one is unverified — which is true of a great many ASVS requirements too, and the
    // difference matters: those could in principle be reached by some check, and these cannot be
    // reached by any, ever. Counting them together lets a reader think the scanner tried.
    let manual_only = buckets.manual_only(config_rules);
    let design_review = manual_only.len();
    if design_review > 0 {
        gaps.push(sv_report::Gap {
            what: format!(
                "{design_review} requirement{} that are design review, not scanning",
                if design_review == 1 { "" } else { "s" }
            ),
            why: "these ask how the system was designed and how it is run \u{2014} whether trust \
                  zones are enforced, whether an incident response plan is rehearsed, whether data \
                  has named owners. No check here reaches them and none ever will, so they are \
                  counted as applicable and unverified, and a person has to answer them."
                .to_owned(),
            reason: sv_report::GapReason::PersonOnly,
            requirements: Vec::new(),
        });
    }
    gaps.extend(dependency_gaps(bill_of_materials));
    manual_only
}

/// Which requirements the app's tests name, and which no test of the app's could show.
fn requirements_for_tests(
    scene: &Scene,
) -> (
    std::collections::BTreeSet<String>,
    std::collections::BTreeSet<String>,
) {
    let Scene {
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        static_scan,
        ctx,
        resolved,
        buckets,
        seals,
    } = *scene;
    let static_scan::StaticScan {
        crashed: _,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
    } = static_scan;
    let Loaded {
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
        ..
    } = loaded;
    let _ = (
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        ctx,
        resolved,
        buckets,
        seals,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
    );

    // Which requirements the app's tests name, read from the files whether or not the tests ran, so
    // the report can tell a requirement nobody has written a test for from one whose test did not
    // run here.
    let named_in_tests: std::collections::BTreeSet<String> = {
        let known: std::collections::BTreeSet<&str> =
            frameworks.requirements.keys().map(String::as_str).collect();
        sv_check::suite::tests_naming_requirements(app_dir, &known)
            .into_iter()
            .flat_map(|t| t.requirement_ids)
            .collect()
    };

    // What an application's own tests cannot show, so it is not listed as a test to write.
    let not_for_tests: std::collections::BTreeSet<String> = buckets
        .applicable
        .iter()
        .filter(|id| config_rules.not_for_tests(frameworks, id))
        .cloned()
        .collect();

    // The owner's answers, from the notes file beside the app. Absent when they have not run
    // `sv notes`, which is the common case and not a gap: the report then says the file exists to
    (named_in_tests, not_for_tests)
}

/// The owner's answers: the notes file, the decisions file, and the manifest's design questions
/// and checks made by hand, with a person's confirmations of what the AI tool said applied.
fn the_owners_word(
    scene: &Scene,
    findings: &mut Vec<sv_check::Finding>,
    gaps: &mut Vec<sv_report::Gap>,
) -> Result<PersonsWord> {
    let Scene {
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        static_scan,
        ctx,
        resolved,
        buckets,
        seals,
    } = *scene;
    let static_scan::StaticScan {
        crashed: _,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
    } = static_scan;
    let Loaded {
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
        ..
    } = loaded;
    let _ = (
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        ctx,
        resolved,
        buckets,
        seals,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
    );

    // The owner's answers, from the notes file beside the app. Absent when they have not run
    // `sv notes`, which is the common case and not a gap: the report then says the file exists to
    // be written.
    let notes_catalog = sv_check::notes::Catalog::load(&notes_path())?;
    let notes_text = std::fs::read_to_string(app_dir.join(&notes_catalog.file));
    let notes_sha256 = notes_text
        .as_ref()
        .ok()
        .map(|text| crate::bundle::sha256(text.as_bytes()));
    let notes = match notes_text {
        Ok(text) => sv_check::notes::evidence(
            &notes_catalog,
            &sv_check::notes::read_answers(&notes_catalog, &text),
            &notes_catalog.file,
            seals,
        ),
        Err(_) => {
            let asked = notes_catalog
                .sections
                .iter()
                .filter(|s| buckets.applicable.contains(&s.id))
                .count();
            if asked > 0 {
                gaps.push(sv_report::Gap {
                    what: format!(
                        "{asked} requirement{} that ask for a written decision",
                        if asked == 1 { "" } else { "s" }
                    ),
                    why: format!(
                        "No tool can answer these: they ask what your rules are, who may do what, \
                         and how long things are kept. Run `sv notes` to write {}, answer the \
                         questions in it, and they become documented. Your AI coding tool can \
                         ask you them: `sv questions` prints them for its chat.",
                        notes_catalog.file
                    ),
                    reason: sv_report::GapReason::PersonOnly,
                    requirements: Vec::new(),
                });
            }
            sv_check::notes::Evidence::default()
        }
    };
    if !notes.unreadable.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!(
                "who wrote {} section{} of {}",
                notes.unreadable.len(),
                if notes.unreadable.len() == 1 { "" } else { "s" },
                notes_catalog.file
            ),
            why: format!(
                "Each section says who wrote it on one line, `{} {}` or `{} {}`, and {} names \
                 somebody else or says both, so nothing was made of it: {}.",
                sv_check::notes::WRITTEN_BY,
                sv_check::notes::BY_OWNER,
                sv_check::notes::WRITTEN_BY,
                sv_check::notes::BY_AI_TOOL,
                if notes.unreadable.len() == 1 {
                    "this one"
                } else {
                    "these"
                },
                notes.unreadable.join(", ")
            ),
            reason: sv_report::GapReason::CouldNotRead,
            requirements: Vec::new(),
        });
    }
    if !notes.not_read.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!(
                "what is under {} heading{} of your own in {}",
                notes.not_read.len(),
                if notes.not_read.len() == 1 { "" } else { "s" },
                notes_catalog.file
            ),
            why: format!(
                "A heading that is not one of the file's questions ends the answer above it, so \
                 what is under it was not read as an answer to anything: {}. Keeping notes of your \
                 own there is fine. If one of them is part of an answer, move it up into that \
                 answer, or use a `####` heading inside the answer instead.",
                notes.not_read.join("; ")
            ),
            reason: sv_report::GapReason::Partial,
            requirements: Vec::new(),
        });
    }
    // A file written by hand under headings of its own, before any question: the Cursor trial's, which held the
    // owner's real decisions and counted for nothing, said only in the counts (backlog 0542).
    if let Some(first) = notes.outside.first() {
        let n = notes.outside.len();
        gaps.push(sv_report::Gap {
            what: format!(
                "what is under {n} heading{} of your own in {}, before any of its questions",
                if n == 1 { "" } else { "s" },
                notes_catalog.file
            ),
            why: format!(
                "An answer counts only under its question's own heading (`## V…`, as `sv notes` writes them), so \
                 nothing under {} counts, the first being \"{first}\". If these are your decisions, have your AI \
                 coding tool record each one with `stackvet_record_answer`, or run `sv notes` to add the questions \
                 and move each answer under its own; then `sv review` makes them yours.",
                if n == 1 { "it" } else { "these" }
            ),
            reason: sv_report::GapReason::Partial,
            requirements: Vec::new(),
        });
    }
    // The decisions the design-time prompts write down (`sv_check::decisions`). Two sections count
    // toward Secure by Design controls, read as the notes are; the file's other sections are not
    // questions, so a heading `sv` does not read for credit is not reported as one left unread.
    let decisions_catalog = sv_check::notes::Catalog::load(&decisions_path())?;
    let decisions_text = std::fs::read_to_string(app_dir.join(&decisions_catalog.file)).ok();
    let decisions =
        decisions_text
            .as_deref()
            .map_or_else(sv_check::notes::Evidence::default, |text| {
                sv_check::notes::evidence(
                    &decisions_catalog,
                    &sv_check::notes::read_answers(&decisions_catalog, text),
                    &decisions_catalog.file,
                    seals,
                )
            });
    if !decisions.unreadable.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!(
                "who wrote {} section{} of {}",
                decisions.unreadable.len(),
                if decisions.unreadable.len() == 1 {
                    ""
                } else {
                    "s"
                },
                decisions_catalog.file
            ),
            why: format!(
                "Each section says who wrote it on one line, `{} {}` or `{} {}`, and {} names \
                 somebody else or says both, so nothing was made of it: {}.",
                sv_check::notes::WRITTEN_BY,
                sv_check::notes::BY_OWNER,
                sv_check::notes::WRITTEN_BY,
                sv_check::notes::BY_AI_TOOL,
                if decisions.unreadable.len() == 1 {
                    "this one"
                } else {
                    "these"
                },
                decisions.unreadable.join(", ")
            ),
            reason: sv_report::GapReason::CouldNotRead,
            requirements: Vec::new(),
        });
    }
    // A review by a person is the one thing that section can ask for, and no tool can do it, so
    // what it says is repeated here, where what was not examined is listed. Its words are not read
    // for a yes or a no, and credit nothing.
    // Who wrote it is said, from the section's own `Written by:` line: a section the AI coding tool
    // wrote, saying no person need look, read as the owner's own judgment (gap analysis 4.2).
    if let Some(said) = decisions_text
        .as_deref()
        .and_then(|text| sv_check::decisions::section(text, sv_check::decisions::BRING_IN_A_PERSON))
    {
        let writer = decisions_text.as_deref().and_then(|text| {
            sv_check::decisions::section_writer(text, sv_check::decisions::BRING_IN_A_PERSON)
        });
        let whose = match writer.as_deref() {
            Some(w) if w.eq_ignore_ascii_case(sv_check::notes::BY_OWNER) => {
                "in a section marked as written by you".to_owned()
            }
            Some(w) if w.eq_ignore_ascii_case(sv_check::notes::BY_AI_TOOL) => {
                "in a section your AI coding tool wrote".to_owned()
            }
            _ => "in a section that does not say who wrote it, so it counts as your AI coding \
                  tool's"
                .to_owned(),
        };
        gaps.push(sv_report::Gap {
            what: "a person's security review of the design".to_owned(),
            why: format!(
                "No tool can make it. Your {} says, under \"{}\", {whose}: \"{said}\"",
                sv_check::decisions::FILE,
                sv_check::decisions::BRING_IN_A_PERSON
            ),
            reason: sv_report::GapReason::PersonOnly,
            requirements: Vec::new(),
        });
    }
    let documented: Vec<sv_check::Verified> = notes
        .documented
        .into_iter()
        .chain(decisions.documented)
        .collect();

    // The design questions, answered in stackvet.toml. `yes` is the owner's word and the weakest
    // tier here; `no`, and a `where` naming a file the app does not have, are findings.
    let design_questions = sv_check::design::Questions::load(&design_questions_path())?;
    let human_checks =
        sv_check::human::HumanChecks::load(&sv_frameworks::data::file("human-checks.json"))?;
    let design_answers: std::collections::BTreeMap<String, sv_check::design::Answer> = manifest
        .design
        .iter()
        .map(|(id, a)| {
            (
                id.clone(),
                sv_check::design::Answer {
                    answer: a.answer.clone(),
                    location: a.r#where.clone(),
                    by: a.by.clone(),
                    recorded: sv_check::seal::owner_recorded(
                        seals,
                        a.seal.as_deref(),
                        &sv_check::seal::design_answer_fields(id, a),
                    ),
                },
            )
        })
        .collect();
    let mut design = sv_check::design::evaluate(
        &design_questions,
        &design_answers,
        &|id| buckets.applicable.iter().any(|a| a == id),
        &|path| app_dir.join(path).exists(),
        // No source file and no dependency manifest is an app not written yet, which is when a
        // decision is a plan. A source file in a language `sv` cannot read is code too (the review
        // of 6 October, item 9).
        scan_report.files_read > 0
            || !scan_report.declared.is_empty()
            || !scan_report.unread_extensions.is_empty(),
    );
    findings.extend(
        design
            .findings
            .iter()
            .cloned()
            .map(|f| at_the_manifest(f, manifest_file)),
    );
    gaps.extend(planned_gaps(&design.planned));
    if !design.unreadable.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!(
                "your answer to {} design question{}",
                design.unreadable.len(),
                if design.unreadable.len() == 1 {
                    ""
                } else {
                    "s"
                }
            ),
            why: format!(
                "stackvet.toml answers {} with a word that is not yes, no, not-sure, or planned, or \
                 says it was answered `by` somebody other than \"owner\" or \"ai-tool\", so \
                 nothing could be made of it: {}.",
                if design.unreadable.len() == 1 {
                    "this"
                } else {
                    "these"
                },
                design.unreadable.join(", ")
            ),
            reason: sv_report::GapReason::CouldNotRead,
            requirements: Vec::new(),
        });
    }
    if !design.unanswered.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!(
                "{} question{} about how this app is built",
                design.unanswered.len(),
                if design.unanswered.len() == 1 {
                    ""
                } else {
                    "s"
                }
            ),
            why: format!(
                "No tool can settle these — whether input is validated on the server, whether the \
                 app's own services authenticate to each other. Answer them in the `[design]` \
                 section of stackvet.toml: {}. Your AI coding tool can ask you them: `sv \
                 questions` prints them for its chat.",
                design.unanswered.join(", ")
            ),
            reason: sv_report::GapReason::PersonOnly,
            requirements: Vec::new(),
        });
    }

    // The checks made by hand, recorded in stackvet.toml. The owner's `done` is their word about
    // what they saw; `problem` is a finding; an old one is out of date and counts for nothing.
    let hand_answers: std::collections::BTreeMap<String, sv_check::hand::Answer> = manifest
        .checked_by_hand
        .iter()
        .map(|(id, a)| {
            (
                id.clone(),
                sv_check::hand::Answer {
                    result: a.result.clone(),
                    on: a.on.clone(),
                    by: a.by.clone(),
                    how: a.how.clone(),
                    recorded: sv_check::seal::owner_recorded(
                        seals,
                        a.seal.as_deref(),
                        &sv_check::seal::hand_check_fields(id, a),
                    ),
                },
            )
        })
        .collect();
    let mut hand = match sv_check::advisories::Day::today() {
        Some(today) => sv_check::hand::evaluate(
            &human_checks,
            &hand_answers,
            &|id| buckets.applicable.iter().any(|a| a == id),
            today,
        ),
        // A clock before 1970 cannot say whether a check is current, so none is counted.
        None => sv_check::hand::Outcome::default(),
    };
    findings.extend(
        hand.findings
            .iter()
            .cloned()
            .map(|f| at_the_manifest(f, manifest_file)),
    );
    if !hand.unreadable.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!(
                "{} check{} made by hand",
                hand.unreadable.len(),
                if hand.unreadable.len() == 1 { "" } else { "s" }
            ),
            why: format!(
                "stackvet.toml records {} in `[checked-by-hand]` in a way nothing could be made \
                 of, so it counts for nothing: {}.",
                if hand.unreadable.len() == 1 {
                    "this"
                } else {
                    "these"
                },
                hand.unreadable
                    .iter()
                    .map(|(id, why)| format!("{id} ({why})"))
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            reason: sv_report::GapReason::CouldNotRead,
            requirements: Vec::new(),
        });
    }
    if !hand.out_of_date.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!(
                "{} check{} made by hand more than {} days ago",
                hand.out_of_date.len(),
                if hand.out_of_date.len() == 1 { "" } else { "s" },
                sv_check::hand::CURRENT_FOR_DAYS
            ),
            why: format!(
                "Certificates expire and apps change, so an old check counts for nothing. Make \
                 {} again and record the new date: {}.",
                if hand.out_of_date.len() == 1 {
                    "it"
                } else {
                    "them"
                },
                hand.out_of_date
                    .iter()
                    .map(|(id, on)| format!("{id}, checked on {on}"))
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            reason: sv_report::GapReason::PersonOnly,
            requirements: Vec::new(),
        });
    }
    // A person confirming what the AI tool said moves it up to their tier, shown as confirmed. What
    // does not hold stays the tool's word and is named with its reason. See `sv_check::confirm`.
    let confirmation = |c: &sv_manifest::Confirmed| sv_check::confirm::Confirmation {
        by: c.by.clone(),
        on: c.on.clone(),
        how: c.how.clone(),
        answer: c.answer.clone(),
        location: c.r#where.clone(),
        result: c.result.clone(),
        seal: c.seal.clone(),
    };
    let design_confirmations: std::collections::BTreeMap<
        String,
        (sv_check::confirm::Confirmation, String, Option<String>),
    > = manifest
        .design
        .iter()
        .filter_map(|(id, a)| {
            let c = a.confirmed.as_ref()?;
            Some((
                id.clone(),
                (confirmation(c), a.answer.clone(), a.r#where.clone()),
            ))
        })
        .collect();
    let hand_confirmations: std::collections::BTreeMap<
        String,
        (sv_check::confirm::Confirmation, String),
    > = manifest
        .checked_by_hand
        .iter()
        .filter_map(|(id, a)| {
            let c = a.confirmed.as_ref()?;
            Some((id.clone(), (confirmation(c), a.result.clone())))
        })
        .collect();
    let modified = |path: &str| {
        std::fs::metadata(app_dir.join(path))
            .and_then(|m| m.modified())
            .ok()
            .and_then(sv_check::advisories::Day::of)
    };
    let (confirmed_design, confirmed_hand) = match sv_check::advisories::Day::today() {
        Some(today) => (
            sv_check::confirm::apply(
                &mut design.stated,
                sv_check::confirm::DESIGN_CONFIRMED,
                &|id| {
                    let (c, answer, location) = design_confirmations.get(id)?;
                    Some((
                        c,
                        sv_check::confirm::Current::Design {
                            answer,
                            location: location.as_deref(),
                            modified: location.as_deref().and_then(modified),
                        },
                    ))
                },
                today,
                seals,
            ),
            sv_check::confirm::apply(
                &mut hand.stated,
                sv_check::confirm::HAND_CONFIRMED,
                &|id| {
                    let (c, result) = hand_confirmations.get(id)?;
                    Some((c, sv_check::confirm::Current::Hand { result }))
                },
                today,
                seals,
            ),
        ),
        None => Default::default(),
    };
    if let Some(why) = manifest.level_from_unanswered_data() {
        gaps.push(sv_report::Gap {
            what: "What information the app holds about people".to_owned(),
            why: why.to_owned(),
            reason: sv_report::GapReason::PersonOnly,
            requirements: Vec::new(),
        });
    }
    if let Some(why) = manifest.level_from_unknown_data() {
        gaps.push(sv_report::Gap {
            what: "A kind of information the app holds that `sv` does not know".to_owned(),
            why,
            reason: sv_report::GapReason::NoReader,
            requirements: Vec::new(),
        });
    }
    let not_counted: Vec<&(String, String)> = confirmed_design
        .not_counted
        .iter()
        .chain(confirmed_hand.not_counted.iter())
        .collect();
    if !not_counted.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!(
                "{} confirmation{} of what your AI coding tool said",
                not_counted.len(),
                if not_counted.len() == 1 { "" } else { "s" }
            ),
            why: format!(
                "stackvet.toml records {} in a way that does not count, so the tool's word is \
                 all there is and the question is asked again: {}.",
                if not_counted.len() == 1 {
                    "this confirmation"
                } else {
                    "these confirmations"
                },
                not_counted
                    .iter()
                    .map(|(id, why)| format!("{id} ({why})"))
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            reason: sv_report::GapReason::CouldNotRead,
            requirements: Vec::new(),
        });
    }
    let attested: Vec<sv_check::Verified> = design
        .attested
        .iter()
        .chain(confirmed_design.confirmed.iter())
        .cloned()
        .collect();
    let by_hand: Vec<sv_check::Verified> = hand
        .by_owner
        .iter()
        .chain(confirmed_hand.confirmed.iter())
        .cloned()
        .collect();
    let stated: Vec<sv_check::Verified> = design
        .stated
        .iter()
        .chain(hand.stated.iter())
        .chain(notes.stated.iter())
        .chain(decisions.stated.iter())
        .cloned()
        .collect();
    // A person's word goes to the report in the one list with the checks, each credit saying
    // which tier it rests on (`sv_check::Tier`, set where it was made); the report reads the tier,
    // not the list (8 October 2026).
    let mut verified = documented;
    verified.extend(attested);
    verified.extend(by_hand);
    verified.extend(stated);
    Ok(PersonsWord {
        verified,
        notes_catalog,
        design_questions,
        human_checks,
        decisions_text,
        notes_sha256,
    })
}

/// The Appendix C requirements the coding rules given to this app come from, for the report's
/// section on how the app is built with AI. The same rules `sv rules` would write.
fn coding_rules_cited(
    buckets: &sv_frameworks::applicability::Buckets,
) -> Result<std::collections::BTreeSet<String>> {
    // The Appendix C requirements the coding rules given to this app come from, for the report's
    // section on how the app is built with AI. The same rules `sv rules` would write.
    let coding_rules = sv_check::coding_rules::CodingRules::load(&coding_rules_path())?;
    let set_aside: std::collections::BTreeSet<&str> = buckets
        .not_applicable
        .iter()
        .map(|n| n.id.as_str())
        .collect();
    let is_set_aside = |id: &str| set_aside.contains(id);
    let coding_rules_cited: std::collections::BTreeSet<String> = coding_rules
        .for_app(Some(&is_set_aside))
        .into_iter()
        .flat_map(|r| r.cites.keys().cloned())
        .collect();
    Ok(coding_rules_cited)
}

/// Everything the stages gathered, for the last one.
pub struct Gathered {
    pub findings: Vec<sv_check::Finding>,
    pub verified: Vec<sv_check::Verified>,
    pub gaps: Vec<sv_report::Gap>,
    pub examined: Vec<sv_report::Examined>,
    pub manual_only: std::collections::BTreeSet<String>,
    pub named_in_tests: std::collections::BTreeSet<String>,
    pub not_for_tests: std::collections::BTreeSet<String>,
    pub coding_rules_cited: std::collections::BTreeSet<String>,
    pub run: RunningApp,
    pub word: PersonsWord,
    pub run_record: sv_report::RunRecord,
}

/// The last stage: what was examined completed, what a person set aside applied, the safe defaults
/// held to the running app, and the report built.
fn put_together(scene: &Scene, gathered: Gathered) -> Result<sv_report::Report> {
    let Scene {
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        static_scan,
        ctx,
        resolved,
        buckets,
        seals,
    } = *scene;
    let static_scan::StaticScan {
        crashed: _,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
    } = static_scan;
    let Loaded {
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
        ..
    } = loaded;
    let _ = (
        app_dir,
        options,
        loaded,
        manifest,
        manifest_file,
        ctx,
        resolved,
        buckets,
        seals,
        listing,
        scan_report,
        bill_of_materials,
        secrets,
        config,
        code,
        frameworks,
        config_rules,
        threat_rules,
        secret_rules,
        adapters,
    );

    let Gathered {
        findings,
        verified,
        mut gaps,
        mut examined,
        manual_only,
        named_in_tests,
        not_for_tests,
        coding_rules_cited,
        run,
        word,
        run_record,
    } = gathered;
    // The app's own image by its digest, read before the run's status is taken apart below (backlog 0238): the local
    // Docker only, and only when the app was run.
    let app_image_digest = match &run.status {
        sv_report::RunStatus::Started { image, .. } => sv_run::docker::image_digest(image),
        _ => None,
    };
    let RunningApp {
        status: run_status,
        note: run_note,
        steps: run_steps,
        test_output,
        tests_examined,
        seen,
        token_fields,
        ..
    } = run;
    let PersonsWord {
        notes_catalog,
        design_questions,
        human_checks,
        decisions_text,
        notes_sha256,
        ..
    } = word;
    examined.push(match &run_status {
        // Started is still only part of what the app could be asked: what sits behind a sign-in
        // it could not reach, and the requirements no question reaches, are in the gaps.
        sv_report::RunStatus::Started { .. } => sv_report::Examined::partly(
            "probe.",
            "the running app was asked what `sv` knows to ask; the gaps say what that could not reach",
        ),
        sv_report::RunStatus::NotAsked { why } | sv_report::RunStatus::CouldNotStart { why } => {
            sv_report::Examined::not_run("probe.", why.clone())
        }
    });
    examined.push(tests_examined);
    // The owner's answers in stackvet.toml are read on every run.
    examined.push(sv_report::Examined::ran("design."));
    examined.push(sv_report::Examined::ran("hand."));
    // The "Safe defaults" section's three switches (`sv_check::decisions`), each held to the check
    // of the running app that sees it. Made before what a person set aside is applied, so a review
    // of one of these findings is applied too (the review of 6 October, item 7); one whose running-app
    // finding a person set aside is dropped after.
    let safe_defaults = decisions_text
        .as_deref()
        .map(sv_check::decisions::safe_defaults)
        .unwrap_or_default();
    let app_ran = matches!(run_status, sv_report::RunStatus::Started { .. });
    // The decisions file is read whenever it is there; its safe defaults only with the app running.
    examined.push(match (&decisions_text, app_ran) {
        (None, _) => sv_report::Examined::not_run(
            "decisions.",
            format!("there is no {} beside the app", sv_check::decisions::FILE),
        ),
        (Some(_), true) => sv_report::Examined::ran("decisions."),
        (Some(_), false) => sv_report::Examined::partly(
            "decisions.",
            "its safe defaults are held to checks of the running app, which was not run",
        ),
    });
    // What a person set aside, matched by the fingerprint the report prints beside each finding.
    // An entry that matches nothing says whether its rule looked this time (deep review R3), so
    // `examined` is complete before this.
    // Merged, marked, the decisions and then the reviews applied, one per line: as `sv check`
    // counts them too (`static_scan::settle`).
    // The folders the owner left out, and which of them count as left out here (ADR-099).
    let left_out = crate::left_out::LeftOut::of(manifest, seals);
    let used = &static_scan.scan_report.not_the_app;
    let reviewed = static_scan::settle(
        app_dir,
        &manifest.finding_review,
        findings,
        static_scan,
        &examined,
        loaded,
        seals,
        &safe_defaults.decided,
        &left_out.counted_out(used),
    );
    let files: Vec<&str> = (static_scan.listing.app_files())
        .map(|f| f.relative.as_str())
        .collect();
    gaps.extend(left_out.gaps(used, &files, &reviewed.left_out));
    // Rules that fired in test code and are information, not findings (ADR-091): kept apart, never counted.
    let (findings, test_information) =
        split_test_information(reviewed.findings, &reviewed.counted_in_test);
    if !safe_defaults.unreadable.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!(
                "{} safe default{} in {}",
                safe_defaults.unreadable.len(),
                if safe_defaults.unreadable.len() == 1 {
                    ""
                } else {
                    "s"
                },
                sv_check::decisions::FILE
            ),
            why: format!(
                "Under \"{}\", each of these switches is decided as one of two values, and these \
                 lines say something else, so nothing was made of them: {}. Write `{}`.",
                sv_check::decisions::SAFE_DEFAULTS,
                safe_defaults.unreadable.join("; "),
                sv_check::decisions::SWITCHES
                    .iter()
                    .map(|s| format!("- {}: {}", s.name, s.safe))
                    .collect::<Vec<_>>()
                    .join("`, `")
            ),
            reason: sv_report::GapReason::CouldNotRead,
            requirements: Vec::new(),
        });
    }
    if !safe_defaults.missing.is_empty() {
        gaps.push(sv_report::Gap {
            what: format!(
                "{} safe default{} not found in {}",
                safe_defaults.missing.len(),
                if safe_defaults.missing.len() == 1 {
                    ""
                } else {
                    "s"
                },
                sv_check::decisions::FILE
            ),
            why: format!(
                "The \"{}\" section has no line deciding {}, so nothing was held to the running \
                 app for it. Write `{}`.",
                sv_check::decisions::SAFE_DEFAULTS,
                safe_defaults.missing.join(", "),
                sv_check::decisions::SWITCHES
                    .iter()
                    .filter(|s| safe_defaults.missing.contains(&s.name))
                    .map(|s| format!("- {}: {}", s.name, s.safe))
                    .collect::<Vec<_>>()
                    .join("`, `")
            ),
            reason: sv_report::GapReason::PersonOnly,
            requirements: Vec::new(),
        });
    }
    let held_safe = safe_defaults.decided.iter().filter(|d| d.safe).count();
    if held_safe > 0 && !app_ran {
        gaps.push(sv_report::Gap {
            what: format!(
                "{held_safe} safe default{} decided in {}",
                if held_safe == 1 { "" } else { "s" },
                sv_check::decisions::FILE
            ),
            why: "Each is held to a check of the running app, and the app was not run, so whether \
                  the app does what was decided was not looked at. `sv report --run` runs it."
                .to_owned(),
            reason: sv_report::GapReason::NotAsked,
            requirements: Vec::new(),
        });
    }
    // Why the app is held to its level, on whose word, and what level 2 would add (the gap
    // analysis of 7 October 2026, finding 17; ADR-024, Later, 9 October 2026).
    let level_why = sv_report::LevelWhy {
        because: manifest.level_because(),
        level_two_more: buckets
            .out_of_level
            .iter()
            .filter(|id| frameworks.get(id.as_str()).is_some_and(|r| r.level == 2))
            .count(),
        // A hints file that cannot be read is a gap in the report, not a line on stderr that the MCP
        // server's caller never sees (backlog 226, part 1, item 6).
        hints: level_hints(manifest, listing, &scan_report.set_apart).unwrap_or_else(|gap| {
            gaps.push(gap);
            Vec::new()
        }),
        confirmed: scope_confirmed(manifest, seals),
    };
    let mut report = sv_report::build(sv_report::Inputs {
        on_the_internet: manifest.app.deployment == sv_manifest::Deployment::Internet,
        ai_tool: sv_check::ai_tool::read(listing, loaded.secret_rules),
        look_away: sv_check::look_away::read(listing, loaded.secret_rules),
        app_name: if manifest.app.name.is_empty() {
            "This app"
        } else {
            &manifest.app.name
        },
        target_level: manifest.target_level(),
        // The run's own start time and id, on every page (backlog 226, part 2, item 12): the
        // clock was read once, when the run began, so the pages agree with `run_record`.
        generated: Some(run_record.describe()),
        made_by: sv_report::MadeBy {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            commit: env!("SV_GIT_COMMIT").to_owned(),
            uncommitted_changes: option_env!("SV_GIT_DIRTY").is_some(),
        },
        run_note,
        run_steps,
        test_output,
        run_status: Some(run_status),
        coding_rules_cited,
        frameworks,
        buckets,
        claims: resolved,
        findings,
        test_information,
        set_aside: reviewed.set_aside,
        reviews_not_counted: reviewed.not_counted,
        verified: &verified,
        gaps,
        manual_only,
        named_in_tests,
        not_for_tests,
        human: Some((&notes_catalog, &design_questions, &human_checks)),
        threats: Some((threat_rules, ctx)),
    });
    report.examined = examined;
    report.level_why = Some(level_why);
    // What kind of run this was, by what did not run (gap analysis 6.1).
    let reach_path = sv_frameworks::data::file("reach.json");
    let reach = std::fs::read_to_string(&reach_path)
        .map_err(anyhow::Error::from)
        .and_then(|text| sv_report::read_reach(&text))
        .with_context(|| format!("reading {}", reach_path.display()))?;
    report.not_run_this_time =
        sv_report::not_run_this_time(&report, &reach, options.run_tools, &scan_report.languages);
    // A plain-language line beside each requirement that has one (backlog 0372).
    let plain: sv_report::PlainRequirements =
        sv_frameworks::data::read_json(&sv_frameworks::data::file("plain-requirements.json"))?;
    report.plain = plain.plain;
    // The paper report.pdf is laid out on (ADR-116).
    report.paper = sv_report::pdf::Paper::named(manifest.app.report_paper.as_deref());
    // What could settle each row left not verified (backlog 0426).
    report.could_settle =
        sv_report::could_settle(&report, &reach, options.run_tools, &scan_report.languages);
    let file_gaps = static_scan.file_gaps();
    report.could_not_run = file_gaps.could_not_run;
    report.partly_read = file_gaps.partly;
    // What else this run read that can move a requirement, so a later run can say why it differs
    // (ADR-083, decision 3).
    let mut run_record = run_record;
    run_record.inputs = Some(sv_report::RunInputs {
        security_notes_sha256: notes_sha256.clone(),
        design_decisions_sha256: decisions_text
            .as_deref()
            .map(|text| crate::bundle::sha256(text.as_bytes())),
        sv_data_sha256: report_lock::data_sha256(),
        sv_data_files: report_lock::data_files_sha256()
            .into_iter()
            .map(|(file, sha256)| sv_report::DataFileHash { file, sha256 })
            .collect(),
        // The helper images by name and digest, and the app's own image by its digest, so the report says what ran
        // (backlog 0238). The app's digest is read from the local Docker only, and only when the app was run.
        helper_images: sv_run::docker::HELPER_IMAGES
            .iter()
            .map(|image| (*image).to_owned())
            .collect(),
        app_image_digest: app_image_digest.clone(),
    });
    report.run_record = Some(run_record);
    report.seen = seen;
    // The bill of materials, written beside the report as `sv sbom` prints it (backlog 0382).
    report.sbom = Some(sv_report::SbomFile {
        // With what the app runs on beside its packages (backlog 0402): the app, its image, and its language with the
        // modules of its own library the code imports.
        json: serde_json::to_string_pretty(&sbom::to_cyclonedx_with(
            bill_of_materials,
            &sbom::runs_on(
                listing,
                &manifest.app.name,
                manifest.stack.run.image.as_deref(),
                app_image_digest.as_deref(),
            ),
        ))? + "\n",
        components: bill_of_materials.components.len(),
        complete: bill_of_materials.is_complete(),
        unread_lists: bill_of_materials.unread.len(),
        at_asked_versions: bill_of_materials.declared_count(),
    });
    // The lines each code finding is on, for "The code" under it (backlog 0378, part 2).
    report.code_lines = crate::code_lines::read(app_dir, &report.findings, secret_rules);
    let locators = &loaded.locators;
    // The lines each question to the owner is about, where a locator speaks to it (backlog 0487).
    crate::code_lines::point_questions(
        app_dir,
        locators,
        &locators.find(listing),
        secret_rules,
        &mut report,
    );
    // The packages each question is about, where a package locator speaks to it (backlog 0490).
    crate::code_lines::point_to_packages(locators, &bill_of_materials.components, &mut report);
    // The app's routes and what is written on each, beside the authorization questions (backlog 0493).
    crate::code_lines::route_questions(listing, &mut report);
    // The ways into the app the code shows, beside the sign-in pathway questions (backlog 0491).
    crate::code_lines::pathway_questions(listing, &mut report);
    // The fields that look like personal data, beside the data-classification questions (backlog 0497).
    crate::code_lines::classification_questions(listing, token_fields, &mut report);
    // Each tool's own report, kept only when asked (backlog 0229, part 4), and otherwise dropped
    // here, so nothing past this point holds the app's code twice.
    if options.keep_tool_output {
        let seen = report.seen.get_or_insert_with(Default::default);
        crate::seen::tool_output(secret_rules, &report.examined, seen);
    }
    for e in &mut report.examined {
        if let Some(tool) = e.tool.as_mut() {
            tool.output_kept = options.keep_tool_output && tool.output.is_some();
            tool.output = None;
        }
    }
    report.manifest_file = manifest_file.to_owned();
    // A contradiction says what in the code contradicted the manifest, so whoever wrote the
    // manifest can see what to correct. "The code says otherwise" alone left the AI coding tool that
    // wrote it with nothing to go on; `sv scope` always said, and now the report does too.
    for claim in report
        .claims
        .iter_mut()
        .filter(|c| c.state == "contradicted")
    {
        if let Some(answer) = scan_report
            .answers
            .iter()
            .find(|a| a.condition.name() == claim.name && a.value == Some(true))
        {
            claim.note = format!(
                "{} What the code shows: {}.",
                claim.note,
                describe(&answer.evidence)
            );
        }
    }
    Ok(report)
}

/// A wrong-password finding says when the start command sets something that looks like a sign-in limit (backlog 0386):
/// in the second loop trial, seven of nine such findings were about a limit raised for the copy `sv` ran alone.
pub(crate) fn limits_set_for_the_run(findings: &mut [sv_check::Finding], start: &str) {
    let Some(said) = sv_run::start_settings::sign_in_limit_note(start) else {
        return;
    };
    for finding in findings
        .iter_mut()
        .filter(|f| f.rule_id == "probe.failed-sign-ins-unlimited")
    {
        finding.description.push(' ');
        finding.description.push_str(&said);
    }
}

/// A setting the start command loosens the copy with earns that copy no credit for the rules it bears on (backlog 0564,
/// ADR-114, Later): what the run saw there is not what the app does as shipped. Each such credit is taken out and said
/// as a gap naming the setting; each finding under one of those rules says it is about the loosened copy. Settings that
/// bear on no rule are only warned about, as before (`sv_run::weakening_note`).
pub(crate) fn loosened_for_the_run(
    findings: &mut [sv_check::Finding],
    verified: &mut Vec<sv_check::Verified>,
    start: &str,
) -> Vec<sv_report::Gap> {
    loosened_in(findings, verified, start, None)
}

/// `loosened_for_the_run` for the settings an environment file sets (backlog 0605, ADR-114, Later): `file` is the one
/// the start command reads them from, as it names it, and `settings` what `env_file_settings` read from it.
pub(crate) fn loosened_in_env_file(
    findings: &mut [sv_check::Finding],
    verified: &mut Vec<sv_check::Verified>,
    settings: &str,
    file: &str,
) -> Vec<sv_report::Gap> {
    loosened_in(findings, verified, settings, Some(file))
}

/// The settings in `settings` that loosen the copy, judged as 0564 does: from the start command when `file` is `None`,
/// otherwise from the environment file `file` the start command reads.
fn loosened_in(
    findings: &mut [sv_check::Finding],
    verified: &mut Vec<sv_check::Verified>,
    settings: &str,
    file: Option<&str>,
) -> Vec<sv_report::Gap> {
    let mut gaps = Vec::new();
    for (setting, rules) in sv_run::start_settings::loosened(settings) {
        let (withheld, kept): (Vec<_>, Vec<_>) = std::mem::take(verified)
            .into_iter()
            .partition(|v| rules.contains(&v.check_id.as_str()));
        *verified = kept;
        for credit in withheld {
            gaps.push(sv_report::Gap {
                what: format!(
                    "{}, by running the app ({})",
                    credit.requirement_ids.join(", "),
                    credit.check_id
                ),
                why: match file {
                    None => format!(
                        "The copy `sv` ran had `{setting}` set in its start command in stackvet.toml, so what the run \
                         saw here is not what the app does as it is shipped, and it is not credited. Take the setting \
                         out of the start command, or set it as the shipped app has it, and run again."
                    ),
                    // The file may be the one the app ships with, or one for running it here: either way the copy had
                    // the setting, and the credit is not the shipped app's to claim unless the setting is taken out.
                    Some(file) => format!(
                        "The copy `sv` ran had `{setting}` set in `{file}`, the environment file its start command \
                         reads, so what the run saw here is of a copy with that setting, and it is not credited. If \
                         the app ships with that file, the setting is loosened there too; if not, take it out of the \
                         file the run reads, or set it as the shipped app has it, and run again."
                    ),
                },
                reason: sv_report::GapReason::LeftOut,
                requirements: credit.requirement_ids.clone(),
            });
        }
        for finding in findings
            .iter_mut()
            .filter(|f| rules.contains(&f.rule_id.as_str()))
        {
            finding.description.push_str(&match file {
                None => format!(
                    " The copy `sv` ran had `{setting}` set in its start command, so this is about that copy, and \
                     the app as it is shipped may not have it."
                ),
                Some(file) => format!(
                    " The copy `sv` ran had `{setting}` set in `{file}`, the environment file its start command reads, \
                     so this is about a copy with that setting; the app as it is shipped has it too only if it ships \
                     with that file."
                ),
            });
        }
    }
    gaps
}

/// Every setting that loosens the copy `sv` ran, from its start command and from the environment files the start
/// command reads (backlog 0564, 0605; ADR-114, Later), each judged as `loosened_for_the_run` judges: the gaps, and the
/// run note's sentence saying which settings files were read and that no other is.
pub(crate) fn loosened_for_the_copy(
    findings: &mut [sv_check::Finding],
    verified: &mut Vec<sv_check::Verified>,
    app_dir: &std::path::Path,
    start: &str,
) -> (Vec<sv_report::Gap>, String) {
    let mut gaps = loosened_for_the_run(findings, verified, start);
    let (read, not_read) = env_files_for_the_run(app_dir, start);
    for (file, settings) in &read {
        gaps.extend(loosened_in_env_file(findings, verified, settings, file));
    }
    (gaps, settings_files_sentence(&read, &not_read))
}

/// The most bytes of an environment file read: a settings file is short.
const ENV_FILE_BYTES: u64 = 1_000_000;

/// The environment files the start command itself reads, each read from the app's folder (mounted at `/app`, where the
/// start command runs) and for its format first (backlog 0605, ADR-114, Later): the ones read, as named with their
/// settings, and for each one not read the sentence why. A file outside the app's folder, missing, too large, or not
/// in the `NAME=value` form is not judged.
pub(crate) fn env_files_for_the_run(
    app_dir: &std::path::Path,
    start: &str,
) -> (Vec<(String, String)>, Vec<String>) {
    use std::path::Component;
    use sv_frameworks::paths::Canonical;
    let mut read = Vec::new();
    let mut not_read = Vec::new();
    for named in sv_run::start_settings::env_files_named_in(start) {
        let relative = named
            .strip_prefix("/app/")
            .unwrap_or(&named)
            .trim_start_matches("./");
        let path = std::path::Path::new(relative);
        let inside = !relative.is_empty()
            && path
                .components()
                .all(|c| matches!(c, Component::Normal(_) | Component::CurDir));
        let full = app_dir.join(path);
        let real_inside = inside
            && match (full.canonical(), app_dir.canonical()) {
                (Ok(file), Ok(root)) => file.starts_with(root),
                _ => false,
            };
        let why = if !inside || (full.exists() && !real_inside) {
            Err("it is outside the app's folder".to_owned())
        } else {
            match std::fs::metadata(&full) {
                Err(_) => Err("it is not in the app's folder".to_owned()),
                Ok(meta) if !meta.is_file() || meta.len() > ENV_FILE_BYTES => {
                    Err("it is not a settings file of a size `sv` reads".to_owned())
                }
                Ok(_) => match std::fs::read_to_string(&full) {
                    Err(_) => Err("it is not text".to_owned()),
                    Ok(text) => sv_run::start_settings::env_file_settings(&text)
                        .map_err(|line| format!("{line}, so its form is not one `sv` reads")),
                },
            }
        };
        match why {
            Ok(settings) => read.push((named, settings)),
            Err(why) => not_read.push(format!(
                "The start command reads `{named}`, which was not read for settings that loosen the copy: {why}."
            )),
        }
    }
    (read, not_read)
}

/// What the report says of the settings files `sv` reads for the run (backlog 0605, the owner's choice (b) of 15
/// October 2026): the environment files the start command names, and no other.
pub(crate) fn settings_files_sentence(read: &[(String, String)], not_read: &[String]) -> String {
    let mut out = match read {
        [] => "Settings the start command sets were read for any that loosen the copy; a settings file the app reads \
               on its own is not read, so a setting loosened in one is not seen."
            .to_owned(),
        files => format!(
            "Settings the start command sets, and the environment file{} it reads ({}), were read for any that loosen \
             the copy; any other settings file the app reads is not read, so a setting loosened in one is not seen.",
            if files.len() == 1 { "" } else { "s" },
            files
                .iter()
                .map(|(f, _)| format!("`{f}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    for why in not_read {
        out.push(' ');
        out.push_str(why);
    }
    out
}

/// The answers that came with less body than their `Content-Length` said, said once (backlog 0598, ADR-021): each was
/// read as no answer, so what the checks would have read from those pages is not assessed, and a page that came blank is
/// never a finding about what it lacks. The app's own log line on it, redacted, beside them.
pub(crate) fn short_answers_gap(
    rules: &sv_check::secrets::SecretRules,
    short: &[String],
    log: Option<&str>,
) -> Option<sv_report::Gap> {
    if short.is_empty() {
        return None;
    }
    let logged = match log {
        Some(line) => {
            let (line, _) = sv_check::secrets::redact_text(rules, line);
            format!(" The app's own log says: `{}`.", line.replace('`', "'"))
        }
        None => String::new(),
    };
    Some(sv_report::Gap {
        what: format!(
            "{} answer{} the app sent with less body than {} said",
            short.len(),
            if short.len() == 1 { "" } else { "s" },
            if short.len() == 1 { "it" } else { "they" },
        ),
        why: format!(
            "The app began {} with a status and a `Content-Length`, then sent less than that: {}. A browser shows such \
             a page blank, so each was read as no answer, and what the checks would have read from it is not \
             assessed.{logged} Fix the error, and run again.",
            if short.len() == 1 {
                "this answer"
            } else {
                "these answers"
            },
            short.join(", ")
        ),
        reason: sv_report::GapReason::Partial,
        requirements: Vec::new(),
    })
}

/// A finding the manifest's answers made, pointed at the manifest by the name it has in this app.
fn at_the_manifest(mut finding: sv_check::Finding, manifest_file: &str) -> sv_check::Finding {
    if finding.location.file == sv_frameworks::names::MANIFEST {
        finding.location.file = manifest_file.to_owned();
    }
    finding
}

/// At level 1, what the app's own code shows that the answers setting the level do not (the gap
/// analysis of 7 October 2026, finding 17; ADR-024, Later, 9 October 2026). Nothing at level 2,
/// where there is nothing more to ask; and nothing, said on standard error, when the list of
/// names cannot be read, since a question missing is not a pass.
/// The hints file read, or the gap that says the code was not compared with the answers that set
/// the level, and why.
fn level_hints_from(path: &Path) -> Result<sv_check::level_hints::Hints, sv_report::Gap> {
    sv_check::level_hints::Hints::load(path).map_err(|why| sv_report::Gap {
        what: "whether the code agrees with the answers that set the level".to_owned(),
        why: format!(
            "sv's file of what to look for ({}) could not be read: {why}. So nothing here says \
             whether the code shows a level 1 app needs more, which is not the same as saying it \
             does not. Reinstall sv, or point SV_DATA_DIR at a complete copy of its data.",
            path.display()
        ),
        reason: sv_report::GapReason::CouldNotRead,
        requirements: Vec::new(),
    })
}

fn level_hints(
    manifest: &sv_manifest::Manifest,
    listing: &sv_scan::files::Listing,
    set_apart: &std::collections::BTreeSet<String>,
) -> Result<Vec<sv_check::level_hints::Hint>, sv_report::Gap> {
    if manifest.target_level() != 1 {
        return Ok(Vec::new());
    }
    let hints = level_hints_from(&sv_frameworks::data::file("level-hints.json"))?;
    let private_audience = matches!(
        manifest.app.audience,
        sv_manifest::Audience::JustMe | sv_manifest::Audience::MyTeam
    );
    Ok(sv_check::level_hints::find(
        listing,
        &hints,
        &sv_check::level_hints::Answers {
            private_audience,
            listed: manifest.data.listed(),
        },
        set_apart,
    ))
}

/// V7.5.1 met whole, or not (ADR-105): whole only when the email change and each of the phone,
/// two-factor, and recovery changes was refused with a wrong password where the right one took, or
/// is confirmed through `sv review` as not offered. Until then every credit stays in part. A
/// `not-offered` list nobody confirmed is said as a gap, since it counts for nothing until then.
pub fn account_changes_whole(
    manifest: &Manifest,
    seals: &sv_check::seal::Checker,
    verified: &mut [sv_check::Verified],
    gaps: &mut Vec<sv_report::Gap>,
) {
    let Some(users) = manifest.stack.run.users.as_ref() else {
        return;
    };
    let confirmed = manifest
        .account_changes_review
        .as_ref()
        .is_some_and(|entry| {
            let fields = sv_check::seal::account_changes_review_fields(entry);
            entry.still_holds_for(manifest)
                && seals
                    .recorded(entry.seal.as_deref(), &sv_check::seal::as_strs(&fields))
                    .is_ok()
        });
    if !users.not_offered.is_empty() && !confirmed {
        gaps.push(sv_report::Gap {
            what: "Account changes listed as not offered, not confirmed by the owner".to_owned(),
            why: format!(
                "stackvet.toml lists {} under `[stack.run.users]` `not-offered`, and nobody has \
                 confirmed it through `sv review`, so it is the AI coding tool's word or a file \
                 edited by hand, and counts for nothing. Until the owner confirms it, V7.5.1 is at \
                 most checked in part: run `sv review` and confirm the list if it is right.",
                users
                    .not_offered
                    .iter()
                    .map(|n| format!("`{n}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            reason: sv_report::GapReason::PersonOnly,
            requirements: vec!["V7.5.1".to_owned()],
        });
    }
    let credited =
        |verified: &[sv_check::Verified], rule: &str| verified.iter().any(|v| v.check_id == rule);
    if !credited(verified, "probe.email-change-without-password") {
        return;
    }
    let mut on_the_owners_word = Vec::new();
    for (kind, _) in sv_manifest::ACCOUNT_CHANGES {
        if credited(verified, &format!("probe.{kind}-change-without-password")) {
            continue;
        }
        if confirmed && users.not_offered.iter().any(|n| n == kind) {
            on_the_owners_word.push(kind);
            continue;
        }
        return;
    }
    for credit in verified
        .iter_mut()
        .filter(|v| v.requirement_ids.iter().any(|r| r == "V7.5.1"))
    {
        credit.whole_after_all();
        if !on_the_owners_word.is_empty() {
            credit.scope = format!(
                "{}; with {} not offered, on the owner's word through `sv review`",
                credit.scope,
                on_the_owners_word.join(" and ")
            );
        }
    }
}

/// Whether the answers that set the level were confirmed through `sv review`, and whether that
/// still holds here (ADR-024, Later, 9 October 2026): the seal must hold on this computer, and
/// the answers must be the ones confirmed. `None` when nobody has confirmed them.
fn scope_confirmed(
    manifest: &sv_manifest::Manifest,
    seals: &sv_check::seal::Checker,
) -> Option<sv_report::ScopeConfirmed> {
    let entry = manifest.scope_review.as_ref()?;
    let fields = sv_check::seal::scope_review_fields(entry);
    Some(
        match seals.recorded(entry.seal.as_deref(), &sv_check::seal::as_strs(&fields)) {
            Err(why) => sv_report::ScopeConfirmed::NotCounted { why: why.why() },
            Ok(_) if !entry.still_holds_for(manifest) => sv_report::ScopeConfirmed::Changed {
                by: entry.by.clone(),
                on: entry.on.clone(),
            },
            Ok(sealed) => sv_report::ScopeConfirmed::Confirmed {
                by: entry.by.clone(),
                on: entry.on.clone(),
                sealed: match sealed {
                    sv_check::seal::Sealed::Here => " on this computer".to_owned(),
                    sv_check::seal::Sealed::Signed { key, from, lock } => {
                        format!(", {}", sv_check::seal::signed_with(&key, from, lock))
                    }
                },
            },
        },
    )
}

#[cfg(test)]
mod timing_tests;

#[cfg(test)]
mod check_timings_tests;

/// Splits the findings into those that count and the test-file information that does not (ADR-091). A
/// test-file information finding that a reviewer's sealed `counts` entry says is real counts (ADR-093).
fn split_test_information(
    findings: Vec<sv_check::Finding>,
    counted_in_test: &[String],
) -> (Vec<sv_check::Finding>, Vec<sv_check::Finding>) {
    findings
        .into_iter()
        .partition(|f| !f.is_test_information() || counted_in_test.contains(&f.fingerprint))
}

#[cfg(test)]
#[path = "assemble_counts_tests.rs"]
mod counts_tests;

#[cfg(test)]
#[path = "assemble_start_limits_tests.rs"]
mod start_limits_tests;

#[cfg(test)]
#[path = "assemble_loosened_tests.rs"]
mod loosened_tests;

#[cfg(test)]
#[path = "assemble_env_file_tests.rs"]
mod env_file_tests;

#[cfg(test)]
#[path = "short_answers_gap_tests.rs"]
mod short_answers_gap_tests;
