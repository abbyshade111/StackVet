# The report does not say the typed-text check did not run when no form is named

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found 14 October 2026 by session documentation, answering backlog 0389. `probe.text-rendered-as-markup` (ADR-097) runs
only when `stackvet.toml` names `[stack.run.users.browser]` with `text-form` or `text-forms`: `sv` finds no form of its
own. With no `browser` table, `checks` in `crates/sv-check/src/browser.rs` returns at its first line and records
nothing; with a `browser` table but no form, its not-assessed lines name V7.4.4 only. Either way the report says nothing
about the typed-text check under V3.2.2, whose listed reasons then come from other checks (in the second loop trial,
email headers), so the owner is not told what one line in `stackvet.toml` would let `sv` try.

What to build: when the run signs in (so the browser could have been used) and no text form is named, a not-assessed
line for V3.2.2 saying the typed-text check was not made and that `browser = { text-form = "...", shows = "..." }`
under `[stack.run.users]` makes it. A line in the report is a change to what it says, so whether this needs a decision
record of its own, or a Later entry on ADR-097, is to be settled in the claim. A test with no browser table, one with a
table and no form, and one with a form; and a deliberate break.
