//! The stage every reading of the app's files goes through, for `sv check` and `sv report` alike.
//!
//! Until 8 October 2026 `sv check` ran the same five scanners as `sv report` (the files listed, the
//! languages and packages recognized, the keys and passwords, the configuration, the code) and then
//! parted from it: it never merged the same weakness reported twice on one line, never marked test
//! code, a folder the manifest sets apart, or a bundled library, never applied what a person had
//! set aside through `sv review`, and counted every finding toward its exit status, where `sv
//! report` counted the findings left after the reviews. So `sv check --fail-on attention` could fail
//! a CI pipeline on a finding the owner had recorded as a false alarm, and the AI coding tool
//! reading that failure would rewrite the code until it stopped (the architecture assessment of 8
//! October 2026, item 3; ADR-023, Later, 8 October 2026). Now both read the app through
//! [`StaticScan::read`] and count what [`settle`] leaves, and the two exit alike on the same folder.

use crate::{Loaded, ReviewLookup, decisions_then_reviews};
use anyhow::Result;
use std::path::Path;

/// A reader of the app's files that crashed: its name, and the place the panic was (backlog 0234).
#[derive(Debug, Clone)]
pub struct Crashed {
    pub scanner: &'static str,
    pub place: String,
}

/// What the five scanners that read the app's files found, built once for a run.
pub struct StaticScan {
    /// The readers that crashed. Each one's results are empty, and the report says so, since an empty result
    /// is not a pass (backlog 0234).
    pub crashed: Vec<Crashed>,
    /// One walk of the folder, shared by every check (DESIGN, "One walk of the app").
    pub listing: sv_scan::files::Listing,
    /// Its languages and frameworks, and the folders the manifest sets apart as applied (ADR-031).
    pub scan_report: sv_scan::ScanReport,
    /// The packages it ships, read from manifests and lockfiles; no network.
    pub bill_of_materials: sv_check::sbom::Sbom,
    pub secrets: sv_check::secrets::SecretScan,
    pub config: sv_check::config::ConfigReport,
    pub code: sv_check::ast::AstScan,
}

impl StaticScan {
    /// Reads the app at `app_dir`, calling `starting` with each stage's number (`REPORT_STAGES`,
    /// 0 to 5) as it begins. `not_the_app` is what the manifest says is not the app, or nothing.
    pub fn read(
        app_dir: &Path,
        not_the_app: &[String],
        loaded: &Loaded,
        starting: &dyn Fn(usize),
    ) -> Result<Self> {
        starting(0);
        // The list of the app's files is the one thing every reader needs. If it crashes, there is no list to read,
        // and an empty list is not an empty app: so no reader runs, and each is recorded as crashed through this one
        // entry (backlog 0234).
        let mut crashed = Vec::new();
        let listing =
            match crate::assemble::guarded("file_list", || sv_scan::files::Listing::of(app_dir)) {
                Ok(listing) => listing,
                Err(place) => {
                    crashed.push(Crashed {
                        scanner: "file_list",
                        place,
                    });
                    sv_scan::files::Listing {
                        root: app_dir.to_path_buf(),
                        ..Default::default()
                    }
                }
            };
        starting(1);
        let scan_report = if listed_crashed(&crashed) {
            sv_scan::ScanReport::default()
        } else {
            match crate::assemble::guarded("scan_report", || {
                sv_scan::scan_listing_app(&listing, &loaded.signatures, not_the_app)
            }) {
                Ok(result) => result?,
                Err(place) => {
                    crashed.push(Crashed {
                        scanner: "scan_report",
                        place,
                    });
                    sv_scan::ScanReport::default()
                }
            }
        };
        starting(2);
        let bill_of_materials = keep(&mut crashed, "bill_of_materials", || {
            sv_check::sbom::build_in(&listing)
        });
        starting(3);
        let secrets = keep(&mut crashed, "secrets", || {
            sv_check::secrets::scan_listing(loaded.secret_rules, &listing)
        });
        starting(4);
        let config = keep(&mut crashed, "config", || {
            sv_check::config::check_dir_in(&listing, &bill_of_materials, loaded.secret_rules)
        });
        starting(5);
        let code = keep(&mut crashed, "code", || {
            let mut code = sv_check::ast::scan_listing(&loaded.ast_rules, &listing);
            let uses = (bill_of_materials.components.iter())
                .map(|c| (c.ecosystem.as_str(), c.name.as_str()))
                .chain(
                    (scan_report.declared.iter()).map(|d| (d.ecosystem.as_str(), d.name.as_str())),
                );
            sv_check::ast::hold_back_for_packages(&loaded.ast_rules, &mut code, uses);
            code
        });
        Ok(StaticScan {
            crashed,
            listing,
            scan_report,
            bill_of_materials,
            secrets,
            config,
            code,
        })
    }

    /// Whether the named reader crashed, so its results are empty and not a pass.
    pub fn crashed(&self, scanner: &str) -> bool {
        listed_crashed(&self.crashed) || self.crashed.iter().any(|c| c.scanner == scanner)
    }

