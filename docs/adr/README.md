# Decision records

A decision record ("ADR", architecture decision record) writes down one decision: what was decided, why, what
else was considered, and what it costs. It is kept so that nobody has to reconstruct the reasoning later from
code, and so that undoing the decision is a choice made knowingly.

## When a record is written, and how it stays true

Since 4 October 2026, at the owner's asking, a record is written with the change that makes the decision, not after
it. (`sv`'s first eleven were each written one to seven days after the decision, and only when a review noticed;
`docs/paper/ADRS.md`.)

- **What needs one.** A change to what counts as evidence or at which tier, to what `sv` runs or connects to, to what it
  writes into someone's folder, to the network fence, a new or removed dependency, a default that changes what a report
  concludes, and every choice the owner makes when a session asks. A change that does none of these needs none.
- **When.** In the same pull request as the decision. For anything substantial, first: the backlog claim adds the
  record as `Status: proposed`, and the pull request that builds it makes it accepted and says where the build
  differs from the plan. A change to an existing decision is a dated "Later" entry on its record, or a new record that
  replaces it; nothing in a record is quietly rewritten.
- **Governs.** Each record lists the files whose change can change its decision. The "Decision records" check
  (`.github/workflows/decision-records.yml`, `tools/adr_check.py`) fails a pull request that touches one of them and
  neither changes the record nor says, on a line of its description, `ADR-0NN: unchanged, because ...` with a reason.
  The reason is the point: it is the moment somebody reads the record against the change. A record may govern a named
  part of a large file, `path::NAME` (ADR-103): then only a change to a line that has NAME as a word asks.
- **References.** `crates/sv-cli/tests/decision_records.rs` fails when a record names a test or a file that no longer
  exists, governs a pattern that matches nothing, or when any record number cited in the code or the documents has no
  record, or a record is missing from the index below.
- **Required.** The owner decided on 4 October 2026 that "Decision records" is a required check on `main`, so a pull
  request that owes a record does not merge.
- **The weekly review** stays as the safety net: once a week a scheduled session reads every record against the
  week's merged pull requests, and reports how many days each new record came after its decision.

## Numbering

One sequence runs across both versions of SecureVibe, so a number always means one decision.

- **ADR-001 to ADR-014 are v1's.** They moved with v1 to the `v1` branch on 26 September 2026 and are at
  `docs/adr/` there, and at the tags `v1-paper` and `v1-final`. ADR-014 is the file named `ADR-011.md` there,
  whose title calls it ADR-014; v1 has no other file for either number.
- **ADR-015 onward are `sv`'s**, and are here.

## v1's records that `sv` still cites

Two of v1's decisions are rules `sv` keeps. The design record (`docs/design/`, "What carries over unchanged") restates
both among "the rules that carry over" (the evidence-tier rule without its number), and ADR-012 is also cited by number
in three other design entries ("Shape", "What the first run found", and "`sv check`: the first findings"), in three
source files, and in two tests:

- **ADR-006, evidence tiers.** AI review alone is never a pass, and a requirement only a person can check
  never passes on its own.
- **ADR-012, what "SecureVibe checks this app" means in another language.** Written after the first app from
  outside, a Python one, was told it was missing `package-lock.json`. `sv` cites it for the rule it drew from
  that incident: a check that does not apply is not a check that failed, a scan that did not run is not a
  clean result, and a wrong statement in a report is worse than a gap in it.

  ADR-012 also ruled out SecureVibe writing its own static-analysis rules for other languages. `sv` later
  wrote such rules for fourteen languages, fifteen counting shell on 30 September 2026 (the design record,
  `docs/design/`, "Rules that read the code"). ADR-018 replaces that ruling, and the rest of ADR-012 stays in force.

## `sv`'s records

