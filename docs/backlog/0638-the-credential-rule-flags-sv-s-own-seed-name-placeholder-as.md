# The credential rule flags sv's own {seed:NAME} placeholder as a code written into the code

**Status:** open

Found in the said-versus-did reading of the owner's Cursor build of 22 October 2026 (backlog 0373; the app and its records are on the owner's drive), checked against the record before filing.

The app's `stackvet.toml` passes its invite into the sign-up form as `invite = "{seed:INVITE}"`, the placeholder
`sv run` fills from what the seed command prints (ADR-124). `secrets.credential-assignment` reported it as "An invite
or access code is written into the code (`invite`)", medium, at `stackvet.toml` line 32, in every check from the
first that saw it to the final report, where it is one of the things to fix. The AI tool rightly left it alone and
said why.

Why: `looks_like_placeholder` in `crates/sv-check/src/secrets.rs` (about line 247) takes a whole-value `{...}` as a
blank only when what is inside is letters, digits, `_`, `.`, brackets, commas, and spaces; the `:` of
`{seed:INVITE}` is not among them, so sv's own placeholder is judged as a literal.

To do: treat `{seed:NAME}`, and any placeholder `sv` itself fills in `stackvet.toml`, as a blank, with a test that the
rule still fires on a real code beside it, and that `{seed:INVITE}x9Q2` (text of its own) is still judged.
