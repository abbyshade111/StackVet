# How `sv` decides what applies (the level, the chapters, the corroborated claims) is told in four places in prose and drawn nowhere

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `README.md:216-226` (the `[design]` answers), `:267-271` (the two answers that set the level, confirmed at
  `sv review`), `:228-244` (AISVS and Secure by Design applicability); `docs/ARCHITECTURE.md:28` ("Nothing in it is
  a fact; `resolve` corroborates ... only ever adds requirements, never removes them") and `:49-50` (stage 4);
  `docs/website/home-page.md:114-121`; ADR-015.
- What is wrong: The applicability decision is the part of `sv` a person most affects (every line of
  `stackvet.toml` they or their tool writes), and the rules (a `false` switches checks off, a line left out is "not
  assessed", the code can add but never remove, two answers set the level, Secure by Design takes the level of the
  ASVS requirement it matches) are each stated once somewhere, never together.
- What to do: One flow diagram on the "How it works" page: `stackvet.toml`'s claims and the code's signatures
  (packages, frameworks) → `resolve` (adds, never removes; a contradicted claim is reported) → the applicability
  rules → four bins per requirement: *applies*, *does not apply* (with its reason: 0403 open asks that the reason
  never states something untrue), *above the app's level*, *not known until a question is answered*; with
  `[app] audience` and `[data] categories` drawn as the two inputs to the level, and a note that the AI tool wrote
  them until `sv review` confirms. Saves little text; it makes the guide's step 4 instruction ("leave a line
  commented out rather than guessing `false`") visible as a consequence rather than a rule to memorize.
- How verified: read.
- Overlap: 0403 (open) is the wording of a reason; none for a picture.
- Needs a decision record?: no; ADR-015 is unchanged.
