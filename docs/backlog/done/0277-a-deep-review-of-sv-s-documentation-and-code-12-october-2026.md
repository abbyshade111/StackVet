# A deep review of sv's documentation and code (12 October 2026), for the coordination trial

**Status:** done, 13 October 2026

Asked for by the owner on 12 October 2026, after the coordination trial (0273) was built: "a deep review of the StackVet
documentation (for consistency, accuracy, etc.) and code (both vulnerabilities and code quality)". Run by session
stackvet-orchestrator, on Mythos 5.1, against `main` at the start of the review (`git log -1` at the time: see the
write-up below). Seven read-only reviewers, one per area (the documents; the MCP server; the fence and `sv run`; what
`sv` writes and shows, and secrets; the checks' honesty and citations; code quality; the tools and CI), with the
earlier reviews' items named to each so nothing is found twice; every finding verified by this session against the code
before it was written down.

**Each finding is an item of its own** (`backlog.py new`), listed here by number when filed, and queued on the
coordination board (`docs/coordination/board.md`), from which the two trial sessions take them through `assign`;
a finding's item says so on its own first lines, and is not taken from the roadmap. The write-up of what was found
sound goes here too, so the next review does not re-read it.

**Run against `main` at 7ec72e0a (12 October 2026, evening).** Seven reviewers, 1.6 million tokens of reading in all; every
finding below was then read against the code by this session before it was filed. 57 items, 0278 to 0334, in severity
order, all queued on the coordination board (the two that are the owner's settings are on hold until the owner decides).

**High (4):**
- 0278 (code quality): A page with one non-ASCII capital before a form can crash the whole running-app stage (tags() lowers with to_lowercase)
- 0279 (documentation): README, ARCHITECTURE, and the MCP catalog count five report files where sv writes seven
- 0280 (checks' honesty): A crash or silence at the attempt past the limit is credited as pushing back (V6.3.1, V6.6.3)
- 0281 (checks' honesty): ast.weak-cipher credits V11.3.1 and V11.3.2 in full for finding none of six named ciphers

**Medium (26):**
- 0282 (MCP server): A named pipe where the build-loop record or report.json sits hangs the MCP server for good
- 0283 (secrets): The AI coding tool's own settings are quoted into the report without redaction
- 0284 (secrets): redact_text cuts only the first line of a private-key block, so the key's body is kept
- 0285 (secrets): A response header's value is redacted without its name, so only the nine listed headers lose a random token
- 0286 (secrets): A sealed report folder is proven sv's wherever it sits and whenever it was written, so a copied or old report passes as current
- 0287 (tools and CI): The Docker Hub token is on the runner's disk while a branch's own tests and build scripts run
- 0288 (tools and CI): py7zr is pinned by version only, so its eight dependencies float in the job that holds the Docker Hub token
- 0289 (tools and CI): A branch's modified rust.yml could publish and sign a latest image that action.yml accepts as genuine (the owner's setting)
- 0290 (tools and CI): The required checks are judged by scripts a pull request can edit (the owner's setting: code-owner review for them)
- 0291 (code quality): Four readers of requirements.txt, each with its own comment and option rules, and no test holding them to one another
- 0292 (code quality): manifest_changed reports not changed when the manifest cannot be read at the end of the run
- 0293 (code quality): Two data files are compiled in, bypassing the one data path, and the guard test cannot see them
- 0294 (documentation): ARCHITECTURE's rule of what sv writes into somebody's folder leaves out stackvet.toml and design-decisions.md, which sv review writes
- 0295 (documentation): ARCHITECTURE's counts are stale: 435 design entries, 91 records, and every crate's line count has moved
- 0296 (documentation): CONTRIBUTING.md states the no-network rule without its two exceptions, and three of its instructions are overtaken by the 8 October layout
- 0297 (documentation): The decision-record index and ADR-057's status line disagree with the records' own status lines
- 0298 (documentation): GAP-ANALYSIS.md is a dated reading with no since note, so twelve headline faults read as open when they are built
- 0299 (documentation): BACKLOG.md's roadmap still describes the one-file-per-item decision as open
- 0300 (the fence): The gateway fence is IPv4-only: an IPv6 gateway on the fenced network is neither removed nor knocked on
- 0301 (the fence): In the plain --internal fallback, a host firewall that drops rather than refuses makes the gateway knock pass while other ports stay reachable
- 0302 (checks' honesty): ast.weak-hash-function (V11.4.1) and ast.unsafe-deserialization (V1.5.2) credit whole requirements from a deny-list
- 0303 (checks' honesty): An in-part probe credit outranks an outside tool's finding
- 0304 (checks' honesty): probe.ai-kill-switch-ignored credits C9.6.1 in full while its own scope says half was not shown
- 0305 (checks' honesty): The secrets scan credits V11.1.1 (a documented key-management policy) for finding no PEM block
- 0306 (checks' honesty): probe.uploaded-file-rendered calls an answer with no Content-Type safe
- 0307 (checks' honesty): config.versions-pinned and the SBOM complete credit give V15.1.2 whole for a lockfile

**Low (27):**
- 0308 (MCP server): resources/read answers differently for a folder that does not exist and one outside the root, which confine.rs deliberately avoids
- 0309 (MCP server): Files in a marked report folder are read whole, with no size cap, before write_report and resources/list decide anything
- 0310 (MCP server): The build-loop record's hash chain does not catch lines cut from the end, and the report's wording says removal is caught
- 0311 (MCP server): stackvet_prompts with a path follows a symlinked stackvet-report folder outside the root
- 0312 (secrets): The run note goes into compliance.md unescaped, and the health path in it is not validated
- 0313 (secrets): A commit message can forge or hide records in the note of AI commits outside a merge
- 0314 (secrets): sv dashboard writes its staging file through fs::write, which follows a link
- 0315 (secrets): Names ending in _PASS are neither scanned nor masked
- 0316 (tools and CI): windows.yml leaves the checkout's token in .git/config while the branch's build scripts and tests run
- 0317 (tools and CI): The images the suite starts are pinned by tag, not digest, in the workflow
- 0318 (tools and CI): The publish step puts the registry token in the shell script text instead of the environment
- 0319 (tools and CI): backlog.py done rewrites files through any symlink inside the repository
- 0320 (tools and CI): fuzz/seed.sh breaks on a file name with a space
- 0321 (tools and CI): prompt_trial.py has the one shell=True in tools/
- 0322 (tools and CI): CLAUDE.md says only pwned_passwords.py needs the network, and cvss4_tables.py does not hold its checkout to the pinned commit
- 0323 (code quality): An unused dependency in each of two crates
- 0324 (code quality): Long functions and long files beyond the recorded split: seven files over 3,000 lines and 68 non-test functions over 150
- 0325 (documentation): Small documentation corrections from the review of 12 October 2026: seven wording and count fixes in one pass
- 0326 (the fence): The name server's builder carries its own copy of the hardening, against ADR-019's record, and the chokepoint test does not cover it
- 0327 (the fence): sv probe reads curl's output with no size cap
- 0328 (the fence): sv probe's one DNS question accepts an answer from any sender
- 0329 (the fence): Host paths go to docker -v in the short form, so an app folder whose path holds a colon cannot be mounted and the run does not say why
- 0330 (the fence): A --tools run is not network-free: Semgrep downloads its packs from semgrep.dev at run time, and no document beside the no-network rule says so
- 0331 (checks' honesty): config.secrets-file-committed is a full credit where its sibling check is in part
- 0332 (checks' honesty): resolve's comment overstates its safety: an owner's no with no corroboration excludes requirements
- 0333 (checks' honesty): A threat reads checked in part when one of its requirements is checked and the rest are not verified
- 0334 (checks' honesty): probe.error-detail-leak's debug-mode credit rests on a finite marker list that misses ASP.NET Core and Laravel

**The owner's decisions, surfaced by the review** (not builds): a GitHub Environment restricted to `main` for the
`publish` job (0289's first half); code-owner review for `.github/workflows/**`, `tools/ci_scope.py`, `tools/adr_check.py`,
`tools/record_numbers.py`, and `action.yml` (0290); and, outside the repository, the local `.claude/settings.json`
pre-approves `git push`, `gh pr`, and `gh api` without a prompt, which reach what `CLAUDE.md` says still needs the owner's
yes (`gh api -X DELETE`, `git push --force`, `gh pr merge --admin`), and `.claude/launch.json` still names v1's `npm run
start` and three scripts under a scratch folder; those two files are not committed, so they are the owner's to change.

**Found sound** (so the next review need not re-read it): path confinement in the MCP server (absolute paths, `..`,
links, Windows forms, the home folder); every write never through a link (staging with `create_new` and rename; the record
with `O_NOFOLLOW`); the ADR-041 refusal of a folder with someone else's files; the seals' mechanics, key permissions, and
constant-time compare; the fencing of app text in every tool answer; the 1 MiB request cap, batch refusal, and the check
time limit; `seen.json`'s redaction by name and pattern (bar the two header and PEM gaps filed); `Secret` redacting on
construction with no accessor; HTML, Markdown, SARIF, JSON, and terminal escaping (bar the run note filed); the fence made
`--internal` and verified by asking Docker, every container through `prepared`, no privileged or host-network or socket
mount anywhere; nothing from the app interpolated into a host shell; the install step's refusals of `-r`, `-e`, index
options, URLs, non-registry packages, and scripts; `sv probe`'s address rules, request count, and redirect handling;
cleanup by label on every exit including panics; every workflow `uses:` pinned to a full SHA with top-level `contents:
read` and no `pull_request_target`, no cache, no `curl | sh`; the Decision-records and number checks failing closed;
`install.sh` downloading nothing and the Dockerfile's digest-pinned bases and non-root user; the tools' list-form
subprocess calls and real self-tests; `status_of` (a finding beats every credit, nothing below Checked becomes Checked);
`resolve` adding and never removing (bar the comment filed); 26 of 35 AST rules finding-only and no credit while any
language is unread; the running-app suites' positive controls and in-part markings (bar the eight filed); the applicability
data's conditions; 24 commands and their flags, the twelve MCP tools, the exit codes, the nine statuses, and every path and
date in the documents; `unwrap`/`expect`/`unreachable` in non-test code all on values that cannot fail; 3,207 tests, none
asserting only no-panic. Not verified: anything that needs a build or a run (the pipe hang, the Unicode crash, IPv6 on the
owner's Colima, macOS case folding); each item says so where it applies. One correction to the review's own brief: the
outside tools run on the owner's computer with a cleared environment (ADR-032), not in containers; the code matches the record.

**Next:** a follow-on review of efficiency (`sv`'s run speed, CI and test time, the size of MCP answers, build time),
asked for by the owner on 12 October 2026, filed the same way and queued behind these.

**Done, 13 October 2026:** the review is complete and each of its 57 findings is an item of its own (0278 to 0334), handed out on the coordination board (0273) or left open with its own status; the ones still open are taken the normal way.
