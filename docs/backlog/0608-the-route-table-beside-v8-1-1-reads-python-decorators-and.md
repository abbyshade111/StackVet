# The route table beside V8.1.1 reads Python decorators and Express calls only, and V8.1.2's field-level lists are not shown

**Status:** claimed by honesty, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Filed by documentation building backlog 0493, 15 October 2026. Severity: low (coverage of a pointer, not a fault).

**What 0493 built.** `crates/sv-check/src/route_guards.rs` lists the routes declared with Python decorators (Flask,
FastAPI) and Express-style `app.get('/path', ...)` calls, with what is written on each, beside V8.1.1 and V8.1.2.

**Not read yet.** Each would want its own reader and a test on a real app of that framework:
- Django's `urls.py` with its views' `@login_required` and `LoginRequiredMixin`;
- Rails' `routes.rb` with a controller's `before_action`;
- ASP.NET's `[Authorize(Roles = ...)]` and minimal-API `.RequireAuthorization()`;
- Spring's `@GetMapping` with `@PreAuthorize`;
- Laravel's `Route::get(...)->middleware('can:...')`;
- Go's `http.HandleFunc` and its routers.

**Not built.** 0493's field-level half for V8.1.2: the serializer or `select`/`only` list per model, from the V15.3.1
design question's `where`. And a check applied to every route at once (`app.use(requireAuth)`, `before_request`) is
named in the table's sentence but not shown as its own row.

**Record.** ADR-022 Later, as 0493's.
