# The ten shown prompts ride in the server instructions on every turn, and again in the spec, guidance, and brief (the owner's decision)

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: MCP token cost. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** 7,662 characters (about 1,900 tokens) of the roughly 2,970-token instructions are the full text of the ten prompts marked `shown` in `data/prompts.json`; the client puts a server's instructions in the system prompt, so this is paid on every turn of the whole session. The identical text is in every `stackvet_spec` answer and in guidance and briefs.

**Where.** `crates/sv-cli/src/mcp/mod.rs:774-781` (`instructions()` appends `crate::prompts_at_start()`), `crates/sv-cli/src/lib.rs:1111-1138`, `catalog.rs:624-637` (the spec), `tools.rs:306-322` (guidance with no topic), and `stackvet_before`.

**Evidence.** `format!("{INSTRUCTIONS}{}...{}", at_a_terminal(...), crate::prompts_at_start())`.

**How it bites.** About 1,900 tokens per turn, every turn, plus the same text again on every spec call. But: this is the decision of 7 October 2026 (`docs/PROMPTS.md:11-13`), and the `delivered` readings in `data/prompts.json` say the "start" trial (prompts given at the start, which was the instructions and the spec together) is where `secrets-in-the-environment` was shown to work with Sonnet while the brief and guidance delivery was not; the revision trial (`PROMPTS.md:54-56`) also leaned on prompts already at the start. The data do not say which of the two places carried the effect.

**Fix.** Give the prompts in one place a tool reads once (the spec is the natural one; the instructions keep one line saying where they are), after a trial reading that separates the two deliveries, or accept the cost and record why. The owner decides; held on the board until then.

**Record.** ADR-035 governs the prompt library and ADR-066 the instructions: a Later entry on whichever the owner's decision changes.

**The owner's word, 12 October 2026:** hold this one until after the loop test, "so we can weigh against the findings
from that as well. The loop test pins to a specific sv build, but still better safe than sorry and shouldn't change while
that test is running." So: not assigned until the loop trial (0257) has finished and its findings are in; then the owner
decides with both in hand.

**The owner's decision, 14 October 2026:** yes, after the loop trial: the ten shown prompts leave the server instructions, which keep one line saying where they are, and are given in one place a tool reads once (the spec); the trial's findings did not rest on the prompts being in the system prompt. A Later entry on ADR-066 (the instructions) and ADR-035 (the library) in the pull request that builds it; the loop trial's pinned build is unaffected, since the trial is over.
