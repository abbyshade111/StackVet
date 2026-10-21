# Level 3 as an opt-in, after the proposals: a manifest answer that holds an app to ASVS level 3, with its requirements shown as what they are

**Status:** partly done: what moved here from 0624: the catalogs opened to level 3 with their entries (the 27 level-3 design questions and the checks by hand for V3.7.5 and V13.2.6), the level-3 locators, V11.2.5 cited by `ast.check-passes-on-error`, and the cookie-size check (V3.3.5); and 0263, the static half of V16.5.4

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

**The owner's go, 20 October 2026** (asked in a session, while 0624 was being claimed): level 3 is to be enabled when the
owner asks for it, taken next after 0624 rather than held until every proposal from 0488 to 0500 is built. What that
changes and adds:

- **AISVS too.** The opt-in holds the app to level 3 of both ASVS 5.0 and AISVS 1.0, so the AISVS level-3
  requirements (C5.1 among them) come with it.
- **The `never` rules for V15.4 and C5.1** (moved here from 0624, part 4): their reason, "level 3 only, above the level
  this app targets", is false for an app held to level 3. They become a level condition, so the requirements apply at
  level 3 and are not applicable, with a true reason, below it.
- **What a level-3 app sees** is DESIGN, "What could be added for the ASVS requirements StackVet says nothing about
  (20 October 2026)": 71 of its 105 requirements are level 3 (the entry was written for 103, and two more were added to it), and 0624 labels each *The owner answers* or *No check
  yet*, so the level-3 rows say what StackVet does for them rather than *No check*. The level-3 additions there, and
  in 0625, become reachable once this lands.
- Unchanged: the opt-in is the owner's answer in `[app]`, never a default; the two-level figures the paper and the
  website quote stay two-level; 0263 is taken with it; and a decision record of its own.

**Moved here from 0624, 20 October 2026, at the owner's word.** 0624 built what it could at levels 1 and 2; what is
level 3 waits for this, because the catalogs take levels 1 and 2 only (`crates/sv-check/tests/design.rs`,
`human_checks.rs`, and the guard in `notes.rs` that reaches every level only for the documentation requirements), and
because a level-3 check changes no report until an app can be held to level 3. Taken with this item, after the opt-in
itself:

- **The catalogs opened to level 3**, with their tests: then the 27 level-3 design questions the design entry lists,
  and the checks by hand for V3.7.5 and V13.2.6.
- **The level-3 locators,** each with its question: V10.2.3 (the scopes the code asks the provider for), V11.3.5
  (functions that both encrypt and compute an HMAC), V15.4.1 (shared objects written from request handlers in a
  threaded app), the web server's package locator extended to V4.2.3 and V4.2.4, and the media server's to V17.2.6.
- **V11.2.5 cited by `ast.check-passes-on-error`** when the error handler that answers "yes" wraps a signature check,
  a MAC check, or a decryption: finding only, and it needs a way for one rule to cite a second requirement under a
  narrower match, with a test that breaks it.
- **The cookie-size check (V3.3.5):** each `Set-Cookie` the run receives whose name and value together pass 4096 bytes
  is a finding; all under is checked in part, for the cookies the run saw.

**The decision** (proposed with the claim, 20 October 2026; accepted as ADR-130 in the pull request that built it):

- **The answer:** `level = 3` under `[app]`. Left out, the level is what it is today, from `[app] audience` and
  `[data] categories` (ADR-024). It only ever raises the level: under ADR-015, what the owner says adds requirements and
  never removes one, so `level = 1` on an app those answers hold to level 2 is refused when the file is read, with the
  two answers that set level 2 named, and `level = 2` is the same as leaving it out. `sv review` asks the owner to
  confirm it, as it does the other two answers, and the report says the level rests on it.
- **What applies at level 3:** every ASVS 5.0 and AISVS 1.0 requirement whose level is 3 or less, decided by the same
  applicability rules as now. The `level2` condition, computed today as the target being exactly 2, becomes the target
  being 2 or more, or a level-3 app would be told a level-2 requirement does not apply. V15.4's and C5.1's `never` rules
  become a level condition with a true reason.
- **What the report shows:** the level-3 requirements with their status like any other, and the level with what set it
  ("held to level 3 because stackvet.toml asks for it"). The two-level figures the paper and the website quote, and
  `docs/COVERAGE.md`'s tables, count every requirement as now and do not change.
- **Unchanged:** an app that does not ask is held to 1 or 2 exactly as before, so no existing report moves.

