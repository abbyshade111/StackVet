//! Each sign of a last-resort error handler, found in its own language's files and nowhere else, and the look-alikes
//! that are not one; the credit made when one is found, and nothing at all when none is (backlog 0263).

use super::*;
use crate::test_support::scratch;

/// What `found` names in a folder holding `files`, as `file:line`.
fn places(files: &[(&str, &str)]) -> Vec<String> {
    let dir = scratch("");
    for (name, text) in files {
        std::fs::write(dir.join(name), text).unwrap();
    }
    let listing = Listing::of(&dir);
    // The setup worked: every file was listed as the app's own code, or finding nothing would mean nothing.
    assert_eq!(listing.app_files().count(), files.len());
    found(&listing).into_iter().map(|(_, at)| at).collect()
}

#[test]
fn each_language_s_sign_is_found_where_it_is_written() {
    assert_eq!(
        places(&[(
            "server.js",
            "process.on('uncaughtException', (e) => log(e));\n\
             app.use((err, req, res, next) => res.status(500).end());\n\
             app.use(function onError(err, req, res, next) { next(err); });\n"
        )]),
        ["server.js:1", "server.js:2", "server.js:3"]
    );
    assert_eq!(
        places(&[(
            "app.ts",
            "process.on(\"unhandledRejection\", report);\n\
             app.use((err: Error, req: Request, res: Response, next: NextFunction) => res.sendStatus(500));\n"
        )]),
        ["app.ts:1", "app.ts:2"]
    );
    assert_eq!(
        places(&[(
            "app.py",
            "@app.errorhandler(Exception)\ndef oops(e):\n    return 'error', 500\n\
             app.add_exception_handler = None\n\
             @app.exception_handler(Exception)\nasync def any_error(request, exc): ...\n\
             handler500 = 'pages.views.server_error'\n"
        )]),
        ["app.py:1", "app.py:5", "app.py:7"]
    );
    assert_eq!(
        places(&[(
            "application_controller.rb",
            "class ApplicationController\n  rescue_from StandardError, with: :oops\nend\n"
        )]),
        ["application_controller.rb:2"]
    );
    assert_eq!(
        places(&[(
            "Errors.java",
            "@RestControllerAdvice\npublic class Errors {}\n"
        )]),
        ["Errors.java:1"]
    );
    assert_eq!(
        places(&[("Program.cs", "app.UseExceptionHandler(\"/error\");\n")]),
        ["Program.cs:1"]
    );
    assert_eq!(
        places(&[(
            "mw.go",
            "package m\nfunc mw() { defer func() { if r := recover(); r != nil { log(r) } }() }\n"
        )]),
        ["mw.go:2"]
    );
}

#[test]
fn look_alikes_are_not_signs() {
    assert!(
        places(&[
            (
                "server.js",
                "app.use((req, res, next) => next());\n\
                 process.on('exit', done);\n\
                 const recover = () => {};\n"
            ),
            (
                "app.py",
                "@app.errorhandler(404)\ndef missing(e):\n    return 'no', 404\n\
                 @app.errorhandler(ValueError)\ndef bad(e): ...\n"
            ),
            (
                "app.rb",
                "rescue_from ActiveRecord::RecordNotFound, with: :not_found\n"
            ),
            // A sign in a file of another language is not read as one.
            (
                "notes.md",
                "process.on('uncaughtException', f) is how Node does it\n"
            ),
        ])
        .is_empty()
    );
}

#[test]
fn a_sign_is_a_credit_for_v16_5_4_and_none_is_nothing() {
    let dir = scratch("");
    std::fs::write(
        dir.join("server.js"),
        "process.on('uncaughtException', log);\napp.use((err, req, res, next) => res.status(500).end());\n",
    )
    .unwrap();
    let mut report = ConfigReport::default();
    check(&Listing::of(&dir), &mut report);
    assert_eq!(report.passed.len(), 1, "{:?}", report.passed);
    let credit = &report.passed[0];
    assert_eq!(credit.check_id, LAST_RESORT);
    assert_eq!(credit.requirement_ids, ["V16.5.4"]);
    assert!(
        credit.scope.contains("server.js:1, server.js:2"),
        "{}",
        credit.scope
    );
    assert!(
        credit.scope.contains("not something files show"),
        "{}",
        credit.scope
    );

    let none = scratch("");
    std::fs::write(
        none.join("server.js"),
        "app.get('/', (req, res) => res.send('hi'));\n",
    )
    .unwrap();
    let mut report = ConfigReport::default();
    check(&Listing::of(&none), &mut report);
    assert!(
        report.passed.is_empty() && report.findings.is_empty(),
        "no handler found says nothing"
    );
}

#[test]
fn v16_5_4_is_manual_only_so_the_credit_is_supporting_evidence_never_checked() {
    // The report shows a credit for a requirement only a person can settle beside it, as supporting evidence, and
    // never counts it as checked (`sv-report`'s `a_clean_check_about_a_design_review_requirement_supports_it_and_does_not_check_it`
    // holds that rule). This holds that V16.5.4 is such a requirement in the data `sv` ships.
    let data = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let config = sv_frameworks::applicability::ApplicabilityConfig::load_v2(
        &data.join("knowledge"),
        &data.join("applicability-v2.json"),
    )
    .unwrap();
    assert_eq!(
        config.verification_class_for("V16.5.4"),
        sv_frameworks::applicability::VerificationClass::ManualOnly
    );
}
