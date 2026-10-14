# StackVet (`sv`): the ten-minute map

What `sv` is made of, how a run goes through it, and where each rule is held. Written 8 October 2026 for a session
or a person opening the repository cold. `docs/design/` is the dated record of every decision as it was made, one file per
entry (437 entries, some 17,800 lines in all, on 13 October 2026); `docs/adr/` holds the decisions that matter most, each with the files it
governs (96 records, ADR-015 to ADR-110, on the same day); this page is the map that
neither of them is. When this page and the code disagree, the code is right and this page is out of date: say so in
the pull request that finds it.

## What it does, in one paragraph

A person builds an app in their own AI coding tool, in any language. The tool writes `stackvet.toml`, a manifest of
what the app claims about itself, from the spec `sv init` prints. `sv` reads the code and the manifest, works out
which requirements of OWASP ASVS 5.0, AISVS 1.0, and the Secure by Design checklist apply, checks what it can (the
files, and, when asked, the app running behind a network fence), and writes reports that say what was verified, at
what strength, and what was not looked at. The same engine answers an AI coding tool over MCP, so the tool can check
as it builds. `sv` never says an app is secure; it says what it checked and what it did not.

## The crates, in the order a run passes through them

Lines are the Rust under each crate's `src/`, with files named `tests.rs` or `*_tests.rs` left out (a test module
written inside another file is counted), rounded to the hundred, on 13 October 2026. They move with every merge:
`find crates/<crate>/src -name '*.rs' ! -name '*tests.rs' | xargs cat | wc -l` counts them again.

| Crate | Lines | What it is |
|---|---|---|
| `sv-frameworks` | 1,400 | The standards as data (`data/frameworks/*.json`), and which requirements apply to an app with these conditions (`data/applicability-v2.json`). Also `data::file`, the one way any run-time read finds `data/` (ADR-036). |
| `sv-manifest` | 4,600 | `stackvet.toml`: what the app claims. Nothing in it is a fact; `resolve` corroborates each claim against the code, and corroboration only ever adds requirements, never removes them. |
| `sv-scan` | 4,300 | One walk of the app's folder (`files::Listing`), its languages and frameworks by signature, what the manifest sets apart as not the app (ADR-031), and the names of a report folder so the walk leaves it out. |
| `sv-check` | 93,500 | Every check: keys and passwords (`secrets`), configuration (`config`), rules that read code with tree-sitter (`ast`, `data/ast-rules.json`), the bill of materials (`sbom`) and advisories, outside tools through adapters, and the checks of a running app (`signed_in`, `probes`, `oidc`, `ai`, `mcp_server`, `fetch`, `live`) written against a trait `Http`, so each runs against a fake app in its unit tests. Also what a person's word is worth: `design`, `hand`, `notes`, `confirm`, `review`, and the seals (`seal`). Produces `Finding` and `Verified`, never a verdict. |
| `sv-run` | 8,700 | Starts the app behind the fence: a Docker network with no gateway, a sidecar that sends every request from inside it, the helpers a run needs (a mail server, a test identity provider, a test model, a headless browser), every container read-only with every capability dropped (ADR-019, `prepared`), the install step for the app's packages (ADR-052), and the run's script (`run_after_cleanup`). Implements `Http` for the checks. Depends on `sv-check` today, the reverse of this layering (BACKLOG, 8 October, item 4). |
| `sv-report` | 10,400 | From findings and credits to a status per requirement (`status_of`), the counts, the gaps, and the eight files a report is written as: `report.html`, `compliance.md`, `security.md`, `low-findings.md`, `stackvet.sarif`, `report.json`, `seen.json`, `sbom.cdx.json`. The one place a conclusion is drawn. |
| `sv-cli` | 24,600 | The `sv` binary and a library under it. The library (`lib.rs`) holds everything a run, a report, a plan, a brief, or a preflight does: the stages of a run (`static_scan`, then `assemble`, one function per stage of putting the report together), what they read (`Loaded`, `ReportOptions`), `report_folder`, the report folder's lock and seal, the exit codes, the bundle, and the MCP server (`mcp/`), which can reach nothing of the command line. The binary (`main.rs`) reads the arguments, runs the commands, and prints. |

