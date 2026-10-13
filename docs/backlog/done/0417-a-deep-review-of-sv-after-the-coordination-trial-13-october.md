# A deep review of sv after the coordination trial (13 October 2026): the last 48 hours' code, quality, requirements and tests, coverage, usability, and documentation

**Status:** done, 13 October 2026

Asked for by the owner on 13 October 2026, after the coordination trial (0273) and the two reviews of 12 October (0277,
0335) were built and their items landed: "another deep review after the latest round", with every finding logged as a
backlog item. Run by session stackvet-deep-review, on Mythos 5.1, against `main` at 19aec8d7. Eight read-only reviewers,
one per area the owner named: the code committed in the last 48 hours (three reviewers: `sv-check`; `sv-run`, `sv-report`,
`sv-manifest`, `sv-scan`, `sv-frameworks`; `sv-cli`, `tools/`, CI, and `data/`); code quality and duplicated functions;
whether the checks and their tests match the text of the requirements they cite; whether the stated coverage is accurate
and how the requirements not assessed could be; usability for people who have never coded, and how `sv` could lessen
their burden by showing them the code a question rests on; and the documentation, where it could be streamlined and
where a diagram or a chart would replace text. Each reviewer was given the 12 October reviews' items and the open list so
nothing is found twice, and every finding is verified by this session against the code before it is filed.

**Each finding is an item of its own** (`backlog.py new`), listed here by number when filed, with the write-up of what
was found sound, so the next review does not re-read it. The coordination board is closed (0273); the items are taken
the normal way, or queued on the board if the owner opens it for this burst (0416).

**Run against `main` at 19aec8d7 (13 October 2026, evening), after 524 commits in the 48 hours before it.** Eight
reviewers, about 2.5 million tokens of reading in all; every finding below was read against the code, the data, or the
document by this session before it was filed, and the ones that change a credit were checked line by line (the rule
data, `policy_answers`, `accepted`, the census, the coverage tables). Nothing was built or run beyond the tools'
read-only modes: `python3 tools/coverage.py --check` passes on the checkout, as do the four tools' self-tests. 83
items, 0418 to 0500, in severity order; the last fourteen are proposals, not faults, and answer the owner's question of
how the requirements no check speaks to could be assessed.

**High (1):**
- 0418 (requirements and tests (citations)): `ast.dynamic-code-execution` credits V1.3.2 whole for Java without looking for SpEL, and three more AST rules credit whole from call lists with gaps

