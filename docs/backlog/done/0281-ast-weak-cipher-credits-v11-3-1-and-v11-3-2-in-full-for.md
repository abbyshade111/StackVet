# ast.weak-cipher credits V11.3.1 and V11.3.2 in full for finding none of six named ciphers

**Status:** done, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** V11.3.1: "Verify that insecure block modes (e.g., ECB) and weak padding schemes (e.g., PKCS#1 v1.5) are not used." V11.3.2: "Verify that only approved ciphers and modes such as AES with GCM are used." The rule never looks for PKCS#1 v1.5 padding (the requirement's own example) nor for any non-approved cipher outside its list (AES-CBC without a MAC, ChaCha without Poly1305, Camellia, SEED). "Only approved are used" cannot follow from "six retired ones were not seen".

**Where.** `data/ast-rules.json` (`ast.weak-cipher`, requirementIds V11.3.1, V11.3.2; looks for ECB or DES, triple DES, RC4, RC2, Blowfish); the credit at `crates/sv-check/src/ast.rs:2030` (`clean_rules`, `Verified::new`, never `.in_part()`); V11.3.2 is `verificationClass: automatable` in `data/knowledge/applicability.json`, so it becomes *checked*.

**Evidence.** The rule has no `findingsOnly` and no in-part marker; `clean_rules` has no `in_part` path at all (`grep in_part ast.rs` finds none).

**How it bites.** A report says V11.3.2 is *checked* for an app using AES-CBC with no MAC.

**Fix.** Add an `inPart` field to the rule schema that `clean_rules` reads (`Verified::in_part`), mark this rule in part, and split V11.3.1 (finding-only until padding is looked for) from V11.3.2; a test that a clean tree credits in part, not whole.

**Record.** ADR-054 governs the AST rules and ADR-053 in-part credit: a Later line on ADR-053 (rules that read code can credit in part).
