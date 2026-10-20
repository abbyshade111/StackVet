# The validation schemas, limits, and transactions the code has, shown beside the V2 questions, and a stated limit tried once over

**Status:** claimed by claude/backlog-work, 20 October 2026

**Was queued on the coordination board** (backlog 0273) until the board was removed on 20 October 2026 (ADR-061, Later); it is now taken the normal way, by a claim.

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V2 Validation and Business Logic (V2.1.1, V2.1.2, V2.1.3, V2.2.1, V2.2.3, V2.3.2, V2.3.3)**

- **V2.1.1** (L1), **V2.1.2**, **V2.1.3** (L2): the documentation of validation rules, of which fields must agree, and
  of per-user and global limits. Notes questions exist for all three. Route 4 is the right one: a locator for
  validation schemas (zod, yup, joi, class-validator, pydantic models, marshmallow, DRF serializers, Rails
  `validates`, Laravel `$request->validate([...])`, Spring `@Valid`) listing each schema's file and its field names,
  shown with the notes question: "StackVet found 7 places where the app says what a field must look like. Are these
  where your rules are written down, and are they all?" A non-programmer can read a field list. The notes section
  stays the owner's answer; `sv`'s list is shown as "what StackVet saw", never written into the notes as if the owner
  said it (ADR-022's rule that an unmarked answer is the tool's). Tier: *documented by the owner*. For V2.1.3 the
  pointer is instead the rate-limit and quota code (see V6.1.1). The partial-checks entry for V2.1.1 proposes showing
  OpenAPI and JSON Schema files; the per-field list from code is new.
- **V2.2.1** (L1), "input is validated to enforce business or functional expectations". Hand check exists. Route 2:
  on `owned.create` (the form the manifest already gives), send a 100,000-character text in the marker field and a
  negative or enormous number in any field the form's own `type="number"` marks; the record stored as sent is a
  finding; refused is *checked in part* ("one form, two kinds of bad value"). Tier: `sv`'s own, in part. This is the
  partial-checks proposal; what I add: the schema locator above as the pointer for the hand check that covers the
  rest, and that `probe.validation-only-in-the-browser` (V2.2.2) already sends a value the browser would refuse, so
  the mechanism is there.
- **V2.2.3** (L2), combinations of related data items reasonable per the app's own rules. No code to point at: the
  rules are the app's own and nowhere `sv` can read them. Manual, with the reason written beside it; it rides on the
  V2.1.2 notes answer.
- **V2.3.2** (L2), limits implemented per the documentation. Hand check exists. Route 2, the pattern `[policy]`
  already uses for `failed-sign-ins`: an optional `[policy] limit = { name = "items per order", path = "...", form = {...},
  at-most = 10 }`; the signed-in user does one more than `at-most`; accepted is a finding citing V2.3.2, refused is
  *checked in part* ("the one limit named"). Tier: `sv`'s own, in part. Same as partial-checks; the manifest shape is
  the addition.
- **V2.3.3** (L2), transactions at the business-logic level. No question today. Route 4: a locator for transactions
  (`BEGIN`, `db.transaction(`, `with transaction.atomic()`, `prisma.$transaction`, `ActiveRecord::Base.transaction`,
  `DB::transaction`, `@Transactional`) and for handlers that write more than one table with none, shown as "these
  handlers change several records at once; these are the places the app groups changes so they all happen or none
  does" with the question "Is every multi-step change inside one of these?" Whether a handler writes two tables is a
  count of model-write calls in one function, which tree-sitter can do without flow analysis. Finding-only when the
  manifest claims `payments` and the code has no transaction anywhere. Tier: *attested by the owner*, with the pointer;
  `sv`'s own only for the finding.

**Checked against `main`, 20 October 2026** (the backlog clean-up): built: `locate.validation-rules`, `locate.transactions`, and V2.1.3 on the availability brakes (39d2c167, 15 October; ADR-022, Later). V2.2.3 is a hand check now (0506), and V2.3.3 has a design question, so both "No question today" lines above are out of date.

**The owner's choices, 20 October 2026, asked in a session.** V2.2.1: declared limits on the `owned` record, plus the
long text, stored whole a low-severity finding. V2.3.2: a `[policy] limit` key, as `failed-sign-ins` is, built next.
V2.3.3: a pointer only, no payments finding, as `docs/PARTIAL-CHECKS.md` says (single-statement writes and document
databases need no transaction).

**V2.2.1 built, 20 October 2026** (ADR-053, Later): `signed_in/input_limits.rs`, the 100,000-character text and the
ranges `limits` declares, each credited in part only as ADR-053's entry says. Left: the V2.3.2 limit and the V2.3.3
pointer to handlers changing several tables.
