# An efficiency review of sv (12 October 2026): run speed, test and CI time, MCP answer size, build time

**Status:** claimed by stackvet-orchestrator, 12 October 2026

Asked for by the owner on 12 October 2026, after the deep review (0277): "a follow-on review ... this time focusing on
potential efficiency gains", before this session's switch to Opus 5.5. Run by session stackvet-orchestrator, on Mythos
5.1, against `main`, by four read-only reviewers: `sv`'s own run speed (the walk, the rules, the probes, the report);
test and CI time (from the timings of the last six runs on `main`, read from GitHub); what the MCP server sends the AI
coding tool and what it costs in tokens; build time and dependency weight. Every finding verified by this session
against the code before filing, each an item of its own queued on the coordination board behind 0277's items, with
the saving it is expected to bring stated in plain terms and marked as a measurement or an estimate.

**Run against `main` at 99e888b0, 12 October 2026, evening.** Four reviewers, about 630,000 tokens of reading; every
finding read against the code by this session before filing. 34 items, 0336 to 0369, in severity order, all queued on
the coordination board behind 0277's; one held for the owner's decision (0338; 0340 was declined the same evening, leaving its comment fix, 0359). Each item says whether its saving
is measured or an estimate, and what the fix must not lose.

**The measurements the review rests on.** CI, from the last six runs on `main`: before the three-part split the Tests
step's median was 832 s, about 125 s of it compiling (0270) and about 700 s running, with runner-to-runner noise of about
300 s; after the split (one run) the parts took 431, 392, and 358 s and the required check was green at about 7.5
minutes, down from a median of 15.1; the image job 240 s beside the suite; the publish job's rebuild 96 s. The build:
91 crates in the lock, one duplicate (`syn`, upstream's), 199 test programs of which about 81 link the 16 grammars
(178 MB of C source, 35 MB compiled), no reqwest/hyper/tokio/openssl at all. The MCP server: about 7,500 tokens per
turn standing cost (the twelve tool descriptions about 4,500, the instructions about 2,970, of which 1,900 are the ten
shown prompts), a check answer about 4,750 tokens for the flask-booking example, the spec 11,800 tokens whole and unpaged.
`sv`'s own run: every `send` in the signed-in suites is one `docker exec` (130 call sites; the batched path has one
caller), the advisory match is components × advisories × affected with two allocations per PyPI comparison, and every
app file is read and decoded five to six times across the static stage.

