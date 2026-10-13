# A suppression comment in the app's code shows in the report as a suppressed finding

**Status:** open

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