    /// The gaps for the readers that crashed: what each was, and where its panic was (backlog 0234).
    pub fn crash_gaps(&self) -> Vec<sv_report::Gap> {
        (self.crashed.iter())
            .map(|c| crate::assemble::crash_gap(label(c.scanner), &c.place))
            .collect()
    }

    /// Every finding of the five scanners, as one list. The bill of materials speaks for itself
    /// here: incomplete is a finding against V15.1.2, complete is evidence for it (`passed`), and
    /// exactly one of the two says anything.
    pub fn findings(&self) -> Vec<sv_check::Finding> {
        let mut findings = self.secrets.findings.clone();
        findings.extend(self.config.findings.iter().cloned());
        findings.extend(sv_check::sbom::incompleteness_finding(
            &self.bill_of_materials,
        ));
        findings.extend(self.code.findings.iter().cloned());
        findings
    }

    /// What was checked and found fine: the configuration checks that passed, and a complete bill
    /// of materials.
    pub fn passed(&self) -> Vec<sv_check::Verified> {
        let mut passed = self.config.passed.clone();
        passed.extend(sv_check::sbom::completeness_verified(
            &self.bill_of_materials,
        ));
        passed
    }

    /// What the checks that read the app's files could not do, for the exit status: a reader that crashed is one
    /// (backlog 0234), so `sv check` exits not assessed for it as `sv report` does.
    pub fn file_gaps(&self) -> crate::exit::Gaps {
        let mut gaps = crate::exit::Gaps::of_files(&self.listing, &self.secrets, &self.code);
        for c in &self.crashed {
            gaps.could_not_run.push(format!(
                "{}: it crashed, so what it reads was not checked",
                label(c.scanner)
            ));
        }
        gaps
    }

    /// The checks that read the app's files, per family, for `examined`. Each one read only part
    /// of the app when a symbolic link was not followed; the rules that read code, also when a
    /// file was not opened or did not parse, or a language had no parser; a single code rule, when
    /// it could not run or had not been taught a language here.
    pub fn examined(&self) -> Vec<sv_report::Examined> {
        let (listing, code, secrets, config) =
            (&self.listing, &self.code, &self.secrets, &self.config);
        let family = |rules: &str, short: Vec<String>| {
            if short.is_empty() {
                sv_report::Examined::ran(rules)
            } else {
                sv_report::Examined::partly(rules, short.join("; "))
            }
        };
        let mut links: Vec<String> = if listing.links.is_empty() {
            Vec::new()
        } else {
            vec![format!(
                "{} symbolic link(s) in the app were not followed",
                listing.links.len()
            )]
        };
        if !listing.special.is_empty() {
            links.push(format!(
                "{} entry(s) in the app that are not ordinary files were not opened",
                listing.special.len()
            ));
        }

        let mut code_short = links.clone();
        if !code.unread_files.is_empty() {
            code_short.push(format!(
                "{} file(s) in a language the rules read were not opened",
                code.unread_files.len()
            ));
        }
        if !code.unread_languages.is_empty() {
            let mut names: Vec<&str> = code.unread_languages.iter().map(String::as_str).collect();
            names.sort_unstable();
            code_short.push(format!("no parser for {}", names.join(", ")));
        }
        if !code.unparsed_files.is_empty() {
            code_short.push(format!(
                "{} file(s) did not parse cleanly",
                code.unparsed_files.len()
            ));
        }
        let mut examined = vec![match self.crashed_why("code") {
            Some(why) => sv_report::Examined::not_run("ast.", why),
            None => family("ast.", code_short),
        }];
        for broken in &code.broken_queries {
            examined.push(sv_report::Examined::partly(
                broken.rule_id.clone(),
                format!(
                    "its query for {} would not compile: {}",
                    broken.language, broken.why
                ),
            ));
        }
        for held in &code.held_back_by_packages {
            examined.push(sv_report::Examined::partly(
                held.rule_id.clone(),
                held_back_why(held),
            ));
        }
        for untaught in &code.untaught {
            examined.push(sv_report::Examined::partly(
                untaught.rule_id.clone(),
                format!("it has not been taught {}", untaught.languages.join(", ")),
            ));
        }

        let mut secrets_short = links.clone();
        if !secrets.coverage.skipped.is_empty() {
            secrets_short.push(format!(
                "{} file(s) were not read",
                secrets.coverage.skipped.len()
            ));
        }
        examined.push(match self.crashed_why("secrets") {
            Some(why) => sv_report::Examined::not_run("secrets.", why),
            None => family("secrets.", secrets_short),
        });

        examined.push(match self.crashed_why("config") {
            Some(why) => sv_report::Examined::not_run("config.", why),
            None => family("config.", links),
        });
        for (id, why) in &config.not_assessed {
            examined.push(sv_report::Examined::not_run(id.clone(), why.clone()));
        }
        examined
    }

