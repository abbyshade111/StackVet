# A question to the owner carries the lines it rests on: a locator rule and a pointer on every question item

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: One cross-cutting item that the proposals above assume**

Every "route 4" proposal needs the same two things once: (1) a locator kind of rule in `data/ast-rules.json`, or a
corroborator that keeps its hit's location (`found_in_code` becoming a list of `file:line`), that finds and does not
judge; and (2) a `rests_on: Vec<Pointer>` on `human::Item`, rendered in `report.html`, `compliance.md`, and the MCP
`questions` section (`crates/sv-cli/src/mcp/check_text.rs:642`), each pointer redacted through `Secret::redact` and
capped at a few lines, with the fixed sentence that it is where to look and not evidence. This is 0397's second part
built generally; it changes no tier, so its record is a Later entry on ADR-022 saying what a question may carry and
that it credits nothing.

**Which questions never get a pointer** (0397 asks for this list): For 0397's question "which questions never get a pointer": V2.2.3 (the app's own business rules), V6.4.4 (a support
process), V7.1.2 (a design choice with no code shape in most apps), V13.2.5 without infrastructure files (the host's
rules), V14.2.4 (a comparison with the owner's own document), V16.2.3 beyond the code's own transports (a host agent),
V4.2.1 (the proxy in front), V7.6.1 (the provider's side). For these the honest item is a hand check whose text says why
nothing in the folder can show it, which `data/human-checks.json` already does for most; the two without any question
(V2.2.3, V13.2.5) should get one.

**The owner's decision, 13 October 2026** (asked by session stackvet-deep-review): build this first, alone, and try it on one chapter's questions before any of the fourteen proposals that assume it (0488 to 0499) is taken; what the trial shows may change the rest. The proposals wait for it.
