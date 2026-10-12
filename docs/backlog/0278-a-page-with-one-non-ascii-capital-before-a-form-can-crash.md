# A page with one non-ASCII capital before a form can crash the whole running-app stage (tags() lowers with to_lowercase)

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: code quality. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** `tags` lowers the page with `str::to_lowercase`, finds positions in the lowered copy, and slices the original `body` at those positions. Some characters change byte length when lowered (`İ` U+0130 goes from 2 bytes to 3; the Kelvin sign U+212A from 3 to 1; `Ω` U+2126 from 3 to 2), so a position in `lower` is not the same position in `body`.

**Where.** `crates/sv-check/src/signed_in/mod.rs:383-400` (`tags`), called from `mod.rs:363` and `:370` (the CSRF token), `passwords.rs:682`, `uploads.rs:1267`.

**Evidence.** `let lower = body.to_lowercase(); … let end = lower[i..].find('>').map_or(body.len(), |e| i + e); body[i..end].to_owned()`. The same crate already knows the rule: `ast/html.rs:180-184` uses `to_ascii_lowercase` and its comment says full lowering can move every position after it; `ai.rs:606-614` guards the same pattern with `get`.

**How it bites.** A sign-in page whose heading holds a capital dotted İ (a Turkish page is enough) before its `<form>`: the slice lands past the end or off a character boundary and panics, or extracts the wrong text so the CSRF token is not found and sign-in fails quietly. The panic is caught by `assemble::guarded("running_app", …)`, which is honest, but that guard covers the entire `--run` stage, so every signed-in check, probe, OIDC check, and AI check is lost and the report says the stage crashed. The fake app's bodies are ASCII, so no test exercises it.

**Fix.** `let lower = body.to_ascii_lowercase();` (one line), and a fake-app page with `İ` before its form as the witness, so the test fails before the fix and passes after.

**Record.** No record governs `signed_in/mod.rs` by name; say so in the pull request. Follow CLAUDE.md's rule: break it first (add the witness page), watch the test go red, then fix.
