# The routes and their guards, as the code says them, shown beside the authorization questions

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V8 Authorization (V8.1.1, V8.1.2)**

- **V8.1.1** (L1) and **V8.1.2** (L2), documented rules for function-level, data-specific, and field-level access.
  Notes questions exist. This is 0397's V8.1.1 case ("the code offered as proof showed two routes with no check at
  all"). Route 4, concretely: a locator that lists every route with the guard on it, as the code says it:
  `@login_required`, `@permission_required`, `@roles_required('admin')`, `before_action :require_admin`, `requireAuth`
  middleware in the route's chain, `[Authorize(Roles = ...)]`, `@PreAuthorize`, Laravel's `->middleware('can:...')`,
  and "none found in this file" for the rest. Shown as a table beside the notes question: "route | who may use it, as
  the code says". A non-programmer can compare "none found" with their own rules. Global middleware the locator does
  not see means "none found" is not a finding; it is the owner's reading. Field-level (V8.1.2): the serializer or
  `select`/`only` list per model, from the V15.3.1 design question's `where`. Tier: *documented by the owner*; the
  table is supporting evidence, never credit; the running probes (V8.2.1, V8.3.1) stay what settles the behavior.
