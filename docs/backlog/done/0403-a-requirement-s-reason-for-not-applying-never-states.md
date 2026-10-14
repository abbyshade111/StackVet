# A requirement's reason for not applying never states something untrue about the app

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found 13 October 2026 in the owner's trial in Antigravity, whose app has `[capabilities.ai] enabled = false` and no AI
feature at all. Its `report.html` listed the AI requirements as not applying, rightly, but with reasons that describe
another app:
- "conversation history is plain rows in the app's own database, scoped to each user" (C8.1.1);
- "A separate content-moderation check on the assistant's answers is not turned on ... It can be turned on later with
  the AI_MODERATION setting" (C7.3.1);
- "The assistant can only answer questions" (C9.2.1).

Each is labelled "claim (...)", as though the owner had said it. Their source is StackVet's own data,
`data/knowledge/applicability.json` and `data/applicability-v2.json` (`notApplicableReason`), written, it seems, for
SecureVibe v1's generated app (`AI_MODERATION` is that template's setting).

A report that states something untrue about the app is the thing StackVet must never do. To build:
- When the app has no AI feature, every AI requirement's reason is that: "This app has no AI features", as some rows
  already say.
- Each `notApplicableReason` is reworded to say what the app's own answer was, in general terms ("`stackvet.toml`
  says the app keeps no document store"), never a detail of one app. Each reason is checked against its condition.
- The label says whose word it rests on: the app's `stackvet.toml`, written by the AI tool unless recorded through
  `sv review`.
- A test fails on any reason that names a setting, a file, or a feature no condition establishes.
