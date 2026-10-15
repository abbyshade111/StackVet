# A cryptographic inventory drafted from what the code shows, and weak key generation reported

**Status:** claimed by documentation, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V11 Cryptography (V11.1.1, V11.1.2, V11.2.1, V11.2.2, V11.6.1)**

- **V11.1.1** (L2), a documented key-management policy, and **V11.1.2** (L2), a cryptographic inventory. Notes questions
  exist; partial-checks has no entry for V11.1.1 and proposes `sv` draft the inventory for V11.1.2. Route 4: the
  inventory `sv` can already assemble from what it reads: every crypto call site the weak-cipher, weak-hash, and
  key-derivation rules match in any mode (the rules' queries find all calls; today only the weak ones are reported),
  the key and certificate files the secret rules and `*.pem`/`*.key`/`*.p12` names find (names and locations, never
  contents), and the crypto libraries in the bill of materials. Shown as "what StackVet found" beside the notes
  question; the owner answers whether it is kept and where keys live. For V11.1.1 the pointer is the key-loading lines
  (`os.environ["SECRET_KEY"]`, `process.env.JWT_SECRET`, a KMS client) with "where does each of these come from, and
  who else has it?" Tier: *documented by the owner*; nothing credited.
- **V11.2.1** (L2), industry-validated implementations. No question today. Route 4: the crypto libraries from the
  bill of materials and the call sites above, with "these are the libraries doing encryption and hashing; is any of it
  written by hand instead?" Route 1, finding-only (partial-checks): an XOR loop over bytes named `encrypt`, a
  `md5`/`sha1` npm package, `crypto-js`'s own `AES` with a string key, or a function named `encrypt`/`hash` that calls no
  library. Absence credits nothing. Tier: *attested by the owner*; `sv`'s own for the finding.
- **V11.2.2** (L2), crypto agility. Design question exists. Route 4: a count of hard-coded algorithm names and key
  lengths by file (`'sha256'`, `'aes-256-gcm'`, `rounds=12`, `2048`) shown with "swapping would mean changing each of
  these N places; is there one setting instead?" Tier: *attested by the owner*. Same as partial-checks.
- **V11.6.1** (L2), approved algorithms for key generation and signing. Hand check exists. Route 1, finding-only:
  `ast.weak-key-generation`: RSA under 2048 bits (`generateKeyPair('rsa', { modulusLength: 1024 })`,
  `rsa.generate_private_key(key_size=1024)`, `openssl genrsa 1024` in scripts), DSA, ECDSA on a retired curve
  (`secp192r1`, `secp224r1`), a public exponent of 3. Absence credits nothing. Route 4: the key-generation and signing
  call sites shown with the hand check. Tier: `sv`'s own (finding-only); *checked by hand*.
