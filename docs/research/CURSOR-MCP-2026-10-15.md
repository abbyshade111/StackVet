# Cursor's MCP page, as read on 15 October 2026

**Source:** <https://cursor.com/docs/mcp>, Cursor's documentation, the page "Model Context Protocol (MCP)" under
Customize. Its text was copied from a browser by the owner on 15 October 2026, because this page could not be reached
from the cloud session that built backlog 0534 (the choice B the owner gave on that item). What follows is the part
`sv` relies on, in excerpts, word for word; the rest of the page (what MCP is, its transports,
one-click installation, the extension API, enterprise controls, images, the security advice beyond its last sentence,
and the FAQ) and the site's navigation are left out.

**What `sv` takes from it** (`crates/sv-check/src/ai_tool.rs`, ADR-049, Later, 15 October 2026):

- The project's file is `.cursor/mcp.json` ("Configuration locations"). `~/.cursor/mcp.json` is in the person's home
  folder, never in the app's, so `sv` does not see it.
- Its servers sit under `mcpServers`, each with `command` and `args` (a server Cursor starts) or with `url` (one it
  connects to), as in `.mcp.json`. So the file is read the way `.mcp.json` is.
- `type` is listed as required for a started server, but the page's own examples leave it out, so `sv` does not
  require it.
- Cursor "asks for approval before using MCP tools by default", and the file has no setting like Claude Code's
  `enableAllProjectMcpServers`, so no note says Cursor starts the file's servers without asking.
- The page describes no other file in `.cursor/`, so those are still named as not read.

---

```text

[Excerpts of the page the owner pasted on 15 October 2026, word for word; whole sections, and parts of these, are left out.]

Using mcp.json

Configure custom MCP servers with a JSON file:

CLI Server - Node.js
{
  "mcpServers": {
    "server-name": {
      "command": "npx",
      "args": ["-y", "mcp-server"],
      "env": {
        "API_KEY": "value"
      }
    }
  }
}

CLI Server - Python
{
  "mcpServers": {
    "server-name": {
      "command": "python",
      "args": ["mcp-server.py"],
      "env": {
        "API_KEY": "value"
      }
    }
  }
}

Remote Server
// MCP server using HTTP or SSE - runs on a server
{
  "mcpServers": {
    "server-name": {
      "url": "http://localhost:3000/mcp",
      "headers": {
        "API_KEY": "value"
      }
    }
  }
}

Static OAuth for remote servers
Add an auth object to remote server entries that use url:
{
  "mcpServers": {
    "oauth-server": {
      "url": "https://api.example.com/mcp",
      "auth": {
        "CLIENT_ID": "your-oauth-client-id",
        "CLIENT_SECRET": "your-client-secret",
        "scopes": ["read", "write"]
      }
    }
  }
}
Field | Required | Description
CLIENT_ID | Yes | OAuth 2.0 Client ID from the MCP provider
CLIENT_SECRET | No | OAuth 2.0 Client Secret (if the provider uses confidential clients)
scopes | No | OAuth scopes to request. If omitted, Cursor will use /.well-known/oauth-authorization-server to discover scopes_supported
auth values support the same interpolation as other fields. Use environment variables for Client ID and Client Secret instead of hardcoding them.

STDIO server configuration
For STDIO servers (local command-line servers), configure these fields in your mcp.json:
Field | Required | Description | Examples
type | Yes | Server connection type | "stdio"
command | Yes | Command to start the server executable. Must be available on your system path or contain its full path. | "npx", "node", "python", "docker"
args | No | Array of arguments passed to the command | ["server.py", "--port", "3000"]
env | No | Environment variables for the server | {"API_KEY": "${env:api-key}"}
envFile | No | Path to an environment file to load more variables | ".env", "${workspaceFolder}/.env"
The envFile option is only available for STDIO servers. Remote servers (HTTP/SSE) do not support envFile.

Configuration locations
Project Configuration: Create .cursor/mcp.json in your project for project-specific tools.
Global Configuration: Create ~/.cursor/mcp.json in your home directory for tools available everywhere.

Config interpolation
Use variables in mcp.json values. Cursor resolves variables in these fields: command, args, env, url, and headers.
Supported syntax:
${env:NAME} environment variables
${userHome} path to your home folder
${workspaceFolder} project root (the folder that contains .cursor/mcp.json)
${workspaceFolderBasename} name of the project root
${pathSeparator} and ${/} OS path separator

Using MCP in chat
Cursor automatically uses MCP tools listed under Available Tools when relevant. This includes Plan Mode. Ask for a specific tool by name or describe what you need. Enable or disable MCP servers from Customize in the sidebar.

Tool approval
Cursor asks for approval before using MCP tools by default. Click the arrow next to the tool name to see arguments.
Run Mode: MCP follows the same Run Modes as terminal commands. For example, in Auto-review mode, allowlisted MCP tools run immediately and everything else is routed through the classifier.

Security considerations
Remember that MCP servers can access external services and execute code on your behalf. Always understand what a server does before installation.
```
