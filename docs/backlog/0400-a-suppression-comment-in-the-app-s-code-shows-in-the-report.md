# A suppression comment in the app's code shows in the report as a suppressed finding

**Status:** claimed by documentation, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found 13 October 2026 in the owner's trial in Antigravity. After `sv report --run` found Bandit's B104 in the app, the
AI tool added `# nosec B104` to the line, telling Bandit to ignore it, and the finding left the report. (It then also
rewrote the line; see the item on empty bind addresses.) A person reading the report would see one finding fewer and
no sign that anything was silenced.

StackVet already has a way to set a finding aside that a person sees and signs: `sv review` records a false alarm or an
accepted risk, and the report lists it under "Set aside" (ADR-023). A suppression comment in the code skips all of
that. To build: StackVet reads the suppression comments of the tools it runs (Bandit's `# nosec`, Semgrep's
`nosemgrep`, gosec's `#nosec`, CodeQL's `lgtm` and others as each tool documents them) and lists each in the report as
a suppressed finding: the rule, the line, and that the code says to ignore it, not a person through `sv review`.
Whether a suppressed finding still counts against its requirement, or shows apart like test code's, is a decision to
ask the owner; it must never read as "nothing found". A fixture with each tool's comment, and a deliberate break.

**Corrected 13 October 2026, the same day it was filed:** much of this is already built. `data/adapters.json` runs Bandit
with `--ignore-nosec`, so a line marked `# nosec` is reported anyway. And `suppressed_by` in `crates/sv-check/src/adapters.rs`
shows a result that Semgrep (`// nosemgrep`), gosec (`#nosec`, when asked to track them), or Brakeman
(`config/brakeman.ignore`) marks as suppressed, with "This one is marked to be ignored ... It is shown here because a
finding somebody chose to hide is still a finding until somebody has looked at why". `looked_away` names the rest.

So in the trial the `# nosec` alone would not have hidden B104 from StackVet's run; the `""` rewrite did (backlog 0399).
The session that filed this said otherwise without reading the adapters first.

What is left: whether the report should also list the suppression comments it finds in the app's own code, as their own
line, so a person sees that something tried to silence a scanner even when StackVet showed the finding anyway (an AI tool
adding them is worth knowing about); and whether any tool StackVet runs (CodeQL's `lgtm`, Opengrep) hides a suppressed
result in a way not yet covered. Check each before building, with a fixture.

