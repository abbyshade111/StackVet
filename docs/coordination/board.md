# The coordination board

Written only by the orchestrator, with `python3 tools/coordination.py`; the role is `docs/COORDINATION.md` (backlog
0273; ADR-061, Later). A building session reads this file and takes the item named beside its name, and nothing else:
a claim is still the record (`tools/backlog.py`), and this board only says who takes what, in which order, and why.

## Sessions

- builder-full — arm: full — holds: 0278 (since 2026-10-12)
- builder-none — arm: none — holds: 0281 (since 2026-10-12)

## Queue

1. 0263 Error-handler signals in the app's code, the static half of V16.5.4 — ready
2. 0279 README, ARCHITECTURE, and the MCP catalog count five report files where sv writes seven — ready
3. 0282 A named pipe where the build-loop record or report.json sits hangs the MCP server for good — ready
4. 0283 The AI coding tool's own settings are quoted into the report without redaction — ready
5. 0284 redact_text cuts only the first line of a private-key block, so the key's body is kept — ready
6. 0285 A response header's value is redacted without its name, so only the nine listed headers lose a random token — ready
7. 0286 A sealed report folder is proven sv's wherever it sits and whenever it was written, so a copied or old report passes as current — ready
8. 0287 The Docker Hub token is on the runner's disk while a branch's own tests and build scripts run — ready
9. 0288 py7zr is pinned by version only, so its eight dependencies float in the job that holds the Docker Hub token — ready
10. 0289 A branch's modified rust.yml could publish and sign a latest image that action.yml accepts as genuine (the owner's setting) — hold: the owner's decision first; see the item
11. 0290 The required checks are judged by scripts a pull request can edit (the owner's setting: code-owner review for them) — hold: the owner's decision first; see the item
12. 0291 Four readers of requirements.txt, each with its own comment and option rules, and no test holding them to one another — ready
13. 0292 manifest_changed reports not changed when the manifest cannot be read at the end of the run — ready
14. 0293 Two data files are compiled in, bypassing the one data path, and the guard test cannot see them — ready
15. 0294 ARCHITECTURE's rule of what sv writes into somebody's folder leaves out stackvet.toml and design-decisions.md, which sv review writes — ready
16. 0295 ARCHITECTURE's counts are stale: 435 design entries, 91 records, and every crate's line count has moved — ready
17. 0296 CONTRIBUTING.md states the no-network rule without its two exceptions, and three of its instructions are overtaken by the 8 October layout — ready
18. 0297 The decision-record index and ADR-057's status line disagree with the records' own status lines — ready
19. 0298 GAP-ANALYSIS.md is a dated reading with no since note, so twelve headline faults read as open when they are built — ready
20. 0299 BACKLOG.md's roadmap still describes the one-file-per-item decision as open — ready
21. 0300 The gateway fence is IPv4-only: an IPv6 gateway on the fenced network is neither removed nor knocked on — ready
22. 0301 In the plain --internal fallback, a host firewall that drops rather than refuses makes the gateway knock pass while other ports stay reachable — ready
23. 0302 ast.weak-hash-function (V11.4.1) and ast.unsafe-deserialization (V1.5.2) credit whole requirements from a deny-list — ready
24. 0303 An in-part probe credit outranks an outside tool's finding — ready
25. 0304 probe.ai-kill-switch-ignored credits C9.6.1 in full while its own scope says half was not shown — ready
26. 0305 The secrets scan credits V11.1.1 (a documented key-management policy) for finding no PEM block — ready
27. 0306 probe.uploaded-file-rendered calls an answer with no Content-Type safe — ready
28. 0307 config.versions-pinned and the SBOM complete credit give V15.1.2 whole for a lockfile — ready
29. 0308 resources/read answers differently for a folder that does not exist and one outside the root, which confine.rs deliberately avoids — ready
30. 0309 Files in a marked report folder are read whole, with no size cap, before write_report and resources/list decide anything — ready
31. 0310 The build-loop record's hash chain does not catch lines cut from the end, and the report's wording says removal is caught — ready
32. 0311 stackvet_prompts with a path follows a symlinked stackvet-report folder outside the root — ready
33. 0312 The run note goes into compliance.md unescaped, and the health path in it is not validated — ready
34. 0313 A commit message can forge or hide records in the note of AI commits outside a merge — ready
35. 0314 sv dashboard writes its staging file through fs::write, which follows a link — ready
36. 0315 Names ending in _PASS are neither scanned nor masked — ready
37. 0316 windows.yml leaves the checkout's token in .git/config while the branch's build scripts and tests run — ready
38. 0317 The images the suite starts are pinned by tag, not digest, in the workflow — ready
39. 0318 The publish step puts the registry token in the shell script text instead of the environment — ready
40. 0319 backlog.py done rewrites files through any symlink inside the repository — ready
41. 0320 fuzz/seed.sh breaks on a file name with a space — ready
42. 0321 prompt_trial.py has the one shell=True in tools/ — ready
43. 0322 CLAUDE.md says only pwned_passwords.py needs the network, and cvss4_tables.py does not hold its checkout to the pinned commit — ready
44. 0323 An unused dependency in each of two crates — ready
45. 0324 Long functions and long files beyond the recorded split: seven files over 3,000 lines and 68 non-test functions over 150 — ready
46. 0325 Small documentation corrections from the review of 12 October 2026: seven wording and count fixes in one pass — ready
47. 0326 The name server's builder carries its own copy of the hardening, against ADR-019's record, and the chokepoint test does not cover it — ready
48. 0327 sv probe reads curl's output with no size cap — ready
49. 0328 sv probe's one DNS question accepts an answer from any sender — ready
50. 0329 Host paths go to docker -v in the short form, so an app folder whose path holds a colon cannot be mounted and the run does not say why — ready
51. 0330 A --tools run is not network-free: Semgrep downloads its packs from semgrep.dev at run time, and no document beside the no-network rule says so — ready
52. 0331 config.secrets-file-committed is a full credit where its sibling check is in part — ready
53. 0332 resolve's comment overstates its safety: an owner's no with no corroboration excludes requirements — ready
54. 0333 A threat reads checked in part when one of its requirements is checked and the rest are not verified — ready
55. 0334 probe.error-detail-leak's debug-mode credit rests on a finite marker list that misses ASP.NET Core and Laravel — ready
56. 0336 No delta mode: an unchanged list of findings is re-read by the AI tool after every edit — ready
57. 0337 The person's questions come whole on every check when they fit, about 540 characters each, though they do not change between edits — ready
58. 0339 Container tests are one program per file and programs run one after another, so about half the suite's time is apps coming up in sequence — ready
59. 0341 Each signed-in request is its own docker exec process; only the anonymous probes are batched — ready
60. 0342 The advisory match is components times advisories times affected ranges, with two string normalizations per PyPI comparison — ready
61. 0343 The publish job builds the image a third time, from scratch, on the critical path, and attests a build nobody drove — ready
62. 0344 Dependencies carry full debug information in every debug build and in each of about 81 grammar-laden test binaries — ready
63. 0345 The fence sentence is pasted into ten tool descriptions, and stated again in the instructions and at the top of every fenced answer — ready
64. 0346 Tool descriptions carry how-to-behave text and the same paragraphs several times, and the instructions restate the catalog — ready
65. 0347 stackvet_spec is 47,187 characters, sent whole every time and never paged, with no test on its size — ready
66. 0348 stackvet_prompts with no argument sends all 36 prompts, twice — ready
67. 0349 The image job runs its two builds, the smoke test, and the Action twice on every pull request, backlog-only ones included — ready
68. 0350 The rest part is the critical path and the only undealt one: a fourth part of sv-check alone would cut the ceiling — ready
69. 0351 Every file of the app is read from disk and decoded five to six times across the static stage — ready
70. 0352 Thirty-odd separate regex passes per file in the secrets scan — ready
71. 0353 One tree-sitter query walk per rule per file: up to 31 walks of each Python file — ready
72. 0354 sv report --run walks the app folder twice more, only to compute the private files — ready
73. 0355 The readiness poll sleeps a fixed 2 s between tries, so an app up in one second is seen at about four — ready
74. 0356 Incremental compilation is on in CI, where every build is fresh — ready
75. 0357 The Dockerfile has no dependency layer, and copies examples/ into the build stage — ready
76. 0358 The native comparison build is the one release build without --locked — ready
77. 0359 The no-cache comment in rust.yml is duplicated and gives two different build times — ready
78. 0360 fuzz/Cargo.lock no longer matches the manifests it locks — ready
79. 0361 Fence tags cost 141 characters per finding where 6 hex digits would do — ready
80. 0362 stackvet_guidance and stackvet_before send each prompt twice, in the text and in the structured result — ready
81. 0363 Every CI part fetches all five container images, 16 to 25 s per part, where each part starts only some — ready
82. 0364 The crash sweeps' time grows with the square of the request count, and is not measured since 0271 — ready
83. 0365 mail() waits a fixed 10 s per flow when the app sends no mail — ready
84. 0366 git log --all --diff-filter=A --name-only walks every commit's diff for the secrets-file check — ready
85. 0367 Quadratic grouping over findings in settle and in the report build — ready
86. 0368 Outside tools run one after another, their start-ups never overlapping — ready
87. 0369 Small run-speed tidiness: double UTF-8 validation, a lowercased copy of every file, and repeated stand-in readiness checks — ready
88. 0338 The ten shown prompts ride in the server instructions on every turn, and again in the spec, guidance, and brief (the owner's decision) — hold: the owner's word: after the loop test (0257) has finished, weighed against its findings

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
