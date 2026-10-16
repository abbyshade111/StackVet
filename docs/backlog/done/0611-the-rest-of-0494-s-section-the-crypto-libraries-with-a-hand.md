# The rest of 0494's section: the crypto libraries with a hand-rolled-crypto finding (V11.2.1), and the hard-coded algorithm names counted for crypto agility (V11.2.2)

**Status:** done, 16 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Filed by documentation building backlog 0494, 15 October 2026. Severity: low (coverage, not a fault).

0494's section of the deep review (backlog 0417) proposes two more pieces its build left out:

- **V11.2.1** (L2), industry-validated implementations: the crypto libraries and call sites (0494's inventory has
  both) shown with "is any of it written by hand instead?", and a finding-only rule for hand-rolled cryptography: an
  XOR loop over bytes in a function named `encrypt`, a `md5` or `sha1` npm package, `crypto-js`'s `AES` with a string
  key, or a function named `encrypt` or `hash` that calls no library.
- **V11.2.2** (L2), crypto agility: a count of hard-coded algorithm names and key lengths by file (`'sha256'`,
  `'aes-256-gcm'`, `rounds=12`, `2048`), shown with "swapping would mean changing each of these places; is there one
  setting instead?"

**Record.** ADR-022 Later for the pointers; the finding's record where it is built.