**High (7):**
- 0336 (MCP token cost): No delta mode: an unchanged list of findings is re-read by the AI tool after every edit
- 0337 (MCP token cost): The person's questions come whole on every check when they fit, about 540 characters each, though they do not change between edits
- 0338 (MCP token cost): The ten shown prompts ride in the server instructions on every turn, and again in the spec, guidance, and brief (the owner's decision)
- 0339 (test and CI time): Container tests are one program per file and programs run one after another, so about half the suite's time is apps coming up in sequence
- 0340 (test and CI time): Each CI part rebuilds the workspace from scratch, about 125 s each, and the no-cache reason no longer distinguishes a cache (the owner's decision)
- 0341 (sv run speed): Each signed-in request is its own docker exec process; only the anonymous probes are batched
- 0342 (sv run speed): The advisory match is components times advisories times affected ranges, with two string normalizations per PyPI comparison

**Medium (13):**
- 0343 (build time): The publish job builds the image a third time, from scratch, on the critical path, and attests a build nobody drove
- 0344 (build time): Dependencies carry full debug information in every debug build and in each of about 81 grammar-laden test binaries
- 0345 (MCP token cost): The fence sentence is pasted into ten tool descriptions, and stated again in the instructions and at the top of every fenced answer
- 0346 (MCP token cost): Tool descriptions carry how-to-behave text and the same paragraphs several times, and the instructions restate the catalog
- 0347 (MCP token cost): stackvet_spec is 47,187 characters, sent whole every time and never paged, with no test on its size
- 0348 (MCP token cost): stackvet_prompts with no argument sends all 36 prompts, twice
- 0349 (test and CI time): The image job runs its two builds, the smoke test, and the Action twice on every pull request, backlog-only ones included
- 0350 (test and CI time): The rest part is the critical path and the only undealt one: a fourth part of sv-check alone would cut the ceiling
- 0351 (sv run speed): Every file of the app is read from disk and decoded five to six times across the static stage
- 0352 (sv run speed): Thirty-odd separate regex passes per file in the secrets scan
- 0353 (sv run speed): One tree-sitter query walk per rule per file: up to 31 walks of each Python file
- 0354 (sv run speed): sv report --run walks the app folder twice more, only to compute the private files
- 0355 (sv run speed): The readiness poll sleeps a fixed 2 s between tries, so an app up in one second is seen at about four

**Low (14):**
- 0356 (build time): Incremental compilation is on in CI, where every build is fresh
- 0357 (build time): The Dockerfile has no dependency layer, and copies examples/ into the build stage
- 0358 (build time): The native comparison build is the one release build without --locked
- 0359 (build time): The no-cache comment in rust.yml is duplicated and gives two different build times
- 0360 (build time): fuzz/Cargo.lock no longer matches the manifests it locks
- 0361 (MCP token cost): Fence tags cost 141 characters per finding where 6 hex digits would do
- 0362 (MCP token cost): stackvet_guidance and stackvet_before send each prompt twice, in the text and in the structured result
- 0363 (test and CI time): Every CI part fetches all five container images, 16 to 25 s per part, where each part starts only some
- 0364 (test and CI time): The crash sweeps' time grows with the square of the request count, and is not measured since 0271
- 0365 (sv run speed): mail() waits a fixed 10 s per flow when the app sends no mail
- 0366 (sv run speed): git log --all --diff-filter=A --name-only walks every commit's diff for the secrets-file check
- 0367 (sv run speed): Quadratic grouping over findings in settle and in the report build
- 0368 (sv run speed): Outside tools run one after another, their start-ups never overlapping
- 0369 (sv run speed): Small run-speed tidiness: double UTF-8 validation, a lowercased copy of every file, and repeated stand-in readiness checks

**The owner's decisions** (held on the board): 0338, whether the ten shown prompts leave the server instructions, which
the 7 October trial readings may have rested on (a trial reading that separates the two deliveries would settle it); 0340,
whether a dependency-only cache keyed on `Cargo.lock` is worth one more input to the build in the job that holds the
Docker Hub token.

**Measured and found efficient** (so the next review need not re-read it): the walk is one for `sv check` and the static
stage, sorted once, symlinks not followed, the 2 MB cap applied before every read; every regex outside tests behind
`LazyLock`/`OnceLock` or compiled once at load; one tree-sitter parse per file in the file's own grammar, queries compiled
once; the anonymous probes in one exec; the process wait loop backing off; the clock offset read once; every fixed wait in
the suites by design (rate-limit minute, idle timeout, code expiry, scanner grace, TOTP step), each printing a line;
`status_of` once per requirement, each output file rendered once, `report.json` serialized once; fingerprints and marks
cached per file; git spawned at most six times per run; the release profile measured (design 0053) and `lto` rightly
left off; tree-sitter's features already lean; no over-featured dependency worth a line; the test-file count is not the
dominant link cost here (118 of 133 `sv-cli` files link almost nothing and drive the binary); lints and suite never
compile twice in one job; `scope`, `lints`, toolchain, checkout, sign-in each under 12 s; the credit log's cost is seconds
and its check is the only thing holding the findings-only lists honest; the 40,000 budget and 15,000 page with the pilot's
reasons, plan's and check's first-page order, the fence header only when a tag is used, the build-loop record on disk
only with a 4 MB cap. What the existing timing tests measure: that stages, tools, suites, and requests are named and
counted once, never a duration or a count of reads or execs, so none would catch a regression of any finding above; the
counting fakes already in the tree (`docker.rs:3832`, `tests/one_walk.rs`) are the right shape for holding the fixes.
Not done: nothing was run or timed by the reviewers themselves; `sv` records per-stage, per-suite, and per-request
timings in `report.json`, so each estimate can be checked against a real report before its fix is built.