The rule of the layering: a crate below decides nothing a crate above would want to overrule. `sv-check` says what
it saw; `sv-report` says what that is called; `sv-cli` says it to a person or a tool.

## A run of `sv report`, stage by stage

1. **The manifest** is read once; its hash goes into the report, so a `stackvet.toml` that changes during the run is
   reported (`report_lock::manifest_changed`).
2. **The report folder is claimed** before anything slow happens: a lock file, the marker that keeps the next walk from
   reading the report as the app, and a refusal of any folder that holds somebody else's files (ADR-041,
   `report_folder::write_report_folder`, the same sequence the MCP server's `stackvet_write_report` runs).
3. **The static stage** (`static_scan::StaticScan::read`): one walk of the folder, the languages and packages, the
   keys and passwords, the configuration, the code. `sv check` runs exactly this and stops; `sv report` goes on. Both
   settle their findings the same way (`static_scan::settle`), so they exit alike on the same folder (ADR-023, Later,
   8 October 2026).
4. **Which requirements apply**: the manifest's claims resolved against what the scan saw (`sv_manifest::resolve`),
   then the frameworks' applicability rules (`sv_frameworks`). A claim the code contradicts is reported as such. Drawn in
   [How `sv` decides what applies](#how-sv-decides-what-applies).
5. **What else was asked for**, each opt-in and each saying in the report when it did not run: a local advisory
   database (`--advisories`), the language's own security tools (on unless `--no-tools`, ADR-111; ADR-032: never a program inside the app),
   a CodeQL results file from the owner's own CI (`--codeql-results`, read only for the commit checked out: ADR-113),
   and the running app (`--run`): the fence, the helpers, the anonymous questions, the signed-in suites as two users
   and an admin, sign-in through a test provider, the AI feature through a test model, the app's own tests inside its
   container (ADR-050), and whether the app was still up at the end (ADR-021).
6. **A person's word**: the design answers in `stackvet.toml`, the checks made by hand, the security notes and the
   decisions file, and the confirmations and reviews `sv review` sealed (ADR-022, ADR-026). Each becomes a `Verified`
   that says which tier it rests on (`sv_check::Tier`).
7. **Settled**: one weakness reported twice on a line merged, test code and bundled libraries marked, the decisions
   held to the running app, a person's reviews applied, one finding per line (ADR-023).
8. **The report built** (`sv_report::build`): per requirement, a finding beats every credit, and otherwise the
   strongest tier there decides (`status_of`). Gaps say what was not examined; `examined` says the same for a program.
9. **Written and sealed**: the eight files of `crates/sv-cli/src/report_files.rs` (the five reports; `low-findings.md`,
   every low-severity finding in full, ADR-095; `seen.json`, what the running app answered with its credentials cut
   out, ADR-082; and `sbom.cdx.json`, the bill of materials), each under a new name and renamed into place, never through a link; the
   folder sealed with this computer's report key so the MCP server can tell `sv`'s report from one anything else wrote
   (ADR-034). The lock goes; the folder stays.

## How `sv` decides what applies

Every requirement of ASVS, AISVS, and the Secure by Design checklist lands in one of four bins for an app
(`sv_frameworks::applicability::bucket`), from what `stackvet.toml` claims and what the code shows
(`sv_manifest::resolve`). This is the part of `sv` a person most affects: every line of `stackvet.toml` they or their
AI coding tool writes.

```text
 stackvet.toml says                        the code shows (signatures:
 (true / false / left out)                 packages, frameworks, files)
          │                                          │
          └──────────────► resolve ◄─────────────────┘
                              │   a yes from either side is a yes: the code can add, never remove
                              │   left out stays unknown, even when the code found nothing
                              │   a false the code contradicts is reported, and counts as yes
                              ▼
               each condition: yes / no / unknown

 [app] audience ──┐
 [data] categories┴──► the level: 2 if customers or the public use it, or it holds sensitive information,
                        or the list of what it holds is missing; otherwise 1. Both answers are the AI
                        coding tool's until you confirm them with sv review.

 for each requirement, in this order:
   above the level ......................................► ABOVE THE APP'S LEVEL
   no rule ties it to a condition .......................► APPLIES
   a condition that brings it in is yes .................► APPLIES
   a condition that brings it in is unknown .............► NOT ASSESSED (the question that would settle it)
   every condition that brings it in is no ..............► DOES NOT APPLY (with its reason)

 A Secure by Design control takes the level of the ASVS requirement that asks the same thing
 (data/sbd-asvs-crosswalk.json), or every level when nothing in ASVS does.
```

So a line left out of `stackvet.toml` costs nothing but a question in the report, while a wrong `false` switches
whole sets of requirements off until the code shows otherwise: that is why the guide's prompt says to leave a line
commented out rather than guess `false`.

## The nine statuses, strongest evidence first

What a requirement that applies is called in a report (`crates/sv-report/src/lib.rs`, `Status`), decided by
`status_of` from its findings and the tiers of its credits: a finding beats every credit, and otherwise the strongest
tier there decides. This is the one drawing of them; the README, the guide, and the glossary link here. Who can grant
each tier is the whole reason `sv review` exists, and why the AI coding tool's word ranks last.

```text
  strongest                                                    who can grant it
 ┌──────────────────────────────────────┐
 │ needs attention                      │  a check sv runs, its own or an outside tool's, finding a problem
 │ checked                              │  a check of sv's own, satisfied
 │ checked in part                      │  a check of sv's own that tried only part of what is asked
 ├──────────────────────────────────────┤  ── nothing below this line is ever checked, or counts toward a threat
 │ tested by the app's own tests        │  the app's tests, run when sv starts the app
 │ documented by the owner              │  the owner, at their own terminal, through sv review
 │ checked by hand by the owner         │  the owner, at their own terminal, through sv review
 │ attested by the owner                │  the owner, at their own terminal, through sv review
 │ stated by the AI coding tool         │  the AI coding tool, or an owner's entry never recorded through sv review
 │ not verified                         │  nobody: nothing produced evidence either way
 └──────────────────────────────────────┘
  weakest

  Not a tier: "does not apply" (left out of the app's requirements) and "not assessed" (a check that could not run,
  or a question nobody answered); neither is a pass, and neither is a failure.
```

| Status | Means | Who can grant it |
|---|---|---|
| needs attention | A check found something and named this requirement. | A check `sv` runs, its own or an outside tool's |
| checked | A check of `sv`'s own ran and was satisfied. One automated check, never a pass. | A check of `sv`'s own |
| checked in part | Every check behind it tried only part of what it asks (ADR-053). | A check of `sv`'s own |
| tested by the app's own tests | The app's tests name the requirement, in code, and passed (ADR-050). The tool wrote both the test and the command. | The app's tests, run when `sv` starts the app (`sv run`, `sv report --run`) |
| documented by the owner | The owner answered its question in the security notes, recorded through `sv review`. A decision, not a reading of the code. | The owner, through `sv review` |
| checked by hand by the owner | The owner watched the app behave and recorded what they saw. | The owner, through `sv review` |
| attested by the owner | The owner answered `yes` to a design question. Asserting a property is not the property. | The owner, through `sv review` |
| stated by the AI coding tool | The tool answered, or nobody recorded that the owner did: the author grading its own work. | The AI coding tool |
| not verified | Nothing produced evidence either way. The honest default, and the common one. | Nobody |

Nothing below *checked in part* is ever *checked*, and none of it counts toward a threat (`threats.rs`). An answer of
the tool's that the owner confirmed through `sv review` keeps its tier and says so ("confirmed through sv review").
*Attested*, *checked by hand*, *stated*, and *not verified* stay on the list of tests to write, unless no test could
show the requirement or a test already names it: a person's word never retires a test (`lib.rs`, `tests_to_write`).

## The five kinds of rule, and where a new one goes

Every rule cites the requirements it speaks to where it is written, and nowhere else; a citation is a claim.

| Kind | Where | A new one |
|---|---|---|
| Keys and passwords | `data/secret-rules.json`, read by `crates/sv-check/src/secrets.rs` | A new entry: the pattern, the first four characters shown, the length, never the key. |
| Rules that read code | `data/ast-rules.json`, a tree-sitter query per language, read by `crates/sv-check/src/ast.rs` | A new entry with a witness per language it is taught; a language it is not taught is a gap, never a clean result (ADR-054). |
| Configuration | Rust, `crates/sv-check/src/config.rs`, each check a function that reads what it needs from the listing | A new function, cited in `RUST_CHECKS` in `tools/coverage.py`. |
| Outside tools | `data/adapters.json`: how each tool is run, in a container of its own, and how its findings map to requirements, read by `crates/sv-check/src/adapters.rs` | A new entry; never a program inside the app (ADR-032). |
| The running app | Rust, `crates/sv-check/src/signed_in/` and `crates/sv-check/src/probes.rs`, written against `Http` so each runs against a fake app in its tests | A new check in the suite that fits, with the fake-app test that shows it fail; cited in `RUST_CHECKS`. |

Four gates hold the citations to the code (`tools/coverage.py`): a requirement id written into Rust that is in
neither `RUST_CHECKS` nor `MENTIONS` stops the script; `docs/COVERAGE.md` and `docs/REQUIREMENTS.md` must be what the
script would write (`crates/sv-check/tests/coverage_doc.rs`); CI's test run logs every credit given (`SV_CREDIT_LOG`),
and a check that credits a requirement listed as finding-only, or never credits one it is listed for, fails
(`--credits`); and every prompt in `data/prompts.json` and `data/design-prompts.json` names requirements that one of
its rules cites, and nothing else.

## The exit codes

This is the one table of them; the README and the guide link here (`crates/sv-cli/src/exit.rs` decides them,
`crates/sv-cli/src/probe_verdict.rs` for `sv probe`).

| Command | 0 | 1 | 2 | 3 |
|---|---|---|---|---|
| `sv check`, `sv report` | Finished | Only with `--fail-on attention`: a finding of low severity or worse, leaving out those marked only for information (`attention:high` for high and critical only) | A check could not run (a file that could not be read or parsed, a language nothing here reads, a code rule that would not compile), no file of the app was read, or, for `sv report --run`, the app could not be started; with `--fail-on not-assessed`, also a symbolic link not followed, an outside tool that did not run, or an `--advisories` comparison that did not cover the app | `sv` itself failed: a folder that is not there, an option it does not know, a `stackvet.toml` that is there and cannot be read (for `sv report`, none at all), or a fault in `sv`; there is no result |
| `sv audit` | Every package was compared, and none matched a known vulnerability | A package matched a known vulnerability | The comparison did not cover the whole app: no database, an ecosystem the database holds nothing about, or a list of packages `sv` could not complete | As above |
| `sv run` | The app started and answered | Never | The app could not be started, or never answered, so nothing about it running was checked | As above |
| `sv probe` | The address answered | Never | The address was not reached, so nothing about it was checked | As above |
| Every other command | Finished | Never | Never | Its own failure |

- `--fail-on any` is both `attention` and `not-assessed`. 1 outranks 2, and 3 is never combined with anything, since a
  run that failed has no result to rank. Ctrl-C keeps 130, the usual code for it.
- `--baseline <an older report folder>`: `--fail-on attention` then exits 1 only for a finding that report did not
  hold. Every finding is still listed and counted, and those it held are marked; a baseline `sv` cannot read, or one
  for an app of another name, stops the run (3).
- What a static run cannot see by its nature, such as requirements nothing verified or a check that needs a Dockerfile
  the app lacks, does not count toward 2, or every run would exit 2 and the status would say nothing.
- By default findings alone do not fail a run (ADR-029): an AI coding tool that reads a failure rewrites the code until
  it stops, so the bar is the person's to set.

## The MCP server (`crates/sv-cli/src/mcp/`)

JSON-RPC over stdio, hand-written, one request at a time, each on a thread with a time limit. The tools are the
commands (`stackvet_check`, `stackvet_write_report`, `stackvet_plan`, `stackvet_spec`,
`stackvet_record_answer`, and the rest in `catalog.rs`), built on the same `assemble_report_saying` as `sv report`,
so what the tool is told is what the report says. Three rules hold everywhere in it: every path stays below the folder
the server was started for (`confine.rs`); the app's own text is fenced as data before it reaches the tool, since the
app can write anything (`sv_report::fence`); and a report is offered as a resource only when its seal shows `sv` wrote
it (`resources.rs`, ADR-034). The server never starts the app and never runs outside tools; it says so and names the
terminal command the person can run.

## The rules that hold everywhere

- **Evidence is honest.** Not assessed is never a pass and never a failure; a check that could not run says so.
  Credit is given only over what was read, in a person's words (`Verified::scope`), and a check that names more
  requirements when it passes than when it fails is wrong by construction (`verified.rs`).
- **Whose word, at which rank.** A check of `sv`'s own, then the app's tests, then the owner's written answer, a check
  by hand, the owner's yes, the tool's yes; nothing a model says makes a requirement *checked* (ADR-022, ADR-050).
- **The fence.** The app runs on a Docker network with no gateway, read-only, every capability dropped; every request
  to it is sent from inside; `sv` opens no network connection of its own, with three exceptions, drawn and listed in
  [The fence](#the-fence-what-can-reach-the-internet-and-what-turns-it-on) below (ADR-019, ADR-027, ADR-052,
  ADR-111). Verified by asking Docker, not by trusting the flag.
- **What `sv` writes into somebody's folder** is the report folder (the record of the build loop, ADR-076, included),
  `security-notes.md`, its section of `AGENTS.md`, and, when the owner runs `sv review` at their own terminal, what
  they recorded and its seal in `stackvet.toml` and seal lines in `design-decisions.md`; nothing else (ADR-017, with
  its Later entry of 5 October 2026); never through a link.
- **Every decision has a record** in the same pull request, and the "Decision records" check holds it (`docs/adr/README.md`).

## The fence: what can reach the internet, and what turns it on

This is the one place the network rule is written out in full; every other page says it in a sentence and links here.
`sv` opens no network connection of its own, with **three exceptions**. Two are off until the person turns them on by
name; the third is on unless they turn it off. The app being checked has no way out at all.

```text
                                          the internet
          ▲                         ▲                              ▲
          │ 1. sv probe <address>   │ 2. install = true            │ 3. outside tools,
          │    (asked for by name)  │    (asked for by name)       │    unless --no-tools
          │                         │                              │    (on by default)
 ┌────────┴─────────────────────────┴──────────────────────────────┴───────────────────┐
 │ this computer                                                                       │
 │                                                                                     │
 │   sv ─── curl, to the address typed      install container      Semgrep (or         │
 │      ─── one DNS question about it       (Docker's python or    Opengrep) fetching  │
 │                                          node image; sees the   its rule packs      │
 │                                          dependency files only, from semgrep.dev    │
 │                                          never the code)                            │
 │                                                                                     │
 │   ┌─ the fence: a Docker network made for the run, with no gateway ──────────────┐  │
 │   │                                                                              │  │
 │   │   ┌───────────────────────────┐        ┌──────────────────────────────────┐  │  │
 │   │   │ the app's container       │ ◄───── │ the sidecar: every request sv    │  │  │
 │   │   │ folder read-only,         │        │ sends to the app starts here     │  │  │
 │   │   │ every capability dropped, │        └──────────────────────────────────┘  │  │
 │   │   │ no keys but placeholders  │                                              │  │
 │   │   └───────────────────────────┘        the stand-ins, when the run needs     │  │
 │   │                                        them: mail catcher, test sign-in      │  │
 │   │                                        provider, test model, name server,    │  │
 │   │                                        headless browser                      │  │
 │   └──────────────────────────────────────────────────────────────────────────────┘  │
 └─────────────────────────────────────────────────────────────────────────────────────┘
```

| | What turns it on | What leaves this computer | Where it is held |
|---|---|---|---|
| 1 | `sv probe <address>`, typed by the person: **off** until asked for | At most four read-only requests through `curl` to the public address typed (five with `--api`), and one question to this computer's own DNS resolver about that name | ADR-027 |
| 2 | `install = true` under `[stack.run]`: **off** until asked for | The app's packages, downloaded from PyPI or npm before the run, in a container of its own that sees only the dependency files, never the app's code or its `.env`, and runs none of the packages' own install code; the report says when it did | ADR-052 |
| 3 | The outside tools, which `sv report` and `sv bundle` run: **on** unless `--no-tools` | Semgrep (or Opengrep in its place) fetches its rule packs from semgrep.dev every time it runs, with its usage reporting off; nothing of the app's is sent. gosec is started so that it cannot download the app's modules | ADR-111 |

Two more things are worth knowing, though neither is `sv` opening a connection. `sv run` (and `sv report --run`)
starts containers, and Docker itself downloads any image it does not have yet. And the advisory data is something the
person downloads and points `sv` at; `sv` never fetches it. The MCP server opens no connection and runs none of the
outside tools.

The fence is checked, not assumed: `sv run` asks Docker whether the network it made has a gateway and refuses to run
if it does (`crates/sv-run/src/docker.rs`, ADR-019). A report that reached the internet any other way is a
vulnerability in `sv` (`SECURITY.md`).

## Where to look

| To change | Start at |
|---|---|
| Which requirements apply to an app | `data/applicability-v2.json`, `crates/sv-frameworks/src/applicability.rs` |
| A rule that reads code | `data/ast-rules.json`, then `python3 tools/coverage.py` (a test holds `docs/COVERAGE.md` current) |
| A check of the running app | `crates/sv-check/src/signed_in/`, against the fake app in its tests; the run's script in `crates/sv-run/src/docker.rs` |
| What a report says | `crates/sv-report/src/` (`lib.rs` decides, `html.rs`, `markdown.rs`, `sarif.rs`, `json.rs` render) |
| What the AI coding tool is told | `crates/sv-cli/src/mcp/catalog.rs` (tools and prompts), `check_text.rs` |
| What a person can set aside or confirm | `crates/sv-check/src/review.rs`, `confirm.rs`, `seal.rs`; `sv review` in `crates/sv-cli/src/review.rs` |
| A data file | `data/README.md` says what each is and what reads it; `sv_frameworks::data::file` finds it |
| A helper two scripts in `tools/` need | `tools/_shared.py` (`write_text`, `slug`); a script imports it after putting its own folder on `sys.path`, since CI runs the tools with `python3 -I` (`crates/sv-cli/tests/tools_shared.rs`) |
| What is still to do | `docs/backlog/`, one file per item with its status on its third line; `docs/BACKLOG.md` holds the rules and the roadmap. Claim an item with `python3 tools/backlog.py claim` before starting it |
| A document, and whether it is live or a dated reading | `docs/README.md` lists every document in `docs/`; the dated readings are in `docs/research/`, and a document added there or in `docs/` needs a row (`crates/sv-cli/tests/docs_index.rs`) |
| Who is building what, while the coordination trial runs | `docs/coordination/board.md`, written only by the orchestrating session with `tools/coordination.py`; the role is `docs/COORDINATION.md` |

`cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --workspace` are what CI runs;
a test that starts the app needs a container backend and says which branch it took without one.
