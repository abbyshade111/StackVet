# A design answer or hand check the tool wrote with by = "ai-tool" has no `mine` in `sv review`

**Status:** done, 14 October 2026

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

**The owner's answer, 14 October 2026:** (c), leave it. A `[design]` answer or a `[checked-by-hand]` result the tool
wrote is confirmed, not owned: `sv review` confirms it once the tool has proposed what to look at, and `by = "owner"`
written by hand is for the rare one that is truly the owner's own decision. Nothing is built; the guide says so in one
sentence, and ADR-026 records the choice (Later, 14 October 2026).
