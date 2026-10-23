# The path rule's false alarms, and how test code is read (23 October 2026)


Backlog 0647, at the owner's word of 23 October 2026, after StackVet's report on itself listed 150 findings of one rule:
"investigate how we can fix the excessive false positives and issue with test code, because otherwise actual issues
will get overlooked." This entry is what was found, and the choices it leaves; nothing is built until the owner picks.

## What the rule does

`ast.file-path-from-value` (V5.3.2, CWE-22 and CWE-73, low confidence) reports a file opened, written, or deleted at a
path that is not written out in full. It never asks where the value came from. A path is spared only when it can be
shown safe: fixed text (every language), a safe helper (`secure_filename`, `path.basename`, a join under `__dirname`;
Python and JavaScript), or, in Python alone, a name followed back to fixed text or to a value the app made
(`safeNamesRead`, backlog 0582). Rust has none of these, so every Rust file call with a value is a finding.

V5.3.2 asks something narrower: "when the application creates file paths for file operations, instead of
user-submitted filenames, it uses internally generated or trusted data, or if user-submitted filenames or file
metadata must be used, strict validation and sanitization must be applied." The rule reports every path that is not
plainly fixed, whether it is user-submitted or not.

## What the 150 on StackVet are

Read by keyword, so roughly: of the 131 in StackVet's own code (the rest are in test modules, backlog 0652), about 52
are paths passed between StackVet's own functions, 33 are files inside the folder being checked (`app_dir.join(...)`),
12 are the report it writes, and the rest are its configuration, its data, a temporary folder, or a cleanup. None comes
from a web request: `sv` takes none.

The one place a less trusted party hands `sv` a path is the MCP server, where an AI coding tool passes arguments. There
the code already does what V5.3.2 asks: every path is resolved against the folder the server was started for and
refused if it lands outside, `..` and symbolic links included (`crates/sv-cli/src/mcp/mod.rs`). The rule cannot see
that, and reports those lines with the rest.

So on StackVet the rule is right that the paths are values, and wrong to call them things to fix: the data is trusted,
or validated where it is not. The cost is the one the owner named. A path that does come from a stranger would be one
line among 150, and nobody reads that far.

## How test code is read

Three ways, which do not agree:

- **The code rules** (the AST rules, Semgrep, the secrets scan) report in test code, mark the finding as in test code,
  and list it apart; it is shown in full and still counts (`Finding::apart`, ADR-023). A Rust file is test code by its
  folder or name, by `#[cfg(test)]` lines inside it, or since 0644 by the `#[cfg(test)]` line that declares it as a
  module (ADR-136), but not yet by `#[cfg(all(test, unix))]` (backlog 0652).
- **Nine checks skip test files altogether,** by name alone (`sv_scan::files::is_test_path`). Six of them gather
  evidence of what the app uses (its sign-in paths, route guards, personal fields, session settings, outbound rules,
  and crypto inventory), and skipping is right there: a test's sign-in path is not the app's. But the weak-key check
  reports findings, and it skips test files where the code rules report and label them. None of the nine knows a Rust
  test module that sits in `src/`.
- **The bill of materials reads test fixtures' lockfiles** as what the app ships, and so said on StackVet that its
  list is not complete (`sbom.incomplete`), from sample lockfiles under `crates/sv-scan/tests/fixtures`.

## The choices

**A. Read where the value came from (recommended).** Keep the rule, and split what it reports:

- **A path built from a request** (a framework's request input reaching the file call, in the same function or through
  names, as `ast/followed.rs` already follows them in Python) stays a thing to fix, at a higher confidence than today.
- **Any other value** stays a finding, still counted and shown in full, but listed apart as worth a look, with what the
  rule could not tell said beside it. It is not hidden: the reports already list findings apart this way (`apart`).
- What counts as a request's input is a list per framework, kept in `data/` beside the rule: Flask and Django
  (`request.args`, `request.files`, `request.GET`), Express (`req.params`, `req.query`, `req.body`, `req.file`), Go's
  `net/http` (`r.URL.Query()`, `r.FormValue`), PHP's `$_GET` and `$_FILES`, Axum's and Actix's extractors (`Path`,
  `Query`, `Form`, `Multipart`), and so on, the frameworks the trial apps used first.
- The cost: a path built from a request through a call the reading does not follow (a helper in another file, a value
  stored and read back) is listed apart rather than among the things to fix. A finding listed apart still counts,
  so V5.3.2 needs attention exactly when it does now; this changes which list a finding is on, not what is verified. (A
  clean run of the rule credits V5.3.2 in part, `inPart`, as now.)
- Rust gains the safe forms the other languages have: names followed back to fixed text, and joins under a constant.

**B. Rust's safe forms only.** Teach Rust what Python and JavaScript know (fixed text through names, a join under a
constant or `env!("CARGO_MANIFEST_DIR")`), and nothing else. Small, and it cuts some of StackVet's 150, but most of them
are real values, and the list stays long for any app whose paths come from its own config.

**C. Use the app's own answers.** When `stackvet.toml`, confirmed in `sv review`, says the app is for its owner alone and
takes no requests (`audience = "just-me"`, `deployment = "local-only"`), list the rule's findings apart for that app.
Cheap, but it rests on answers rather than code, and an app described wrongly loses its path findings from the list.

**D. Leave the rule, and group its lines.** As low-severity findings are grouped (ADR-095), show the rule once with its
count and its files, and its lines in full in a file beside the report. The list gets short, but a real fault is still
one line inside the group.

**For test code, whichever is chosen:**

- **One reading of test code for every check,** moved to `sv-scan` so the nine checks use the one the code rules use,
  Rust's declared and `#[cfg(...)]` modules included.
- **The weak-key check reports in test code** and lists the finding apart, as the code rules do, rather than skipping
  the file.
- **The bill of materials leaves out lockfiles in test folders and in folders named as not the app,** and says so, as
  it already says what it could not read.

Each choice is a decision record of its own when it is built (what counts as evidence, and at which tier). Then
StackVet's report on itself is made again.

## The owner's answer, 23 October 2026

Asked which choice, the owner answered "A: read the source (Recommended)", and for test code chose all three: "One
reading for all checks", "Weak-key check labels, not skips", and "SBOM skips test lockfiles". Built as backlog 0653 to
0656.
