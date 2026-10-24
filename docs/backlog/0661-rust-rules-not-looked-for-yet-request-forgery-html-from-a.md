# Rust rules not looked for yet: request forgery, HTML from a value, CORS with credentials, and nine more

**Status:** partly done: four rules still to teach Rust: backup-codes-stored-plain, password-expiry-enforced, model-download-not-pinned, and dynamic-code-execution

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

**`fetch-address-from-request` built, 24 October 2026** (ADR-138, Later). Its Rust query reads `reqwest`, a client's
methods, and the `http` crate's request builder, given an Axum or Actix extractor, directly or through two names; its
`argumentNamesDepth` is 2 for every language. `crates/sv-check/src/ast/fetch_tests.rs` holds eleven Rust cases, and
`examples/rust-notes/src/main.rs` a fault and its safe form. Broken on purpose: with the Rust request-input pattern
emptied, all six found cases and the example's fault went red.

**`html-from-value` built, 24 October 2026** (ADR-142, Later). Axum's `Html(…)` and Actix's HTML `.body(…)`, given
HTML built there with `format!` or `+`; a `format!` is read piece by piece, so `format!("<p>{}</p>",
html_escape::encode_text(&name))` is safe and `format!("<p>{name}</p>")` is not. Twelve Rust cases in
`crates/sv-check/src/ast/html_tests.rs`, and a fault and its safe form in `examples/rust-notes`, whose V1.2.1 now
needs attention. Broken on purpose: with the `format!` reading off, the four escaped forms and the example's safe line
went red; with the inline-name check off, `format!("<p>{n}</p>")` went unfound. A template's own `|safe` is not read.

**`cors-any-origin-with-credentials` built, 24 October 2026.** tower-http's `CorsLayer::very_permissive()`, or a
`CorsLayer` given `AllowOrigin::mirror_request()` with `allow_credentials(true)`, and actix-cors's `Cors::permissive()`,
or `allow_any_origin()` with `supports_credentials()`, in either order in one chain. tower-http's `Any` with
credentials is not reported: tower-http refuses it when the app starts, so it never serves a request. Only a call
whose own method is a CORS one is matched, so a `Router::new()…layer(…)` around the chain is not a second finding.
Nine witnesses in `crates/sv-check/src/ast/tests.rs`, a one-finding test in
`crates/sv-check/src/ast/cors_rust_tests.rs`, and a fault and its safe form in `examples/rust-notes`, whose V3.4.2 now
needs attention. Broken on purpose: with the Rust pattern emptied, the five found witnesses and the example's fault went
red; with the method-name restriction loosened, the one-finding test saw a second finding at the router's line. No
decision record: one more language for a rule that only ever reports, with nothing new in how it reads.

**`template-built-from-value` built, 24 October 2026.** Tera's `Tera::one_off`, `render_str`, and
`add_raw_template`, minijinja's `render_str`, `template_from_str`, `template_from_named_str`, and `add_template`, and
Handlebars' `render_template` and `register_template_string`, given template text not fixed in the code; where a
name comes first, the text is the second argument. `include_str!` and a file read at a path written out are the
app's own templates. Nine witnesses in `crates/sv-check/src/ast/tests.rs`, and a fault and its safe form in
`examples/rust-notes`, whose V1.3.7 now needs attention. Broken on purpose: without the second-argument setting, two
found cases went unfound and a fixed one was reported; with the Rust pattern emptied, the five found witnesses and the
example's fault went red. No decision record, as for the CORS rule.

**`insecure-random-for-code` built, 24 October 2026.** The `fastrand` crate's functions and rand's `SmallRng` used
where it is made, whose value is given a name that says it is a sign-in, reset, verification, or backup code, or made
in a function so named (the names Go's form reads). Rust's usual generators (`rand::random`, `thread_rng()`,
`StdRng`) are built for secrets and are not reported; a `SmallRng` kept in a variable is not followed, as a generator
kept in a variable is not in the other languages. Eight witnesses in `crates/sv-check/src/ast/tests.rs`, and a fault
and its safe form in `examples/rust-notes`, whose V11.5.1 now needs attention. Broken on purpose: with the Rust module
pattern emptied, the four found witnesses and the example's fault went red. No decision record, as for the CORS rule.

**`request-body-passed-whole` built, 24 October 2026** (ADR-138, Later). diesel's `values` and `set`, and sea-orm's
`from_json` and `set_from_json`, given a `Json` or `Form` extractor's whole body, directly or through a name. A new
rule option, `argumentNamesWhole` (`data/README.md`), follows a name only when the argument is that name, so a struct
or tuple built from the body's fields is the safe form it is; in JavaScript the rule now also finds the body passed
through a name. Eight Rust cases and two JavaScript ones in `crates/sv-check/src/ast/body_whole_tests.rs`, and a fault
and its safe form in `examples/rust-notes`, whose V15.3.3 now needs attention. Broken on purpose: with
`argumentNamesWhole` off, the two Rust safe forms and the JavaScript one (as JavaScript and TypeScript) were reported.

**`account-found-by-provider-email` built, 24 October 2026.** sqlx's `bind`, diesel's and sea-orm's `eq`, and an app's
own `find_user_by_email` and its like, given the email address a provider sent: the `openidconnect` crate's
`claims.email()`, or a user-info answer's `userinfo.email` or `userinfo["email"]`, directly or through a name. A query
written inside sqlx's own `query!` macro is not read (backlog 0660 reads the standard macros only), and the rule's Rust
words say so. Six witnesses in `crates/sv-check/src/ast/tests.rs`, and a fault and its safe form in
`examples/rust-notes`. The example now names `openidconnect`, so `sv` reads it as an app that signs people in through a
provider, and its snapshot gains that scope's requirements: V10.5.2 needs attention, from the fault, and eighteen more
(OAuth and OIDC, V10.x, with V6.8.x, V7.1.3, and V7.6.x) are not verified, as nothing here checks them. Broken on
purpose: with the Rust pattern emptied, the three found witnesses and the example's fault went red. No decision record,
as for the CORS rule.

**`hand-rolled-cryptography` built, 24 October 2026.** A Rust function or method whose name says it encrypts,
decrypts, or hides data, and whose body uses XOR (`^` or `^=`) itself, with the names and the XOR the other languages
read, except that `^` after a colon is a format string's centering (`{name:^8}`), not XOR. The rule's fix now names the
`aes-gcm` crate for Rust. Five Rust cases in `crates/sv-check/src/ast/hand_rolled_tests.rs`, and a fault and its safe
form in `examples/rust-notes`, whose V11.2.1 now needs attention. Broken on purpose: without the colon rule the
centering case was reported; with the Rust name pattern emptied, the found case and the example's fault went red; and
with the Rust query removed, the rules refused to load. No decision record, as for the CORS rule.
