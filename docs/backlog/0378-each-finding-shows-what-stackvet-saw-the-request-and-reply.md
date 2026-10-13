# Each finding shows what StackVet saw: the request and reply, the lines of code, or the tool's own words

**Status:** claimed by securevibe-trial-prep, 12 October 2026

From the owner's review of the report on 12 October 2026, after an inventory of what a run gathers against what
the report shows (`docs/website/home-page.md` is the words; the inventory is the session's page "StackVet Report
Inventory"). The owner: use "the observability features and scan findings to provide the actual evidence for what was
found where possible", and the outside tools' findings "have been the most important time and time again".
Read on the second loop trial's build `haiku-a-5` (`~/sv-loop2/runs/haiku-a-5-out` and `-tools-out`).

`Finding.evidence` already holds the ids of the exchanges a running-app check recorded (`guess-0` to `guess-6` for
unlimited failed sign-ins, `burst-1` to `burst-11` for an unlimited create rate), and `seen.json` keeps each exchange
(method, path, status, headers, body), but no report reads either: a person sees the finding's words and nothing of
what showed it. To build, under each finding, "What StackVet saw":

- **A running-app finding:** each exchange it names, as the request (method and path) and the reply (status, the
  headers that matter to the finding, a short excerpt of the body), a few at most with how many more.
- **A code finding:** the lines it is on.
- **An outside tool's finding:** the tool's own message, its rule and a link to it, and the tool's version.
- **Redaction first:** everything shown passes through `sv`'s redaction (a key shows its first four characters and its
  length), and cookies, `Set-Cookie`, `Authorization`, and form tokens (a `csrf_token` field) are masked. A reply on
  `haiku-a-5` holds a form's anti-forgery token in full; it must not reach a report. Tested by breaking it: a reply
  holding a key-like value and a token is shown masked, and the test fails when the mask is removed.
