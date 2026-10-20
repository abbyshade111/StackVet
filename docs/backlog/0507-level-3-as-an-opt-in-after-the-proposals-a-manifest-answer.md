# Level 3 as an opt-in, after the proposals: a manifest answer that holds an app to ASVS level 3, with its requirements shown as what they are

**Status:** open

**Was queued on the coordination board** (backlog 0273) until the board was removed on 20 October 2026 (ADR-061, Later); it is now taken the normal way, by a claim.

Filed 14 October 2026 at the owner's word, from the walk-through of the parked items. `sv` holds every app to level 1
or 2, from two answers in `stackvet.toml`; level 3 is about ninety more ASVS requirements, nearly all documentation and
process ("verify that a policy exists"), with no check in the code, so an app held to it today would mostly gain *not
assessed* rows and questions nobody can answer by looking at code. That changes once the pointer on every question
(0487, done) and the proposals that rest on it (0488 to 0500) are built, since those are what make such questions
answerable. Then: an opt-in answer in `[app]` that holds the app to level 3, the level-3 requirements shown with their
status like any other (never counted as a gap in the two-level figures the paper and the website quote), the
applicability data read for them (it already carries levels), and 0263 (the static half of V16.5.4, set aside because
no report has a line for a level-3 requirement) taken with it. A decision record of its own: it changes what a report
concludes and which requirements apply. **Held until the proposals are built**; kept beside 0263.

**Checked against `main`, 20 October 2026** (the backlog clean-up): still nothing built (`target_level` returns 1 or 2). Of the proposals it waits for, 0489 and 0496 are partly done; 0500 is done with this clean-up, its two leftovers filed apart. Then it waits on the owner's go and a decision record of its own, with 0263 taken with it.
