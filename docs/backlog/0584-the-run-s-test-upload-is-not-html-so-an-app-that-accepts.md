# The run's test upload is not HTML, so an app that accepts only .html stops four upload requirements with no way to say what it accepts

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Opus run's reading (backlog 0577), 15 October 2026. Severity: medium.

**What was seen.** The app takes single-file HTML games only, and refuses anything else with 400; sv's upload checks
send a GIF first (the "ordinary file" control) and stop when it is refused, so V5.2.x stays unassessed with the reason
"an ordinary file was not accepted", and nothing in `stackvet.toml`'s `upload` entry lets the app say what it takes.

**Fix.** An `accepts` field on `upload` (a list of extensions or media types); the control upload is a well-formed file
of the first accepted kind (an HTML sample for `.html`, a PNG for images, ...), and the oversize, wrong-type, and
double-extension checks use that kind. The spec and `sv init` name the field.

**Record.** ADR-046 (uploads) Later, since what sv sends changes.
