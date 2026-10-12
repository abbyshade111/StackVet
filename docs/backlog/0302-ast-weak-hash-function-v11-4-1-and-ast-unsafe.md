# ast.weak-hash-function (V11.4.1) and ast.unsafe-deserialization (V1.5.2) credit whole requirements from a deny-list

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** V11.4.1: "Verify that only approved hash functions are used for general cryptographic use cases, including digital signatures, HMAC, KDF, and random bit generation..." (MD4, RIPEMD-160, CRC used as a MAC are not looked for; "only approved" is not shown). V1.5.2: "Verify that deserialization of untrusted data enforces safe input handling, such as using an allowlist of object types ... Deserialization mechanisms that are explicitly defined as insecure must not be used with untrusted input." The second sentence is tested; the allowlist half is not.

**Where.** `data/ast-rules.json`: `ast.weak-hash-function` looks for MD5 or SHA-1 and credits V11.4.1; `ast.unsafe-deserialization` looks for pickle, Marshal, or PHP's unserialize and credits V1.5.2 (L2). Neither credit is in part (the item above explains why `clean_rules` cannot mark one).

**Evidence.** The two rules' `requirementIds` with no in-part marker.

**How it bites.** Both read as *checked* where only part of each requirement was looked at.

**Fix.** In-part marking for both, with the item above's `inPart` field; `docs/COVERAGE.md` regenerated.

**Record.** ADR-053 and ADR-054: covered by the item above's Later line.
