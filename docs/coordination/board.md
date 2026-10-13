# The coordination board

Written only by the orchestrator, with `python3 tools/coordination.py`; the role is `docs/COORDINATION.md` (backlog
0273; ADR-061, Later). A building session reads this file and takes the item named beside its name, and nothing else:
a claim is still the record (`tools/backlog.py`), and this board only says who takes what, in which order, and why.

## Sessions

- builder-full — arm: full — holds: 0310 (since 2026-10-12) — next: 0318
- builder-none — arm: none — holds: 0302 (since 2026-10-12) — next: 0296

## Queue

1. 0263 Error-handler signals in the app's code, the static half of V16.5.4 — ready
2. 0279 README, ARCHITECTURE, and the MCP catalog count five report files where sv writes seven — ready
3. 0287 The Docker Hub token is on the runner's disk while a branch's own tests and build scripts run — ready
4. 0288 py7zr is pinned by version only, so its eight dependencies float in the job that holds the Docker Hub token — ready
5. 0289 A branch's modified rust.yml could publish and sign a latest image that action.yml accepts as genuine (the owner's setting) — hold: the owner's decision first; see the item
6. 0290 The required checks are judged by scripts a pull request can edit (the owner's setting: code-owner review for them) — hold: the owner's decision first; see the item
7. 0291 Four readers of requirements.txt, each with its own comment and option rules, and no test holding them to one another — ready
8. 0293 Two data files are compiled in, bypassing the one data path, and the guard test cannot see them — ready
9. 0294 ARCHITECTURE's rule of what sv writes into somebody's folder leaves out stackvet.toml and design-decisions.md, which sv review writes — ready
10. 0295 ARCHITECTURE's counts are stale: 435 design entries, 91 records, and every crate's line count has moved — ready
11. 0297 The decision-record index and ADR-057's status line disagree with the records' own status lines — ready
12. 0298 GAP-ANALYSIS.md is a dated reading with no since note, so twelve headline faults read as open when they are built — ready
13. 0299 BACKLOG.md's roadmap still describes the one-file-per-item decision as open — ready
14. 0300 The gateway fence is IPv4-only: an IPv6 gateway on the fenced network is neither removed nor knocked on — ready
15. 0301 In the plain --internal fallback, a host firewall that drops rather than refuses makes the gateway knock pass while other ports stay reachable — ready
16. 0303 An in-part probe credit outranks an outside tool's finding — ready
17. 0305 The secrets scan credits V11.1.1 (a documented key-management policy) for finding no PEM block — ready
18. 0307 config.versions-pinned and the SBOM complete credit give V15.1.2 whole for a lockfile — ready
19. 0308 resources/read answers differently for a folder that does not exist and one outside the root, which confine.rs deliberately avoids — ready
20. 0311 stackvet_prompts with a path follows a symlinked stackvet-report folder outside the root — ready
21. 0317 The images the suite starts are pinned by tag, not digest, in the workflow — ready
22. 0319 backlog.py done rewrites files through any symlink inside the repository — ready
23. 0320 fuzz/seed.sh breaks on a file name with a space — ready
24. 0321 prompt_trial.py has the one shell=True in tools/ — ready
25. 0322 CLAUDE.md says only pwned_passwords.py needs the network, and cvss4_tables.py does not hold its checkout to the pinned commit — ready
26. 0323 An unused dependency in each of two crates — ready
27. 0324 Long functions and long files beyond the recorded split: seven files over 3,000 lines and 68 non-test functions over 150 — ready
28. 0325 Small documentation corrections from the review of 12 October 2026: seven wording and count fixes in one pass — ready
29. 0326 The name server's builder carries its own copy of the hardening, against ADR-019's record, and the chokepoint test does not cover it — ready
30. 0327 sv probe reads curl's output with no size cap — ready
31. 0328 sv probe's one DNS question accepts an answer from any sender — ready
32. 0329 Host paths go to docker -v in the short form, so an app folder whose path holds a colon cannot be mounted and the run does not say why — ready
33. 0330 A --tools run is not network-free: Semgrep downloads its packs from semgrep.dev at run time, and no document beside the no-network rule says so — ready
34. 0331 config.secrets-file-committed is a full credit where its sibling check is in part — ready
35. 0332 resolve's comment overstates its safety: an owner's no with no corroboration excludes requirements — ready
36. 0333 A threat reads checked in part when one of its requirements is checked and the rest are not verified — ready
37. 0334 probe.error-detail-leak's debug-mode credit rests on a finite marker list that misses ASP.NET Core and Laravel — ready
38. 0339 Container tests are one program per file and programs run one after another, so about half the suite's time is apps coming up in sequence — ready
39. 0341 Each signed-in request is its own docker exec process; only the anonymous probes are batched — ready
40. 0342 The advisory match is components times advisories times affected ranges, with two string normalizations per PyPI comparison — ready
41. 0343 The publish job builds the image a third time, from scratch, on the critical path, and attests a build nobody drove — ready
42. 0344 Dependencies carry full debug information in every debug build and in each of about 81 grammar-laden test binaries — ready
43. 0349 The image job runs its two builds, the smoke test, and the Action twice on every pull request, backlog-only ones included — ready
44. 0350 The rest part is the critical path and the only undealt one: a fourth part of sv-check alone would cut the ceiling — ready
45. 0351 Every file of the app is read from disk and decoded five to six times across the static stage — ready
46. 0352 Thirty-odd separate regex passes per file in the secrets scan — ready
47. 0353 One tree-sitter query walk per rule per file: up to 31 walks of each Python file — ready
48. 0354 sv report --run walks the app folder twice more, only to compute the private files — ready
49. 0355 The readiness poll sleeps a fixed 2 s between tries, so an app up in one second is seen at about four — ready
50. 0356 Incremental compilation is on in CI, where every build is fresh — ready
51. 0357 The Dockerfile has no dependency layer, and copies examples/ into the build stage — ready
52. 0358 The native comparison build is the one release build without --locked — ready
53. 0359 The no-cache comment in rust.yml is duplicated and gives two different build times — ready
54. 0360 fuzz/Cargo.lock no longer matches the manifests it locks — ready
55. 0363 Every CI part fetches all five container images, 16 to 25 s per part, where each part starts only some — ready
56. 0364 The crash sweeps' time grows with the square of the request count, and is not measured since 0271 — ready
57. 0365 mail() waits a fixed 10 s per flow when the app sends no mail — ready
58. 0366 git log --all --diff-filter=A --name-only walks every commit's diff for the secrets-file check — ready
59. 0367 Quadratic grouping over findings in settle and in the report build — ready
60. 0368 Outside tools run one after another, their start-ups never overlapping — ready
61. 0369 Small run-speed tidiness: double UTF-8 validation, a lowercased copy of every file, and repeated stand-in readiness checks — ready
62. 0338 The ten shown prompts ride in the server instructions on every turn, and again in the spec, guidance, and brief (the owner's decision) — hold: the owner's word: after the loop test (0257) has finished, weighed against its findings
63. 0336 No delta mode: an unchanged list of findings is re-read by the AI tool after every edit — hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
64. 0337 The person's questions come whole on every check when they fit, about 540 characters each, though they do not change between edits — hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
65. 0345 The fence sentence is pasted into ten tool descriptions, and stated again in the instructions and at the top of every fenced answer — hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
66. 0346 Tool descriptions carry how-to-behave text and the same paragraphs several times, and the instructions restate the catalog — hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
67. 0347 stackvet_spec is 47,187 characters, sent whole every time and never paged, with no test on its size — hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
68. 0348 stackvet_prompts with no argument sends all 36 prompts, twice — hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
69. 0361 Fence tags cost 141 characters per finding where 6 hex digits would do — hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
70. 0362 stackvet_guidance and stackvet_before send each prompt twice, in the text and in the structured result — hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished

## Log

- 2026-10-12 13:41 — opened by stackvet-orchestrator
- 2026-10-12 13:41 — session builder-full added, arm full
- 2026-10-12 13:41 — session builder-none added, arm none
- 2026-10-12 13:41 — queued 0263
- 2026-10-12 14:49 — queued 0278
- 2026-10-12 14:49 — queued 0279
- 2026-10-12 14:49 — queued 0280
- 2026-10-12 14:49 — queued 0281
- 2026-10-12 14:49 — queued 0282
- 2026-10-12 14:49 — queued 0283
- 2026-10-12 14:49 — queued 0284
- 2026-10-12 14:49 — queued 0285
- 2026-10-12 14:49 — queued 0286
- 2026-10-12 14:49 — queued 0287
- 2026-10-12 14:49 — queued 0288
- 2026-10-12 14:49 — queued 0289, on hold: the owner's decision first; see the item
- 2026-10-12 14:49 — queued 0290, on hold: the owner's decision first; see the item
- 2026-10-12 14:49 — queued 0291
- 2026-10-12 14:49 — queued 0292
- 2026-10-12 14:49 — queued 0293
- 2026-10-12 14:49 — queued 0294
- 2026-10-12 14:49 — queued 0295
- 2026-10-12 14:49 — queued 0296
- 2026-10-12 14:49 — queued 0297
- 2026-10-12 14:49 — queued 0298
- 2026-10-12 14:49 — queued 0299
- 2026-10-12 14:49 — queued 0300
- 2026-10-12 14:49 — queued 0301
- 2026-10-12 14:49 — queued 0302
- 2026-10-12 14:49 — queued 0303
- 2026-10-12 14:49 — queued 0304
- 2026-10-12 14:49 — queued 0305
- 2026-10-12 14:49 — queued 0306
- 2026-10-12 14:49 — queued 0307
- 2026-10-12 14:49 — queued 0308
- 2026-10-12 14:49 — queued 0309
- 2026-10-12 14:49 — queued 0310
- 2026-10-12 14:49 — queued 0311
- 2026-10-12 14:49 — queued 0312
- 2026-10-12 14:49 — queued 0313
- 2026-10-12 14:49 — queued 0314
- 2026-10-12 14:49 — queued 0315
- 2026-10-12 14:49 — queued 0316
- 2026-10-12 14:49 — queued 0317
- 2026-10-12 14:49 — queued 0318
- 2026-10-12 14:49 — queued 0319
- 2026-10-12 14:49 — queued 0320
- 2026-10-12 14:49 — queued 0321
- 2026-10-12 14:49 — queued 0322
- 2026-10-12 14:49 — queued 0323
- 2026-10-12 14:49 — queued 0324
- 2026-10-12 14:49 — queued 0325
- 2026-10-12 14:49 — queued 0326
- 2026-10-12 14:49 — queued 0327
- 2026-10-12 14:49 — queued 0328
- 2026-10-12 14:49 — queued 0329
- 2026-10-12 14:49 — queued 0330
- 2026-10-12 14:49 — queued 0331
- 2026-10-12 14:49 — queued 0332
- 2026-10-12 14:49 — queued 0333
- 2026-10-12 14:49 — queued 0334
- 2026-10-12 15:08 — queued 0336
- 2026-10-12 15:08 — queued 0337
- 2026-10-12 15:08 — queued 0338, on hold: the owner's decision first; see the item
- 2026-10-12 15:08 — queued 0339
- 2026-10-12 15:08 — queued 0340, on hold: the owner's decision first; see the item
- 2026-10-12 15:08 — queued 0341
- 2026-10-12 15:08 — queued 0342
- 2026-10-12 15:08 — queued 0343
- 2026-10-12 15:08 — queued 0344
- 2026-10-12 15:08 — queued 0345
- 2026-10-12 15:08 — queued 0346
- 2026-10-12 15:08 — queued 0347
- 2026-10-12 15:08 — queued 0348
- 2026-10-12 15:08 — queued 0349
- 2026-10-12 15:08 — queued 0350
- 2026-10-12 15:08 — queued 0351
- 2026-10-12 15:08 — queued 0352
- 2026-10-12 15:08 — queued 0353
- 2026-10-12 15:08 — queued 0354
- 2026-10-12 15:08 — queued 0355
- 2026-10-12 15:08 — queued 0356
- 2026-10-12 15:08 — queued 0357
- 2026-10-12 15:08 — queued 0358
- 2026-10-12 15:08 — queued 0359
- 2026-10-12 15:08 — queued 0360
- 2026-10-12 15:08 — queued 0361
- 2026-10-12 15:08 — queued 0362
- 2026-10-12 15:08 — queued 0363
- 2026-10-12 15:08 — queued 0364
- 2026-10-12 15:08 — queued 0365
- 2026-10-12 15:08 — queued 0366
- 2026-10-12 15:08 — queued 0367
- 2026-10-12 15:08 — queued 0368
- 2026-10-12 15:08 — queued 0369
- 2026-10-12 15:11 — dropped 0338 from the queue
- 2026-10-12 15:11 — queued 0338, on hold: the owner's word: after the loop test (0257) has finished, weighed against its findings
- 2026-10-12 15:15 — dropped 0340 from the queue
- 2026-10-12 15:18 — assigned 0278 to builder-full (full): pair 1 with 0280: both high, a small Rust fix in the signed-in suite with a fake-app witness, different files
- 2026-10-12 15:18 — assigned 0280 to builder-none (none): pair 1 with 0278: both high, a small Rust fix in the signed-in suite with a fake-app witness, different files
- 2026-10-12 15:34 — finished 0280 by builder-none (none): https://github.com/abbyshade111/StackVet/pull/1561
- 2026-10-12 15:35 — assigned 0281 to builder-none (none): pair 2 with 0282 (for builder-full when it frees): each a Rust fix with a red-first test and a record line
- 2026-10-12 15:39 — dropped 0336 from the queue
- 2026-10-12 15:39 — queued 0336, on hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
- 2026-10-12 15:39 — dropped 0337 from the queue
- 2026-10-12 15:39 — queued 0337, on hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
- 2026-10-12 15:39 — dropped 0345 from the queue
- 2026-10-12 15:39 — queued 0345, on hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
- 2026-10-12 15:39 — dropped 0346 from the queue
- 2026-10-12 15:39 — queued 0346, on hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
- 2026-10-12 15:39 — dropped 0347 from the queue
- 2026-10-12 15:39 — queued 0347, on hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
- 2026-10-12 15:39 — dropped 0348 from the queue
- 2026-10-12 15:39 — queued 0348, on hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
- 2026-10-12 15:39 — dropped 0361 from the queue
- 2026-10-12 15:39 — queued 0361, on hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
- 2026-10-12 15:39 — dropped 0362 from the queue
- 2026-10-12 15:39 — queued 0362, on hold: the owner's word: changes what the AI tool is told; after the loop test (0257) has finished
- 2026-10-12 15:43 — finished 0278 by builder-full (full): https://github.com/abbyshade111/StackVet/pull/1563
- 2026-10-12 15:43 — assigned 0282 to builder-full (full): pair 2 with 0281 (builder-none): each a Rust fix with a red-first test and a record line
- 2026-10-12 16:35 — assigned 0284 to builder-none (none) as next: pair 3 with 0285: each a redaction fix with a test key built from pieces at run time, different files (secrets.rs, seen.rs)
- 2026-10-12 16:35 — assigned 0285 to builder-full (full) as next: pair 3 with 0284: each a redaction fix with a test key built from pieces at run time, different files (seen.rs, secrets.rs)
- 2026-10-12 17:07 — finished 0282 by builder-full (full): https://github.com/abbyshade111/StackVet/pull/1570; now holds 0285
- 2026-10-12 17:07 — finished 0281 by builder-none (none): https://github.com/abbyshade111/StackVet/pull/1571; now holds 0284
- 2026-10-12 17:07 — assigned 0286 to builder-none (none) as next: pair 4 with 0283: each a medium security fix in Rust that owes a Later entry on its record and a test, different files (report_seal.rs, ai_tool.rs)
- 2026-10-12 17:07 — assigned 0283 to builder-full (full) as next: pair 4 with 0286: each a medium security fix in Rust that owes a Later entry on its record and a test, different files (ai_tool.rs, report_seal.rs)
- 2026-10-12 17:49 — finished 0285 by builder-full (full): https://github.com/abbyshade111/StackVet/pull/1573; now holds 0283
- 2026-10-12 17:49 — finished 0283 by builder-full (full): https://github.com/abbyshade111/StackVet/pull/1577
- 2026-10-12 17:49 — finished 0284 by builder-none (none); now holds 0286
- 2026-10-12 17:49 — assigned 0314 to builder-full (full): pair 5 with 0309 (builder-none, next after 0286, which shares its files): each a small fix to how sv-cli reads or writes a file it does not own
- 2026-10-12 17:49 — assigned 0313 to builder-full (full) as next: pair 6 with 0292 (for builder-none later): each makes a reader of git or the manifest say what it could not read, with a test
- 2026-10-12 17:49 — assigned 0309 to builder-none (none) as next: pair 5 with 0314 (builder-full): each a small fix to how sv-cli reads or writes a file it does not own; after 0286, which touches the same report_seal.rs
- 2026-10-12 18:28 — finished 0286 by builder-none (none): https://github.com/abbyshade111/StackVet/pull/1581; now holds 0309
- 2026-10-12 18:28 — assigned 0292 to builder-none (none) as next: pair 6 with 0313 (builder-full): each makes a reader of git or the manifest say what it could not read; after 0309, which shares report_lock.rs, in the same session
- 2026-10-12 18:47 — finished 0314 by builder-full (full): https://github.com/abbyshade111/StackVet/pull/1582; now holds 0313
- 2026-10-12 18:47 — finished 0313 by builder-full (full): https://github.com/abbyshade111/StackVet/pull/1585
- 2026-10-12 18:47 — assigned 0315 to builder-full (full): pair 7 with 0306 (for builder-none later): each a small detection fix with a red-first test, no coverage files touched
- 2026-10-12 18:47 — assigned 0304 to builder-full (full) as next: pair 8 with 0302 (for builder-none later): each marks a credit in part and regenerates the coverage files, so the two run one after the other, never at once
- 2026-10-12 18:55 — finished 0309 by builder-none (none): https://github.com/abbyshade111/StackVet/pull/1586; now holds 0292
- 2026-10-12 18:55 — assigned 0306 to builder-none (none) as next: pair 7 with 0315 (builder-full): each a small detection fix with a red-first test, no coverage files touched; uploads.rs, which nobody else holds
- 2026-10-12 19:27 — finished 0292 by builder-none (none): https://github.com/abbyshade111/StackVet/pull/1588; now holds 0306
- 2026-10-12 19:27 — assigned 0312 to builder-none (none) as next: pair 9 with 0310 (for builder-full later): each makes report text say no more than it should, with a test; markdown.rs and sv-run's manifest check, which nobody else holds
- 2026-10-12 19:46 — finished 0315 by builder-full (full): https://github.com/abbyshade111/StackVet/pull/1592; now holds 0304
- 2026-10-12 19:46 — finished 0306 by builder-none (none): https://github.com/abbyshade111/StackVet/pull/1591; now holds 0312
- 2026-10-12 19:46 — assigned 0310 to builder-full (full) as next: pair 9 with 0312 (builder-none): each makes report text say no more than it should, with a test; build_loop.rs and the report's wording, which nobody else holds
- 2026-10-12 19:46 — assigned 0316 to builder-none (none) as next: pair 10 with 0318 (for builder-full later): each a one-line CI fix keeping a token out of reach of a branch's code, in different workflow files (windows.yml, rust.yml)
- 2026-10-12 19:54 — finished 0304 by builder-full (full): https://github.com/abbyshade111/StackVet/pull/1594; now holds 0310
- 2026-10-12 19:54 — assigned 0318 to builder-full (full) as next: pair 10 with 0316 (builder-none): each a one-line CI fix keeping a token out of reach of a branch's code; rust.yml, apart from builder-none's windows.yml
- 2026-10-12 20:07 — finished 0312 by builder-none (none): https://github.com/abbyshade111/StackVet/pull/1597; now holds 0316
- 2026-10-12 20:07 — assigned 0302 to builder-none (none) as next: pair 8 with 0304 (builder-full, merged): each marks credits in part and regenerates the coverage files; builder-full holds no coverage item now (0310, then 0318)
- 2026-10-12 20:25 — finished 0316 by builder-none (none): https://github.com/abbyshade111/StackVet/pull/1598; now holds 0302
- 2026-10-12 20:26 — assigned 0296 to builder-none (none) as next: pair 11 with 0298 (for builder-full later): each a documentation fix that brings a page in line with the code and records, in files nobody else holds; no coverage files