**Medium (23):**
- 0419 (code: `sv-check` (checks' honesty)): The CSP-reporting credit (V3.4.7) is whole "on all N routes" while a named route with no policy at all is skipped
- 0420 (code: `sv-check` (checks' honesty)): A whole V3.5.1 credit can rest on a `creates` entry refused for any reason: no control shows that request works with its token
- 0421 (CI and `tools/`): A backlog-only pull request skips the board test, so a done note on a queued item turns main red after a green check
- 0422 (CI): A pull request that edits release.yml runs it with contents, id-token, and attestations write before any merge (the owner's setting)
- 0423 (code quality and duplication): The reading of answers past the sign-in and code limits is one algorithm written twice, and the copies have drifted
- 0424 (usability): The report's "What to do next" speaks the terminal's language and never names `sv review` or the full run
- 0425 (usability): A person's own decision, typed to the tool, can only become theirs by hand-editing security-notes.md; `sv review` has no "this is mine"
- 0426 (usability): A "not verified" row never says what would verify it, though sv knows and the website's example promises it
- 0427 (usability): The questions to the person are asked in the standard's own jargon, and only the AI tool stands between it and them
- 0428 (usability): The guide says the setup prompts are untried; the setup page says they were tried in Antigravity and the setup worked
- 0429 (`sv-report`, coverage accuracy, and `sv-cli`): After ADR-111, the report's "Outside tools" section, the coverage tables, and `sv bundle --help` still say the tools need `--tools`
- 0430 (requirements and tests (citations)): About 75 checks are printed in the coverage tables as able to mark *checked* where the code only ever credits in part, and no gate holds the list
- 0431 (requirements and tests (citations)): `secrets.credential-assignment` names V13.2.3 (no default credentials) on every hard-coded key, after filtering out the values a default credential has
- 0432 (requirements and tests (citations)): One missing header marks all four header requirements *needs attention*, and one missing cookie attribute marks both cookie requirements
- 0433 (coverage accuracy): The website's "StackVet can check" column is the "can settle" number, which counts 40 requirements it can only ever fail
- 0434 (coverage accuracy): V12.1.2 is credited *checked* by a Semgrep clean run on any Go or Python app, while the hand check beside it says sv does not check it
- 0435 (documentation): README's "Where it is" is 2,200 words under one heading, and the heading does not say what is under it
- 0436 (documentation): The path from "I built an app" to "I have a report" is drawn nowhere a person reads, though the paper's figure already draws it
- 0437 (documentation): The no-network rule is stated six times with two different exception lists since ADR-111, and wants one home and a picture
- 0438 (documentation): The nine evidence tiers are listed in five places and drawn in none, and no list says who can grant each
- 0439 (documentation): `sv review` and the seals are explained in full twice for the same reader, with different details in each
- 0440 (documentation): The getting-started guide carries three readers' material in one 6,300-word file, and the setup is the smallest third
- 0441 (documentation): `docs/` mixes live references with five dated investigations and two generated files, and no index says which is which

**Low (45):**
- 0442 (code: `sv-check`): After a rate limiter's wait, an AI finding names the limiter's answer as its evidence, not the answer it judged
- 0443 (code: `sv-check`): Commit subjects and trailers are quoted into the AI-commits note unredacted, and a key there cannot be taken out of history
- 0444 (code: `sv-check`): The group A credits can be whole with the health path's answer left out by the rate limiter, and then read "the app's answers on ,"
- 0445 (code: `sv-check`): The bill of materials lists no modules, saying nothing, when the stdlib data file cannot be read
- 0446 (code: `sv-check`): `sign_out_visible` is now always in part, but its chain and comment still say "one drawn is one sample"
- 0447 (code: `sv-run`, `sv-report`, `sv-scan`): A threat's "n of m requirements checked" counts a requirement checked in part as checked
- 0448 (code: `sv-run`, `sv-report`, `sv-scan`): A tab before `--hash` in requirements.txt makes the whole file read as unpinned, since the 0291 reader splits on a space only
- 0449 (code: `sv-run`, `sv-report`, `sv-scan`): `sbom_document`'s fallback writes a CycloneDX file with no components and no caveat, though its comment says the file says so
- 0450 (code: `sv-run`, `sv-report`, `sv-scan`): On an older Docker that gives IPv4-only networks a link-local IPv6 address, the fence refuses every run with daemon.json advice that cannot apply
- 0451 (code: `sv-cli` and the MCP server): `sv review` reads report.json whole, through a link or a pipe, and repeats its count without the seal
- 0452 (code: `sv-cli` and the MCP server): When every finding is fixed, the next check never names the ones that went
- 0453 (code: `sv-cli` and the MCP server): A finding the tool never saw in full is listed as "unchanged since your check" after a check that came in parts
- 0454 (code: `sv-cli` and the MCP server): resources/list reads and parses report.json a second time, whole, for each sealed folder, after the seal checked it
- 0455 (code quality and duplication): Five base64 implementations in two crates, with two decoders that disagree on the alphabet
- 0456 (code quality and duplication): Three wildcard matchers, two of them in the same crate
- 0457 (code quality and duplication): Seven hand-rolled "move to a character boundary" loops in three files
- 0458 (code quality and duplication): Line number from a byte offset is written nine times, in two conventions
- 0459 (code quality and duplication): The secret rules are loaded by six separate paths, each with its own story when they cannot be read
- 0460 (code quality and duplication): The four-line JSON data-file reader is pasted into every `load`
- 0461 (code quality and duplication): Six small pairs that should be one function each
- 0462 (code quality and duplication): The test helpers are written once per file: scratch folders in 52 files, the binary runner in 28, and five more
- 0463 (code quality and duplication): Two page readers compile a regular expression on every call
- 0464 (code quality and duplication): tools/: `write_text` is defined in twelve scripts, `slug` in two, and `git` three ways
- 0465 (code quality and duplication): `allowed + 2` in the sign-in guessing guard can wrap in a release build
- 0466 (requirements and tests (citations)): `probe.plain-http-served` credits V12.2.1 whole when port 80 refuses the connection and in part when it redirects
- 0467 (requirements and tests (citations)): ADR-096's whole credit needs only one named route, and the credit's evidence does not name the route answers it rests on
- 0468 (requirements and tests (citations)): The `out-of-band-auth` condition excludes V6.6.2 for an app whose emailed sign-in codes `sv` itself checks against V6.6.2
- 0469 (requirements and tests (citations)): `config.workflow-checkout-keeps-token` credits AC.12.2 whole for one of its two clauses
- 0470 (coverage accuracy): "Can be credited" says "the ones a check can mark *checked*", and counts three that can only ever be *checked in part*
- 0471 (coverage accuracy): The short version's "only these kinds of run could have reached N requirements" counts a tool of another language as a reach
- 0472 (coverage accuracy): COVERAGE.md's "Level 1 with no check at all (9)" calls them every app's baseline, though four apply only to an OAuth authorization server
- 0473 (usability): The first thing a person who has never opened a terminal is told to do is open a terminal, and nothing says how
- 0474 (usability): The person's last step, `sv review`, is the only step without a prompt that walks them to it
- 0475 (usability): Error messages a person meets through the tool name terminal-only fixes
- 0476 (usability): `sv --help` and every command's `--help` but `review` say nothing to a person who is not a programmer
- 0477 (usability): The starter file tells the person to read 400 lines of TOML before trusting the report, where the report already shows the answers
- 0478 (usability): Small wording for the non-programmer reader in the guide and the glossary, and the glossary's "Report" entry that 0413 misses
- 0479 (documentation): The exit codes are explained three times, in three scopes, and never as a table
- 0480 (documentation): The README's MCP section lists twelve tools in one 230-word sentence, where a table and one loop diagram would do
- 0481 (documentation): Notes for people who used StackVet before 9 October sit in the first-time reader's path
- 0482 (documentation): README points at the guide twice, and its "Reading order" sends every reader to the 240,000-word design record
- 0483 (documentation): The coverage tables are the data for a chart that would show in one glance what forty rows say, and the website will need it
- 0484 (documentation): CLAUDE.md and CONTRIBUTING.md carry the same five rules in two wordings
- 0485 (documentation): How `sv` decides what applies (the level, the chapters, the corroborated claims) is told in four places in prose and drawn nowhere
- 0486 (documentation): BACKLOG.md's roadmap is four dated amendments and a 1,000-word status snapshot, where a table from the status lines would do

**Proposals: how the requirements no check speaks to could be assessed (14):**
- 0487 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): A question to the owner carries the lines it rests on: a locator rule and a pointer on every question item
- 0488 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): Output escaping switched off, read from the template engine's own setting, and in part credited when it is on
- 0489 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): The validation schemas, limits, and transactions the code has, shown beside the V2 questions, and a stated limit tried once over
- 0490 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): V4.2.1: the server packages named beside the smuggling question, and why nothing in the fence can settle it
- 0491 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): Every sign-in pathway the code has, listed for the owner, and one the manifest does not name reported
- 0492 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): A user ends their other sessions, re-authenticating first, and an admin ends a user's, tried from the manifest
- 0493 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): The routes and their guards, as the code says them, shown beside the authorization questions
- 0494 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): A cryptographic inventory drafted from what the code shows, and weak key generation reported
- 0495 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): mTLS as a derived condition, and a client certificate accepted without a trust check reported
- 0496 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): Outside services the code names and the run saw, beside the communication question, and an undeclared one reported
- 0497 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): The personal-data columns the schema shows, and what a signed-in token carries, beside the classification question
- 0498 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): A request value read without its source, reported for V15.3.7, and the fix-within-days number counted as the owner's documentation
- 0499 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): A refused bypass attempt looked for in the app's own log, in part, with the log destinations the code names shown
- 0500 (coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2)): The test identity provider observes the OAuth client's checks (audience, issuer, back-channel logout), and the database account name is read
**The owner's decisions, surfaced by the review** (not builds): a GitHub Environment restricted to `v*` tags for the
release job (0422, the same class as 0289 for a second workflow and a second consumer, the Scoop bucket); which
generated number the website's "StackVet can check" column reads, now that "can settle" counts 40 requirements a check
can only ever fail (0433); whether the third `sv review` answer, "this is my own decision, the tool only wrote it down",
is wanted (0425: it changes how an answer reaches the owner's tiers, so its record is proposed in the claim); and
whether CLAUDE.md keeps its own copy of the five rules CONTRIBUTING.md states (0484).

**Records owed, by the items that say so:** ADR-096 (0419, 0467), ADR-053 (0418, 0420, 0466), ADR-104 (0421), ADR-108
(0422), ADR-026 (0425), ADR-059 (0430), ADR-040 (0431), ADR-047 (0432), ADR-018 (0434), the record governing
`data/applicability-v2.json` (0468), and ADR-022 (0487, which credits nothing and says so). Each is a Later entry
written in the pull request that builds the item, as CLAUDE.md asks.

**What this round shows about the last round.** Of the 91 items filed on 12 October, the fixes that landed close their
items as stated; three reviewers read the fixes for 0278, 0280, 0281, 0283, 0284, 0285, 0291, 0302, 0304, 0306, 0307,
0313, 0315, 0327, 0328, 0331, 0334, 0342, 0352, 0353, 0366, 0367, 0368, 0336, 0345 to 0348, 0362, 0286, 0309, 0311,
0343, 0356, 0358, 0300, 0301, 0410, 0414, 0329, 0355, 0326 and found each right, with these edges: the delta mode's
all-fixed and paged cases (0452, 0453), the seal's second read of `report.json` (0454), the 0291 reader's tab (0448),
and the 0333 wording's in-part case (0447). The recurring fault is still the citation: one high and six mediums here
are a check crediting a requirement whole from a list, a finding naming requirements it did not find failing, or a
public count that reads "can settle" as "can check". The fastest guard is 0430's: the credit log carrying the in-part
mark, so the generated coverage cannot say more than the code does.

**Found sound** (so the next review need not re-read it): `rust.yml`'s pins, token handling, the four-part suite with
no gap, the image built once and the same image published, `windows.yml`, `decision-records.yml`, the Dockerfile's
dependency layer, `action.yml` passing `--no-tools` only when the image knows it; every `tools/*.py` subprocess in list
form, UTF-8 and LF on every write, links refused before writes; the MCP server's spec in parts, prompts listing, pipe
refusals, one answer for outside and missing, the folder-bound seal, the delta mode's fingerprints and pages (bar
0452 and 0453), the descriptions' size test, the fence on every quoted line; `account_changes_whole` and `left_out`
(ADR-105, ADR-099) with their forged-entry tests; `report_lock`'s caps; `code_lines`' bounds and redaction; `seen.rs`'s
header cut; the setup prompts' install-after-yes rule. The fence: every container through `prepared`, the network by
one function with `--internal --ipv6=false`, `gateway_verdict` failing closed at every step, a plain network never
passing, nothing from Docker or the app reaching a shell unparsed, `--mount` quoting. The report: `status_of`
untouched, `!v.in_part` in 0303's fix right, notes and low groups never touching a status, all new app text escaped,
`kept_help_uri` keeping `javascript:` out of links. The manifest: `resolve` unchanged. In `sv-check`: the uploads'
three-file sets, `tags()` and every lowercase-then-slice site, `records_by_hash` against forged markers, the one-walk
AST query, `block_end` and the header, parameter, form, and flag redaction, `run_capped`, `answer_from`, the advisory
index. Citations: every `ai.rs` and `mcp_server.rs` credit in part with a scope naming the unshown clause; the ten
12 October fixes reachable by reading match their requirement's text with red-before tests; nine of ten
applicability conditions sampled; the escaping, confinement, docker command building, status wording, mutex, and
time-arithmetic patterns in the whole tree. Documents: ARCHITECTURE.md is the model the other pages should follow; the
guide's steps 1 to 5, the website's words, the glossary (bar one entry), the brand page, the coverage tables' "How to
read this", and the report-file and tool counts.

**Not verified:** anything that needs a build or a run: the sign-in guard's wrap (0465), the delta mode on a real
paged check (0453), the Docker version boundary for link-local IPv6 (0450), whether this repository's settings grant a
pull request run write permissions (0422), Semgrep's credit arriving on a real Go app (0434), and which tests hold
paths a document move would touch (0441). Each item says so where it applies.
