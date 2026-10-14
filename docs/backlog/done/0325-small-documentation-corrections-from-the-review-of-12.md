# Small documentation corrections from the review of 12 October 2026: seven wording and count fixes in one pass

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: documentation. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** Seven small places where a document names a thing the reader will not find, or a count that has moved, or breaks the house standard.

**Where.** (a) `CLAUDE.md:29-31`: the `tools/` list leaves out `coordination.py`. (b) `docs/PARTIAL-CHECKS.md:8-9`: "By 8 October 2026, 63 of these 382 (22, 38, 3)"; recounted against `docs/REQUIREMENTS.md` today: 76 (31, 42, 3). (c) `docs/ARCHITECTURE.md:104`: the exit codes are also `sv run`'s (`crates/sv-cli/src/exit.rs:1-2`, `README.md:57`). (d) `docs/DASHBOARD.md:6-7`: "the first under 'Next' in `docs/BACKLOG.md`"; there is no such section; it is item 0227 under `docs/backlog/`. (e) `docs/GETTING-STARTED.md:98`: `stackvet_questions` is now `stackvet_check`'s section `questions` (9 October), and `:611` "Each is in `docs/BACKLOG.md`" is now `docs/backlog/`. (f) `docs/PROMPTS.md:38` and `:46`: "not tested" is the old label; the labels since 10 October are "Shown to work", "Tried, not shown to work", "Not tried yet", and `data/prompts.json` holds none untested. (g) Oxford comma: `README.md:57`, `CLAUDE.md:4` and `:38`, `docs/BACKLOG.md:134`, and the help text at `crates/sv-cli/src/main.rs:484`.

**Evidence.** Each checked against the code or the generated file named.

**How it bites.** Small each; together they are what makes a reader stop trusting the page.

**Fix.** One pass by one session over the seven places; each is a line. Filed as one item because one builder makes them all in one pull request and no two sessions write the same file.

**Record.** ADR-061 governs `docs/BACKLOG.md`, ADR-035 the prompt labels, ADR-053 `PARTIAL-CHECKS.md`: "unchanged, because" lines (wording only).