    /// The rules whose clean result a package the app uses held back, as gaps in the written report.
    pub(crate) fn package_gaps(&self) -> Vec<sv_report::Gap> {
        (self.code.held_back_by_packages.iter())
            .map(|held| sv_report::Gap {
                what: format!(
                    "{}, for code that goes through {}",
                    held.rule_id, held.package
                ),
                why: held_back_why(held),
                reason: sv_report::GapReason::NoReader,
                requirements: Vec::new(),
            })
            .collect()
    }
}

impl StaticScan {
    /// Why the reader is not run, when it crashed.
    pub(crate) fn crashed_why(&self, scanner: &str) -> Option<String> {
        if !self.crashed(scanner) {
            return None;
        }
        if self.crashed.iter().any(|c| c.scanner == scanner) {
            return Some(format!(
                "the reader of {} crashed, so nothing it reads was checked",
                label(scanner)
            ));
        }
        Some(format!(
            "the reader of {} did not run, because the list of the app's files crashed, so nothing it reads was checked",
            label(scanner)
        ))
    }
}

/// Whether the list of the app's files crashed, so that no reader ran on it (backlog 0234).
fn listed_crashed(crashed: &[Crashed]) -> bool {
    crashed.iter().any(|c| c.scanner == "file_list")
}

/// The reader's name for a person: what its gap and its `examined` entry say.
fn label(scanner: &str) -> &'static str {
    match scanner {
        "file_list" => "the list of the app's files",
        "scan_report" => "the app's languages and packages",
        "bill_of_materials" => "the packages the app ships",
        "secrets" => "the keys and passwords in the app's files",
        "config" => "the app's configuration files",
        "code" => "the app's code",
        _ => "one of the app's files",
    }
}

/// Runs a reader, and if it crashes, records it and gives back its empty result (backlog 0234).
fn keep<T: Default>(
    crashed: &mut Vec<Crashed>,
    scanner: &'static str,
    run: impl FnOnce() -> T,
) -> T {
    if listed_crashed(crashed) {
        return T::default();
    }
    match crate::assemble::guarded(scanner, run) {
        Ok(value) => value,
        Err(place) => {
            crashed.push(Crashed { scanner, place });
            T::default()
        }
    }
}

/// Why a package the app uses kept a code rule from claiming anything, in the words both the gap and
/// the `examined` entry use.
fn held_back_why(held: &sv_check::ast::HeldBackByPackage) -> String {
    format!(
        "the app uses {} ({}), which builds queries through calls of its own this rule does not \
         read ({}), so finding nothing is not evidence; what it found stands",
        held.package, held.ecosystem, held.calls
    )
}

/// The findings as every report lists them and every exit status counts them, from `findings`
/// (the static scan's, and whatever else the run found): the same weakness on the same line from
/// two tools merged into one; test code, a folder the manifest sets apart, and a bundled library
/// marked; the decisions held to the running app (`decided`) and then what a person set aside
/// through `sv review` applied (`reviews`, ADR-023); and one finding per line of code. `examined`
/// is what this run looked at, complete, so an entry that matches nothing can say whether its rule
/// looked (deep review R3).
#[allow(clippy::too_many_arguments)]
pub fn settle(
    app_dir: &Path,
    reviews: &[sv_manifest::FindingReview],
    findings: Vec<sv_check::Finding>,
    scan: &StaticScan,
    examined: &[sv_report::Examined],
    loaded: &Loaded,
    seals: &sv_check::seal::Checker,
    decided: &[sv_check::decisions::Decided],
    left_out: &[String],
) -> sv_check::review::Outcome {
    // The same weakness on the same line, reported by two tools, is one thing to fix.
    let mut findings = sv_check::finding::merge_same_place(findings);
    // Rust keeps its unit tests beside the code, so the file's name cannot say which is which.
    sv_check::finding::mark_rust_test_code(app_dir, &mut findings);
    // What the manifest says is not the app is listed with test and sample code, as the scan used
    // it: none when the list would have set apart all the app's code (ADR-031).
    sv_check::finding::mark_not_the_app(&scan.scan_report.not_the_app, &mut findings);
    // A copy of another project's library kept in the app is listed apart, named for it.
    sv_check::bundled::mark_bundled_libraries(app_dir, &mut findings);
    let lookup = ReviewLookup {
        app_dir,
        examined,
        listing: &scan.listing,
        code: &scan.code,
        secrets: &scan.secrets,
        ast_rules: &loaded.ast_rules,
        secret_rules: loaded.secret_rules,
    };
    let mut outcome = decisions_then_reviews(findings, decided, |mut findings| {
        sv_check::review::fill_fingerprints(app_dir, &mut findings);
        sv_check::review::apply(
            app_dir,
            reviews,
            findings,
            sv_check::advisories::Day::today().unwrap_or(sv_check::advisories::Day(0)),
            seals,
            &|rule, file| lookup.looked(rule, file),
        )
    });
    // Last, what lies in a folder the owner left out, with their confirmation, is set aside, keys
    // and passwords apart (ADR-099): after the reviews, so an entry set aside by a person is
    // reported as theirs.
    let (findings, left) = crate::left_out::split(outcome.findings, left_out);
    outcome.findings = findings;
    outcome.left_out = left;
    outcome
}
