# The personal-data columns the schema shows, and what a signed-in token carries, beside the classification question

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V14 Data Protection (V14.1.1, V14.1.2, V14.2.4)**

- **V14.1.1** (L2), sensitive data identified and classified, and **V14.1.2** (L2), protection requirements per level
  documented. Hand check (V14.1.1) and notes question (V14.1.2) exist; `stackvet.toml` already asks the data
  categories that decide the level. Route 4, the one a non-programmer can read best: the columns and fields that look
  personal, by table, from migrations, models, and schemas (`email`, `phone`, `dob`, `birth`, `address`, `ssn`,
  `passport`, `card`, `iban`, `diagnosis`, `health`, `salary`, and the categories the manifest names), shown as
  "these look like personal data; your classification should name each, and say which level it gets". Also the
  partial-checks V14.1.1 idea: decode a token the app hands a signed-in user (a JWT, a base64 cookie) and show which of
  those fields it carries in plain sight, since the requirement names "the plaintext payload inside a JWT" (the run
  already holds the token; show field names only, never values). Tier: *documented by the owner*; the list is an
  observation.
- **V14.2.4** (L2), controls implemented as the data-protection document says. Hand check exists; partial-checks says
  no useful check, and I agree: it compares behavior with the owner's own document. What can help: group under it the
  results of the checks that are its measurable parts (V14.2.1 password in URL, V14.3.2 private page cached, V16.2.5
  sensitive data in logs, V14.3.1 cleared at sign-out), as the threat model groups requirements, so the owner reads one
  place. No new check. Tier: *checked by hand*.
