//! Writing the report again after `sv review` with the options of the last one (backlog 0578).
//!
//! The answers a person records in `sv review` count only once the report is written again, and the full report is
//! made with `sv report --run`. Written again with plain `sv report`, it replaced the folder whole: the running-app
//! checks, the test run, and the outside tools' results were gone, and the person either ran everything twice or ended
//! with a weaker report than before the review. So the report now records the options it was written with
//! (`RunRecord::options`), `sv review` ends by writing it again with them, and `sv report` says so before it writes a
//! report without `--run` over one that had it.

use std::io::Write;
use std::path::Path;

/// The options `sv review` may give `sv report`, each with whether a value follows it. Anything else in a report's
/// record is refused, so the record can only ever ask for what the person could have typed for a report.
const REPEATED: [(&str, bool); 7] = [
    ("--run", false),
    ("--slow", false),
    ("--tools", false),
    ("--no-tools", false),
    ("--keep-tool-output", false),
    ("--advisories", true),
    ("--codeql-results", true),
];

/// The options of a report, as its run record keeps them: those that change what it holds, in a fixed order, with a
/// path made absolute so it names the same folder whichever folder `sv review` is run from.
pub fn options_of(
    run_the_app: bool,
    slow: bool,
    run_tools: bool,
    keep_tool_output: bool,
    advisories_dir: Option<&Path>,
    codeql_results: Option<&Path>,
) -> Vec<String> {
    let mut options = Vec::new();
    if run_the_app {
        options.push("--run".to_owned());
        if slow {
            options.push("--slow".to_owned());
        }
    }
    options.push(if run_tools { "--tools" } else { "--no-tools" }.to_owned());
    if keep_tool_output {
        options.push("--keep-tool-output".to_owned());
    }
    for (option, path) in [
        ("--advisories", advisories_dir),
        ("--codeql-results", codeql_results),
    ] {
        if let Some(path) = path {
            options.push(option.to_owned());
            options.push(
                std::path::absolute(path)
                    .unwrap_or_else(|_| path.to_path_buf())
                    .display()
                    .to_string(),
            );
        }
    }
    options
}

/// The options the last report in `folder` was written with, when `sv` can show it wrote that report: the AI coding
/// tool can write `report.json`, and what is read here is run. Otherwise why not, for the person to read.
pub fn last_options(folder: &Path) -> Result<Vec<String>, String> {
    let bytes = crate::report_seal::proven_report(folder)?;
    let report: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| format!("its report.json could not be read ({e})"))?;
    let Some(listed) = report["run_record"]["options"].as_array() else {
        return Err(
            "it does not say which options it was written with: it was written by an older sv, by `sv bundle`, or \
             through StackVet's tools in your AI coding tool"
                .to_owned(),
        );
    };
    let mut options = Vec::new();
    let mut listed = listed.iter();
    while let Some(option) = listed.next() {
        let option = option
            .as_str()
            .ok_or("its list of options holds something that is not an option")?;
        let Some(&(_, valued)) = REPEATED.iter().find(|(name, _)| *name == option) else {
            return Err(format!(
                "its list of options holds `{option}`, which sv review does not repeat"
            ));
        };
        options.push(option.to_owned());
        if valued {
            let value = listed
                .next()
                .and_then(serde_json::Value::as_str)
                .filter(|v| !v.is_empty() && !v.starts_with("--"))
                .ok_or_else(|| format!("its list of options gives `{option}` no value"))?;
            options.push(value.to_owned());
        }
    }
    Ok(options)
}

/// Whether the report in `folder` had the running-app checks: its record says it was written with `--run`, or, for a
/// report made before the record kept its options, it says the app was asked to start. Read whether or not `sv` can
/// show it wrote it: this only decides whether a warning is said.
pub fn last_had_run(folder: &Path) -> bool {
    let path = folder.join("report.json");
    if !std::fs::symlink_metadata(&path).is_ok_and(|m| m.is_file()) {
        return false;
    }
    let mut text = String::new();
    let read = std::fs::File::open(&path).and_then(|file| {
        std::io::Read::read_to_string(
            &mut std::io::Read::take(file, crate::mcp::MAX_RESOURCE_BYTES),
            &mut text,
        )
    });
    let Ok(report) = read
        .ok()
        .and_then(|_| serde_json::from_str::<serde_json::Value>(&text).ok())
        .ok_or(())
    else {
        return false;
    };
    match report["run_record"]["options"].as_array() {
        Some(options) => options.iter().any(|o| o == "--run"),
        None => report["run_status"]["state"]
            .as_str()
            .is_some_and(|state| state != "not-asked"),
    }
}

