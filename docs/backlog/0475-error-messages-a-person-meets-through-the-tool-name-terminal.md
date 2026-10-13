# Error messages a person meets through the tool name terminal-only fixes

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: usability. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-cli/src/lib.rs:2057-2070` ("Give an empty folder, or a new one, with --out"), reached from
  `stackvet_write_report` whose own parameter is `out` (`catalog.rs:645`); `crates/sv-manifest/src/lib.rs:3008`
  ("no stackvet.toml in …. Run `sv init` and give the spec to your AI coding tool"), reached from every tool but
  `stackvet_spec`; `crates/sv-cli/src/report_lock.rs:344-353` ("Wait for that run to finish and run this again,
  or {elsewhere}"); `crates/sv-cli/src/doctor.rs:134-141` ("`sv init` prints the description … which writes it").
- What is wrong: in the MCP server these messages go to the tool, and the tool passes them on. A tool told to
  "run `sv init`" cannot (the container has no terminal for the person) when `stackvet_spec` is the same thing
  one call away; "--out" is not a thing the tool can pass. Each message is right at a terminal and one word off in
  the server. The server already rewrites some sentences for its reader (`mod.rs:936-946`, `why_not_run` and its
  two siblings); these are not rewritten. I did not check whether `tools.rs` wraps them.
- What to do: name both routes in the message where both exist (``with --out at a terminal, or `out` in
  stackvet_write_report``; ``run `sv init`, or call stackvet_spec``), or have the server substitute, as it does
  for `why_not_run`. A test per message in the server's tests (`mcp/tests.rs`) that the tool's name appears.
- How verified: read; whether the server rewrites any of these before answering is not verified.
- Overlap: 0308 and 0282 are about other server answers; 0217 part 3 (`stackvet_status`). None found for this.
- Needs a decision record?: no.
