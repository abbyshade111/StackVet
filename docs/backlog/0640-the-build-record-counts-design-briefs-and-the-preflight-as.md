# The build record counts design briefs and the preflight as checks of the app

**Status:** claimed by claude/0640, 22 October 2026

Found in the said-versus-did reading of the owner's Cursor build of 22 October 2026 (backlog 0373; the app and its records are on the owner's drive), checked against the record before filing.

The record of that build holds 13 `stackvet_check` calls, made at about four moments, and 5 `stackvet_before` briefs
and one `stackvet_preflight` call among its 34 lines. The report said "its AI coding tool asked sv 27 times …, 15 of
them a check of the app" (18 of 34 on a later copy). `crates/sv-cli/src/build_loop.rs` (about line 769) counts as a
check every line that carries counts, so a brief or a preflight that returns counts is one, and the report's "first
check" was a brief made before any code was written.

To do: count as a check only a `stackvet_check` call (and say the others apart: "4 briefs before building, 1
preflight"); make "the first check" the first `stackvet_check`. A test with a record holding briefs, a preflight, and
checks.

**Claimed, 22 October 2026,** at the owner's word ("Please continue with 0638 and the other items filed today and outside
the roadmap"), with 0638, each to be built in a pull request of its own.
