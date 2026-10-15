//! The fourteen files a report is written as: each name, what kind of file it is, and how it is rendered.
//!
//! The one table in this crate that every reader of a report folder's names derives from (BACKLOG,
//! "From the architecture assessment of 8 October 2026", item 5): `write_report` writes these in this
//! order, the seal covers them (`report_seal::SEALED`), and the MCP server offers them as resources.
//! `sv-scan`'s walk leaves a folder out that holds nothing but these names and the marker and lock,
//! and that crate cannot see this one, so the names are its (`sv_scan::ecosystems::REPORT_FILES`)
//! and this table's are held to them by the compiler: the build fails when a name here is not the
//! name there, in the same place. Before 8 October 2026 the names (five then) were written out four times,
//! held together by a test.

/// One file of a report.
pub struct ReportFile {
    /// Its name in the report folder.
    pub name: &'static str,
    /// What kind of file it is, as the MCP server tells a client.
    pub mime: &'static str,
    /// The report rendered as this file.
    pub render: fn(&sv_report::Report) -> String,
}

/// The fourteen files, in the order they are written and sealed.
pub const REPORT_FILES: [ReportFile; 14] = [
    ReportFile {
        name: "report.html",
        mime: "text/html",
        render: sv_report::html::page,
    },
    ReportFile {
        name: "compliance.md",
        mime: "text/markdown",
        render: sv_report::markdown::compliance,
    },
    ReportFile {
        name: "security.md",
        mime: "text/markdown",
        render: sv_report::markdown::security,
    },
    // Every low-severity finding in full; the reports group them by rule (ADR-095).
    ReportFile {
        name: sv_report::LOW_FINDINGS_FILE,
        mime: "text/markdown",
        render: sv_report::markdown::low_findings,
    },
    // What to do before going live, in the order of a deployment; credits nothing (backlog 0589, ADR-121).
    ReportFile {
        name: sv_report::checklist::FILE,
        mime: "text/markdown",
        render: sv_report::checklist::render,
    },
    // The report in short pages, one per purpose, cut whole from the others (backlog 0590).
    ReportFile {
        name: sv_report::pages::FIX_NOW.file,
        mime: "text/markdown",
        render: sv_report::pages::fix_now,
    },
    ReportFile {
        name: sv_report::pages::DECISIONS.file,
        mime: "text/markdown",
        render: sv_report::pages::decisions,
    },
    ReportFile {
        name: sv_report::pages::NOT_EXAMINED.file,
        mime: "text/markdown",
        render: sv_report::pages::not_examined,
    },
    ReportFile {
        name: sv_report::pages::INFORMATION.file,
        mime: "text/markdown",
        render: sv_report::pages::information,
    },
    ReportFile {
        name: sv_report::SARIF_FILE,
        mime: "application/sarif+json",
        render: sv_report::sarif::render,
    },
    ReportFile {
        name: "report.json",
        mime: "application/json",
        render: sv_report::json::to_string,
    },
    // What `sv` saw of the running app, credentials removed (ADR-082).
    ReportFile {
        name: sv_report::seen::FILE,
        mime: "application/json",
        render: sv_report::seen::render,
    },
    // The app's software bill of materials, as `sv sbom` prints it (backlog 0382).
    ReportFile {
        name: sv_report::SBOM_FILE,
        mime: "application/json",
        render: sv_report::sbom_document,
    },
    // The same report as one file to read or print, written by `sv` itself (ADR-116).
    ReportFile {
        name: sv_report::pdf::FILE,
        mime: "application/pdf",
        render: sv_report::pdf::render,
    },
];

/// The names alone, in the same order.
pub const NAMES: [&str; REPORT_FILES.len()] = {
    let mut names = [""; REPORT_FILES.len()];
    let mut i = 0;
    while i < REPORT_FILES.len() {
        names[i] = REPORT_FILES[i].name;
        i += 1;
    }
    names
};

/// The table and `sv-scan`'s list are the same names in the same order, or nothing compiles.
const _: () = {
    let theirs = sv_scan::ecosystems::REPORT_FILES;
    assert!(NAMES.len() == theirs.len());
    let mut i = 0;
    while i < NAMES.len() {
        assert!(
            same(NAMES[i], theirs[i]),
            "a report file's name differs between sv-cli's report_files and sv-scan's REPORT_FILES"
        );
        i += 1;
    }
};

/// `a == b`, in a form the compiler can run while it builds.
const fn same(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_file_is_rendered_from_the_report_and_named_for_its_kind() {
        let app =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
        let report = crate::assemble_report(
            &app,
            &crate::ReportOptions::reading_only("a test"),
            &crate::Loaded::load().unwrap(),
        )
        .unwrap();
        for file in &REPORT_FILES {
            let text = (file.render)(&report);
            assert!(!text.is_empty(), "{} renders to nothing", file.name);
            let kind = match file.name.rsplit('.').next().unwrap() {
                "md" => "markdown",
                extension => extension,
            };
            assert!(
                file.mime.contains(kind),
                "{} is offered as {}",
                file.name,
                file.mime
            );
        }
    }
}
