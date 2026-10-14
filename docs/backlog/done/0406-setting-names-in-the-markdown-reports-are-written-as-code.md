# Setting names in the Markdown reports are written as code, not escaped

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found 13 October 2026 in the owner's trial in Antigravity. The report's `security.md` writes a setting's name with its
square brackets escaped, `\[stack.run.users\]`, so a Markdown viewer does not read it as a link. Antigravity's viewer
showed that line as "under stack.run.usersstack.run.usersstack.run.users". It was the one sentence telling the person
how to get the sign-in checks to run. GitHub's viewer shows it correctly.

To build: in `compliance.md`, `security.md`, and `low-findings.md`, write the names of settings, tables, files, and
commands as code (`` `[stack.run.users]` ``), which every viewer shows as written and where nothing needs escaping.
Keep `inert` for the text an app supplies, where escaping is the point. Find each place that writes a `stackvet.toml`
name in prose. A test fails on an escaped bracket outside code in the reports' fixed wording.
