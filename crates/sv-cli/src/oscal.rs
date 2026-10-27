//! A report's results as OSCAL assessment results (ADR-156, backlog 0678), for a continuous-authorization pipeline.
//!
//! OSCAL (Open Security Controls Assessment Language) is NIST's machine-readable form for what an assessment observed and
//! concluded; the tools an authorizing official uses read it. This writes one `assessment-results` document from a
//! report `sv` can show it wrote, in OSCAL 1.2.3, the version whose schema the tests hold it to.
//!
//! What it may conclude is ADR-156's, and the format itself enforces the heart of it: an OSCAL finding always concludes
//! `satisfied` or `not-satisfied`, with nothing between. So only two statuses become findings: *needs attention* is
//! `not-satisfied`, and *checked* is `satisfied`, each for the ASVS requirement's own objective. Every other status
//! that rests on something is an observation, with how it was made (`TEST`, `EXAMINE`, or `INTERVIEW`) and whose it is,
//! and no finding; *not verified* is listed as an objective with nothing observed. The NIST SP 800-53 controls from the
//! crosswalk (`data/asvs-800-53-crosswalk.json`) are links of each ASVS objective, `rel="related"`: never reviewed
//! controls, and never given a state, because a control is broader than any one ASVS requirement.
//!
//! The reviewed controls are the ASVS requirements that apply, by their ids made into OSCAL ids (`V6.2.1` is `v6.2.1`).
//! OWASP publishes ASVS 5.0 as JSON and CycloneDX but not as an OSCAL catalog, so the document names ASVS in its back
//! matter and defines each objective in place, with the requirement's own words. `sv` writes no assessment plan;
//! `import-ap` points at a resource saying so.
//!
//! Never written into the report folder: a folder holding anything `sv` does not write there is not a report folder of
//! `sv`'s (H6), and its seal could no longer be shown.

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

/// The OSCAL version written, and the one whose schema the tests hold the document to.
pub const OSCAL_VERSION: &str = "1.2.3";

/// The namespace of the properties `sv` adds, as OSCAL asks for properties outside its own.
const NS: &str = "https://stackvet.dev/ns/oscal";

/// What an export rests on besides the report itself: where the report came from, and what it was of.
#[derive(Debug, Clone, Default)]
pub struct Provenance {
    /// The SHA-256 of `report.json`'s bytes, as its seal was checked against them.
    pub report_sha256: String,
    /// The seal line of the report folder's marker (ADR-034), when it has one.
    pub seal: Option<String>,
    /// The commit of the app the report is of, when it was read at the time, and whether there were changes not
    /// committed.
    pub app_commit: Option<(String, bool)>,
    /// The image `sv` ran in, as whoever ran it named it (`STACKVET_IMAGE`: the GitHub Action names the image whose
    /// signature it checked, by digest).
    pub image: Option<String>,
}

/// How each status of `sv`'s is told in OSCAL: how it was observed, by whom, and whether `sv` concluded anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Told {
    /// `TEST`, `EXAMINE`, or `INTERVIEW`.
    pub method: &'static str,
    /// Whose observation it is.
    pub actor: Actor,
    /// The finding's state and reason, only where `sv` concluded one.
    pub concluded: Option<(&'static str, &'static str)>,
    /// What the status means, said once for every requirement holding it.
    pub means: &'static str,
}

/// Whose an observation is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    /// `sv`'s own checks.
    Sv,
    /// The app's own tests, which the AI coding tool wrote.
    AppTests,
    /// The app's owner, in their own words.
    Owner,
    /// The AI coding tool that wrote the app.
    AiTool,
}