| Record | Decision |
|---|---|
| [ADR-015](ADR-015.md) | What the owner says can add requirements and never remove one, and silence is not a "no" |
| [ADR-016](ADR-016.md) | The OWASP data files: one copy while both versions lived here, and two since |
| [ADR-017](ADR-017.md) | `sv` never writes the app's code |
| [ADR-018](ADR-018.md) | `sv` checks apps written in any language, with rules of its own among the checks (replaces ADR-012's ruling against such rules) |
| [ADR-019](ADR-019.md) | `sv` runs the app in a container on a network with no way out (replaces v1's ADR-010 choice, for `sv`) |
| [ADR-020](ADR-020.md) | `sv` is written in Rust, a memory-safe language (replaces v1's ADR-001 choice, for `sv`) |
| [ADR-021](ADR-021.md) | A crash's or a rate limiter's answer is never read as the app refusing |
| [ADR-022](ADR-022.md) | Whose word counts, and at which tier (extends v1's ADR-006, for `sv`) |
| [ADR-023](ADR-023.md) | False alarms and accepted risks a person records, and test code's findings listed apart |
| [ADR-024](ADR-024.md) | An unanswered data list holds the app to ASVS level 2 |
| [ADR-025](ADR-025.md) | `sv run` has an end: time limits, Ctrl-C that cleans up, and leftovers removed by the next run |
| [ADR-026](ADR-026.md) | The owner's word counts only when `sv review` recorded it (changes part of ADR-022 and ADR-023) |
| [ADR-027](ADR-027.md) | `sv probe` asks only public addresses, and only the ones it checked |
| [ADR-028](ADR-028.md) | Decide before you build: the design comes first, and nothing is credited for it |
| [ADR-029](ADR-029.md) | Exit codes: 2 only when a check could not run, 1 only when asked, 3 when `sv` failed |
| [ADR-030](ADR-030.md) | A plan before any code: `sv plan` and `securevibe_plan`, crediting nothing |
| [ADR-031](ADR-031.md) | A `not-the-app` list that would set apart all of the app's code is not used |
| [ADR-032](ADR-032.md) | Git, run in the app's folder, runs no program the app's repository names |
| [ADR-033](ADR-033.md) | CVSS v4 scores, computed with FIRST's own tables |
| [ADR-034](ADR-034.md) | A report is offered as `sv`'s only when its seal shows `sv` wrote it |
| [ADR-035](ADR-035.md) | A preflight of the run settings, read from the code and never run |
| [ADR-036](ADR-036.md) | `sv` finds its data beside itself, and an install does not live in a working folder |
| [ADR-037](ADR-037.md) | A Python project with no lockfile does not pin, `setup.py` and `setup.cfg` included |
| [ADR-038](ADR-038.md) | The SQL injection probe only reads, only on `sv`'s own copy of the app, and only ever finds |
| [ADR-039](ADR-039.md) | The sign-in token checks, and which key addresses they may name |
| [ADR-040](ADR-040.md) | A credential name over a sentence is reported low, and says so |
| [ADR-041](ADR-041.md) | One run at a time in a report folder, held by a lock file `sv` writes there |
| [ADR-042](ADR-042.md) | The test model answers in the shape the app asked for, and a wrong shape refused is credit for C7.1.1 |
| [ADR-043](ADR-043.md) | A seal is a signature any computer can check, against keys the owner chose to trust |
| [ADR-044](ADR-044.md) | The coding prompts shown to work reach the AI tool when it builds what they are for |
| [ADR-045](ADR-045.md) | The app's own tools are called in a loop only when the owner marks them read-only |
| [ADR-046](ADR-046.md) | Compressed archives sent to the upload, to the limits the owner states |
| [ADR-047](ADR-047.md) | V3.4.3 is credited only for a Content-Security-Policy with the directives it names |
| [ADR-048](ADR-048.md) | The weak password-derivation rule cites storing passwords as well as making keys |
| [ADR-049](ADR-049.md) | `sv` reads the AI coding tool's own files in the project folder, apart from the app's grade |
| [ADR-050](ADR-050.md) | The app's own tests are a tier of their own, below an automated check |
| [ADR-051](ADR-051.md) | The tests must pass before a pull request merges into main, and every commit on main is tested |
| [ADR-052](ADR-052.md) | Packages installed before the run, outside the fence, in a container that sees only the dependency files |
| [ADR-053](ADR-053.md) | Another user's records are checked for reading, listing, changing, and deleting, and one read alone is "checked in part" |
| [ADR-054](ADR-054.md) | Templates and notebooks are read for what they can hold, and never pass silently |
| [ADR-055](ADR-055.md) | Two checks stop resting on one sample: cross-site access asks the private pages, and a common password is three |
| [ADR-056](ADR-056.md) | An error answer is credited only when the app was made to give one, and it was clean |
| [ADR-057](ADR-057.md) | A dashboard for `sv`, optional, with history kept only when the owner asks, outside the app's folder (accepted, 8 October 2026) |
| [ADR-058](ADR-058.md) | A private set of pages for the owner to read `sv`'s documentation, in the home folder, without the paper |
| [ADR-059](ADR-059.md) | Every check that gives credit is seen withholding it in the test suite, or the build fails |
| [ADR-060](ADR-060.md) | One file per design entry, so two pull requests stop colliding in the design record |
| [ADR-061](ADR-061.md) | One file per backlog item, with a status line, so what is open is data and two claims meet only on their own item |
| [ADR-062](ADR-062.md) | The product is StackVet and the command stays `sv`: one place for every name, the old names read for a window, the paper and v1 untouched (accepted, 9 October 2026) |
| [ADR-063](ADR-063.md) | StackVet's logo is the code bracket, in terracotta, as the owner chose |
| [ADR-064](ADR-064.md) | A tool that does not answer, and C9.1.1 checked in part |
| [ADR-065](ADR-065.md) | Hidden characters sent into the AI feature, for C2.1.2 and C2.1.5 |
| [ADR-066](ADR-066.md) | The MCP server: what it does and does not do for an AI coding tool, and how its text is held |
| [ADR-067](ADR-067.md) | A session that is one fixed key, found from repeated sign-ins (V7.2.2) |
| [ADR-068](ADR-068.md) | An MCP link to a remote server over plain HTTP, found in the app's code (C10.3.1) |
| [ADR-069](ADR-069.md) | A line break and a Bcc header in the address a reset is mailed to (V1.3.11) |
| [ADR-070](ADR-070.md) | Client-side technology that is no longer supported, found in the app's code (V3.7.1) |
| [ADR-071](ADR-071.md) | The AI service's failure found in the app's own output (V16.3.4, in part) |
| [ADR-072](ADR-072.md) | The app's log files served to anybody who asks (V16.4.2) |
| [ADR-073](ADR-073.md) | Token use recorded for each user, found in the record of one model call (C12.2.5, in part) |
| [ADR-074](ADR-074.md) | Why a caught prompt injection was stopped, written down with it (C12.1.2, in part) |
| [ADR-075](ADR-075.md) | A tool action the AI took, written down with its argument (C12.4.2, in part) |
| [ADR-076](ADR-076.md) | The build loop written down as it happens, and the report saying what it shows |
| [ADR-077](ADR-077.md) | The readers of untrusted input fuzzed weekly, apart from the workspace and the tests |
| [ADR-078](ADR-078.md) | CodeQL for Ruby, and for Java only where it reads the code without building it or reaching the network |
| [ADR-079](ADR-079.md) | An input flagged as an attack, and whether the flag stopped it (C11.4.2) |
| [ADR-080](ADR-080.md) | The published image signed with GitHub's own keyless signing, and what it was built from published with it |
| [ADR-081](ADR-081.md) | A GitHub Action that runs the published image on an app's pull requests |
| [ADR-082](ADR-082.md) | What `sv` saw of the running app kept beside the report, with credentials removed (accepted) |
| [ADR-083](ADR-083.md) | History that can show a trend: each requirement's status in each run, and why two runs differ (accepted, 11 October 2026) |
| [ADR-084](ADR-084.md) | More in the build-loop record: each call's outcome, what was handed over, and what changed between checks (accepted, 11 October 2026) |
| [ADR-085](ADR-085.md) | A stand-in name server on the fenced network records the names an app asks for (accepted) |
| [ADR-086](ADR-086.md) | The observability dashboard adds five things to `sv dashboard`, as the owner chose on 11 October 2026 (accepted, 11 October 2026) |
| [ADR-087](ADR-087.md) | A `creates` entry may name where its record is read back, so the owner field is tried on it (accepted, 11 October 2026) |
| [ADR-088](ADR-088.md) | Four credits and gaps the part 4 audit found wrong, corrected (kill-switch status, skipped private pages, admin-action credit, model files not read whole); accepted 11 October 2026 |
| [ADR-089](ADR-089.md) | A test-only 7z reader, `py7zr` pinned to one version, to check the 7z archives the upload check sends (accepted, 11 October 2026) |
| [ADR-090](ADR-090.md) | The part 4 audit's one-sample credits are checked in part, as the owner approved on 11 October 2026; the admin-by-address check is left as it is (accepted) |
| [ADR-091](ADR-091.md) | Four Bandit rules in test code are information, not findings, and a person reviews them (accepted, 11 October 2026) |
| [ADR-092](ADR-092.md) | A note lists the commits an AI coding tool made straight onto the default branch (accepted, 11 October 2026) |
| [ADR-093](ADR-093.md) | A reviewer can say a test-file information finding is real, and then it counts (accepted, 11 October 2026) |
| [ADR-094](ADR-094.md) | The report says what Docker read of the app's container, and it decides no requirement (accepted, 11 October 2026) |
| [ADR-095](ADR-095.md) | Low-severity findings are grouped by rule in the reports, and listed in full in a file beside them (accepted, 11 October 2026) |
| [ADR-096](ADR-096.md) | The header, cookie, and content-type checks ask every route stackvet.toml names (accepted, 11 October 2026) |
| [ADR-097](ADR-097.md) | The browser's typed-text check on every text form stackvet.toml names (accepted, 12 October 2026) |
| [ADR-098](ADR-098.md) | `sv review` says which findings it never offers, and each finding in the report names its next step (accepted, 11 October 2026) |
| [ADR-099](ADR-099.md) | The owner can leave a folder out of the checks, with a reason the report prints, and keys are still looked for there (accepted, 12 October 2026) |
| [ADR-100](ADR-100.md) | `sv` keeps two files of its own, an opt-in log and a crash file, and never puts an argument or a path in either (accepted, 12 October 2026) |
| [ADR-101](ADR-101.md) | `sv connect` prints the settings an AI coding tool needs and writes nothing, and one prompt sets StackVet up (accepted, 12 October 2026) |
| [ADR-102](ADR-102.md) | `sv report` says how long its slow stages take, how far a long wait is, and the slowest parts, unless `--quiet` (accepted, 12 October 2026) |
| [ADR-103](ADR-103.md) | A record can govern a named part of a large file, and the check asks only when that part changes (accepted, 12 October 2026) |
| [ADR-104](ADR-104.md) | A pull request that changes only backlog items runs only the tests that read them (accepted, 12 October 2026) |
| [ADR-105](ADR-105.md) | The phone, two-factor, and recovery changes are tried as the email change is, and V7.5.1 is whole only when each is refused or confirmed not offered (accepted, 12 October 2026) |
| [ADR-106](ADR-106.md) | The suite runs as three jobs side by side, and the required check waits for them all (accepted, 12 October 2026) |
| [ADR-107](ADR-107.md) | The StackVet mark in the heading of `report.html` and the dashboard, drawn in the page, with the pages' policy unchanged (accepted, 12 October 2026) |
| [ADR-108](ADR-108.md) | The Windows program is published as a signed GitHub Release when the owner pushes a version tag (accepted, 12 October 2026) |
| [ADR-109](ADR-109.md) | The second loop trial tests `sv`'s waiting features on two briefs, with an outside check the builder never sees (accepted, 12 October 2026) |
| [ADR-110](ADR-110.md) | The reports lead with the things to fix, and say what was not examined right after (accepted, 12 October 2026) |
| [ADR-111](ADR-111.md) | The outside tools run with every `sv report` and `sv bundle` unless `--no-tools` is given (accepted, 13 October 2026) |
| [ADR-112](ADR-112.md) | A security contact that is a placeholder is a finding of its own, and cites no requirement (accepted, 14 October 2026) |
| [ADR-113](ADR-113.md) | A CodeQL results file from the owner's own CI is read when given, and credited only for the commit checked out (accepted, 14 October 2026) |
| [ADR-114](ADR-114.md) | The copy `sv` runs keeps the app's limit on wrong passwords; only a limit on every sign-in is raised for it (accepted, 14 October 2026) |
| [ADR-115](ADR-115.md) | CodeQL runs only when the owner turns it on, for the uses its license allows (accepted, 14 October 2026) |
| [ADR-116](ADR-116.md) | The report is also written as a PDF, by `sv` itself (accepted, 14 October 2026) |
| [ADR-117](ADR-117.md) | Rich text and user-supplied markup are asked about, and an AI feature is one way in (accepted, 14 October 2026) |
| [ADR-118](ADR-118.md) | Several flows and WebSocket paths in stackvet.toml, so the checks that try one can try each (accepted, 15 October 2026) |
| [ADR-119](ADR-119.md) | A probe reads its answer against its own setup and its own control (accepted, 15 October 2026) |
| [ADR-120](ADR-120.md) | The guide's word budget is 4,500 (accepted, 15 October 2026) |
| [ADR-121](ADR-121.md) | The checklist before going live, a report file of its own that credits nothing (accepted, 15 October 2026) |
| [ADR-122](ADR-122.md) | The hook record, kept by a hook the person installs, read only with their consent (accepted, 15 October 2026) |
| [ADR-123](ADR-123.md) | A way into the app the code shows that stackvet.toml's list of sign-in pathways does not name is a finding (accepted, 15 October 2026) |
| [ADR-124](ADR-124.md) | A value the sign-up needs and the manifest must not carry is printed by the seed and cited as `{seed:NAME}` (accepted, 16 October 2026) |
| [ADR-125](ADR-125.md) | Ending sessions is tried from the manifest: a user ends their others, and an admin ends a user's (accepted, 16 October 2026) |
| [ADR-126](ADR-126.md) | A member's own page is saved through a `creates` entry and fetched where it is served (accepted, 16 October 2026) |
| [ADR-127](ADR-127.md) | The test sign-in provider mints access tokens and logout tokens, so the app's own checks of them are seen (accepted, 16 October 2026) |
| [ADR-128](ADR-128.md) | A Dependabot pull request that only moves pins owes the decision records no line (accepted, 20 October 2026) |
| [ADR-129](ADR-129.md) | stackvet.dev has a page for each framework, built from the data the checks keep, and never copied (accepted, 20 October 2026) |
| [ADR-130](ADR-130.md) | Level 3 is the owner's opt-in, `[app] level = 3`, and it only ever raises the level (accepted, 20 October 2026) |
| [ADR-131](ADR-131.md) | The report says what the AI coding tool did that is worth a look, as information that decides nothing (accepted, 21 October 2026) |
| [ADR-132](ADR-132.md) | stackvet.dev is built from the repository first, and where it is hosted is chosen after (accepted, 22 October 2026) |
| [ADR-133](ADR-133.md) | AC.4.1 credits a review by hand only when the reviewer is not the person who asked for the code (accepted, 23 October 2026) |
| [ADR-134](ADR-134.md) | A seed may print numbered values, and each sign-up takes the next one unused (accepted, 23 October 2026) |
| [ADR-135](ADR-135.md) | A program started with its arguments as a list, and no shell, is not a shell command built from a value (accepted, 23 October 2026) |
| [ADR-136](ADR-136.md) | A Rust module that is compiled only for tests is read as test code (accepted, 23 October 2026) |
| [ADR-137](ADR-137.md) | A check that reads code for a setting does not read it in a comment, or in a file of another programming language (accepted, 23 October 2026) |
| [ADR-138](ADR-138.md) | The path rule reads where a value came from: a path built from a request is a thing to fix (accepted, 23 October 2026) |
| [ADR-139](ADR-139.md) | One reading of test code for every check (accepted, 23 October 2026) |
| [ADR-140](ADR-140.md) | The bill of materials leaves out lockfiles in test code and in folders named as not the app (accepted, 23 October 2026) |
| [ADR-141](ADR-141.md) | stackvet.toml sets environment variables for the copy of the app `sv` runs, and the starter stops putting the app's data under /tmp (accepted, 24 October 2026) |
| [ADR-142](ADR-142.md) | The code rules read the calls inside Rust's standard macros, and a clean result in Rust says so (accepted, 24 October 2026) |
| [ADR-143](ADR-143.md) | The coding rules carry a few lines for each framework the app's package lists name, each tied to the check that catches its fault (accepted, 24 October 2026) |
| [ADR-144](ADR-144.md) | `sv rules` adds a one-line import of `AGENTS.md` to the app's own `CLAUDE.md`, and says which tools still will not see the rules (accepted, 24 October 2026) |
| [ADR-145](ADR-145.md) | The dynamic-code rule reads Rust's embedded interpreters, and a clean Rust app is credited in part (accepted, 24 October 2026) |
| [ADR-146](ADR-146.md) | The TURN relay's own settings file is read, and a relay that refuses internal addresses is credited in part (accepted, 24 October 2026) |
| [ADR-147](ADR-147.md) | An authorization server's own published details, and its lifetime settings, are read (accepted, 24 October 2026) |
| [ADR-148](ADR-148.md) | The test provider mints access tokens for another person and with less scope, and the app's API is asked about both (accepted, 24 October 2026) |
| [ADR-149](ADR-149.md) | Malformed messages are sent to the app's signaling WebSocket, and a normal one after them must still be answered (proposed, 24 October 2026) |
| [ADR-150](ADR-150.md) | CodeQL's C/C++ suite reads an app's C and C++ code without building it, for integer overflows (V1.4.2) (accepted, 24 October 2026) |

## Names in the records

Records written before 8 October 2026 name the files and tools as they were called then: `securevibe.toml`,
`securevibe_*` tools, `~/.config/securevibe`, `.securevibe-report.lock`, `org.securevibe.run`. Since the rename (ADR-062)
the code writes and reads `stackvet.toml`, `stackvet_*`, `~/.config/stackvet`, `.stackvet-report.lock` and
`org.stackvet.*`, and still reads the old names until 1 January 2027. The records are not rewritten for this; the third
weekly review (12 October 2026) noted it here once instead of in every record.

## Where v1's records disagree with what v1 built

v1's records are archived on the `v1` branch and are not edited; the owner chose on 30 September 2026 to record
their corrections here instead (BACKLOG, "Records that disagree with what was built"). Each was checked against
the `v1` branch and its history on that day.

- **ADR-001** cites V15.1.2 (keep an inventory of third-party components) for choosing "TypeScript everywhere
  with a single npm install". A language choice is not an inventory; ADR-007, which ships the inventory, cites
  the same requirement correctly. For `sv`, ADR-020 replaces this record's choice.
- **ADR-008** lists three AI providers: `anthropic`, `null`, and `scripted`. OpenAI and Google providers were added
  on 18 September 2026 (`7ecb4c3`, `71fae08`), with a choice of service for each step (`4b947ac`), and the record
  was not updated.
- **ADR-010** says generated code's network access is not restricted, and lists macOS `sandbox-exec` among the
  alternatives set aside. On 18 September 2026, two days after it was accepted, v1 fenced the network of the code
  it ran (`27b85e2`, `pipeline/net-fence.ts`): loopback only, through `sandbox-exec` on macOS and a network
  namespace on Linux, as v1's `docs/CONTRACTS.md` describes. The record was not updated, and v1's `README.md`
  still says "Network access is **not** restricted". For `sv`, ADR-019 replaces this record's choice.
- **ADR-012** cites "ADR-011's sibling change". No record carries the number ADR-011 (the file named `ADR-011.md`
  is ADR-014, as above). The change it means is `dca2e6c`, "Say what was read, and stop scoring code nobody read".
- **ADR-013** says its PDF writer "lays it out on A4 pages", and its consequences, updated by `2ef4149`, say the
  paper size is a setting with US Letter the default and A4 the other choice. The consequences are what v1 does.
