# Rust rules not looked for yet: request forgery, HTML from a value, CORS with credentials, and nine more

**Status:** open

Found 24 October 2026, building backlog 0649. Of the 42 code rules, 18 read Rust and 24 say why not
(`nothingToFind`). Three of those are really nothing to find in Rust (a model loaded with pickle; a switch that turns
request-forgery protection off, which Rust's frameworks do not have; Python's debug switches). The other twelve are
gaps: Rust can have the fault, and it is not looked for yet.

- **`fetch-address-from-request`**: an outgoing request whose address comes from the incoming one (server-side request
  forgery), with `reqwest` or `hyper`. Common, and serious.
- **`html-from-value`**: `Html(format!(…))` and a template's `|safe`; needs backlog 0660 first.
- **`cors-any-origin-with-credentials`**: tower-http's `CorsLayer` or Actix's `Cors` letting every site in with
  credentials.
- **`template-built-from-value`**: a template made from text at run time (Tera's `one_off`, minijinja's
  `render_str`).
- **`insecure-random-for-code`**: a code or token made with `SmallRng` or `fastrand`.
- **`request-body-passed-whole`**, **`account-found-by-provider-email`**, **`hand-rolled-cryptography`**,
  **`backup-codes-stored-plain`**, **`password-expiry-enforced`**, **`model-download-not-pinned`** (the `hf-hub`
  crate), and **`dynamic-code-execution`** (an embedded interpreter such as rhai, which is rare).

To do: teach each a Rust query, with a fault and its safe form added to `examples/rust-notes` and its witnesses, worst
first, request forgery first. Each becomes a part of this item as it is taken.

**Order:** after 0660.
