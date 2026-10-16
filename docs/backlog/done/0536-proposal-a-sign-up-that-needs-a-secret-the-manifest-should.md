# Proposal: a sign-up that needs a secret the manifest should not carry (an invite code), without weakening the fence

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502). A proposal with a decision for the owner.

**What was seen.** The Cursor app gates sign-up on an invite code read from the environment; the tool wrote "The invite
code is FAMILY_INVITE_CODE in the environment. It is not written in this file" in stackvet.toml, and every sign-up sv
sent answered 403, so the sign-up control, the lockout probe, the validation probe, and the reveal checks all ran
without an account of their own. The VS Code tool put a fixed test code in the code and the manifest
(`invite_code = "stackvet-test-invite"`), which worked and is itself a finding (see the `INVITE_*` item).

**The choice.** The transcript reading proposes the same thing from the other side: `[stack.run.signup]` carrying extra fixed form fields and an environment variable for the run (`env = { FAMILY_INVITE_CODE = "{invite}" }`, a value sv makes), documented in `sv init`'s comments. (a) a seed-made value: the manifest's seed command prints `NAME=value` lines sv substitutes into the
sign-up form as `{seed:NAME}`; (b) an environment name the manifest cites, `{env:NAME}`, that sv reads from the
person's environment at run time and never writes down; (c) leave it: say in preflight that a sign-up field with no
value in the manifest will refuse sv's accounts, and let the app accept a test code when `STACKVET_RUN=1`. The
reviewing session recommends (a): the value never leaves the fence and the manifest stays free of secrets.

**Record.** A new ADR: what sv reads at run time and what the manifest may cite.

**The owner's choice, 14 October 2026:** (a), the seed-made value: the manifest's seed command prints NAME=value lines and the sign-up form cites {seed:NAME}; the value is made inside the fence at run time and never written down. A new ADR in the pull request that builds it (what sv reads at run time and what the manifest may cite).

**Built, 16 October 2026 (honesty):** the owner's choice (a). The seed may print `NAME=value` lines; a request in
`[stack.run.users]` cites one as `{seed:NAME}`; the value is filled into the requests that cite it and taken out of
everything the run writes down. A manifest citing `{seed:…}` with no `seed` is refused. ADR-124.

**Closed by the board, 16 October 2026:** built by honesty and merged (PR 2100, ADR-124); left open for the board to close.
