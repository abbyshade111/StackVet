# Three framework forms the code rules do not read yet: FastAPI's CORS setting, sqlx's queries, and Gin's redirect

**Status:** open

Found 24 October 2026, building backlog 0663. Every line in `data/stack-guidance.json` must carry a fault, written in
its framework, that the code rule it names reports (`crates/sv-check/tests/stack_guidance.rs`). Three lines were
dropped because the rule they named does not read that framework's way of writing the fault. Each is a gap in the
rule, not in the advice, and each line goes back into `data/stack-guidance.json` when its rule reads it.

1. **FastAPI's CORS setting.** `ast.cors-any-origin-with-credentials` reads only flask-cors in Python. Starlette's
   `CORSMiddleware` (which FastAPI uses), given `allow_origins=["*"]` and `allow_credentials=True`, sends back the
   calling site's own address when a request carries cookies, so every site can call the app as its visitor. Not
   looked for. Teach the Python query `add_middleware(CORSMiddleware, ...)` with both arguments, and the safe form
   with a list of sites.
   **Part status:** open

2. **sqlx's query functions, in Rust.** `ast.sql-built-by-hand` reads a method call such as rusqlite's
   `conn.execute(&format!(...))`, but not a function called by its path: `sqlx::query(&format!(...))`,
   `sqlx::query_as(...)`, which most Axum and Actix apps use. Teach the Rust query a scoped call whose last name is
   `query`, `query_as`, or `query_scalar`, given a `format!`.
   **Part status:** open

3. **Gin's redirect, in Go.** `ast.open-redirect` reads net/http's `http.Redirect(w, r, address, code)`, whose address
   is the third argument. Gin's `c.Redirect(code, address)` has two, so `c.Redirect(302, c.Query("to"))` is not
   found. Teach the Go query Gin's form (and Echo's `c.Redirect(code, url)`), with a fixed path as the safe form.
   **Part status:** open

Each part: a fault and its safe form in the rule's tests, the line put back in `data/stack-guidance.json` with its
fault, and the rule broken on purpose to watch the stack-guidance test go red.