/// What `sv report` says before it writes a report without `--run` over one that had it, or `None`. It goes on
/// either way: a person may want a quick one.
pub fn losing_the_run(folder: &Path, run_the_app: bool) -> Option<String> {
    (!run_the_app && last_had_run(folder)).then(|| {
        format!(
            "The last report in {} had the running-app checks; this one will not, and will replace it. Add --run \
             to keep them, or run `sv review`, which writes the report again with the options of the last one.",
            folder.display()
        )
    })
}

/// The command a person types to write the report again, for when `sv review` does not.
fn typed(app_dir: &Path, options: &[String]) -> String {
    let mut words = vec!["sv report".to_owned()];
    if app_dir != Path::new(".") {
        words.push(shown(&app_dir.display().to_string()));
    }
    words.extend(options.iter().map(|o| shown(o)));
    words.join(" ")
}

/// A word as a person would type it: quoted when it holds a space or a quote.
fn shown(word: &str) -> String {
    if word.contains([' ', '\'', '"']) {
        format!("'{}'", word.replace('\'', r"'\''"))
    } else {
        word.to_owned()
    }
}

/// The end of `sv review` when it recorded something: the report written again with the options of the last one, by
/// `run` (the `sv report` command with these arguments, giving back its exit status), unless `no_report`. When the
/// last report's options cannot be read, nothing is run and the person is told what to type. Nor when the last report
/// had `--run` and `starts_containers` says this computer cannot start the app here, as inside the container
/// `sv review` runs in for a person with only Docker: that report would have the running-app checks *not assessed*,
/// and replace the one that had them.
pub fn write_again(
    app_dir: &Path,
    no_report: bool,
    out: &mut dyn Write,
    starts_containers: &dyn Fn() -> Result<(), String>,
    run: &mut dyn FnMut(&[String]) -> anyhow::Result<i32>,
) -> anyhow::Result<()> {
    let folder = sv_scan::ecosystems::default_report_dir_in(app_dir);
    let options = match last_options(&folder) {
        Ok(options) => options,
        Err(why) => {
            writeln!(
                out,
                "\nWhat you recorded counts once the report is written again. The last report in {} was not \
                 read for its options ({why}), so it is not written again here: run `sv report` yourself, with the \
                 options you wrote it with (`--run` for the running-app checks).",
                folder.display()
            )?;
            return Ok(());
        }
    };
    let command = typed(app_dir, &options);
    if no_report {
        writeln!(
            out,
            "\nWhat you recorded counts once the report is written again. With --no-report it is not written \
             here; the last one was written with: {command}"
        )?;
        return Ok(());
    }
    if options.iter().any(|o| o == "--run") {
        if let Err(why) = starts_containers() {
            writeln!(
                out,
                "\nWhat you recorded counts once the report is written again. The last one was written with --run, \
                 and the app cannot be started from here ({why}), so it is not written again here: a report written \
                 now would replace the running-app checks with *not assessed*. Run this where you wrote the last \
                 one: {command}"
            )?;
            return Ok(());
        }
    }
    writeln!(
        out,
        "\nWriting the report again with the options of the last one, so what you recorded counts: {command}"
    )?;
    out.flush()?;
    let mut args = vec![app_dir.display().to_string()];
    args.extend(options);
    match run(&args) {
        Ok(code) if code <= crate::exit::NOT_ASSESSED => Ok(()),
        Ok(code) => anyhow::bail!(
            "the report was not written again (`sv report` ended with exit status {code}); what you recorded is \
             kept, and counts once you run: {command}"
        ),
        Err(e) => Err(e.context(format!(
            "the report was not written again; what you recorded is kept, and counts once you run: {command}"
        ))),
    }
}

#[cfg(test)]
#[path = "report_again_tests.rs"]
mod tests;
