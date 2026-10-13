# The report's "What to do next" speaks the terminal's language and never names `sv review` or the full run

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: usability. Severity: medium (every reader of every
report; cheap). Verified against the code by the reviewing session before filing.

**What is wrong.** "What to do next" is the one place in the report that says "you are done, here is what you still
owe", and for the person this product is for it is in the wrong language and incomplete. Step 2 sends them to a
terminal command and a TOML section ("`sv notes` writes the questions out; the rest are the [design] section of
stackvet.toml"), where their path is to ask their AI tool to ask them the questions (`stackvet_check`, section
`questions`, which the build prompt and the server instructions already arrange). Step 3 tells a non-programmer to
"Write tests for the N requirements", which only the tool can do. No step says "record what you answered as yours
with `sv review`", even when the report shows the tool's word standing behind answers (`counts.stated > 0`, or notes
sections marked `Written by: AI coding tool`). No step says "run the full report", although the line just above it
(`not_run_line`) has counted how many requirements only `--run` or `--tools` can reach, and the report already holds
the exact command (`ReportOptions::why_not_run`, filled by the MCP server with `at_a_terminal`'s quoted path). The
same route words are in "What only you can check" (`human.rs:88-94`).

**Where.** `crates/sv-report/src/bluf.rs:336-419` (`next_steps`; the strings at 354, 377-380, 391, 404, 414-415),
rendered at `markdown.rs:237-249` and `html.rs:329-338`; `crates/sv-check/src/human.rs:88-94`; the count at
`bluf.rs:296-312`; the command at `crates/sv-cli/src/mcp/mod.rs:936-939` and `assemble.rs:1346-1357`.

**Fix.** Phrase each step as what to do in the tool the person uses, and add two steps, each only when it applies.
Step 2: "Answer the 6 questions at level 1 that no tool can settle. Ask your AI tool to ask you them one at a time
(it calls stackvet_check with section "questions" and records your answers); at a terminal, `sv questions` prints
them." Step 3: "Ask your AI tool to write tests for the N requirements a test could settle." New, when any
requirement rests on the tool's word: "Record the answers that are yours: `sv review` in a terminal in the app's
folder (the tool cannot do this for you; the guide's step 5 says how)." New, when `not_run_this_time` is `Some`:
"Run the full report, which starts the app and runs the outside tools: <the command from why_not_run>. N requirements
can only be checked that way." Give `human.rs::what_to_do` the same two voices. Must not lose: the steps stay derived
from the counts, with a test per step; the terminal command stays the one `at_a_terminal` quotes; the report never
says the app is secure once the steps are done.

**Record.** None (no tier, default, or write changes). `ADR-110: unchanged, because` the order of the sections is kept.
