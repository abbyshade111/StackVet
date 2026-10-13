# The setup prompts work with any AI tool that takes an MCP server

**Status:** done, 13 October 2026

The owner's decision of 13 October 2026, during the setup prompts' trial in Antigravity: "keeping up multiple tools to
support is not feasible. The prompts should ask how that tool sets up an MCP server and then tell it to set up the sv
MCP." The trial had stopped, as designed, at prompt 3's step 6, because `sv connect` writes settings only for Claude,
VS Code, and Cursor.

To build:
- `sv connect` with no tool named prints a tool-neutral description of StackVet's MCP server for the folder: its name
  (`stackvet`), that it talks over standard input and output, and the exact command and arguments (docker's full path
  and `--network none` included, or the installed copy's path). `claude`, `vscode`, and `cursor` stay, for the guide.
- Prompt 3, the Mac and Linux version and the Windows one, asks the AI tool how its own app is given a local MCP server,
  from that app's own documentation, and to say so and stop when it is not sure; to show the settings first, with
  StackVet's command and arguments exactly as printed, and write them only after a yes; and, when the app keeps one
  settings file for the whole account (Antigravity does: `~/.gemini/config/mcp_config.json`), to say so, and that
  StackVet is then set up for that folder only, and ask before changing it.
- The tests that hold prompt 3 to the three tools' names and files hold it instead to the neutral command, run through
  `sv`, and to naming no app.
