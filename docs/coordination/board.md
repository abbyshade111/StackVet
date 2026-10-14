# The coordination board

Written only by the orchestrator, with `python3 tools/coordination.py`; the role is `docs/COORDINATION.md` (backlog
0273; ADR-061, Later). A building session reads this file and takes the item named beside its name, and nothing else:
a claim is still the record (`tools/backlog.py`), and this board only says who takes what, in which order, and why.

## Sessions

- honesty — arm: full — holds: 0466 (since 2026-10-13) — next: 0467, 0468, 0446, 0388, 0463
- wording — arm: full — holds: 0404 (since 2026-10-14) — next: 0409, 0386, 0411, 0459, 0369
- documentation — arm: full — holds: 0400 (since 2026-10-14) — next: 0441, 0483, 0464, 0407, 0462

## Queue

1. 0471 The short version's "only these kinds of run could have reached N requirements" counts a tool of another language as a reach — ready
2. 0383 The spec's advice to loosen the test copy's sign-in limit defeats the wrong-password check — ready
3. 0385 An account change's form token is read from the change's own address, and the refusal is blamed on the page — ready
4. 0387 Test names are matched to their requirement by shared words, which flags sound tests — ready
5. 0478 Small wording for the non-programmer reader in the guide and the glossary, and the glossary's "Report" entry that 0413 misses — ready
6. 0403 A requirement's reason for not applying never states something untrue about the app — ready
7. 0424 The report's "What to do next" speaks the terminal's language and never names `sv review` or the full run — ready
8. 0426 A "not verified" row never says what would verify it, though sv knows and the website's example promises it — ready
9. 0427 The questions to the person are asked in the standard's own jargon, and only the AI tool stands between it and them — ready
10. 0425 A person's own decision, typed to the tool, can only become theirs by hand-editing security-notes.md; `sv review` has no "this is mine" — ready
11. 0473 The first thing a person who has never opened a terminal is told to do is open a terminal, and nothing says how — ready
12. 0474 The person's last step, `sv review`, is the only step without a prompt that walks them to it — ready
13. 0475 Error messages a person meets through the tool name terminal-only fixes — ready
14. 0476 `sv --help` and every command's `--help` but `review` say nothing to a person who is not a programmer — ready
15. 0477 The starter file tells the person to read 400 lines of TOML before trusting the report, where the report already shows the answers — ready
16. 0372 Each requirement in the report stated in plain language, with checks named in words — ready
17. 0397 Each question to the owner comes with the code it rests on — ready
18. 0401 The AI tool tells the person everything it changes, and StackVet shows what changed between checks — ready
19. 0389 The typed-text check was answered in few of the second loop trial's builds — ready
20. 0487 A question to the owner carries the lines it rests on: a locator rule and a pointer on every question item — ready
21. 0456 Three wildcard matchers, two of them in the same crate — ready
22. 0460 The four-line JSON data-file reader is pasted into every `load` — ready
23. 0461 Six small pairs that should be one function each — ready
24. 0274 Read a CodeQL results file from the owner's own CI, credited only for the commit being checked — ready
25. 0377 One report holds both the running app's findings and the outside tools' — ready
26. 0341 Each signed-in request is its own docker exec process; only the anonymous probes are batched — ready
27. 0324 Long functions and long files beyond the recorded split: seven files over 3,000 lines and 68 non-test functions over 150 — hold: last and alone, after everything that touches the files it splits (the owner's order of 13 October)
28. 0290 The required checks are judged by scripts a pull request can edit (the owner's setting: code-owner review for them) — hold: the owner's decision first: code-owner review for the workflow and check scripts; see the item
29. 0338 The ten shown prompts ride in the server instructions on every turn, and again in the spec, guidance, and brief (the owner's decision) — hold: the owner's word: weighed against the loop trial's findings; see the item
30. 0263 Error-handler signals in the app's code, the static half of V16.5.4 — hold: set aside by the owner on 12 October 2026 until sv holds an app to level 3 (the item's own text)
31. 0365 mail() waits a fixed 10 s per flow when the app sends no mail — hold: build only if a real report's timings show the cost, and none does (the item's own text)
32. 0376 Build stackvet.dev from the settled words and direction C — hold: after 0433 settles which number the site reads, and the owner's go on the site
33. 0373 A short walk-through of a setup, as a video or a click-through slide show — hold: the owner's: a video or a click-through of a setup
34. 0374 Confirm the license of every tool and data source the website credits — hold: the owner's: the licenses the website credits
35. 0375 `sv connect --docker` on Windows, so StackVet can run in Docker there too — hold: waits on a decision about StackVet in Docker on Windows (ADR-108, Later)
36. 0488 Output escaping switched off, read from the template engine's own setting, and in part credited when it is on — hold: a proposal; waits on the pointer's trial (0487), the owner's word of 13 October
37. 0489 The validation schemas, limits, and transactions the code has, shown beside the V2 questions, and a stated limit tried once over — hold: a proposal; waits on the pointer's trial (0487)
38. 0490 V4.2.1: the server packages named beside the smuggling question, and why nothing in the fence can settle it — hold: a proposal the owner wants back as a yes/no question with the pointer's result
39. 0491 Every sign-in pathway the code has, listed for the owner, and one the manifest does not name reported — hold: a proposal; waits on the pointer's trial (0487)
40. 0492 A user ends their other sessions, re-authenticating first, and an admin ends a user's, tried from the manifest — hold: a proposal; waits on the pointer's trial (0487)
41. 0493 The routes and their guards, as the code says them, shown beside the authorization questions — hold: a proposal; waits on the pointer's trial (0487)
42. 0494 A cryptographic inventory drafted from what the code shows, and weak key generation reported — hold: a proposal; waits on the pointer's trial (0487)
43. 0495 mTLS as a derived condition, and a client certificate accepted without a trust check reported — hold: a proposal the owner wants back as a yes/no question with the pointer's result
44. 0496 Outside services the code names and the run saw, beside the communication question, and an undeclared one reported — hold: a proposal; waits on the pointer's trial (0487)
45. 0497 The personal-data columns the schema shows, and what a signed-in token carries, beside the classification question — hold: a proposal; waits on the pointer's trial (0487)
46. 0498 A request value read without its source, reported for V15.3.7, and the fix-within-days number counted as the owner's documentation — hold: a proposal; waits on the pointer's trial (0487)
47. 0499 A refused bypass attempt looked for in the app's own log, in part, with the log destinations the code names shown — hold: a proposal; waits on the pointer's trial (0487)
48. 0500 The test identity provider observes the OAuth client's checks (audience, issuer, back-channel logout), and the database account name is read — hold: a proposal the owner wants back as a yes/no question with the pointer's result

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
- 2026-10-13 22:30 — finished 0421 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1762; now holds 0501
- 2026-10-13 22:30 — finished 0501 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1767; now holds 0430
- 2026-10-13 22:30 — finished 0430 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1772; now holds 0429
- 2026-10-13 22:30 — finished 0429 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1775; now holds 0418
- 2026-10-13 22:30 — finished 0418 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1777; now holds 0419
- 2026-10-13 22:30 — finished 0419 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1779; now holds 0420
- 2026-10-13 22:30 — finished 0428 by wording (full): https://github.com/abbyshade111/StackVet/pull/1761; now holds 0445
- 2026-10-13 22:30 — finished 0445 by wording (full): https://github.com/abbyshade111/StackVet/pull/1766; now holds 0449
- 2026-10-13 22:30 — finished 0449 by wording (full): https://github.com/abbyshade111/StackVet/pull/1769; now holds 0448
- 2026-10-13 22:30 — finished 0448 by wording (full): https://github.com/abbyshade111/StackVet/pull/1771; now holds 0451
- 2026-10-13 22:30 — finished 0451 by wording (full): https://github.com/abbyshade111/StackVet/pull/1774; now holds 0452
- 2026-10-13 22:30 — finished 0452 by wording (full): https://github.com/abbyshade111/StackVet/pull/1778
- 2026-10-13 22:30 — finished 0437 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1763; now holds 0325
- 2026-10-13 22:30 — finished 0325 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1765; now holds 0322
- 2026-10-13 22:30 — finished 0322 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1768; now holds 0484
- 2026-10-13 22:30 — finished 0484 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1770; now holds 0482
- 2026-10-13 22:30 — finished 0482 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1773; now holds 0480
- 2026-10-13 22:30 — finished 0480 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1776
- 2026-10-13 22:30 — assigned 0431 to honesty (full) as next: lane 2 of honesty: the hard-coded-key finding cites the default-credential requirement (secrets.rs, ADR-040)
- 2026-10-13 22:30 — assigned 0432 to honesty (full) as next: lane 2 of honesty: one missing header marks four requirements failing (probes.rs, ADR-047)
- 2026-10-13 22:30 — assigned 0434 to honesty (full) as next: lane 2 of honesty: V12.1.2 credited by a Semgrep clean run on any Go or Python app (adapters.json, ADR-018)
- 2026-10-13 22:30 — assigned 0466 to honesty (full) as next: lane 2 of honesty: plain-http whole on a refusal, in part on a redirect (production.rs)
- 2026-10-13 22:30 — assigned 0467 to honesty (full) as next: lane 2 of honesty: ADR-096's one-route bar and the evidence ids (probes.rs)
- 2026-10-13 22:30 — assigned 0468 to honesty (full) as next: lane 2 of honesty: the out-of-band condition excludes V6.6.2 for emailed codes (applicability data, sv-manifest)
- 2026-10-13 22:30 — assigned 0453 to wording (full): lane 2 of wording: a paged first check marks never-shown findings as seen (mcp tools.rs, parts.rs)
- 2026-10-13 22:30 — assigned 0454 to wording (full) as next: lane 2 of wording: resources/list reads report.json a second time, uncapped (resources.rs)
- 2026-10-13 22:30 — assigned 0450 to wording (full) as next: lane 2 of wording: the fence's daemon.json advice on an older Docker (docker.rs)
- 2026-10-13 22:30 — assigned 0393 to wording (full) as next: lane 2 of wording: the nightly release build's unused-name warning (assemble.rs)
- 2026-10-13 22:30 — assigned 0406 to wording (full) as next: lane 2 of wording: setting names in the Markdown reports written as code (sv-report markdown)
- 2026-10-13 22:30 — assigned 0399 to wording (full) as next: lane 2 of wording: an empty bind address is every interface, as 0.0.0.0 is (the example app and the check that reads it)
- 2026-10-13 22:30 — assigned 0435 to documentation (full): lane 2 of documentation: README's 2,200-word section gets headings and tables
- 2026-10-13 22:30 — assigned 0481 to documentation (full) as next: lane 2 of documentation: the migration notes move out of the first-time reader's path (README, guide)
- 2026-10-13 22:30 — assigned 0479 to documentation (full) as next: lane 2 of documentation: the exit codes as one table in a reference page (README, ARCHITECTURE, guide)
- 2026-10-13 22:30 — assigned 0439 to documentation (full) as next: lane 2 of documentation: sv review explained once, with a sequence diagram (README, guide)
- 2026-10-13 22:30 — assigned 0438 to documentation (full) as next: lane 2 of documentation: the nine tiers drawn once with who grants each (README, guide, ARCHITECTURE)
- 2026-10-13 22:30 — assigned 0436 to documentation (full) as next: lane 2 of documentation: the flow from an app to a report drawn where a person reads (README, guide)
- 2026-10-13 22:50 — finished 0420 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1780; now holds 0431
- 2026-10-13 22:50 — finished 0435 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1782; now holds 0481
- 2026-10-13 23:21 — finished 0431 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1784; now holds 0432
- 2026-10-13 23:21 — finished 0432 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1787; now holds 0434
- 2026-10-13 23:21 — finished 0434 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1790; now holds 0466
- 2026-10-13 23:21 — finished 0453 by wording (full): https://github.com/abbyshade111/StackVet/pull/1783; now holds 0454
- 2026-10-13 23:21 — finished 0454 by wording (full): https://github.com/abbyshade111/StackVet/pull/1788; now holds 0450
- 2026-10-13 23:21 — finished 0450 by wording (full): https://github.com/abbyshade111/StackVet/pull/1791; now holds 0393
- 2026-10-13 23:21 — finished 0481 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1785; now holds 0479
- 2026-10-13 23:21 — finished 0479 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1789; now holds 0439
- 2026-10-13 23:21 — finished 0439 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1792; now holds 0438
- 2026-10-13 23:21 — assigned 0469 to honesty (full) as next: lane 3 of honesty: AC.12.2 credited whole for one of two clauses (workflows.rs)
- 2026-10-13 23:21 — assigned 0470 to honesty (full) as next: lane 3 of honesty: the coverage summary's can-be-credited definition (coverage.py)
- 2026-10-13 23:21 — assigned 0472 to honesty (full) as next: lane 3 of honesty: the level-1-with-no-check paragraph, one line in coverage.py
- 2026-10-13 23:21 — assigned 0444 to honesty (full) as next: lane 3 of honesty: group A credits whole with the health path left out (probes.rs)
- 2026-10-13 23:21 — assigned 0447 to wording (full) as next: lane 3 of wording: a threat's n-of-m count reads in part as checked (sv-report threats.rs)
- 2026-10-13 23:21 — assigned 0442 to wording (full) as next: lane 3 of wording: an AI finding names the limiter's answer as its evidence (ai.rs, recording.rs)
- 2026-10-13 23:21 — assigned 0443 to wording (full) as next: lane 3 of wording: commit subjects quoted into the AI-commits note unredacted (config.rs, git.rs)
- 2026-10-13 23:21 — assigned 0440 to documentation (full) as next: lane 3 of documentation: the guide split by the day the reader needs it (GETTING-STARTED.md)
- 2026-10-13 23:21 — assigned 0485 to documentation (full) as next: lane 3 of documentation: how sv decides what applies, drawn once (README, ARCHITECTURE, home-page)
- 2026-10-13 23:21 — assigned 0486 to documentation (full) as next: lane 3 of documentation: the roadmap as a table from the status lines (BACKLOG.md, backlog.py)
- 2026-10-13 23:50 — finished 0393 by wording (full): https://github.com/abbyshade111/StackVet/pull/1794; now holds 0406
- 2026-10-13 23:50 — finished 0438 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1796; now holds 0436
- 2026-10-13 23:50 — finished 0436 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1797; now holds 0440
- 2026-10-13 23:50 — assigned 0455 to documentation (full) as next: lane 4 of documentation: five base64 implementations made one (ssh_format.rs, browser.rs, tokens.rs, public_keys.rs, docker.rs); no other lane touches these
- 2026-10-13 23:50 — assigned 0457 to documentation (full) as next: lane 4 of documentation: the seven char-boundary loops made two helpers (bundled.rs, files.rs, docker.rs)
- 2026-10-13 23:50 — assigned 0458 to documentation (full) as next: lane 4 of documentation: line number from a byte offset written once (sv-scan, sv-check readers)
- 2026-10-14 00:21 — finished 0469 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1798; from its lane, behind 0466
- 2026-10-14 00:21 — finished 0470 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1802; from its lane, behind 0466
- 2026-10-14 00:21 — finished 0472 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1804; from its lane, behind 0466
- 2026-10-14 00:21 — finished 0444 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1805; from its lane, behind 0466
- 2026-10-14 00:21 — finished 0406 by wording (full): https://github.com/abbyshade111/StackVet/pull/1800; now holds 0399
- 2026-10-14 00:21 — finished 0440 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1799; now holds 0485
- 2026-10-14 00:21 — finished 0485 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1803; now holds 0486
- 2026-10-14 00:21 — assigned 0433 to honesty (full) as next: lane 4 of honesty: the website's column reads the credited count, the owner's choice; the coverage items it waited on are done
- 2026-10-14 00:21 — assigned 0423 to honesty (full) as next: lane 4 of honesty: the past-the-limit reading written twice (signin.rs, codes.rs)
- 2026-10-14 00:21 — assigned 0384 to honesty (full) as next: lane 4 of honesty: the wrong-password check on a made-up name reads a lock as no lock (signin.rs, the example app)
- 2026-10-14 00:50 — finished 0399 by wording (full): https://github.com/abbyshade111/StackVet/pull/1807; now holds 0447
- 2026-10-14 00:50 — finished 0447 by wording (full): https://github.com/abbyshade111/StackVet/pull/1809; now holds 0442
- 2026-10-14 00:50 — finished 0442 by wording (full): https://github.com/abbyshade111/StackVet/pull/1810; now holds 0443
- 2026-10-14 00:50 — finished 0486 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1806; now holds 0455
- 2026-10-14 00:51 — assigned 0404 to wording (full) as next: lane 5 of wording: the build record survives the report folder being deleted (sv-cli)
- 2026-10-14 00:51 — assigned 0392 to wording (full) as next: lane 5 of wording: sv-run's stopped-command tests fail on Windows now and then (sv-run lib.rs)
- 2026-10-14 00:51 — assigned 0395 to wording (full) as next: lane 5 of wording: fuzz/seed.sh prints macOS's padded counts (fuzz, one sv-cli test)
- 2026-10-14 00:51 — assigned 0391 to wording (full) as next: lane 5 of wording: the second loop trial's scorer self-test fails on Windows (tools/score_loop2.py)
- 2026-10-14 00:51 — assigned 0409 to wording (full) as next: lane 5 of wording: findings.sarif renamed to a name that does not read as problems (report_files.rs, action.yml)
- 2026-10-14 00:51 — assigned 0405 to documentation (full) as next: lane 5 of documentation: a placeholder security contact is noted (SECURITY.md and the check)
- 2026-10-14 00:51 — assigned 0400 to documentation (full) as next: lane 5 of documentation: a suppression comment shows in the report as a suppressed finding (adapters.rs); no other lane touches it now
- 2026-10-14 00:51 — assigned 0276 to documentation (full) as next: lane 5 of documentation: the Docker teardown test that named another container (container_record_tests.rs)
- 2026-10-14 01:20 — finished 0433 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1812; from its lane, behind 0466
- 2026-10-14 01:20 — finished 0423 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1816; from its lane, behind 0466
- 2026-10-14 01:20 — finished 0443 by wording (full): https://github.com/abbyshade111/StackVet/pull/1813; now holds 0404
- 2026-10-14 01:20 — finished 0455 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1811; now holds 0457
- 2026-10-14 01:20 — finished 0457 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1815; now holds 0458
- 2026-10-14 01:21 — assigned 0465 to honesty (full) as next: lane 5 of honesty: allowed + 2 can wrap in a release build (signin.rs, sv-manifest's bound)
- 2026-10-14 01:21 — assigned 0446 to honesty (full) as next: lane 5 of honesty: sign_out_visible's dead in_part_if (browser.rs; 0455 is done, so nothing else touches it)
- 2026-10-14 01:21 — assigned 0388 to honesty (full) as next: lane 5 of honesty: the open-redirect rule flags a redirect to a fixed address (ast-rules.json, the example app)
- 2026-10-14 01:50 — finished 0384 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1817; from its lane, behind 0466
- 2026-10-14 01:50 — finished 0392 by wording (full): https://github.com/abbyshade111/StackVet/pull/1821; from its lane, behind 0404
- 2026-10-14 01:50 — finished 0458 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1819; now holds 0405
- 2026-10-14 01:51 — assigned 0386 to wording (full) as next: lane 6 of wording: the settings a start command gives the copy sv runs are not shown in the report (sv-run, sv-report)
- 2026-10-14 01:51 — assigned 0411 to wording (full) as next: lane 6 of wording: the OSV advisory files parsed on one thread per core (sbom/advisories.rs; honesty's lane does not touch it)
- 2026-10-14 01:51 — assigned 0441 to documentation (full) as next: lane 6 of documentation: the docs index and the move of the dated investigations; the coverage items it waited on are done
- 2026-10-14 01:51 — assigned 0483 to documentation (full) as next: lane 6 of documentation: the coverage chart generated by coverage.py, same lane as the docs move
- 2026-10-14 01:51 — assigned 0464 to documentation (full) as next: lane 6 of documentation: tools/ shared helpers (write_text, slug, git) made one
- 2026-10-14 02:20 — finished 0465 by honesty (full): https://github.com/abbyshade111/StackVet/pull/1826; from its lane, behind 0466
- 2026-10-14 02:20 — finished 0395 by wording (full): https://github.com/abbyshade111/StackVet/pull/1822; from its lane, behind 0404
- 2026-10-14 02:20 — finished 0391 by wording (full): https://github.com/abbyshade111/StackVet/pull/1827; from its lane, behind 0404
- 2026-10-14 02:20 — finished 0405 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1823; now holds 0400
- 2026-10-14 02:20 — finished 0276 by documentation (full): https://github.com/abbyshade111/StackVet/pull/1825; from its lane, behind 0400
- 2026-10-14 02:20 — assigned 0463 to honesty (full) as next: lane 6 of honesty: two page readers compile a regex per call (signed_in/mod.rs, owner_field.rs)
- 2026-10-14 02:20 — assigned 0459 to wording (full) as next: lane 7 of wording: the secret rules loaded by six paths made one (sv-cli lib.rs, main.rs, review.rs; docker.rs)
- 2026-10-14 02:20 — assigned 0369 to wording (full) as next: lane 7 of wording: small run-speed tidiness (files.rs, script.rs, docker.rs, tech-signatures.json), same lane as 0459 for docker.rs
- 2026-10-14 02:21 — assigned 0407 to documentation (full) as next: lane 7 of documentation: the build prompt and prompt 6 take in what the Antigravity trial showed (data/prompts.json, docs/prompts)
- 2026-10-14 02:21 — assigned 0462 to documentation (full) as next: lane 7 of documentation: the test helpers written once per file, shared (sv-cli tests, test_support.rs)
