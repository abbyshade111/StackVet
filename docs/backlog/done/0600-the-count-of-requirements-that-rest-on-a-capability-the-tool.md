# The count of requirements that rest on a capability the tool answered `false` is not shown, and this run had thirteen such answers

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the clean Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: medium (honesty of the summary).

**What happened.** The tool answered `false` on 13 capability lines it did not check (the 14 October text does not ask
it to confirm them); not assessed fell to 2 and not applicable rose to 79 on the manifest's word. The summary's counts
show neither how many "does not apply" rest on an unverified `false` nor which.

**Fix.** A line in the summary, "N requirements do not apply because stackvet.toml says the app has no X, Y, Z; sv did
not check those answers", and in the applicability table the `by` of each capability answer beside it (0580's
proposals mechanism gives a way to confirm them in sv review).

**Record.** ADR-117 Later (how an unanswered or tool-answered capability is read).

**Done, 15 October 2026.** Under the counts in compliance.md and the HTML report, one line now counts the requirements
that do not apply only because stackvet.toml says no, names those answers, and says `sv` did not check them
(`sv_report::resting_on_a_no`); with none, there is no line. An exclusion on the code's word, or on a yes, is not
counted. The table "What the app says about itself" gains "who said it": a capability line records nobody, so an
answer there reads "not recorded, so read as your AI coding tool's", and the JSON report's claim carries `by =
"ai-tool"`; an unanswered one reads "nobody". Not built: a capability line has no `by` to show, and no way for a
person to confirm one in `sv review` exists; that is filed as backlog 0606. ADR-117, a Later entry of today. Held by
`crates/sv-report/src/resting_on_a_no_tests.rs` and `crates/sv-cli/tests/resting_on_a_no.rs`; each went red with its
part undone (counting an exclusion on a yes, and a capability answer shown as nobody's).