/// How `status`, as `report.json` writes it, is told; `None` for *not verified*, and for a status this `sv` does not
/// know, which is then listed as nothing observed rather than guessed at.
pub fn told(status: &str) -> Option<Told> {
    let t = |method, actor, concluded, means| {
        Some(Told {
            method,
            actor,
            concluded,
            means,
        })
    };
    match status {
        "needs-attention" => t(
            "TEST",
            Actor::Sv,
            Some(("not-satisfied", "fail")),
            "An automated check of sv's found something and named this requirement.",
        ),
        "checked" => t(
            "TEST",
            Actor::Sv,
            Some(("satisfied", "pass")),
            "An automated check of sv's that names this requirement ran and was satisfied. It is one check, not a \
             verification of everything the requirement asks.",
        ),
        "checked-in-part" => t(
            "TEST",
            Actor::Sv,
            None,
            "Checks of sv's that name this requirement ran and were satisfied, and each tried only part of what it \
             asks, so sv concludes nothing about the requirement as a whole.",
        ),
        "app-tested" => t(
            "TEST",
            Actor::AppTests,
            None,
            "The app's own tests name this requirement and passed. The AI coding tool wrote them, and sv does not read \
             whether they ask what the requirement asks, so sv concludes nothing.",
        ),
        "documented" => t(
            "EXAMINE",
            Actor::Owner,
            None,
            "The owner answered this requirement's question in the app's security notes, which sv read. A written \
             answer, not a check of the app, so sv concludes nothing.",
        ),
        "by-hand" => t(
            "INTERVIEW",
            Actor::Owner,
            None,
            "The owner checked this by hand and recorded what they saw. Their word, which sv does not repeat, so sv \
             concludes nothing.",
        ),
        "attested" => t(
            "INTERVIEW",
            Actor::Owner,
            None,
            "The owner answered a design question about this requirement. Their word, not a check, so sv concludes \
             nothing.",
        ),
        "stated" => t(
            "INTERVIEW",
            Actor::AiTool,
            None,
            "The AI coding tool that wrote the app answered a design question about this requirement: the author's own \
             word about its work, not a check, so sv concludes nothing.",
        ),
        _ => None,
    }
}

/// A UUID that is the same for the same report and name, so the same report exported twice gives the same document:
/// the first sixteen bytes of a SHA-256, marked as a name-based (version 5) UUID as OSCAL's pattern requires.
fn uuid(report_sha256: &str, name: &str) -> String {
    let hex = crate::bundle::sha256(format!("{report_sha256}/{name}").as_bytes());
    let mut b: Vec<u8> = (0..16)
        .map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).expect("hex"))
        .collect();
    b[6] = (b[6] & 0x0f) | 0x50;
    b[8] = (b[8] & 0x3f) | 0x80;
    let h: String = b.iter().map(|x| format!("{x:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &h[0..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..32]
    )
}

/// An ASVS id as an OSCAL control id: `V6.2.1` is `v6.2.1`.
fn control_id(asvs: &str) -> String {
    asvs.to_ascii_lowercase()
}

fn prop(name: &str, value: &str) -> Value {
    json!({ "name": name, "ns": NS, "value": value })
}

