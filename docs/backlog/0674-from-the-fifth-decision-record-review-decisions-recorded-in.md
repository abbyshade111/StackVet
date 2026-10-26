# From the fifth decision-record review: decisions recorded in a backlog plan only (0625 findings-only checks, Rust added to code rules)

**Status:** claimed by claude/0674, 26 October 2026

Found by the fifth weekly review (0673), 26 October 2026. For the owner to decide; nothing here was changed.

Backlog 0625's plan (24 October) says its Phase A and E checks, findings only, get "no decision record". Six such checks have merged on that word (#2321, #2323, #2344, #2345, #2346, #2347: default credentials, LaTeX shell escape, weak key exchange, one-time-code time, email sign-in, DOM clobbering), and #2321 added a "findings-only" flag to `tools/coverage.py` and `data/secret-rules.json`. Eight of the eleven Rust-for-code-rules pull requests (0661: #2286, #2288, #2291, #2298, #2301, #2304, #2309, #2312) and #2311 (three rules read FastAPI CORS, sqlx, Gin) say the same. Earlier findings-only checks (ADR-065 to 075, 112, 123) each got a record. The question: does a plan in a backlog item stand for the record, or should the policy "a rule that only reports needs no record" be one short ADR, so that the reasons are in the place the rules say to look? ADR-018's Later of 26 October counts the rules now.

Small, unrecorded and cheap to undo (listed so a later review does not find them again): #2346 (`config.email-code-sign-in` reads only the owner's declared `email-code`, because reading the code from the signed-in suite turned 34 tests red); #2238 (`{seed:NAME}` is a blank to the credential rule); #2251 (`docker exec rm -f --` in place of `sh -c`).

**The owner's choice, 26 October 2026,** asked with a recommendation: "One short ADR (Recommended)". ADR-155 will state
the policy that a check which only reports a finding, crediting nothing, needs no record of its own, say when one still
does, and list the pull requests that relied on backlog 0625's plan and the Rust rules' for it.
