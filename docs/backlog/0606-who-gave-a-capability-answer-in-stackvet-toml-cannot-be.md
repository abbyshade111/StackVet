# Who gave a capability answer in stackvet.toml cannot be recorded, and no person can confirm one in sv review

**Status:** open

Filed by documentation building backlog 0600, 15 October 2026. Severity: medium (whose word a "does not apply" rests
on).

**What is missing.** A design answer and a check by hand carry `by`, and a person can confirm the AI tool's through
`sv review` (`confirmed`, sealed). A capability line under `[capabilities]` or `[repository]` is a bare `true` or
`false`: it records nobody, so since 0600 the report reads every such answer as the AI coding tool's and says `sv` did
not check the ones that rule requirements out. Nothing lets a person say "I looked, and the app takes no uploads", so a
"no" the owner did check reads the same as one the tool guessed.

**A way.** A `[capabilities-review]` table that `sv review` writes and seals, as `[scope-review]` holds the audience and
the data list (ADR-024), naming each answer confirmed and holding while the answers stay the same; the report's
"who said it" then reads "confirmed by the owner through sv review", and 0600's count names those apart. A decision
for the owner first: whether a confirmed "no" should change anything beyond the label.

**Record.** ADR-117 Later, or a new record if the owner wants the confirmation to change what counts.
