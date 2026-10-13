# The setup page walks the person through making the full report

**Status:** claimed by securevibe-trial-prep, 13 October 2026

The owner's decision of 13 October 2026, at the end of the first build in the Antigravity trial: "users need to be able
to run the full report. It doesn't have to be through the MCP, but the AI tool with access to the GitHub repo should be
able to walk the user through running the report if they want it."

The setup page stops at a connected StackVet and the first build prompt. The report the AI tool can make through the
MCP server neither starts the app nor runs the outside tools (ADR-066: both run other people's code, so they stay the
person's decision at a terminal), and the StackVet it talks to is a container with none of the tools. The full report
is `sv report --run`, by StackVet installed on the computer, as the guide's "Installing StackVet on your computer, for
`--run`" says: Scoop on Windows (which the Windows setup prompt already installs), Homebrew or a hand build on a Mac
or Linux.

To build: a prompt 6 on `docs/prompts/setup.md`, "Make the full report", naming no app, that has the AI tool check
whether StackVet is installed on the computer, install it the guide's way for that computer only with a yes for each
step, run `sv doctor` and say what is ready, run `sv report . --run` only after a yes (asking, rather than going
around it, when its own commands are kept from Docker), and walk the person through `report.html` from the top. Below
it, the page says that answers count as the person's own only once recorded with `sv review`, which they type
themselves. A test holds the prompt to naming no app, to commands `sv` takes, and to the guide section it points to.
ADR-066 is unchanged: the MCP server still runs neither the tools nor the app; the person approves each at a terminal.
