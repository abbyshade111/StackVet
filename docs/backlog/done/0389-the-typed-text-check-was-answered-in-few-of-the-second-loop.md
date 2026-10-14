# The typed-text check was answered in few of the second loop trial's builds

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the second loop trial (backlog 0257, `docs/prompts/loop-2/results.md`), 13 October 2026. An open question, not
yet a fault.

`probe.text-rendered-as-markup` (ADR-097) was answered in 14 of the 20 club builds of the drafts trial and in only 8
of phase A's 20, 4 of B1's 20, and 1 of B2's 20; no build listed `text-forms`. Why it was not answered in the rest was
not settled the night of the trial: the gaps the reports name for V3.2.2 are about email headers. To read: for a build
where it went unanswered, which forms `sv` found, and which step stopped short.

**Answered 14 October 2026 (session documentation), from the code; the builds were not on hand.** The typed-text
check is answered only when `stackvet.toml` names `[stack.run.users.browser]` with `text-form` or `text-forms` and the
sign-in gives the browser a cookie: `sv` finds no form of its own (`crates/sv-check/src/browser.rs`, `checks`).
So "which forms `sv` found" is none, in every build: it tries only the forms named, and the step that stopped short in
a build left unanswered is the first, a form never named. The settings template does document `text-form` (a line left
commented out in `crates/sv-manifest/src/spec.rs`), so whether the check ran came down to whether the AI tool
uncommented it. Two more things were found, each now an item: the report says nothing about the typed-text check when
no form is named (0503), and the trial's score counts only the list form `text-forms`, so it shows 0 forms listed even
for the 28 builds that were credited, which must have named one with `text-form` (0504). Not verified: build by build,
that each unanswered build named no form rather than failing at sign-in; the builds are under `~/sv-loop2` on the
owner's computer, and 0504 re-scores them when they are next on hand.
