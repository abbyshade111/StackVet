# A refusal the owner makes at the AI coding tool's own prompt is seen by nothing av reads: look for it through the tool's hooks

**Status:** claimed by Claude Code session (AgentVet), 22 November 2026

Asked for by the owner on 22 November 2026 ("queue the hooks work"), after a trial of `av proxy` on their own computer
on 21 November (ADR-159, "As built, part 6"; backlog 0716). Open for any session. It is the one of the design's six
groups, "Trying again after a no", that no record can show for the owner's own refusals.

**What the trial showed.** `av proxy` was put in front of `sv`'s MCP server for one app, in Claude Code's desktop app.
An ordinary call and a call the server turned down were each written down. Two calls the owner refused at Claude
Code's own permission prompt left no line in the pass-through's record, and Claude Code's own log for the server shows
no call after the last one that was allowed. A call refused at the tool's prompt never reaches the server, so nothing
that sits on the server's connection can see it. The pass-through shows what a server turned down; it cannot show
what a person did.

**What to find out, before building anything.**

1. Which of Claude Code's hooks fire when a person refuses a tool call, and what each is handed: read Claude Code's
   own hooks page, as ADR-122 did for the hooks `sv connect claude --hook` installs today (`PostToolUse`,
   `PostToolUseFailure`, and `Stop`), and try it. A hook that fires before a tool runs, or on a permission decision,
   is the candidate. Say what was read and what was tried, apart.
2. Whether the hook is told enough to match a refusal with what came next: the tool's name and what it was given, so
   that "the same thing, tried again" and "the same end, reached another way" can be told from a different request.
3. What the other tools `sv connect` knows offer: whether any has a hook for this at all. A tool with none is said to
   have none; the report then says this group cannot be shown for it.

**What it would become, if the hooks allow it.** A line in the hook record for a refusal: the time, the tool's name,
and the same cut and key-cut account of what it was given that a command gets today, never the person's words or
the tool's. Then a reading in `av_record::groups` that lists, under "Trying again after a no", a refused call
followed by one that does the same thing, with what it could not tell. ADR-122's consent holds: the record is read
only for a run that asks for it. What the hook record keeps is ADR-122's to decide, so a change there is a Later
entry on it, and the owner is asked first if it keeps more than a command's line does today.

**Not this item.** Stopping an action. ADR-159's decision 11 keeps that behind a decision of its own.