/// The OSCAL assessment-results document for `report` (a `report.json`), with the 800-53 `crosswalk`.
pub fn assessment_results(
    report: &Value,
    crosswalk: &Value,
    provenance: &Provenance,
) -> Result<Value> {
    let id = |name: &str| uuid(&provenance.report_sha256, name);
    let app = report["app_name"].as_str().unwrap_or("the app");
    let started = report["run_record"]["started"].as_str().context(
        "the report does not say when its run started, which OSCAL needs for every observation",
    )?;
    let sv = &report["sv"];
    let sv_version = sv["version"].as_str().unwrap_or("unknown");
    let sv_commit = sv["commit"].as_str().unwrap_or("unknown");
    let text = &report["requirement_text"];
    let rows: Vec<&Value> = report["requirements"]
        .as_array()
        .context("the report lists no requirements")?
        .iter()
        // ASVS only: the crosswalk is ASVS's. AISVS and Secure by Design stay in report.json (ADR-156, Consequences).
        .filter(|r| {
            r["id"].as_str().is_some_and(|i| {
                i.starts_with('V') && i[1..].starts_with(|c: char| c.is_ascii_digit())
            })
        })
        .collect();
    if rows.is_empty() {
        bail!("the report holds no ASVS requirement that applies, so there is nothing to export");
    }
    let nist_source = crosswalk["catalog"]["source"].as_str().unwrap_or("");

    let mut objectives = Vec::new();
    let mut observations = Vec::new();
    let mut findings = Vec::new();
    let mut counts: std::collections::BTreeMap<&str, usize> = Default::default();
    for row in &rows {
        let rid = row["id"].as_str().expect("filtered on it");
        let status = row["status"].as_str().unwrap_or("not-verified");
        *counts.entry(status).or_default() += 1;
        let cid = control_id(rid);
        let objective_id = format!("{cid}_obj");
        let words = row["description"]
            .as_str()
            .or_else(|| text[rid].as_str())
            .unwrap_or("The requirement's text was not in the report.");
        let related: Vec<Value> = crosswalk["requirements"][rid]
            .as_object()
            .map(|pairs| {
                pairs
                    .iter()
                    .map(|(control, because)| {
                        let c = &crosswalk["controls"][control];
                        json!({
                            "href": format!("{nist_source}#{control}"),
                            "rel": "related",
                            "text": format!(
                                "NIST SP 800-53 {} {}: asks the same thing in part ({}); not assessed as a control",
                                c["label"].as_str().unwrap_or(control),
                                c["title"].as_str().unwrap_or(""),
                                because.as_str().unwrap_or("")
                            ),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut objective = json!({
            "control-id": cid,
            "description": format!("OWASP ASVS 5.0.0 {rid}"),
            "parts": [{ "id": objective_id, "name": "objective", "prose": words }],
            "props": [prop("sv-status", status)],
            "remarks": match told(status) {
                Some(t) => format!("sv's status: {status}. {}", t.means),
                None => format!("sv's status: {status}. Nothing assessed this requirement: not a pass, and not a failure."),
            },
        });
        if !related.is_empty() {
            objective["links"] = Value::Array(related);
        }
        objectives.push(objective);

        let Some(t) = told(status) else { continue };
        let evidence: Vec<String> = [
            "checked_by",
            "tested_by",
            "documented_by",
            "attested_by",
            "by_hand",
            "supported_by",
        ]
        .iter()
        .flat_map(|field| row[*field].as_array().into_iter().flatten())
        .filter_map(|c| c["check_id"].as_str().map(str::to_owned))
        .collect();
        let whose: Vec<&str> = ["attested_by", "by_hand", "documented_by"]
            .iter()
            .flat_map(|field| row[*field].as_array().into_iter().flatten())
            .filter_map(|c| c["whose"].as_str())
            .collect();
        let found = row["findings"].as_array().map_or(0, Vec::len);
        let mut description = t.means.to_owned();
        if !evidence.is_empty() {
            description.push_str(&format!(" What it rests on: {}.", evidence.join(", ")));
        }
        if found > 0 {
            description.push_str(&format!(
                " Findings named against it in the report: {found}."
            ));
        }
        if let Some(who) = whose.first() {
            description.push_str(&format!(" Whose word: {who}."));
        }
        let actor = match t.actor {
            Actor::Sv => json!({ "type": "assessment-platform", "actor-uuid": id("platform/sv") }),
            Actor::AppTests => json!({ "type": "tool", "actor-uuid": id("component/app-tests") }),
            Actor::AiTool => json!({ "type": "tool", "actor-uuid": id("component/ai-tool") }),
            Actor::Owner => json!({ "type": "party", "actor-uuid": id("party/owner") }),
        };
        let observation_uuid = id(&format!("observation/{rid}"));
        observations.push(json!({
            "uuid": observation_uuid,
            "title": format!("{rid}: {status}"),
            "description": description,
            "methods": [t.method],
            "origins": [{ "actors": [actor.clone()] }],
            "collected": started,
        }));
        if let Some((state, reason)) = t.concluded {
            findings.push(json!({
                "uuid": id(&format!("finding/{rid}")),
                "title": format!("{rid}: {status}"),
                "description": format!(
                    "{} sv's conclusion is for this ASVS requirement's own objective only, not for any NIST SP \
                     800-53 control linked to it.",
                    t.means
                ),
                "origins": [{ "actors": [actor] }],
                "target": {
                    "type": "objective-id",
                    "target-id": objective_id,
                    "status": { "state": state, "reason": reason },
                },
                "related-observations": [{ "observation-uuid": observation_uuid }],
            }));
        }
    }

    let count_line = counts
        .iter()
        .map(|(s, n)| format!("{n} {s}"))
        .collect::<Vec<_>>()
        .join(", ");
    let mut metadata_props = vec![
        prop("sv-version", sv_version),
        prop("sv-commit", sv_commit),
        prop(
            "sv-uncommitted-changes",
            if sv["uncommitted_changes"].as_bool().unwrap_or(false) {
                "true"
            } else {
                "false"
            },
        ),
        prop("report-sha256", &provenance.report_sha256),
    ];
    if let Some(run) = report["run_record"]["run_id"]
        .as_str()
        .filter(|r| !r.is_empty())
    {
        metadata_props.push(prop("run-id", run));
    }
    if let Some(seal) = &provenance.seal {
        metadata_props.push(prop("report-seal", seal));
    }
    if let Some((commit, changed)) = &provenance.app_commit {
        metadata_props.push(prop("app-commit", commit));
        metadata_props.push(prop(
            "app-uncommitted-changes",
            if *changed { "true" } else { "false" },
        ));
    }
    if let Some(image) = &provenance.image {
        metadata_props.push(prop("image", image));
    }
    let not_said = if provenance.app_commit.is_none() {
        " The app's commit was not recorded with this export."
    } else {
        ""
    };

    Ok(json!({
        "assessment-results": {
            "uuid": id("document"),
            "metadata": {
                "title": format!("StackVet results for {app} against OWASP ASVS 5.0.0"),
                "last-modified": started,
                "version": format!("sv {sv_version} (commit {sv_commit})"),
                "oscal-version": OSCAL_VERSION,
                "props": metadata_props,
                "parties": [{ "uuid": id("party/owner"), "type": "person", "name": format!("The owner of {app}") }],
                "remarks": format!(
                    "Written by sv from its report of {app} (ADR-156). Only 'checked' is concluded as satisfied and only \
                     'needs attention' as not satisfied, each for an ASVS requirement's own objective; every other \
                     status is an observation with no conclusion, and 'not verified' is neither a pass nor a failure. \
                     NIST SP 800-53 controls appear only as related links, never as assessed controls.{not_said}"
                ),
            },
            // OSCAL allows objectives defined in place only here, beside the results, not inside one.
            "local-definitions": { "objectives-and-methods": objectives },
            "import-ap": {
                "href": format!("#{}", id("resource/no-plan")),
                "remarks": "sv writes no assessment plan: the plan is ASVS 5.0.0 at the app's target level, as the report says.",
            },
            "results": [{
                "uuid": id("result"),
                "title": format!("sv's report of {app}"),
                "description": format!(
                    "The ASVS 5.0.0 requirements that apply to {app} at level {}, with what sv's report holds for \
                     each: {count_line}.",
                    report["target_level"].as_u64().unwrap_or(0)
                ),
                "start": started,
                "local-definitions": {
                    "components": [
                        {
                            "uuid": id("component/sv"),
                            "type": "software",
                            "title": "sv (StackVet)",
                            "description": format!("sv {sv_version}, commit {sv_commit}: the checks that wrote the report."),
                            "status": { "state": "operational" },
                        },
                        {
                            "uuid": id("component/app-tests"),
                            "type": "software",
                            "title": "The app's own tests",
                            "description": "The tests in the app's own code, written by the AI coding tool and run by the app's own command.",
                            "status": { "state": "operational" },
                        },
                        {
                            "uuid": id("component/ai-tool"),
                            "type": "software",
                            "title": "The AI coding tool that wrote the app",
                            "description": "Its answers to design questions are its own word about its work.",
                            "status": { "state": "operational" },
                        },
                    ],
                    "assessment-assets": {
                        "assessment-platforms": [{
                            "uuid": id("platform/sv"),
                            "title": "sv (StackVet)",
                            "uses-components": [{ "component-uuid": id("component/sv") }],
                        }],
                    },
                },
                "reviewed-controls": {
                    "description": "The OWASP ASVS 5.0.0 requirements that apply, by their ids made lower case. No NIST SP 800-53 control is reviewed here.",
                    "control-selections": [{
                        "include-controls": rows.iter().map(|r| json!({ "control-id": control_id(r["id"].as_str().unwrap_or("")) })).collect::<Vec<_>>(),
                    }],
                },
                "observations": observations,
                "findings": findings,
            }],
            "back-matter": {
                "resources": [
                    {
                        "uuid": id("resource/no-plan"),
                        "title": "No assessment plan",
                        "description": "sv writes no OSCAL assessment plan. What was assessed is OWASP ASVS 5.0.0 at the app's target level; what applied and why is in sv's report.",
                    },
                    {
                        "uuid": id("resource/asvs"),
                        "title": "OWASP Application Security Verification Standard 5.0.0",
                        "description": "The requirements each objective quotes, CC BY-SA 4.0, OWASP Foundation.",
                        "rlinks": [{ "href": "https://github.com/OWASP/ASVS/releases/tag/v5.0.0_release" }],
                    },
                    {
                        "uuid": id("resource/nist-800-53"),
                        "title": format!(
                            "NIST SP 800-53 Rev. 5, version {}",
                            crosswalk["catalog"]["version"].as_str().unwrap_or("unknown")
                        ),
                        "description": "The catalog the related controls are named from, through sv's crosswalk (data/asvs-800-53-crosswalk.json).",
                        "rlinks": [{ "href": nist_source }],
                    },
                ],
            },
        }
    }))
}

/// Takes `--oscal FILE` out of `sv report`'s arguments, as `--fail-on` and `--baseline` take theirs.
pub fn take(args: &[String]) -> Result<(Option<PathBuf>, Vec<String>)> {
    let mut rest = Vec::new();
    let mut file = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        if arg == "--oscal" {
            let path = it
                .next()
                .context("--oscal needs the file to write the OSCAL assessment results to")?;
            if file.replace(PathBuf::from(path)).is_some() {
                bail!("--oscal is given twice: give one file");
            }
        } else {
            rest.push(arg.clone());
        }
    }
    Ok((file, rest))
}

/// The OSCAL document for the report in `report_dir`, when `sv` can show it wrote it; `app_dir`, when known, is read
/// for the app's commit now.
pub fn export(report_dir: &Path, app_dir: Option<&Path>) -> Result<String> {
    let (bytes, place) = crate::report_seal::proven_report_wherever(report_dir).map_err(|why| {
        anyhow::anyhow!(
            "sv cannot show it wrote the report in {}: {why}. An OSCAL file is written only from a report sv wrote \
             on this computer and nothing has changed since; write the report again, or pass --oscal to `sv report` \
             to write both in one run.",
            report_dir.display()
        )
    })?;
    let report: Value =
        serde_json::from_slice(&bytes).context("report.json is not JSON sv can read")?;
    let crosswalk: Value =
        sv_frameworks::data::read_json(&sv_frameworks::data::file("asvs-800-53-crosswalk.json"))?;
    let seal = sv_scan::ecosystems::report_marker_in(report_dir)
        .and_then(|m| std::fs::read_to_string(m).ok())
        .and_then(|text| {
            text.lines().find_map(|l| {
                l.strip_prefix(crate::report_seal::SEAL_LINE)
                    .map(str::to_owned)
            })
        });
    let app_commit = app_dir.and_then(|dir| {
        sv_check::git::head_commit(dir).map(|c| {
            (
                c,
                sv_check::git::has_uncommitted_changes(dir).unwrap_or(true),
            )
        })
    });
    let provenance = Provenance {
        report_sha256: crate::bundle::sha256(&bytes),
        seal: seal.map(|s| match place {
            crate::report_seal::Place::Here => s,
            crate::report_seal::Place::Elsewhere => {
                format!("{s} (sealed in the folder it was written in, and moved since)")
            }
        }),
        app_commit,
        image: std::env::var("STACKVET_IMAGE")
            .ok()
            .filter(|s| !s.trim().is_empty()),
    };
    let document = assessment_results(&report, &crosswalk, &provenance)?;
    Ok(serde_json::to_string_pretty(&document).expect("a JSON value serializes") + "\n")
}

/// `sv export --oscal [FOLDER] [--out FILE]`: a report folder, or an app folder whose `stackvet-report` holds one, as
/// OSCAL assessment results, printed or written to FILE. It reads the report and writes nothing else.
pub fn command(args: &[String]) -> Result<()> {
    let mut oscal = false;
    let mut out = None;
    let mut folder = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--oscal" => oscal = true,
            "--out" => {
                out = Some(PathBuf::from(
                    it.next().context("--out needs the file to write")?,
                ))
            }
            other if other.starts_with('-') => bail!("unknown option: {other}"),
            other if folder.is_none() => folder = Some(PathBuf::from(other)),
            _ => bail!("`sv export` takes one folder"),
        }
    }
    if !oscal {
        bail!(
            "`sv export` writes one format so far: give --oscal, for OSCAL assessment results (NIST's format)"
        );
    }
    let given = folder.unwrap_or_else(|| PathBuf::from("."));
    // An app folder is read for its commit; a report folder alone does not say which app it is of.
    let (report_dir, app_dir) = if given.join("report.json").is_file() {
        (given, None)
    } else {
        (
            sv_scan::ecosystems::default_report_dir_in(&given),
            Some(given),
        )
    };
    let document = export(&report_dir, app_dir.as_deref())?;
    match out {
        Some(file) => {
            refuse_inside(&report_dir, &file)?;
            std::fs::write(&file, document)
                .with_context(|| format!("writing {}", file.display()))?;
            eprintln!("Wrote the OSCAL assessment results to {}.", file.display());
        }
        None => print!("{document}"),
    }
    Ok(())
}

/// Refuses a file inside the report folder: it would make the folder hold a file `sv` does not write there, and its
/// seal could no longer be shown (H6).
pub fn refuse_inside(report_dir: &Path, file: &Path) -> Result<()> {
    let parent = file
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    // A folder below the report folder is inside it too.
    let inside = match (
        sv_frameworks::paths::canonical(parent),
        sv_frameworks::paths::canonical(report_dir),
    ) {
        (Ok(parent), Ok(report)) => parent.starts_with(report),
        _ => false,
    };
    if inside {
        bail!(
            "{} is inside the report folder; write the OSCAL file somewhere else, because a report folder holds only \
             the files sv writes there",
            file.display()
        );
    }
    Ok(())
}
