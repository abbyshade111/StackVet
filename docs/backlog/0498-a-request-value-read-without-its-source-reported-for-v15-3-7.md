# A request value read without its source, reported for V15.3.7, and the fix-within-days number counted as the owner's documentation

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V15 Secure Coding and Architecture (V15.1.1, V15.1.3, V15.2.2, V15.3.7)**

- **V15.1.1** (L1), documented remediation time frames for vulnerable components. Notes question exists, and the
  number itself already has a place: `[policy] fix-within-days`, which `sv audit` uses. Proposal, a tier decision for
  the owner: a `fix-within-days` the owner recorded through `sv review` (ADR-026) is the documentation the requirement
  asks for, and counts as *documented by the owner* for V15.1.1 without a notes section repeating it; the question
  then carries the manifest line as its pointer. Tier: *documented by the owner*. (A Later entry on ADR-022: a manifest
  number as a documented answer.) Partial-checks says ask the owner; the place exists, so make it count.
- **V15.1.3** (L2), documentation of time-consuming or resource-demanding functionality. Notes question exists.
  Route 4: the heavy features the code shows (upload handling, image and PDF libraries, export endpoints, calls to a
  model, password hashing with its cost parameter, search without a limit) listed beside the question. Same as
  partial-checks. Tier: *documented by the owner*.
- **V15.2.2** (L2), defenses against loss of availability from those features. Hand check exists. Route 4: the
  defenses the code shows, each with its line: body-size limits (`client_max_body_size`, body-parser `limit`,
  `MAX_CONTENT_LENGTH`, `DATA_UPLOAD_MAX_MEMORY_SIZE`), rate-limit libraries, a job queue (celery, bull, sidekiq),
  timeouts on outgoing calls (`timeout=` on `requests`, axios `timeout`), shown as "these are the brakes StackVet
  found" beside the heavy features of V15.1.3. The running checks that exist (`probe.create-rate-unlimited` V2.4.1,
  `probe.oversized-file-accepted` V5.2.1) are shown under it as related. A new running observation, "the copy
  StackVet ran accepted a 20 MB request body", is worth showing as supporting but not as a finding: the requirement is
  about documented strategies, and a limit may sit at the proxy. Tier: *checked by hand*, with the pointer.
- **V15.3.7** (L2), defenses against HTTP parameter pollution, "particularly if the application framework makes no
  distinction about the source of request parameters". No question today; 0147 found the running probe's result
  "means nothing without knowing the app", and I agree. Route 1, finding-only and new: the requirement's own clause
  names something the code shows plainly, a read that merges sources: PHP `$_REQUEST`, Express's deprecated
  `req.param()`, Flask's `request.values`, ASP.NET's `Request["x"]`, Rails' `params` where the same key is read for an
  access decision, Go's `r.FormValue` (query and body merged). `ast.request-parameter-source-merged` citing V15.3.7,
  finding-only, with the line shown: "this line reads a value without saying whether it came from the address or the
  form". Absence credits nothing. Tier: `sv`'s own (finding-only).
