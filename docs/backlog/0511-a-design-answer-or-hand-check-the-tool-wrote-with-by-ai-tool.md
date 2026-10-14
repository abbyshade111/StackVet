# A design answer or hand check the tool wrote with by = "ai-tool" has no `mine` in `sv review`

**Status:** open

Filed by session honesty on 14 October 2026, from backlog 0425, as the half of it left unbuilt (ADR-026, Later, 14
October 2026). Severity: low. Needs the owner's word before it is built.

- Where: `crates/sv-cli/src/review.rs`. A `[design]` answer or a `[checked-by-hand]` result with `by = "ai-tool"` is
  offered in `sv review` only when it carries a `confirmed` proposal (`Waiting::Confirmation`), and then only to be
  confirmed as the tool's word. 0425 asked for `mine` on these too, beside the security notes sections it was built
  for.
- Why it was left: the answers a person gives in chat reach `security-notes.md` through `stackvet_record_answer`, and
  that is where `mine` now is. A `[design]` or `[checked-by-hand]` entry with `by = "ai-tool"` is usually the tool's
  own reading of the code, and an app whose `stackvet.toml` the tool filled in can have dozens. Offering each with
  `mine` would lengthen every `sv review` and invite a person to make a reading of the code their own decision
  without looking.
- What to decide: (a) offer `mine` on each such entry, as 0425 proposed; (b) offer it only on entries the tool marks
  as the person's words (a new field, which the tool cannot make true, so it would only choose what is asked); or
  (c) leave it, since the person can still write `by = "owner"` and record it.
- Needs a decision record?: yes, a Later entry on ADR-026 if built.

**The owner's decision, 14 October 2026** (asked by session honesty): (c), leave it. A design entry or a hand check the tool wrote is confirmed in `sv review`, not owned; the answers a person gives in chat reach `security-notes.md`, where `mine` is; the two design answers that are the person's own (the level) are confirmed through the scope step; and for the rare entry that is truly the person's, `by = "owner"` by hand remains. Close the item with this reason, add one sentence to the guide saying so, and record the choice as a Later line on ADR-026 in the same pull request.
