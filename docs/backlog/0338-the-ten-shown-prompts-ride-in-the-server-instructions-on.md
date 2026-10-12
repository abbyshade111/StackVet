# The ten shown prompts ride in the server instructions on every turn, and again in the spec, guidance, and brief (the owner's decision)

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: MCP token cost. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** 7,662 characters (about 1,900 tokens) of the roughly 2,970-token instructions are the full text of the ten prompts marked `shown` in `data/prompts.json`; the client puts a server's instructions in the system prompt, so this is paid on every turn of the whole session. The identical text is in every `stackvet_spec` answer and in guidance and briefs.

**Where.** `crates/sv-cli/src/mcp/mod.rs:774-781` (`instructions()` appends `crate::prompts_at_start()`), `crates/sv-cli/src/lib.rs:1111-1138`, `catalog.rs:624-637` (the spec), `tools.rs:306-322` (guidance with no topic), and `stackvet_before`.

**Evidence.** `format!("{INSTRUCTIONS}{}...{}", at_a_terminal(...), crate::prompts_at_start())`.

**How it bites.** About 1,900 tokens per turn, every turn, plus the same text again on every spec call. But: this is the decision of 7 October 2026 (`docs/PROMPTS.md:11-13`), and the `delivered` readings in `data/prompts.json` say the "start" trial (prompts given at the start, which was the instructions and the spec together) is where `secrets-in-the-environment` was shown to work with Sonnet while the brief and guidance delivery was not; the revision trial (`PROMPTS.md:54-56`) also leaned on prompts already at the start. The data do not say which of the two places carried the effect.

**Fix.** Give the prompts in one place a tool reads once (the spec is the natural one; the instructions keep one line saying where they are), after a trial reading that separates the two deliveries, or accept the cost and record why. The owner decides; held on the board until then.

**Record.** ADR-035 governs the prompt library and ADR-066 the instructions: a Later entry on whichever the owner's decision changes.
