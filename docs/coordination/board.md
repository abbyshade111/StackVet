# The coordination board

Written only by the orchestrator, with `python3 tools/coordination.py`; the role is `docs/COORDINATION.md` (backlog
0273; ADR-061, Later). A building session reads this file and takes the item named beside its name, and nothing else:
a claim is still the record (`tools/backlog.py`), and this board only says who takes what, in which order, and why.

## Sessions

- honesty — arm: full — holds: 0421 (since 2026-10-13) — next: 0501, 0430, 0429, 0418, 0419, 0420
- wording — arm: full — holds: 0428 (since 2026-10-13) — next: 0445, 0449, 0448, 0451, 0452
- documentation — arm: full — holds: 0437 (since 2026-10-13) — next: 0325, 0322, 0484, 0482, 0480

## Queue

1. 0431 `secrets.credential-assignment` names V13.2.3 (no default credentials) on every hard-coded key, after filtering out the values a default credential has — ready
2. 0432 One missing header marks all four header requirements *needs attention*, and one missing cookie attribute marks both cookie requirements — ready
3. 0434 V12.1.2 is credited *checked* by a Semgrep clean run on any Go or Python app, while the hand check beside it says sv does not check it — ready
4. 0466 `probe.plain-http-served` credits V12.2.1 whole when port 80 refuses the connection and in part when it redirects — ready
5. 0467 ADR-096's whole credit needs only one named route, and the credit's evidence does not name the route answers it rests on — ready
6. 0468 The `out-of-band-auth` condition excludes V6.6.2 for an app whose emailed sign-in codes `sv` itself checks against V6.6.2 — ready
7. 0469 `config.workflow-checkout-keeps-token` credits AC.12.2 whole for one of its two clauses — ready
8. 0470 "Can be credited" says "the ones a check can mark *checked*", and counts three that can only ever be *checked in part* — ready
9. 0433 The website's "StackVet can check" column is the "can settle" number, which counts 40 requirements it can only ever fail — ready
10. 0471 The short version's "only these kinds of run could have reached N requirements" counts a tool of another language as a reach — ready
11. 0472 COVERAGE.md's "Level 1 with no check at all (9)" calls them every app's baseline, though four apply only to an OAuth authorization server — ready
12. 0444 The group A credits can be whole with the health path's answer left out by the rate limiter, and then read "the app's answers on ," — ready
13. 0446 `sign_out_visible` is now always in part, but its chain and comment still say "one drawn is one sample" — ready
14. 0447 A threat's "n of m requirements checked" counts a requirement checked in part as checked — ready
15. 0383 The spec's advice to loosen the test copy's sign-in limit defeats the wrong-password check — ready
16. 0384 The wrong-password check on a made-up name reads a lock on real accounts as no limit — ready
17. 0385 An account change's form token is read from the change's own address, and the refusal is blamed on the page — ready
18. 0388 The open-redirect rule flags a redirect to a fixed address — ready
19. 0387 Test names are matched to their requirement by shared words, which flags sound tests — ready
20. 0478 Small wording for the non-programmer reader in the guide and the glossary, and the glossary's "Report" entry that 0413 misses — ready
21. 0453 A finding the tool never saw in full is listed as "unchanged since your check" after a check that came in parts — ready
22. 0454 resources/list reads and parses report.json a second time, whole, for each sealed folder, after the seal checked it — ready
23. 0442 After a rate limiter's wait, an AI finding names the limiter's answer as its evidence, not the answer it judged — ready
24. 0443 Commit subjects and trailers are quoted into the AI-commits note unredacted, and a key there cannot be taken out of history — ready
25. 0450 On an older Docker that gives IPv4-only networks a link-local IPv6 address, the fence refuses every run with daemon.json advice that cannot apply — ready
26. 0393 Nightly: release build warns: unused name in assemble.rs guarded — ready
27. 0391 Nightly: Windows fails: the_second_loop_trials_scorer_passes_its_self_test — ready
28. 0392 Nightly: Windows fails: sv-run's stopped-command tests, now and then — ready
29. 0399 An empty bind address is every interface, as 0.0.0.0 is — ready
30. 0400 A suppression comment in the app's code shows in the report as a suppressed finding — ready
31. 0403 A requirement's reason for not applying never states something untrue about the app — ready
32. 0405 A placeholder security contact is noted — ready
33. 0406 Setting names in the Markdown reports are written as code, not escaped — ready
34. 0409 Rename findings.sarif to a name that does not read as problems, when no other session is working — ready
35. 0395 fuzz/seed.sh prints macOS's padded counts, so its test fails on a Mac — ready
36. 0424 The report's "What to do next" speaks the terminal's language and never names `sv review` or the full run — ready
37. 0426 A "not verified" row never says what would verify it, though sv knows and the website's example promises it — ready
38. 0427 The questions to the person are asked in the standard's own jargon, and only the AI tool stands between it and them — ready
39. 0425 A person's own decision, typed to the tool, can only become theirs by hand-editing security-notes.md; `sv review` has no "this is mine" — ready
40. 0473 The first thing a person who has never opened a terminal is told to do is open a terminal, and nothing says how — ready
41. 0474 The person's last step, `sv review`, is the only step without a prompt that walks them to it — ready
42. 0475 Error messages a person meets through the tool name terminal-only fixes — ready
43. 0476 `sv --help` and every command's `--help` but `review` say nothing to a person who is not a programmer — ready
44. 0477 The starter file tells the person to read 400 lines of TOML before trusting the report, where the report already shows the answers — ready
45. 0372 Each requirement in the report stated in plain language, with checks named in words — ready
46. 0397 Each question to the owner comes with the code it rests on — ready
47. 0401 The AI tool tells the person everything it changes, and StackVet shows what changed between checks — ready
48. 0404 The build record survives the report folder being deleted, and the report says when it may have been — ready
49. 0407 The build prompt and prompt 6 take in what the Antigravity trial showed — ready
50. 0386 The settings a start command gives the copy sv runs are not shown in the report — ready
51. 0389 The typed-text check was answered in few of the second loop trial's builds — ready
52. 0487 A question to the owner carries the lines it rests on: a locator rule and a pointer on every question item — ready
53. 0423 The reading of answers past the sign-in and code limits is one algorithm written twice, and the copies have drifted — ready
54. 0465 `allowed + 2` in the sign-in guessing guard can wrap in a release build — ready
55. 0455 Five base64 implementations in two crates, with two decoders that disagree on the alphabet — ready
56. 0456 Three wildcard matchers, two of them in the same crate — ready
57. 0457 Seven hand-rolled "move to a character boundary" loops in three files — ready
58. 0458 Line number from a byte offset is written nine times, in two conventions — ready
59. 0459 The secret rules are loaded by six separate paths, each with its own story when they cannot be read — ready
60. 0460 The four-line JSON data-file reader is pasted into every `load` — ready
61. 0461 Six small pairs that should be one function each — ready
62. 0462 The test helpers are written once per file: scratch folders in 52 files, the binary runner in 28, and five more — ready
63. 0463 Two page readers compile a regular expression on every call — ready
64. 0464 tools/: `write_text` is defined in twelve scripts, `slug` in two, and `git` three ways — ready
65. 0369 Small run-speed tidiness: double UTF-8 validation, a lowercased copy of every file, and repeated stand-in readiness checks — ready
66. 0411 Parse the OSV advisory files on one thread per core, keeping the findings' order — ready
67. 0276 A Docker teardown test once named another container than the one it made (sv-run, 12 October 2026) — ready
68. 0274 Read a CodeQL results file from the owner's own CI, credited only for the commit being checked — ready
69. 0435 README's "Where it is" is 2,200 words under one heading, and the heading does not say what is under it — ready
70. 0436 The path from "I built an app" to "I have a report" is drawn nowhere a person reads, though the paper's figure already draws it — ready
71. 0438 The nine evidence tiers are listed in five places and drawn in none, and no list says who can grant each — ready
72. 0439 `sv review` and the seals are explained in full twice for the same reader, with different details in each — ready
73. 0440 The getting-started guide carries three readers' material in one 6,300-word file, and the setup is the smallest third — ready
74. 0441 `docs/` mixes live references with five dated investigations and two generated files, and no index says which is which — ready
75. 0479 The exit codes are explained three times, in three scopes, and never as a table — ready
76. 0481 Notes for people who used StackVet before 9 October sit in the first-time reader's path — ready
77. 0483 The coverage tables are the data for a chart that would show in one glance what forty rows say, and the website will need it — ready
78. 0485 How `sv` decides what applies (the level, the chapters, the corroborated claims) is told in four places in prose and drawn nowhere — ready
79. 0486 BACKLOG.md's roadmap is four dated amendments and a 1,000-word status snapshot, where a table from the status lines would do — ready
80. 0377 One report holds both the running app's findings and the outside tools' — ready
81. 0341 Each signed-in request is its own docker exec process; only the anonymous probes are batched — ready
82. 0324 Long functions and long files beyond the recorded split: seven files over 3,000 lines and 68 non-test functions over 150 — hold: last and alone, after everything that touches the files it splits (the owner's order of 13 October)
83. 0290 The required checks are judged by scripts a pull request can edit (the owner's setting: code-owner review for them) — hold: the owner's decision first: code-owner review for the workflow and check scripts; see the item
84. 0338 The ten shown prompts ride in the server instructions on every turn, and again in the spec, guidance, and brief (the owner's decision) — hold: the owner's word: weighed against the loop trial's findings; see the item
85. 0263 Error-handler signals in the app's code, the static half of V16.5.4 — hold: set aside by the owner on 12 October 2026 until sv holds an app to level 3 (the item's own text)
86. 0365 mail() waits a fixed 10 s per flow when the app sends no mail — hold: build only if a real report's timings show the cost, and none does (the item's own text)
87. 0376 Build stackvet.dev from the settled words and direction C — hold: after 0433 settles which number the site reads, and the owner's go on the site
88. 0373 A short walk-through of a setup, as a video or a click-through slide show — hold: the owner's: a video or a click-through of a setup
89. 0374 Confirm the license of every tool and data source the website credits — hold: the owner's: the licenses the website credits
90. 0375 `sv connect --docker` on Windows, so StackVet can run in Docker there too — hold: waits on a decision about StackVet in Docker on Windows (ADR-108, Later)
91. 0488 Output escaping switched off, read from the template engine's own setting, and in part credited when it is on — hold: a proposal; waits on the pointer's trial (0487), the owner's word of 13 October
92. 0489 The validation schemas, limits, and transactions the code has, shown beside the V2 questions, and a stated limit tried once over — hold: a proposal; waits on the pointer's trial (0487)
93. 0490 V4.2.1: the server packages named beside the smuggling question, and why nothing in the fence can settle it — hold: a proposal the owner wants back as a yes/no question with the pointer's result
94. 0491 Every sign-in pathway the code has, listed for the owner, and one the manifest does not name reported — hold: a proposal; waits on the pointer's trial (0487)
95. 0492 A user ends their other sessions, re-authenticating first, and an admin ends a user's, tried from the manifest — hold: a proposal; waits on the pointer's trial (0487)
96. 0493 The routes and their guards, as the code says them, shown beside the authorization questions — hold: a proposal; waits on the pointer's trial (0487)
97. 0494 A cryptographic inventory drafted from what the code shows, and weak key generation reported — hold: a proposal; waits on the pointer's trial (0487)
98. 0495 mTLS as a derived condition, and a client certificate accepted without a trust check reported — hold: a proposal the owner wants back as a yes/no question with the pointer's result
99. 0496 Outside services the code names and the run saw, beside the communication question, and an undeclared one reported — hold: a proposal; waits on the pointer's trial (0487)
100. 0497 The personal-data columns the schema shows, and what a signed-in token carries, beside the classification question — hold: a proposal; waits on the pointer's trial (0487)
101. 0498 A request value read without its source, reported for V15.3.7, and the fix-within-days number counted as the owner's documentation — hold: a proposal; waits on the pointer's trial (0487)
102. 0499 A refused bypass attempt looked for in the app's own log, in part, with the log destinations the code names shown — hold: a proposal; waits on the pointer's trial (0487)
103. 0500 The test identity provider observes the OAuth client's checks (audience, issuer, back-channel logout), and the database account name is read — hold: a proposal the owner wants back as a yes/no question with the pointer's result

## Log

- 2026-10-13 20:33 — opened by stackvet-deep-review
- 2026-10-13 20:33 — queued 0421, marked
- 2026-10-13 20:33 — queued 0430, marked
- 2026-10-13 20:33 — queued 0418, marked
- 2026-10-13 20:33 — queued 0419, marked
- 2026-10-13 20:33 — queued 0420, marked
- 2026-10-13 20:33 — queued 0431, marked
- 2026-10-13 20:33 — queued 0432, marked
- 2026-10-13 20:33 — queued 0434, marked
- 2026-10-13 20:33 — queued 0466, marked
- 2026-10-13 20:33 — queued 0467, marked
- 2026-10-13 20:33 — queued 0468, marked
- 2026-10-13 20:33 — queued 0469, marked
- 2026-10-13 20:33 — queued 0470, marked
- 2026-10-13 20:33 — queued 0433, marked
- 2026-10-13 20:33 — queued 0471, marked
- 2026-10-13 20:33 — queued 0472, marked
- 2026-10-13 20:33 — queued 0444, marked
- 2026-10-13 20:33 — queued 0446, marked
- 2026-10-13 20:33 — queued 0447, marked
- 2026-10-13 20:33 — queued 0383, marked
- 2026-10-13 20:33 — queued 0384, marked
- 2026-10-13 20:33 — queued 0385, marked
- 2026-10-13 20:33 — queued 0388, marked
- 2026-10-13 20:33 — queued 0387, marked
- 2026-10-13 20:33 — queued 0429, marked
- 2026-10-13 20:33 — queued 0437, marked
- 2026-10-13 20:33 — queued 0428, marked
- 2026-10-13 20:33 — queued 0325, marked
- 2026-10-13 20:33 — queued 0322, marked
- 2026-10-13 20:33 — queued 0478, marked
- 2026-10-13 20:33 — queued 0445, marked
- 2026-10-13 20:33 — queued 0449, marked
- 2026-10-13 20:33 — queued 0448, marked
- 2026-10-13 20:33 — queued 0451, marked
- 2026-10-13 20:33 — queued 0452, marked
- 2026-10-13 20:33 — queued 0453, marked
- 2026-10-13 20:33 — queued 0454, marked
- 2026-10-13 20:33 — queued 0442, marked
- 2026-10-13 20:33 — queued 0443, marked
- 2026-10-13 20:33 — queued 0450, marked
- 2026-10-13 20:33 — queued 0393, marked
- 2026-10-13 20:33 — queued 0391, marked
- 2026-10-13 20:33 — queued 0392, marked
- 2026-10-13 20:33 — queued 0399, marked
- 2026-10-13 20:33 — queued 0400, marked
- 2026-10-13 20:33 — queued 0403, marked
- 2026-10-13 20:33 — queued 0405, marked
- 2026-10-13 20:33 — queued 0406, marked
- 2026-10-13 20:33 — queued 0409, marked
- 2026-10-13 20:33 — queued 0395, marked
- 2026-10-13 20:33 — queued 0424, marked
- 2026-10-13 20:33 — queued 0426, marked
- 2026-10-13 20:33 — queued 0427, marked
- 2026-10-13 20:33 — queued 0425, marked
- 2026-10-13 20:33 — queued 0473, marked
- 2026-10-13 20:33 — queued 0474, marked
- 2026-10-13 20:33 — queued 0475, marked
- 2026-10-13 20:33 — queued 0476, marked
- 2026-10-13 20:33 — queued 0477, marked
- 2026-10-13 20:33 — queued 0372, marked
- 2026-10-13 20:33 — queued 0397, marked
- 2026-10-13 20:33 — queued 0401, marked
- 2026-10-13 20:33 — queued 0404, marked
- 2026-10-13 20:33 — queued 0407, marked
- 2026-10-13 20:33 — queued 0386, marked
- 2026-10-13 20:33 — queued 0389, marked
- 2026-10-13 20:33 — queued 0487, marked
- 2026-10-13 20:33 — queued 0423, marked
- 2026-10-13 20:33 — queued 0465, marked
- 2026-10-13 20:33 — queued 0455, marked
- 2026-10-13 20:33 — queued 0456, marked
- 2026-10-13 20:33 — queued 0457, marked
- 2026-10-13 20:33 — queued 0458, marked
- 2026-10-13 20:33 — queued 0459, marked
- 2026-10-13 20:33 — queued 0460, marked
- 2026-10-13 20:33 — queued 0461, marked
- 2026-10-13 20:33 — queued 0462, marked
- 2026-10-13 20:33 — queued 0463, marked
- 2026-10-13 20:33 — queued 0464, marked
- 2026-10-13 20:33 — queued 0369, marked
- 2026-10-13 20:33 — queued 0411, marked
- 2026-10-13 20:33 — queued 0276, marked
- 2026-10-13 20:33 — queued 0274, marked
- 2026-10-13 20:33 — queued 0435, marked
- 2026-10-13 20:33 — queued 0436, marked
- 2026-10-13 20:33 — queued 0438, marked
- 2026-10-13 20:33 — queued 0439, marked
- 2026-10-13 20:33 — queued 0440, marked
- 2026-10-13 20:33 — queued 0441, marked
- 2026-10-13 20:33 — queued 0479, marked
- 2026-10-13 20:33 — queued 0480, marked
- 2026-10-13 20:33 — queued 0481, marked
- 2026-10-13 20:33 — queued 0482, marked
- 2026-10-13 20:33 — queued 0483, marked
- 2026-10-13 20:33 — queued 0484, marked
- 2026-10-13 20:33 — queued 0485, marked
- 2026-10-13 20:33 — queued 0486, marked
- 2026-10-13 20:33 — queued 0377, marked
- 2026-10-13 20:33 — queued 0341, marked
- 2026-10-13 20:33 — queued 0324, on hold: last and alone, after everything that touches the files it splits (the owner's order of 13 October), marked
- 2026-10-13 20:33 — queued 0290, on hold: the owner's decision first: code-owner review for the workflow and check scripts; see the item, marked
- 2026-10-13 20:33 — queued 0338, on hold: the owner's word: weighed against the loop trial's findings; see the item, marked
- 2026-10-13 20:33 — queued 0263, on hold: set aside by the owner on 12 October 2026 until sv holds an app to level 3 (the item's own text), marked
- 2026-10-13 20:33 — queued 0365, on hold: build only if a real report's timings show the cost, and none does (the item's own text), marked
- 2026-10-13 20:33 — queued 0376, on hold: after 0433 settles which number the site reads, and the owner's go on the site, marked
- 2026-10-13 20:33 — queued 0373, on hold: the owner's: a video or a click-through of a setup, marked
- 2026-10-13 20:33 — queued 0374, on hold: the owner's: the licenses the website credits, marked
- 2026-10-13 20:33 — queued 0375, on hold: waits on a decision about StackVet in Docker on Windows (ADR-108, Later), marked
- 2026-10-13 20:33 — queued 0488, on hold: a proposal; waits on the pointer's trial (0487), the owner's word of 13 October, marked
- 2026-10-13 20:33 — queued 0489, on hold: a proposal; waits on the pointer's trial (0487), marked
- 2026-10-13 20:33 — queued 0490, on hold: a proposal the owner wants back as a yes/no question with the pointer's result, marked
- 2026-10-13 20:33 — queued 0491, on hold: a proposal; waits on the pointer's trial (0487), marked
- 2026-10-13 20:33 — queued 0492, on hold: a proposal; waits on the pointer's trial (0487), marked
- 2026-10-13 20:33 — queued 0493, on hold: a proposal; waits on the pointer's trial (0487), marked
- 2026-10-13 20:33 — queued 0494, on hold: a proposal; waits on the pointer's trial (0487), marked
- 2026-10-13 20:33 — queued 0495, on hold: a proposal the owner wants back as a yes/no question with the pointer's result, marked
- 2026-10-13 20:33 — queued 0496, on hold: a proposal; waits on the pointer's trial (0487), marked
- 2026-10-13 20:33 — queued 0497, on hold: a proposal; waits on the pointer's trial (0487), marked
- 2026-10-13 20:33 — queued 0498, on hold: a proposal; waits on the pointer's trial (0487), marked
- 2026-10-13 20:33 — queued 0499, on hold: a proposal; waits on the pointer's trial (0487), marked
- 2026-10-13 20:33 — queued 0500, on hold: a proposal the owner wants back as a yes/no question with the pointer's result, marked
- 2026-10-13 20:44 — session honesty added, arm full
- 2026-10-13 20:44 — session wording added, arm full
- 2026-10-13 20:44 — session documentation added, arm full
- 2026-10-13 20:44 — assigned 0421 to honesty (full): lane 1 of honesty: a two-line CI fix that can turn main red for anyone, first
- 2026-10-13 20:44 — assigned 0430 to honesty (full) as next: lane 1 of honesty: the credit log carries the in-part mark, the gate the rest of the lane leans on
- 2026-10-13 20:44 — assigned 0429 to honesty (full) as next: lane 1 of honesty: the ADR-111 wording, in this lane because it touches coverage.py like 0430
- 2026-10-13 20:44 — assigned 0418 to honesty (full) as next: lane 1 of honesty: the high finding, four AST rules crediting whole from call lists
- 2026-10-13 20:44 — assigned 0419 to honesty (full) as next: lane 1 of honesty: the CSP-reporting credit whole on a route with no policy (probes.rs, ADR-096)
- 2026-10-13 20:44 — assigned 0420 to honesty (full) as next: lane 1 of honesty: the forgery credit with no control (forgery.rs, ADR-053)
- 2026-10-13 20:44 — assigned 0428 to wording (full): lane 1 of wording: the guide and the setup page disagree on whether the prompts were tried; a test holds them together
- 2026-10-13 20:44 — assigned 0445 to wording (full) as next: lane 1 of wording: the bill of materials silent when the stdlib file cannot be read (runs_on.rs)
- 2026-10-13 20:44 — assigned 0449 to wording (full) as next: lane 1 of wording: the SBOM fallback's comment and behavior disagree (sv-report lib.rs)
- 2026-10-13 20:44 — assigned 0448 to wording (full) as next: lane 1 of wording: a tab before --hash reads the whole requirements file as unpinned (ecosystems.rs)
- 2026-10-13 20:44 — assigned 0451 to wording (full) as next: lane 1 of wording: sv review's uncapped read of report.json (review.rs)
- 2026-10-13 20:44 — assigned 0452 to wording (full) as next: lane 1 of wording: the delta mode never names the findings that went when all are fixed (mcp tools.rs)
- 2026-10-13 20:44 — assigned 0437 to documentation (full): lane 1 of documentation: the no-network rule's six statements, wrong in three files today
- 2026-10-13 20:44 — assigned 0325 to documentation (full) as next: lane 1 of documentation: the seven small corrections from the 12 October review, same files as 0437
- 2026-10-13 20:44 — assigned 0322 to documentation (full) as next: lane 1 of documentation: CLAUDE.md's network note and the pinned checkout in cvss4_tables.py
- 2026-10-13 20:44 — assigned 0484 to documentation (full) as next: lane 1 of documentation: CLAUDE.md stays self-contained and CONTRIBUTING points to it (the owner's choice)
- 2026-10-13 20:44 — assigned 0482 to documentation (full) as next: lane 1 of documentation: README's second pointer and its reading order, same file as 0437
- 2026-10-13 20:44 — assigned 0480 to documentation (full) as next: lane 1 of documentation: README's MCP section as a table, same file as 0437
- 2026-10-13 21:03 — queued 0501, marked
- 2026-10-13 21:03 — assigned 0501 to honesty (full) into the lane after 0421: a flaky test in sv-run's own suite that can fail any pull request; found by honesty on 0421, so it goes next on that line
