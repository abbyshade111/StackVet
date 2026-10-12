# The coordination board

Written only by the orchestrator, with `python3 tools/coordination.py`; the role is `docs/COORDINATION.md` (backlog
0273; ADR-061, Later). A building session reads this file and takes the item named beside its name, and nothing else:
a claim is still the record (`tools/backlog.py`), and this board only says who takes what, in which order, and why.

## Sessions

- builder-full — arm: full — holds: nothing
- builder-none — arm: none — holds: nothing

## Queue

1. 0263 Error-handler signals in the app's code, the static half of V16.5.4 — ready
2. 0278 A page with one non-ASCII capital before a form can crash the whole running-app stage (tags() lowers with to_lowercase) — ready
3. 0279 README, ARCHITECTURE, and the MCP catalog count five report files where sv writes seven — ready
4. 0280 A crash or silence at the attempt past the limit is credited as pushing back (V6.3.1, V6.6.3) — ready
5. 0281 ast.weak-cipher credits V11.3.1 and V11.3.2 in full for finding none of six named ciphers — ready
6. 0282 A named pipe where the build-loop record or report.json sits hangs the MCP server for good — ready
7. 0283 The AI coding tool's own settings are quoted into the report without redaction — ready
8. 0284 redact_text cuts only the first line of a private-key block, so the key's body is kept — ready
9. 0285 A response header's value is redacted without its name, so only the nine listed headers lose a random token — ready
10. 0286 A sealed report folder is proven sv's wherever it sits and whenever it was written, so a copied or old report passes as current — ready
11. 0287 The Docker Hub token is on the runner's disk while a branch's own tests and build scripts run — ready
12. 0288 py7zr is pinned by version only, so its eight dependencies float in the job that holds the Docker Hub token — ready
13. 0289 A branch's modified rust.yml could publish and sign a latest image that action.yml accepts as genuine (the owner's setting) — hold: the owner's decision first; see the item
14. 0290 The required checks are judged by scripts a pull request can edit (the owner's setting: code-owner review for them) — hold: the owner's decision first; see the item
15. 0291 Four readers of requirements.txt, each with its own comment and option rules, and no test holding them to one another — ready
16. 0292 manifest_changed reports not changed when the manifest cannot be read at the end of the run — ready
17. 0293 Two data files are compiled in, bypassing the one data path, and the guard test cannot see them — ready
18. 0294 ARCHITECTURE's rule of what sv writes into somebody's folder leaves out stackvet.toml and design-decisions.md, which sv review writes — ready
19. 0295 ARCHITECTURE's counts are stale: 435 design entries, 91 records, and every crate's line count has moved — ready
20. 0296 CONTRIBUTING.md states the no-network rule without its two exceptions, and three of its instructions are overtaken by the 8 October layout — ready
21. 0297 The decision-record index and ADR-057's status line disagree with the records' own status lines — ready
22. 0298 GAP-ANALYSIS.md is a dated reading with no since note, so twelve headline faults read as open when they are built — ready
23. 0299 BACKLOG.md's roadmap still describes the one-file-per-item decision as open — ready
24. 0300 The gateway fence is IPv4-only: an IPv6 gateway on the fenced network is neither removed nor knocked on — ready
25. 0301 In the plain --internal fallback, a host firewall that drops rather than refuses makes the gateway knock pass while other ports stay reachable — ready
26. 0302 ast.weak-hash-function (V11.4.1) and ast.unsafe-deserialization (V1.5.2) credit whole requirements from a deny-list — ready
27. 0303 An in-part probe credit outranks an outside tool's finding — ready
28. 0304 probe.ai-kill-switch-ignored credits C9.6.1 in full while its own scope says half was not shown — ready
29. 0305 The secrets scan credits V11.1.1 (a documented key-management policy) for finding no PEM block — ready
30. 0306 probe.uploaded-file-rendered calls an answer with no Content-Type safe — ready
31. 0307 config.versions-pinned and the SBOM complete credit give V15.1.2 whole for a lockfile — ready
32. 0308 resources/read answers differently for a folder that does not exist and one outside the root, which confine.rs deliberately avoids — ready
33. 0309 Files in a marked report folder are read whole, with no size cap, before write_report and resources/list decide anything — ready
34. 0310 The build-loop record's hash chain does not catch lines cut from the end, and the report's wording says removal is caught — ready
35. 0311 stackvet_prompts with a path follows a symlinked stackvet-report folder outside the root — ready
36. 0312 The run note goes into compliance.md unescaped, and the health path in it is not validated — ready
37. 0313 A commit message can forge or hide records in the note of AI commits outside a merge — ready
38. 0314 sv dashboard writes its staging file through fs::write, which follows a link — ready
39. 0315 Names ending in _PASS are neither scanned nor masked — ready
40. 0316 windows.yml leaves the checkout's token in .git/config while the branch's build scripts and tests run — ready
41. 0317 The images the suite starts are pinned by tag, not digest, in the workflow — ready
42. 0318 The publish step puts the registry token in the shell script text instead of the environment — ready
43. 0319 backlog.py done rewrites files through any symlink inside the repository — ready
44. 0320 fuzz/seed.sh breaks on a file name with a space — ready
45. 0321 prompt_trial.py has the one shell=True in tools/ — ready
46. 0322 CLAUDE.md says only pwned_passwords.py needs the network, and cvss4_tables.py does not hold its checkout to the pinned commit — ready
47. 0323 An unused dependency in each of two crates — ready
48. 0324 Long functions and long files beyond the recorded split: seven files over 3,000 lines and 68 non-test functions over 150 — ready
49. 0325 Small documentation corrections from the review of 12 October 2026: seven wording and count fixes in one pass — ready
50. 0326 The name server's builder carries its own copy of the hardening, against ADR-019's record, and the chokepoint test does not cover it — ready
51. 0327 sv probe reads curl's output with no size cap — ready
52. 0328 sv probe's one DNS question accepts an answer from any sender — ready
53. 0329 Host paths go to docker -v in the short form, so an app folder whose path holds a colon cannot be mounted and the run does not say why — ready
54. 0330 A --tools run is not network-free: Semgrep downloads its packs from semgrep.dev at run time, and no document beside the no-network rule says so — ready
55. 0331 config.secrets-file-committed is a full credit where its sibling check is in part — ready
56. 0332 resolve's comment overstates its safety: an owner's no with no corroboration excludes requirements — ready
57. 0333 A threat reads checked in part when one of its requirements is checked and the rest are not verified — ready
58. 0334 probe.error-detail-leak's debug-mode credit rests on a finite marker list that misses ASP.NET Core and Laravel — ready

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
