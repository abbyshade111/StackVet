# Test names are matched to their requirement by shared words, which flags sound tests

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the second loop trial (backlog 0257, `docs/prompts/loop-2/results.md`), 13 October 2026.

`tests.name-does-not-match-requirement` says a test named for a requirement "shares no words with it". Three of the ten
findings the trial read by hand at random were this, and all three were sound tests named in other words:
`test_V7_3_1_session_unused_past_the_limit_is_refused` (V7.3.1, the inactivity timeout) and two
`test_V8_2_1_admin_page_refused_to_ordinary_user` (V8.2.1, function-level access). The finding is `info` at low
confidence, but it appeared in 48 of the 89 reports (the twenty controls of the drafts trial among them). Matching on a few synonyms per
requirement, or saying only when the test's words point at a different requirement, would cut the noise.
